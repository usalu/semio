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
