// 🧮 Read-only census of cache coverage: every tracked project manifest target, classified with the workspace's own cache policy, plus the targets the nx inference plugin synthesizes from playground rows.
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const TICKET = join(import.meta.dir, "..");
const OUT = join(TICKET, "🗑️generated", process.argv[2] ?? "cache-s-census.json");
const strip = (s: string) => s.replace(/️/g, "");
const PROJECT_FILE = "\u{1F4CB}️project.json";
const POLICY = JSON.parse(readFileSync(join(ROOT, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json"), "utf8"));
const NX = JSON.parse(readFileSync(join(ROOT, "nx.json"), "utf8"));
const NX_IGNORED_SEGMENTS = new Set(["🧫️fixtures", "🧫️examples", "🧪️fixtures", "🧪️examples"].map(strip));
const DIAGNOSTIC = new Set(["audit", "bench", "cache-report", "cache-verify", "ci-baseline", "disk-report", "doctor", "perf-baseline", "test-inventory", "test-doctor", "parity"]);
const OUTPUTLESS = /^(test|check|lint|typecheck|format-check|verify|stdio|graph-check|policy-check|artifact-check|artifact-package-contract|test-contract)(-|$)/;
const BUILD_FAMILY = /^(build|wasm|native-build|package|extension-package|cpp-build|generate|schema-generate)(-|$)/;

const git = Bun.spawnSync(["git", "-c", "core.quotepath=false", "ls-files", "-z"], { cwd: ROOT });
const tracked: string[] = new TextDecoder().decode(git.stdout).split("\0").filter((f) => f && existsSync(join(ROOT, f)));
const trackedSet = new Set(tracked);

const matchesCommand = (name: string, commands: string[]) => commands.some((c) => name === c || name.startsWith(c + "-"));
const matchesUncached = (name: string) => POLICY.uncachedExact.includes(name) || matchesCommand(name, POLICY.uncached);
const mutatingName = (name: string) => /(?:^|-)(?:clean|gc|prune|setup|fuzz)(?:-|$)/.test(name);
const liveName = (name: string) => /(?:^|-)(?:e2e|live)(?:-|$)/.test(name);
const cacheableFamily = (name: string) =>
  /^(test(?:-|$)|check(?:-|$)|lint(?:-|$)|typecheck(?:-|$)|format-check(?:-|$)|generate(?:-|$)|schema(?:-|$)|verify(?:-|$)|stdio(?:-|$)|build(?:-|$)|wasm$|native-build$|package$|extension-package$|cpp-(?:configure|build|test)$|graph-check$|policy-check$|artifact-check$|artifact-package-contract$|toolchain$)/.test(name) ||
  /(?:^|-)contract(?:-|$)/.test(name) ||
  (/^generator(?:-|$)/.test(name) && !name.includes("generator-inputs"));
const verifyCommand = (t: any) => /(?:^|[\s"'])verify(?:[\s"']|$)/.test(t?.options?.command ?? "");

function effectivePolicy(name: string, target: any): { cache: boolean; continuous: boolean } {
  let out: any;
  if (matchesCommand(name, POLICY.continuous)) out = { ...target, cache: false, continuous: true };
  else if (target.cache === false) out = { ...target, cache: false };
  else if (matchesUncached(name)) out = { ...target, cache: false };
  else if (POLICY.cachedExact.includes(name)) out = { ...target, cache: true };
  else if (mutatingName(name) || liveName(name)) out = { ...target, cache: false };
  else if (cacheableFamily(name) || verifyCommand(target)) out = { ...target, cache: true };
  else out = { ...target, cache: target.cache !== false };
  return { cache: out.cache === true, continuous: out.continuous === true };
}

function ownerOf(file: string): string {
  const first = file.split("/")[0];
  if (!file.includes("/")) return "root";
  return first === ".🧬semio" ? "tickets(excluded)" : first;
}

function fileStatus(file: string): string {
  const segs = file.split("/").map(strip);
  if (segs[0] === strip(".🧬semio")) return "ticket-excluded";
  if (segs.includes("pkg")) return "nxignore-pkg";
  if (segs.slice(0, -1).some((s) => NX_IGNORED_SEGMENTS.has(s))) return "nxignore-fixture-or-example";
  if (segs[segs.length - 1] !== strip(PROJECT_FILE)) return "not-plugin-include-basename";
  return "nx-project";
}

const byStatus: Record<string, number> = {};
const byBasename: Record<string, number> = {};
const projectCandidates = tracked.filter((f) => f.endsWith("project.json"));
for (const f of projectCandidates) {
  const s = fileStatus(f);
  byStatus[s] = (byStatus[s] ?? 0) + 1;
  const b = f.split("/").pop()!;
  byBasename[b] = (byBasename[b] ?? 0) + 1;
}

const inputKind = (inputs: unknown): string => {
  if (inputs === undefined) return "absent";
  if (!Array.isArray(inputs)) return "invalid";
  if (inputs.length === 0) return "empty";
  const kinds = new Set<string>();
  for (const item of inputs) {
    if (typeof item === "string") kinds.add(["default", "^default", "production", "^production", "sharedGlobals", "schemaSources", "cargo", "go", "uv", "dotnet", "nativeSources", "nativeTestSources"].includes(item) ? "named" : "explicit");
    else kinds.add("object");
  }
  return [...kinds].sort().join("+");
};

const outputKind = (outputs: unknown): string => {
  if (outputs === undefined) return "absent";
  if (!Array.isArray(outputs)) return "invalid";
  return outputs.length === 0 ? "empty" : "declared";
};

type Row = Record<string, any>;
const targets: Row[] = [];
const projects: Row[] = [];
const parseErrors: string[] = [];
const nxProjectFiles = projectCandidates.filter((f) => fileStatus(f) === "nx-project").sort();
const nameToProject = new Map<string, Row>();

for (const file of nxProjectFiles) {
  let json: any;
  try {
    json = JSON.parse(readFileSync(join(ROOT, file), "utf8"));
  } catch (e) {
    parseErrors.push(file);
    continue;
  }
  const root = file.split("/").slice(0, -1).join("/") || ".";
  const owner = ownerOf(file);
  const entry: Row = { file, root, owner, name: json.name ?? null, targetNames: [] };
  projects.push(entry);
  if (json.name) nameToProject.set(json.name, entry);
  for (const [name, t] of Object.entries<any>(json.targets ?? {})) {
    const defaults = POLICY.targetDefaults?.[name] ?? {};
    const inherited = { ...defaults, ...t, ...(t.cache === false && t.inputs === undefined ? { inputs: ["default"] } : {}) };
    const eff = effectivePolicy(name, inherited);
    const executor = t.executor ?? "repo:script";
    const noop = executor === "nx:noop" || executor === "repo:noop";
    const command: string | null = typeof t.command === "string" ? t.command : typeof t.options?.command === "string" ? t.options.command : t.options?.commands ? "commands" : null;
    const commandForm = typeof t.command === "string" ? "shorthand" : typeof t.options?.command === "string" ? "options.command" : t.options?.commands ? "options.commands" : "none";
    const rawInputs = t.inputs ?? defaults.inputs;
    const rawOutputs = t.outputs ?? defaults.outputs;
    const dependsOn: unknown[] = Array.isArray(t.dependsOn) ? t.dependsOn : [];
    const cls = classify(name, t, eff, noop, command !== null);
    const flags: string[] = [];
    const deterministic = cls === "deterministic-cacheable" || cls === "deterministic-but-cache-off";
    if (deterministic && rawInputs === undefined) flags.push("no-inputs");
    if (deterministic && rawOutputs === undefined) flags.push("no-outputs");
    if (deterministic && Array.isArray(rawOutputs) && rawOutputs.length === 0 && !OUTPUTLESS.test(name)) flags.push("outputs-empty-suspect");
    if (cls === "deterministic-but-cache-off") flags.push("cache-off-deterministic");
    if (cls === "unclassified-cache-off") flags.push("needs-decision");
    if (t.cache === undefined && (cls === "deterministic-cacheable")) flags.push("cache-implicit-by-policy");
    entry.targetNames.push(name);
    targets.push({
      project: json.name ?? file,
      file,
      root,
      owner,
      name,
      executor,
      executorDeclared: t.executor !== undefined,
      commandForm,
      command: command === null ? null : command.slice(0, 200),
      configurations: Object.keys(t.configurations ?? {}),
      inputs: { kind: inputKind(t.inputs), declaredKind: inputKind(t.inputs), effectiveKind: inputKind(rawInputs) },
      outputs: { kind: outputKind(t.outputs), count: Array.isArray(t.outputs) ? t.outputs.length : null },
      cache: t.cache === undefined ? "absent" : String(t.cache),
      continuous: t.continuous === true,
      dependsOn: dependsOn.filter((d): d is string => typeof d === "string"),
      dependsOnObjects: dependsOn.filter((d) => typeof d !== "string").length,
      orchestrator: t.metadata?.semio?.orchestrator === true,
      execution: t.metadata?.semio?.execution?.kind ?? null,
      effectiveCache: eff.cache,
      effectiveContinuous: eff.continuous,
      classProposal: cls,
      flags,
    });
  }
}

function classify(name: string, t: any, eff: { cache: boolean; continuous: boolean }, noop: boolean, hasCommand: boolean): string {
  if (t.continuous === true || eff.continuous) return "continuous";
  if (liveName(name)) return "test-against-live";
  if (matchesUncached(name)) return DIAGNOSTIC.has(name) ? "diagnostic-uncached" : "side-effecting";
  if (mutatingName(name)) return "side-effecting";
  if (eff.cache) return "deterministic-cacheable";
  if (noop || !hasCommand) return "aggregate";
  if (cacheableFamily(name)) return "deterministic-but-cache-off";
  return "unclassified-cache-off";
}

// Totals per class and owner.
const CLASSES = ["deterministic-cacheable", "deterministic-but-cache-off", "continuous", "side-effecting", "test-against-live", "diagnostic-uncached", "aggregate", "unclassified-cache-off"];
const byClass: Record<string, number> = Object.fromEntries(CLASSES.map((c) => [c, 0]));
const byOwner: Record<string, Record<string, number>> = {};
for (const t of targets) {
  byClass[t.classProposal]++;
  byOwner[t.owner] ??= Object.fromEntries(CLASSES.map((c) => [c, 0]));
  byOwner[t.owner][t.classProposal]++;
}
const flagCounts: Record<string, number> = {};
const flagByOwner: Record<string, Record<string, number>> = {};
for (const t of targets) for (const f of t.flags) {
  flagCounts[f] = (flagCounts[f] ?? 0) + 1;
  flagByOwner[t.owner] ??= {};
  flagByOwner[t.owner][f] = (flagByOwner[t.owner][f] ?? 0) + 1;
}

// Which targetDefaults keys (nx.json has none; policy.json has the workspace set) hit tracked targets.
const policyKeys = Object.keys(POLICY.targetDefaults ?? {});
const policyKeyHits: Record<string, number> = Object.fromEntries(policyKeys.map((k) => [k, targets.filter((t) => t.name === k).length]));

// Residual deterministic targets not covered by a by-name default (targetDefaults key or cacheable name family).
const deterministic = targets.filter((t) => t.classProposal === "deterministic-cacheable" || t.classProposal === "deterministic-but-cache-off");
const coveredByName = deterministic.filter((t) => (POLICY.targetDefaults?.[t.name] !== undefined) || cacheableFamily(t.name));
const residualNames: Record<string, number> = {};
for (const t of deterministic.filter((t) => !coveredByName.includes(t))) residualNames[t.name] = (residualNames[t.name] ?? 0) + 1;
const needsOutputs = deterministic.filter((t) => t.flags.includes("no-outputs") || t.flags.includes("outputs-empty-suspect"));
const buildFamilyNoOutputs = needsOutputs.filter((t) => BUILD_FAMILY.test(t.name));

// dependsOn resolution against tracked manifests; synthesized names are matched by the playground grammar.
const SYNTH = /^(prepare|activate|serve|dev|build|run|smoke|mcp|session|runtime-input-admission|build-all-playground-cdn-sites)(-|$)/;
const depStats = { total: 0, resolved: 0, synthesizedCandidate: 0, unresolved: 0, caret: 0 };
const unresolved: Record<string, number> = {};
const targetKey = new Set(targets.map((t) => t.project + ":" + t.name));
for (const t of targets) for (const d of t.dependsOn) {
  depStats.total++;
  if (d.startsWith("^")) { depStats.caret++; continue; }
  const [proj, tgt] = d.includes(":") ? [d.slice(0, d.lastIndexOf(":")), d.slice(d.lastIndexOf(":") + 1)] : [t.project, d];
  if (targetKey.has(proj + ":" + tgt)) depStats.resolved++;
  else if (SYNTH.test(tgt)) depStats.synthesizedCandidate++;
  else { depStats.unresolved++; unresolved[d] = (unresolved[d] ?? 0) + 1; }
}

// Playground grammar synthesized by 🎮️playground-targets for every Cargo component row (admitted path).
const cargoFiles = tracked.filter((f) => f.split("/").pop() === "Cargo.toml" && !["ticket-excluded", "nxignore-pkg", "nxignore-fixture-or-example"].includes(fileStatus(f.replace(/Cargo\.toml$/, PROJECT_FILE))) && !f.startsWith(".🧬semio"));
const playgroundRows: Row[] = [];
let cargoComponentCount = 0;
for (const f of cargoFiles) {
  const text = readFileSync(join(ROOT, f), "utf8");
  if (!/component-kind\s*=\s*"(plugin|extension)"/.test(text) || !/\[package\.metadata\.component\][\s\S]*?\bpackage\s*=/.test(text)) continue;
  cargoComponentCount++;
  const blocks = text.split(/\[\[package\.metadata\.semio\.playground\]\]/).slice(1);
  for (const block of blocks) {
    const body = block.split(/\n\[/)[0];
    const variant = body.match(/variant\s*=\s*"([^"]+)"/)?.[1] ?? null;
    playgroundRows.push({ variant, file: f, mcpHost: /mcpHost/.test(body), nativeHost: /nativeHost/.test(body), distDir: /distDir/.test(body), app: /^app\s*=/m.test(body) });
  }
}
const variants = [...new Set(playgroundRows.map((r) => r.variant))];
const crates = new Set(playgroundRows.map((r) => r.file));
const mcpVariants = playgroundRows.filter((r) => r.mcpHost).length;
const perVariantFamilies = [
  "prepare-V-react-prototype", "prepare-V-react-release", "prepare-V-react",
  "activate-V-react-prototype", "activate-V-react-release",
  "serve-V-react-prototype", "serve-V-react-release", "dev-V-react-prototype", "dev-V-react-release",
  "prepare-V-wgpu-prototype", "prepare-V-wgpu-release", "prepare-V-wgpu",
  "activate-V-wgpu-prototype", "activate-V-wgpu-release",
  "serve-V-wgpu-prototype", "serve-V-wgpu-release", "dev-V-wgpu-prototype", "dev-V-wgpu-release",
  "prepare-V-native-prototype", "prepare-V-native-release",
  "run-V-native-prototype", "run-V-native-release", "smoke-V-native-prototype", "smoke-V-native-release",
  "build-V-react-release", "build-V-react", "build-V-site", "session-V (plugin-registry)", "runtime-input-admission-V (refusal only)",
];
const familyClass: Record<string, string> = {
  "prepare-V-react": "deterministic-but-cache-off", "prepare-V-wgpu": "deterministic-but-cache-off", "prepare-V-native": "deterministic-but-cache-off",
  "activate-V-react": "deterministic-cacheable", "activate-V-wgpu": "side-effecting (live publish, per comment)",
  "serve": "continuous", "dev": "continuous", "mcp": "continuous",
  "run-V-native": "test-against-live (live run)", "smoke-V-native": "test-against-live",
  "build-V-react": "deterministic-but-cache-off", "build-V-site": "aggregate (noop, cache off)", "session": "deterministic-cacheable", "runtime-input-admission": "deterministic-but-cache-off (refusal path)",
};
const variantCount = variants.length;
const perVariantTotal = { react: 9, wgpu: 9, native: 6, build: 3, session: 1 };
const perVariantTotalAll = Object.values(perVariantTotal).reduce((a, b) => a + b, 0);
const platformDerived = {
  variants: variantCount,
  playgroundRows: playgroundRows.length,
  crateFilesWithRows: crates.size,
  componentCargoManifests: cargoComponentCount,
  rowsWithMcpHost: mcpVariants,
  rowsWithDistDir: playgroundRows.filter((r) => r.distDir).length,
  perVariantSynthesizedTargets: perVariantTotalAll,
  perVariantMcpTargets: 4,
  approxSynthesizedTotal: variantCount * perVariantTotalAll + mcpVariants * 4 + crates.size + (playgroundRows.some((r) => r.distDir) ? 1 : 0),
  perVariantFamilies,
  familyClass,
  frameworkOsDevTrackedTargets: nameToProject.get("@semio-tech/framework-os-dev")?.targetNames.length ?? null,
  frameworkOsDevPrototypeKeysTracked: nameToProject.get("@semio-tech/framework-os-dev")?.targetNames.filter((n: string) => n.includes("prototype")).length ?? null,
};

// Per-target cache classes for the synthesized playground grammar, per variant (cache flag as authored in 🎮️playground-targets/🟨️.mjs).
const synthPerVariant = {
  "prepare-V-{react,wgpu,native}-{prototype,release}": { count: 6, cache: false, cls: "deterministic-but-cache-off (orchestrator)" },
  "prepare-V-{react,wgpu}": { count: 2, cache: false, cls: "deterministic-but-cache-off (merged alias)" },
  "activate-V-react-{prototype,release}": { count: 2, cache: true, cls: "deterministic-cacheable" },
  "activate-V-wgpu-{prototype,release}": { count: 2, cache: false, cls: "side-effecting (live publish, deliberate)" },
  "serve-V-{react,wgpu}-{prototype,release}, dev-V-{react,wgpu}-{prototype,release}": { count: 8, cache: false, cls: "continuous" },
  "run-V-native-{prototype,release}": { count: 2, cache: false, cls: "test-against-live (live run)" },
  "smoke-V-native-{prototype,release}": { count: 2, cache: false, cls: "test-against-live" },
  "build-V-react-release, build-V-react": { count: 2, cache: false, cls: "deterministic-but-cache-off" },
  "build-V-site (per row)": { count: 1, cache: false, cls: "aggregate (repo:noop)" },
  "session-V (plugin-registry)": { count: 1, cache: true, cls: "deterministic-cacheable" },
  "mcp-V-{stdio,http}-{prototype,release} (mcpHost rows only)": { count: 4, cache: false, cls: "continuous" },
  "build (per crate with rows)": { count: crates.size, cache: false, cls: "aggregate (repo:noop)" },
  "build-all-playground-cdn-sites (any distDir row)": { count: playgroundRows.some((r) => r.distDir) ? 1 : 0, cache: false, cls: "side-effecting (CDN publish)" },
};

const out = {
  generatedBy: "aud-cachecov-tools/census.ts",
  root: ROOT,
  policySource: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json",
  nxPlugins: NX.plugins,
  nxJsonTargetDefaultsCount: Object.keys(NX.targetDefaults ?? {}).length,
  nxIgnoredRule: "**/🧫️fixtures/**, **/🧪️fixtures/**, *examples/**, **/pkg/**, /.🧬semio/**",
  trackedCount: tracked.length,
  projectCandidateFiles: { total: projectCandidates.length, byStatus, byBasename },
  nxProjects: { count: projects.length, withTargets: projects.filter((p) => p.targetNames.length > 0).length, parseErrors },
  targetCount: targets.length,
  byClass,
  byOwner,
  flagCounts,
  flagByOwner,
  policyTargetDefaults: { keys: policyKeys, targetHits: policyKeyHits },
  deterministicTargets: { total: deterministic.length, coveredByName: coveredByName.length, residualByName: residualNames, noOutputsOrEmptySuspect: needsOutputs.length, buildFamilyNoOutputs: buildFamilyNoOutputs.length },
  dependsOn: { ...depStats, unresolvedTop: Object.entries(unresolved).sort((a, b) => b[1] - a[1]).slice(0, 80) },
  synthesized: { ...platformDerived, synthPerVariant, playgroundRowsSample: playgroundRows.slice(0, 5) },
  projects: projects.map((p) => ({ file: p.file, name: p.name, root: p.root, owner: p.owner, targetNames: p.targetNames })),
  targets,
};

mkdirSync(join(TICKET, "🗑️generated"), { recursive: true });
writeFileSync(OUT, JSON.stringify(out, null, 1));
console.log(JSON.stringify({ trackedProjectCandidates: projectCandidates.length, byStatus, nxProjects: projects.length, targets: targets.length, byClass, variants: variantCount, playgroundRows: playgroundRows.length, crates: crates.size, depStats, parseErrors: parseErrors.length, deterministic: deterministic.length, coveredByName: coveredByName.length, noOutputsOrEmptySuspect: needsOutputs.length, buildFamilyNoOutputs: buildFamilyNoOutputs.length, approxSynthesizedTotal: platformDerived.approxSynthesizedTotal, frameworkOsDevTrackedTargets: platformDerived.frameworkOsDevTrackedTargets }, null, 1));
