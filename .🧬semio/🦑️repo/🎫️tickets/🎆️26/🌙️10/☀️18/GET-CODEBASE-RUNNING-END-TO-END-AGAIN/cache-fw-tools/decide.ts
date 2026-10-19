import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { ROOT, TICKET, projectFiles, readJson } from "./lib.ts";
import { apply, parse, prop, setProps, type Edit, type Node } from "./jsonspan.ts";
import { MANUAL } from "./manual.ts";

const APPLY = process.argv.includes("--apply");
const POLICY = JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"), "utf8"));
const census = JSON.parse(readFileSync(join(TICKET, "🗑️generated/cache-fw-census-before.json"), "utf8"));
const analysis: any[] = JSON.parse(readFileSync(join(TICKET, "🗑️generated/cache-fw-analysis.json"), "utf8"));
const taxonomy = JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8"));
const contractTargets = new Set<string>();
for (const g of Object.values<any>(taxonomy.generatorContracts ?? {})) { if (g.target) contractTargets.add(g.target); if (g.checkTarget) contractTargets.add(g.checkTarget); }

const classOf = new Map<string, string>();
for (const t of census.targets) if (t.owner === "🧰️framework") classOf.set(t.file + "|" + t.name, t.classProposal);
const byName = new Map(analysis.map((a) => [a.name, a]));

const TEST_ENV = [{ env: "SEMIO_TEST_LEVEL" }, { env: "SEMIO_TEST_BUDGET_MS" }, { env: "SEMIO_BUILD_BUDGET_MS" }];
const CARGO_FILES = ["{workspaceRoot}/Cargo.toml", "{workspaceRoot}/Cargo.lock", "{workspaceRoot}/.cargo/config.toml", "{workspaceRoot}/rust-toolchain.toml", "{workspaceRoot}/rustfmt.toml"];
const GO_FILES = ["{workspaceRoot}/go.work", "{workspaceRoot}/go.work.sum"];
const PRODUCTION = /^(build|wasm|native-build|package|extension-package)(-|$)/;
const OUTPUTLESS = /^(test|check|lint|typecheck|format-check|verify|stdio|conformance|preview|boundaries|info|canonical|contract|graph-check|policy-check|artifact-check)(-|$)|(-check|-contract|-source|-native|-oracle|-laws|-ownership)$/;

const SKIP_CLASSES = new Set(["continuous", "side-effecting", "test-against-live", "diagnostic-uncached", "aggregate"]);
const log: any = { changed: [], keptOff: [], skipped: [], needsOutputs: [], unchangedCached: [], projects: [] };

const moduleRoot = (p: string) => {
  const s = p.split("/");
  const i = s.indexOf("🔨️modules");
  if (i >= 0 && i + 1 < s.length) return s.slice(0, i + 2).join("/");
  if (s[1] === "🛍️products" && s.length >= 3) return s.slice(0, 3).join("/");
  return s.slice(0, 2).join("/");
};

function devDependencyRoots(cargoRoot: string): string[] {
  const ct = join(ROOT, cargoRoot, "Cargo.toml");
  if (!existsSync(ct)) return [];
  const out: string[] = [];
  let section = "";
  for (const line of readFileSync(ct, "utf8").split("\n")) {
    const h = line.match(/^\s*\[([^\]]+)\]/);
    if (h) { section = h[1]!; continue; }
    if (!/dev-dependencies/.test(section)) continue;
    const pm = line.match(/path\s*=\s*"([^"]+)"/);
    if (pm) out.push(resolve("/", join(cargoRoot, pm[1]!)).slice(1));
  }
  return out;
}

const perFile: Record<string, { text: string; edits: Edit[] }> = {};
const summary: any = { files: 0 };

for (const file of projectFiles) {
  const text = readFileSync(join(ROOT, file), "utf8");
  const json = JSON.parse(text);
  if (!Object.keys(json.targets ?? {}).length) continue;
  const root = dirname(file);
  const ast = parse(text);
  const targetsNode = prop(ast, "targets")!.value;
  const nativeRoot = json.metadata?.nativeRoot ?? root;
  const nativeProject = existsSync(join(ROOT, nativeRoot, "Cargo.toml"));
  const goProject = existsSync(join(ROOT, root, "go.mod"));
  const artifactTs = json.tags?.includes("role:artifact") && json.tags?.includes("language:typescript");
  const a = byName.get(json.name);
  const extrasDirs = new Set<string>();
  for (const e of a?.extras ?? []) extrasDirs.add(moduleRoot(e));
  if (nativeProject) for (const d of devDependencyRoots(nativeRoot)) extrasDirs.add(d);
  if (root.startsWith("🧰️framework/🛍️products/📓️print/")) extrasDirs.add("🧰️framework/🛍️products/📓️print");
  const defaults: string[] = json.namedInputs?.default ?? ["{projectRoot}/**/*", "sharedGlobals"];
  const under = (p: string, d: string) => p === d || p.startsWith(d + "/");
  const covered = (p: string) => (a?.defaultDirs ?? [root]).some((d: string) => under(p, d)) || under(p, root);
  const readGlobs = [...extrasDirs].filter((p) => !covered(p)).map((p) => `{workspaceRoot}/${p}/**/*`).sort();
  const usesReads = readGlobs.length > 0;
  const edits: Edit[] = [];
  let flipped = 0;
  const groups = new Set<string>();
  for (const [name, t] of Object.entries<any>(json.targets)) {
    const cls = classOf.get(file + "|" + name);
    if (!cls) { log.skipped.push([json.name, name, "not-in-census"]); continue; }
    const id = json.name + ":" + name;
    const tnode = prop(targetsNode, name)!.value;
    if (SKIP_CLASSES.has(cls)) { log.keptOff.push([id, cls, "census-class"]); continue; }
    const manual = MANUAL[id] ?? MANUAL["*:" + name];
    const exec = t.metadata?.semio?.execution?.kind;
    if (manual?.skip) { log.skipped.push([id, manual.skip]); continue; }
    if (manual?.off) {
      log.keptOff.push([id, "manual", manual.off]);
      if (t.cache !== false) edits.push(...setProps(text, tnode, [["cache", false]], ["executor"]));
      continue;
    }
    if (exec === "orchestration" || t.metadata?.semio?.orchestrator === true) { log.keptOff.push([id, "orchestrator", "orchestration driver; its child tasks are cached"]); continue; }
    if (name === "canonical-architecture") { log.keptOff.push([id, "policy", "policy targetDefaults.canonical-architecture.cache=false: laws read nextest artifacts and exact-cargo law groups across crates beyond the declared project inputs"]); if (t.cache !== false) edits.push(...setProps(text, tnode, [["cache", false]], ["executor"])); continue; }
    if (artifactTs && /^(build|test)$/.test(name)) { log.skipped.push([id, "plugin forces cache=false for role:artifact TypeScript build/test"]); continue; }
    if (nativeProject && /^wasm(-release)?$/.test(name) && json.name !== "@semio-tech/framework-renderer-wgpu") { log.skipped.push([id, "plugin owns wasm target (cache=false, mode-scoped outputs)"]); continue; }
    if (t.metadata?.semio?.wasmPackageSourceV1) { log.skipped.push([id, "plugin owns wasm package output"]); continue; }
    if (contractTargets.has(id)) {
      if (t.cache !== true) edits.push(...setProps(text, tnode, [["cache", true]], ["executor"]));
      log.changed.push([id, "generator-contract", t.cache === true ? "unchanged" : "cache:true (inputs/outputs assembled from taxonomy generatorContracts)"]);
      flipped++;
      continue;
    }
    const entries: [string, unknown][] = [];
    if (t.cache !== true) entries.push(["cache", true]);
    const policyInputs = POLICY.targetDefaults?.[name]?.inputs;
    const testish = /^test(-|$)/.test(name);
    let base: unknown[] = policyInputs ? [...policyInputs] : PRODUCTION.test(name) ? ["production", "^production"] : ["default", "^default"];
    const used: string[] = [];
    if (testish) { base = ["default", "^default", "testEnv"]; used.push("testEnv"); }
    if (usesReads && !PRODUCTION.test(name)) { base = [...base, "reads"]; used.push("reads"); }
    const consumed = (t.dependsOn ?? []).filter((d: unknown) => typeof d === "string" && !/^workspace:deps-|^repo:generator-inputs$/.test(d as string));
    if (consumed.length && !JSON.stringify(t.inputs ?? []).includes("dependentTasksOutputFiles")) base = [...base, { dependentTasksOutputFiles: "**/*" }];
    if (nativeProject) { base = [...base, "cargoToolchain"]; used.push("cargoToolchain"); }
    else if (goProject) { base = [...base, "goToolchain"]; used.push("goToolchain"); }
    if (manual?.inputs) base = manual.inputs(base);
    if (t.inputs === undefined || manual?.forceInputs) { entries.push(["inputs", base]); for (const g of used) groups.add(g); }
    else if (t.cache !== true && Array.isArray(t.inputs)) {
      const seen = new Set<string>();
      const union = [...base, ...t.inputs].filter((x) => { const k = JSON.stringify(x); if (seen.has(k)) return false; seen.add(k); return true; });
      if (union.length !== t.inputs.length || union.some((x, i) => JSON.stringify(x) !== JSON.stringify(t.inputs[i]))) { entries.push(["inputs", union]); for (const g of used) groups.add(g); }
    }
    const hasOutputs = Array.isArray(t.outputs) && t.outputs.length > 0;
    if (!hasOutputs && t.outputs === undefined) {
      let out: string[] | undefined = manual?.outputs;
      if (!out) {
        if (OUTPUTLESS.test(name) || (nativeProject && PRODUCTION.test(name))) out = [];
      }
      if (out === undefined) log.needsOutputs.push([id, t.options?.command?.slice(0, 100)]);
      else entries.push(["outputs", out]);
    }
    if (!entries.length) { log.unchangedCached.push(id); continue; }
    edits.push(...setProps(text, tnode, entries, ["executor"]));
    log.changed.push([id, cls, entries.map(([k]) => k).join(",")]);
    flipped++;
  }
  const defaultNode = (prop(ast, "namedInputs") && prop(prop(ast, "namedInputs")!.value, "default")) || undefined;
  if (flipped > 0) {
    const GROUPS: Record<string, unknown[]> = { testEnv: TEST_ENV, cargoToolchain: CARGO_FILES, goToolchain: GO_FILES };
    const wantGroups = [...groups].filter((g) => g !== "reads" && json.namedInputs?.[g] === undefined);
    const needsGlobals = json.namedInputs?.default !== undefined && !defaults.includes("sharedGlobals");
    if (usesReads || needsGlobals || wantGroups.length) {
      const entries: [string, unknown][] = [];
      if (needsGlobals) entries.push(["default", [...defaults, "sharedGlobals"]]);
      if (usesReads) entries.push(["reads", readGlobs]);
      if (usesReads && nativeProject && json.namedInputs?.nativeTestSources === undefined) entries.push(["nativeTestSources", ["default", "reads"]]);
      for (const g of wantGroups) entries.push([g, GROUPS[g]!]);
      const ni = prop(ast, "namedInputs");
      if (ni) edits.push(...setProps(text, ni.value, entries, []));
      else edits.push(...setProps(text, ast, [["namedInputs", Object.fromEntries(entries)]], ["projectType", "name"]));
      log.projects.push([json.name, readGlobs.length, needsGlobals, wantGroups]);
    }
  }
  if (edits.length) { perFile[file] = { text, edits }; summary.files++; }
}

const outDir = join(TICKET, "🗑️generated");
writeFileSync(join(outDir, "cache-fw-plan.json"), JSON.stringify(log, null, 1));
console.log({ files: summary.files, changed: log.changed.length, keptOff: log.keptOff.length, skipped: log.skipped.length, needsOutputs: log.needsOutputs.length, unchangedCached: log.unchangedCached.length, projectsWithDefaultEdit: log.projects.length });

if (process.argv.includes("--stage")) {
  const dir = join(TICKET, "🗑️generated/cache-fw-stage");
  const { mkdirSync } = await import("node:fs");
  mkdirSync(dir, { recursive: true });
  let n = 0;
  const index: string[] = [];
  for (const [file, { text, edits }] of Object.entries(perFile)) {
    const next = apply(text, edits);
    JSON.parse(next);
    writeFileSync(join(dir, `${n}.old.json`), text);
    writeFileSync(join(dir, `${n}.new.json`), next);
    index.push(`${n} ${file}`);
    n++;
  }
  writeFileSync(join(dir, "index.txt"), index.join("\n"));
}

if (APPLY) {
  let written = 0, stale = 0;
  for (const [file, { text, edits }] of Object.entries(perFile)) {
    const abs = join(ROOT, file);
    if (readFileSync(abs, "utf8") !== text) { stale++; console.log("stale, skipped:", file); continue; }
    const next = apply(text, edits);
    JSON.parse(next);
    writeFileSync(abs, next);
    written++;
  }
  console.log({ written, stale });
}
