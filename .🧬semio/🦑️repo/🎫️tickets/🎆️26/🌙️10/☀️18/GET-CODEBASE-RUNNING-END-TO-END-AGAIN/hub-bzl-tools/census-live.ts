// 🌎️ Hub census on the LIVE Bazel adapter: static graph + hub Cargo projects inferred like cargoTargets/componentBuildTargets; read-only, one process, no bun repo.
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join, dirname } from "node:path";
const ROOT = "/Users/ueli/Documents/semio", C = join(import.meta.dir, "..");
const OUT = join(C, "🗑️generated", "hub-bzl");
const L = join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const api: any = await import(join(L, "🏗️build/🔧️tools/🏗️bazel/📜️script.ts"));
const strip = (s: string) => s.replace(/️/g, "");
const HUB = strip("🌎️hub/");
const graph = JSON.parse(readFileSync(join(OUT, "graph.json"), "utf8"));
const git = Bun.spawnSync(["git", "-c", "core.quotepath=false", "ls-files", "-z"], { cwd: ROOT });
const tracked: string[] = new TextDecoder().decode(git.stdout).split("\0").filter(Boolean);
const toml = (f: string) => readFileSync(join(ROOT, f), "utf8");
const cargo = tracked.filter((f) => strip(f).startsWith(HUB) && f.endsWith("/Cargo.toml"));
let inferredProjects = 0, inferredTargets = 0;
const nativeTarget = (root: string, kind: string) => ({
  execution: { kind: "command", command: `bun ${JSON.stringify("⚡️caching/🦀️cargo/📜️script.ts")} native cargo ${kind} --manifest ${JSON.stringify(root + "/Cargo.toml")}`, cwd: "." },
  cache: true, outputs: kind === "build" ? [`{projectRoot}/dist/build/{options.env.SEMIO_BUILD_MODE}`] : [], parallelism: kind === "build" ? undefined : false,
  inputs: [kind === "test" ? "nativeTestSources" : "nativeSources", kind === "test" ? "^nativeTestSources" : "^nativeSources"],
  metadata: { semio: { execution: { version: 1, kind: "native", scope: kind === "test" ? "test" : "source" }, cache: { version: 1, class: "deterministic", reason: "pure-function-of-inputs" } } },
});
const byRoot = new Map<string, any>(Object.values<any>(graph.projects).map((p) => [p.root, p]));
for (const f of cargo) {
  const text = toml(f);
  if (!/^\[package\]/m.test(text)) continue;
  const root = dirname(f), name = text.match(/^name\s*=\s*"([^"]+)"/m)![1];
  let project = byRoot.get(root);
  if (!project) { project = { name, root, targets: {} }; graph.projects[name] = project; byRoot.set(root, project); inferredProjects++; }
  for (const kind of ["build", "check", "test"]) if (!project.targets[kind]) { project.targets[kind] = nativeTarget(root, kind); inferredTargets++; }
  const component = /\[package\.metadata\.component\][\s\S]*?\bpackage\s*=/.test(text) && /component-kind\s*=\s*"(plugin|extension)"/.test(text);
  if (component && !project.targets.component) {
    project.targets.component = { execution: { kind: "command", command: `bun "🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts" component --manifest "${root}/Cargo.toml"`, cwd: "." }, cache: false,
      outputs: [`{workspaceRoot}/${strip("dist")}/component-model/${name}`], dependsOn: ["@semio-tech/os-plugin-describe-rs:build", "@semio-tech/framework-plugin-web:support", "workspace:deps-wasm-opt"],
      metadata: { semio: { orchestrator: true, cache: { version: 1, class: "driver-owned", reason: "freshness-owned-by-driver" } } } };
    inferredTargets++;
  }
}
const names = new Set<string>();
for (const p of Object.values<any>(graph.projects)) for (const t of Object.keys(p.targets)) names.add(p.name + ":" + t);
for (const p of Object.values<any>(graph.projects)) for (const t of Object.values<any>(p.targets)) {
  if (!Array.isArray(t.dependsOn)) continue;
  t.dependsOn = t.dependsOn.filter((d: any) => { const s = typeof d === "string" ? d : d.target, op = s.replace(/^\^/, ""); if (!op.includes(":")) return true; const [pn, tn] = op.split(":"); return names.has(pn + ":" + tn); });
}
for (const p of Object.values<any>(graph.projects)) if (strip(p.root).startsWith(HUB)) for (const t of Object.values<any>(p.targets)) if (t.cache !== false && t.inputs === undefined) t.inputs = ["default"];
const hubTasks = [...names].filter((n) => strip(graph.projects[Object.keys(graph.projects).find((k) => n.startsWith(k + ":"))!]?.root ?? "").startsWith(HUB));
const hubSet = new Set<string>();
for (const p of Object.values<any>(graph.projects)) if (strip(p.root).startsWith(HUB)) for (const t of Object.keys(p.targets)) hubSet.add(p.name + ":" + t);
const plan = api.planBazelTasksV1(graph, { tasks: [...hubSet], args: [] });
const emitted = api.bazelActionsV1(graph, plan, false);
const index = api.bazelSourceIndexV1(tracked.filter((f) => !f.startsWith(".🧬semio")));
const rows: any[] = [];
for (const a of emitted.values()) {
  const task = plan.tasks[a.id]; if (!hubSet.has(a.id)) continue;
  const cov = api.bazelTaskCoverageV1(task, a.cache);
  let files = -1;
  try { const sel = index.selectAll(api.bazelTaskInputsV1(graph, task, "ignore")); files = sel.marks.reduce((x: number, y: number) => x + y, 0) + sel.extra.length; } catch (e) { files = -2; }
  rows.push({ id: a.id, project: task.project, target: task.target, root: strip(task.root), family: cov.family, disposition: cov.disposition, kind: a.kind, cache: a.cache.cache, reason: a.cache.reason ?? null, exclusive: !!a.exclusive, files, outputs: task.definition.outputs ?? null, hub: true });
}

const closures: any[] = [];
for (const id of hubSet) {
  const one = api.planBazelTasksV1(graph, { tasks: [id], args: [] }), acts = api.bazelActionsV1(graph, one, false);
  const ids = Object.keys(one.tasks);
  const uncached = [...acts.values()].filter((a: any) => !a.cache.cache && a.kind !== "launch" && a.id !== id).map((a: any) => a.id + "|" + a.cache.reason);
  closures.push({ id, tasks: ids.length, uncachedAncestors: uncached });
}
writeFileSync(join(OUT, "closures.json"), JSON.stringify(closures));
mkdirSync(OUT, { recursive: true });
writeFileSync(join(OUT, "census-live.json"), JSON.stringify(rows));
const count = (f: (r: any) => string) => { const m: Record<string, number> = {}; for (const r of rows) m[f(r)] = (m[f(r)] ?? 0) + 1; return Object.fromEntries(Object.entries(m).sort()); };
console.log(JSON.stringify({ inferredProjects, inferredTargets, hubTasks: rows.length, planTasks: Object.keys(plan.tasks).length,
  byFamily: count((r) => r.family), byDisposition: count((r) => r.disposition), byKind: count((r) => r.kind), byReason: count((r) => String(r.reason)), byFamilyDisp: count((r) => r.family + "/" + r.disposition) }, null, 1));
