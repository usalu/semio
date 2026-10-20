/** 📊 Counts how every task of the static whole-graph reconstruction executes under Bazel before and after the owner decisions: the universe is the reconstruction of `bzl-coverage-tools/graph.ts` (authored targets plus the synthesised playground targets), every authored target is resolved through the cache policy module and the manifests of the chosen root, the cache decision and the neutral disposition follow `bazelCacheDecisionV1` and `taskDispositionV1`.
 *
 *   bun census.ts <before|after> <out.json>     # before: this checkout and its policy; after: the overlay /private/tmp/ucov and the staged policy
 */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, normalize } from "node:path";
import { loadAt } from "./policy-at.ts";

const WS = "/Users/ueli/Documents/semio", G = join(import.meta.dir, "../🗑️generated/bzl-nocache"), mode = process.argv[2] as "before" | "after", out = process.argv[3]!;
const root = WS, OVERLAY = process.env.SEMIO_OVERLAY;
const GG = OVERLAY ? join(G, "../bzl-nocache-after") : G;
const graph = JSON.parse(readFileSync(join(GG, "graph.json"), "utf8"));
const policy = await loadAt(root);
const coverage = await import(join(G, "coverage-ov/coverage.ts"));
const contracts: Record<string, any> = JSON.parse(readFileSync(`${WS}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, "utf8")).generatorContracts;
const injected = new Map<string, string[]>();
for (const c of Object.values<any>(contracts)) { if (c.ownership !== "owned") continue; for (const o of c.outputRoots) { const t = o.producer?.target ?? c.target; (injected.get(t) ?? injected.set(t, []).get(t)!).push(`{workspaceRoot}/${o.path}`); } }
const scripts = new Map<string, string | undefined>();
const scriptOf = (cwd: string, command: string | undefined): string | undefined => {
  const m = command && /^bun\s+"?(\.{0,2}\/?[^"\s]*📜️script\.ts)"?/u.exec(command);
  if (!m) return undefined;
  const path = normalize(join(WS, cwd, m[1]!));
  if (!scripts.has(path)) scripts.set(path, existsSync(path) ? readFileSync(path, "utf8").slice(0, 400000) : undefined);
  return scripts.get(path);
};
type Row = { project: string; decision: string | null; outputsDeclared: boolean; manifest: string | null; command: string | null; id: string; target: string; family: string; disposition: string; reason: string | null; origin: string; declaredReason: boolean };
const rows: Row[] = [];
for (const project of Object.values<any>(graph.projects)) {
  const manifestRel = join(project.root === "." ? "" : project.root, "📋️project.json"), manifestPath = OVERLAY && existsSync(join(OVERLAY, manifestRel)) ? join(OVERLAY, manifestRel) : join(root, manifestRel);
  const manifest = existsSync(manifestPath) ? JSON.parse(readFileSync(manifestPath, "utf8")) : { targets: {} };
  for (const [name, def] of Object.entries<any>(project.targets)) {
    const authored = manifest.targets?.[name], id = `${project.name}:${name}`;
    let cache: boolean, continuous: boolean, decision: any, outputs: any, origin: string;
    if (authored) {
      const base = { ...(policy.POLICY.targetDefaults?.[name] ?? {}), ...authored, ...(authored.cache === false && authored.inputs === undefined ? { inputs: ["default"] } : {}), options: authored.options ?? {} };
      if (typeof authored.command === "string" && !base.options.command) base.options = { ...base.options, command: authored.command };
      const inj = injected.get(id); if (inj?.length) base.outputs = inj;
      let resolved = policy.module.targetPolicy(name, structuredClone(base), policy.POLICY);
      if (policy.module.cachedOutputsAdmittedV1) resolved = policy.module.cachedOutputsAdmittedV1(resolved);
      cache = resolved.cache === true; continuous = resolved.continuous === true; decision = resolved.metadata?.semio?.cache; outputs = resolved.outputs; origin = "authored";
    } else { cache = def.cache === true; continuous = def.continuous === true; decision = def.metadata?.semio?.cache; outputs = def.outputs; origin = "synthesised"; }
    const noop = def.execution.kind === "noop", klass = decision?.class, forbidden = klass !== undefined && klass !== "deterministic";
    const first = def.execution.command ?? (typeof def.execution.commands?.[0] === "string" ? def.execution.commands[0] : def.execution.commands?.[0]?.command);
    let result: { cache: true } | { cache: false; reason: string };
    if (continuous) result = { cache: false, reason: "continuous" };
    else if (noop) result = { cache: true };
    else if (forbidden) result = { cache: false, reason: klass };
    else if (!cache) result = { cache: false, reason: "declared" };
    else if (klass !== "deterministic" && def.cache !== true && !cache) result = { cache: false, reason: "undeclared" };
    else if (!Array.isArray(outputs)) result = { cache: false, reason: "unbounded" };
    else result = { cache: true };
    const input = { project: project.name, target: name, root: project.root, kind: noop ? "noop" : "command", ...(first !== undefined ? { command: first } : {}), ...(scriptOf(def.execution.cwd ?? project.root, first) !== undefined ? { script: scriptOf(def.execution.cwd ?? project.root, first) } : {}), ...(Array.isArray(outputs) ? { outputs } : {}), cache: result };
    const cov = coverage.taskCoverageV1(input);
    rows.push({ project: project.name, decision: decision ? `${decision.class}/${decision.reason}` : null, outputsDeclared: Array.isArray(outputs), manifest: authored ? manifestPath : null, command: first ?? null, id, target: name, family: cov.family, disposition: cov.disposition, reason: result.cache ? null : (result as any).reason, origin, declaredReason: !result.cache && (result as any).reason !== "declared" });
  }
}
const count = (f: (r: Row) => string) => { const m: Record<string, number> = {}; for (const r of rows) m[f(r)] = (m[f(r)] ?? 0) + 1; return Object.fromEntries(Object.entries(m).sort()); };
const report = { mode, tasks: rows.length, byDisposition: count((r) => r.disposition), byReason: count((r) => r.reason ?? "cached"), nonRust: count((r) => ["cargo", "wasm", "aggregate"].includes(r.family) ? "rust-or-aggregate" : r.disposition + (r.reason ? ":" + r.reason : "")), byFamilyDisposition: count((r) => `${r.family} ${r.disposition}`), declaredByFamily: count((r) => r.reason === "declared" ? r.family : "-") };
mkdirSync(dirname(out), { recursive: true });
writeFileSync(out, JSON.stringify({ report, rows }));
console.log(JSON.stringify(report, null, 1));
