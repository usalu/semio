// 🧪️ Test-only loader: redirects the managed graph storage to a scratch directory and optionally applies a staged ops.json in memory, so the real inference modules run outside the repo CLI without touching the live tree. Used by the main process and by every farm worker of the harness.
import { readFileSync, mkdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { plugin } from "bun";

export const WORKSPACE = process.env.GP_WORKSPACE ?? "/private/tmp/graph-parallel/ws";
export const LIBRARY = join(WORKSPACE, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
export const SCRATCH = process.env.GP_SCRATCH ?? "/private/tmp/graph-parallel/run";

const applyOps = (text, ops, path) => {
  let out = text;
  for (const { anchor, replacement } of ops) {
    if (out.includes(replacement) && !out.includes(anchor)) continue;
    if (out.split(anchor).length !== 2) throw Error("anchor drift in " + path + ": " + anchor.slice(0, 80));
    out = out.replace(anchor, () => replacement);
  }
  return out;
};

export function installVariant() {
  if (globalThis.__gpVariant) return;
  globalThis.__gpDir = name => { const path = join(SCRATCH, name); mkdirSync(path, { recursive: true }); return path; };
  globalThis.__gpVariant = true;
  const variant = process.env.GP_VARIANT && process.env.GP_VARIANT !== "live" ? JSON.parse(readFileSync(process.env.GP_VARIANT, "utf8")) : { edits: [], newFiles: [] };
  const stagedRoot = process.env.GP_VARIANT && process.env.GP_VARIANT !== "live" ? dirname(dirname(process.env.GP_VARIANT)) : "";
  const editsByFile = new Map();
  for (const op of variant.edits) { const abs = join(WORKSPACE, op.file); (editsByFile.get(abs) ?? editsByFile.set(abs, []).get(abs)).push(op); }
  const stagedByLive = new Map(variant.newFiles.map(item => [join(WORKSPACE, item.file), resolve(stagedRoot, item.from)]));
  const liveByStaged = new Map([...stagedByLive].map(([live, staged]) => [staged, live]));
  const dir = name => { const path = join(SCRATCH, name); mkdirSync(path, { recursive: true }); return path; };
  const q = JSON.stringify, scratch = name => `(process.getBuiltinModule("fs").mkdirSync(${q(join(SCRATCH, name))},{recursive:true}),${q(join(SCRATCH, name))})`;
  plugin({
    name: "graph-parallel-variant",
    setup(build) {
      build.onLoad({ filter: /\.mjs$/u }, async (args) => {
        const path = args.path, livePath = liveByStaged.get(path) ?? path;
        if (!livePath.startsWith(LIBRARY)) return { contents: await Bun.file(path).text(), loader: "js" };
        const ops = editsByFile.get(livePath);
        const isStaged = livePath !== path;
        let text = await Bun.file(path).text();
        if (ops) text = applyOps(text, ops, livePath);
        if (process.env.GP_LEDGER && /\/🏗️build\/📥️inputs\/💾️storage\/🟨️\.mjs$/u.test(livePath)) {
          if (text.includes("batches=new AsyncLocalStorage()")) {
            text = text.replace("if(!scope){write();return true;}", "if(!scope){const accounting=globalThis.__gpAccounting;return accounting?accounting.publish(plan,write,batches.getStore()!==undefined):(write(),true);}");
            text = text.replace("export function adoptBuildGraphWorkerLeavesV1(workspace,leaves){", "export function adoptBuildGraphWorkerLeavesV1(workspace,leaves){if(globalThis.__gpAccounting)return globalThis.__gpAccounting.adopt(leaves,name=>rmSync(join(globalThis.__gpInference,name),{force:true}));");
            text = text.replace("export function grantBuildGraphWorkerBytesV1(workspace,bytes){", "export function grantBuildGraphWorkerBytesV1(workspace,bytes){if(!writers.getStore())return globalThis.__gpAccounting?globalThis.__gpAccounting.grant(bytes):()=>{};");
            text = text.replace("export function buildGraphWriterPresenceV1(){return writers.getStore()?.presence??workerWriters.getStore()?.presence;}", "export function buildGraphWriterPresenceV1(){return writers.getStore()?.presence??workerWriters.getStore()?.presence??globalThis.__gpPresence;}");
          } else text = text.replace("if(!scope){write();return true;}", "if(!scope&&globalThis.__gpPublish)return globalThis.__gpPublish(path,plan,write);if(!scope){write();return true;}");
        }
        if (/\/📥️inference\/📍️coordinates\/🟨️\.mjs$/u.test(livePath)) text = text.replace('return repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"inference");', `return ${scratch("inference")};`);
        if (/\/🏗️build\/📥️inputs\/🟨️\.mjs$/u.test(livePath)) text = text.replace(/repositoryBuildGraphWriterDirectoryV1\(workspaceRoot,\s*([^)]+)\)/gu, "globalThis.__gpDir($1)");
        if (process.env.GP_TOLERANT && /\/📥️inference\/📦️project-nodes\/🟨️\.mjs$/u.test(livePath)) text = text.replace('catch (error) { if (error?.code !== "BUILD_SOURCE_UNPARSEABLE") throw error; return { unavailable: unavailableProjectV1(root, name, workspaceRoot, error) }; }', 'catch (error) { if (error?.code !== "BUILD_SOURCE_UNPARSEABLE") { if (process.env.GP_TOLERANT) return { unavailable: { root, project: name, source: "harness", sha256: "0".repeat(64), reason: String(error?.message).slice(0, 2048) } }; throw error; } return { unavailable: unavailableProjectV1(root, name, workspaceRoot, error) }; }');
        if (/\/📥️inference\/📦️project-nodes\/🟨️\.mjs$/u.test(livePath)) text = text.replace('import { inferenceInventoryReferencesHoldV1 } from "../../../../🏗️build/👁️observation/📥️reuse/🟨️.mjs";', "const inferenceInventoryReferencesHoldV1=async()=>true;");
        text = text.replace(/(\bfrom\s*|\bimport\s*\(\s*|\bimport\s+)(["'])(\.{1,2}\/[^"']+)\2/gu, (whole, head, quote, spec) => {
          const target = resolve(dirname(livePath), spec), mapped = stagedByLive.get(target);
          return mapped ? head + quote + mapped + quote : isStaged ? head + quote + target + quote : whole;
        });
        return { contents: text, loader: "js" };
      });
    },
  });
  return variant;
}

/** 🧪️ GP_LEDGER=1: the real optional ledger (SQLite gate + intent + clean, synchronous=FULL) and the real in-memory quota on the scratch image, so the harness pays what a graph writer pays for every optional leaf. */
export async function installLedger() {
  if (!process.env.GP_LEDGER || globalThis.__gpPublish) return;
  const base = join(LIBRARY, "🏗️build/📥️inputs/💾️storage");
  const { openBuildGraphOptionalLedgerV1 } = await import(join(base, "📒️ledger/🟨️.mjs"));
  const { createBuildGraphOptionalQuotaV1 } = await import(join(base, "🟨️.mjs"));
  const root = SCRATCH, big = 6 * 2 ** 30;
  mkdirSync(join(root, "inference"), { recursive: true });
  const control = { signal: new AbortController().signal, remainingMilliseconds: () => null, publish: () => {}, yieldContinuation: async () => {} };
  const image = { root, maximumPhysicalBytes: big, filesystemBytes: big, recheck: async () => {}, port: { observe: async () => ({ root, maximumPhysicalBytes: big }) } };
  const ledger = await openBuildGraphOptionalLedgerV1({ image, control, leaseDirectory: join(root, "lease"), maximumBytes: 2 ** 30, minimumAvailableBytes: 1 });
  const quota = createBuildGraphOptionalQuotaV1({ maximumBytes: 2 ** 30, usedBytes: ledger.observe().usedBytes, availableBytes: big, minimumAvailableBytes: 1 });
  const stats = globalThis.__gpLedgerStats = { publishes: 0, declined: 0, wallMs: 0 };
  const { createOptionalAccountingV1 } = await import(join(base, "🏊️leaves/🟨️.mjs")).catch(() => ({}));
  if (createOptionalAccountingV1) {
    const counted = { publish: (plan, write) => { const started = performance.now(); try { const accepted = ledger.publish(plan, write); if (accepted) stats.publishes++; else stats.declined++; return accepted; } finally { stats.wallMs += performance.now() - started; } } };
    globalThis.__gpAccounting = createOptionalAccountingV1({ ledger: counted, quota });
    globalThis.__gpPresence = new Set();
    globalThis.__gpInference = join(root, "inference");
    return;
  }
  globalThis.__gpLedger = { ledger, quota };
  globalThis.__gpPublish = (path, plan, write) => {
    const reservation = quota.reserve(plan);
    if (!reservation) { stats.declined++; return false; }
    const started = performance.now();
    try { if (!ledger.publish(plan, write)) { stats.declined++; return false; } reservation.commit(); stats.publishes++; return true; } finally { reservation.release(); stats.wallMs += performance.now() - started; }
  };
}
