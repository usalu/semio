// 🧪️ Read-only test-closure census: each first-party crate's own test compilation units (lib under cfg(test), bin and integration test targets, examples for --all-targets) and its first-party dev-dependency closures, measured through the canonical readRustSourceClosureV1 counter for native, wasm32-unknown-unknown and wasm32-wasip2.
// @see 📓️EXECUTOR-RULES.md and ../🗑️generated/crate-census.csv
import {existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync} from "node:fs";
import {basename, dirname, join, resolve} from "node:path";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1, rustCfgMatchesV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const ROOT = "/Users/ueli/Documents/semio";
const GEN = resolve(import.meta.dir, "..", "🗑️generated");
const CSV = join(GEN, "crate-census.csv");
const HARD = {files: 32, bytes: 262144, lines: 4000};
const LABELS = ["native", "wasm32-unknown-unknown", "wasm32-wasip2"];
const TRIPLE: Record<string, string | undefined> = {native: undefined, "wasm32-unknown-unknown": "wasm32-unknown-unknown", "wasm32-wasip2": "wasm32-wasip2"};
const ONLY = process.env.ONLY ? new Set(process.env.ONLY.split(",")) : undefined;

const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const hostTriple = execFileSync(rustc, ["-vV"], {encoding: "utf8"}).split("\n").find(line => line.startsWith("host: "))!.slice(6).trim();
const cfgOf = (triple?: string): string[] => execFileSync(rustc, ["--print", "cfg", ...(triple ? ["--target", triple] : [])], {encoding: "utf8"}).trim().split("\n").map(value => value.replace(/=(".*")$/u, (_, quoted) => "=" + JSON.parse(quoted)));
const CFG = new Map(LABELS.map(label => [label, cfgOf(TRIPLE[label])]));
const tripleOf = (label: string) => TRIPLE[label] ?? hostTriple;

const wsRoot = Bun.TOML.parse(readFileSync(join(ROOT, "Cargo.toml"), "utf8")) as any;
const wsDeps: Record<string, any> = wsRoot.workspace?.dependencies ?? {};

type Limit = {files: number; bytes: number; lines: number};
type Spec = {key: string; kind: "normal" | "build" | "dev"; target?: string; dir: string; optional: boolean; features: string[]; defaults: boolean};
type Node = {dir: string; name: string; parsed: any; table: Record<string, string[]>; specs: Spec[]; limit: Limit; libEntry?: string};
type Input = {path: string; bytes: number; lines: number; kind: string};
type Measured = {files: number; bytes: number; lines: number; ratio: number; binding: string; error: string; inputs: Input[]};
type Target = {kind: "lib-test" | "bin-test" | "integration" | "example"; name: string; entry: string; test: boolean};
type State = {requested: Set<string>; defaults: boolean; viaDev: boolean; viaNormal: boolean};

const specsOf = (parsed: any, dir: string): Spec[] => {
  const out: Spec[] = [];
  const add = (table: any, kind: Spec["kind"], target?: string) => {
    for (const [key, raw] of Object.entries(table ?? {}) as [string, any][]) {
      const spec = typeof raw === "string" ? {} : raw;
      const inherited = spec.workspace === true;
      const ws = inherited ? (wsDeps[key] ?? wsDeps[spec.package ?? key] ?? {}) : {};
      const path = spec.path ?? ws.path;
      if (typeof path !== "string") continue;
      out.push({key, kind, target, dir: resolve(spec.path === undefined && inherited ? ROOT : dir, path), optional: (spec.optional ?? ws.optional) === true, features: [...(ws.features ?? []), ...(spec.features ?? [])], defaults: (spec["default-features"] ?? spec.default_features ?? ws["default-features"] ?? ws.default_features ?? true) !== false});
    }
  };
  add(parsed.dependencies, "normal");
  add(parsed["build-dependencies"], "build");
  add(parsed["dev-dependencies"], "dev");
  for (const [target, body] of Object.entries(parsed.target ?? {}) as [string, any][]) {
    add(body.dependencies, "normal", target);
    add(body["build-dependencies"], "build", target);
    add(body["dev-dependencies"], "dev", target);
  }
  return out;
};

const nodeCache = new Map<string, Node | null>();
const loadNode = (dir: string): Node | null => {
  if (nodeCache.has(dir)) return nodeCache.get(dir)!;
  const manifest = join(dir, "Cargo.toml");
  if (!existsSync(manifest)) { nodeCache.set(dir, null); return null; }
  const parsed = Bun.TOML.parse(readFileSync(manifest, "utf8")) as any;
  const declared = parsed.package?.metadata?.semio?.["rust-source-boundary"];
  const limit: Limit = declared ? {files: Math.min(HARD.files, declared.maxSourceFiles), bytes: Math.min(HARD.bytes, declared.maxSourceBytes), lines: Math.min(HARD.lines, declared.maxActiveLines)} : {...HARD};
  const libEntry = resolve(dir, parsed.lib?.path ?? "src/lib.rs");
  const node: Node = {dir, name: parsed.package?.name ?? basename(dir), parsed, table: parsed.features ?? {}, specs: specsOf(parsed, dir), limit, libEntry: existsSync(libEntry) ? libEntry : undefined};
  nodeCache.set(dir, node);
  return node;
};

const closureOf = (table: Record<string, string[]>, roots: string[]): Set<string> => {
  const out = new Set<string>(), queue = [...roots];
  while (queue.length) {
    const token = queue.pop()!;
    if (out.has(token)) continue;
    out.add(token);
    if (token.startsWith("dep:")) continue;
    const [head, rest] = token.split("/") as [string, string | undefined];
    const name = head.replace(/\?$/u, "");
    if (table[name]) queue.push(...table[name]);
    if (rest !== undefined && !head.endsWith("?")) out.add(name);
  }
  return out;
};

const applies = (spec: Spec, label: string): boolean => {
  if (spec.target === undefined) return true;
  if (!spec.target.startsWith("cfg(")) return spec.target === tripleOf(label);
  try { return rustCfgMatchesV1(spec.target.slice(4, -1), CFG.get(label)!); } catch { return true; }
};

const resolveGraph = (rootDir: string, label: string) => {
  const states = new Map<string, State>(), enabled = new Map<string, Set<string>>(), queue: string[] = [];
  const touch = (dir: string, requested: string[], defaults: boolean, viaDev: boolean, viaNormal: boolean) => {
    let state = states.get(dir), changed = false;
    if (!state) { state = {requested: new Set(), defaults: false, viaDev: false, viaNormal: false}; states.set(dir, state); changed = true; }
    for (const r of requested) if (!state.requested.has(r)) { state.requested.add(r); changed = true; }
    if (defaults && !state.defaults) { state.defaults = true; changed = true; }
    if (viaDev && !state.viaDev) { state.viaDev = true; changed = true; }
    if (viaNormal && !state.viaNormal) { state.viaNormal = true; changed = true; }
    if (changed) queue.push(dir);
  };
  touch(rootDir, [], true, false, true);
  while (queue.length) {
    const dir = queue.shift()!, state = states.get(dir)!, node = loadNode(dir);
    if (!node) continue;
    const seen = closureOf(node.table, [...(state.defaults ? ["default"] : []), ...state.requested]);
    enabled.set(dir, seen);
    const activated = new Set([...seen].map(token => token.startsWith("dep:") ? token.slice(4) : token));
    const pairs = [...seen].filter(token => token.includes("/")).map(token => {
      const slash = token.indexOf("/"), head = token.slice(0, slash);
      return {dep: head.replace(/\?$/u, ""), optional: head.endsWith("?"), feat: token.slice(slash + 1)};
    });
    for (const spec of node.specs) {
      if (!applies(spec, label)) continue;
      if (spec.kind === "dev" && dir !== rootDir) continue;
      if (spec.optional && !activated.has(spec.key)) continue;
      const req = [...spec.features, ...pairs.filter(pair => pair.dep === spec.key && (!pair.optional || activated.has(pair.dep))).map(pair => pair.feat)];
      touch(spec.dir, req, spec.defaults, state.viaDev || spec.kind === "dev", state.viaNormal && spec.kind !== "dev");
    }
  }
  return {states, enabled};
};

const LABEL = process.env.C4P1_LABEL ?? "after";
const BASE_MAP: Record<string, string> = process.env.C4P1_BASE_MAP ? JSON.parse(readFileSync(process.env.C4P1_BASE_MAP, "utf8")) : {};
const readFile = async (path: string): Promise<Uint8Array | null> => {
  try { return readFileSync(BASE_MAP[path] ?? path); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return null; throw error; }
};

const cfgList = (label: string, features: Set<string>, test: boolean): string[] => [
  ...CFG.get(label)!, "debug_assertions",
  ...[...features].filter(f => !f.startsWith("dep:") && !f.includes("/") && !f.endsWith("?")).map(f => "feature=" + f),
  ...(test ? ["test"] : []),
];

const measure = async (entry: string, node: Node, cfg: string[]): Promise<Measured> => {
  try {
    const inputs = await readRustSourceClosureV1([entry], ROOT, cfg, readFile, {manifestDirectory: node.dir, generated: () => undefined});
    const files = inputs.length, bytes = inputs.reduce((sum, input) => sum + input.bytes, 0), lines = inputs.reduce((sum, input) => sum + input.activeLines, 0);
    const parts = {files: files / node.limit.files, bytes: bytes / node.limit.bytes, lines: lines / node.limit.lines};
    const binding = Object.entries(parts).sort((a, b) => b[1] - a[1])[0]![0];
    return {files, bytes, lines, ratio: Math.max(parts.files, parts.bytes, parts.lines), binding, error: "", inputs: inputs.map(input => ({path: input.path, bytes: input.bytes, lines: input.activeLines, kind: input.kind}))};
  } catch (error) {
    return {files: 0, bytes: 0, lines: 0, ratio: NaN, binding: "error", error: String((error as Error).message ?? error).replaceAll("\n", " ").slice(0, 220), inputs: []};
  }
};

const normalCache = new Map<string, Measured>();
const normalLib = async (node: Node, label: string, features: Set<string>): Promise<Measured> => {
  const cfg = cfgList(label, features, false);
  const key = label + "|" + node.dir + "|" + cfg.filter(c => c.startsWith("feature=")).sort().join(",");
  if (!normalCache.has(key)) normalCache.set(key, await measure(node.libEntry!, node, cfg));
  return normalCache.get(key)!;
};

const autoEntries = (dir: string): [string, string][] => {
  if (!existsSync(dir)) return [];
  const out: [string, string][] = [];
  for (const row of readdirSync(dir, {withFileTypes: true}).sort((a, b) => a.name.localeCompare(b.name))) {
    if (row.isFile() && row.name.endsWith(".rs")) out.push([basename(row.name, ".rs"), join(dir, row.name)]);
    else if (row.isDirectory() && existsSync(join(dir, row.name, "main.rs"))) out.push([row.name, join(dir, row.name, "main.rs")]);
  }
  return out;
};

const targetsOf = (node: Node, features: Set<string>): Target[] => {
  const p = node.parsed, out: Target[] = [], seen = new Set<string>();
  const ready = (table: any) => (table?.["required-features"] ?? []).every((f: string) => features.has(f));
  const add = (kind: Target["kind"], name: string, entry: string, test: boolean, required: boolean) => {
    if (!required || seen.has(entry) || !existsSync(entry)) return;
    seen.add(entry);
    out.push({kind, name, entry, test});
  };
  if (node.libEntry && p.lib?.test !== false) add("lib-test", node.name, node.libEntry, true, true);
  for (const bin of p.bin ?? []) add("bin-test", bin.name, resolve(node.dir, bin.path ?? `src/bin/${bin.name}.rs`), bin.test !== false, ready(bin));
  if (p.autobins !== false) {
    add("bin-test", node.name, resolve(node.dir, "src/main.rs"), true, true);
    for (const [name, entry] of autoEntries(join(node.dir, "src/bin"))) add("bin-test", name, entry, true, true);
  }
  for (const t of p.test ?? []) add("integration", t.name, resolve(node.dir, t.path ?? `tests/${t.name}.rs`), true, ready(t));
  if (p.autotests !== false) for (const [name, entry] of autoEntries(join(node.dir, "tests"))) add("integration", name, entry, true, true);
  for (const e of p.example ?? []) add("example", e.name, resolve(node.dir, e.path ?? `examples/${e.name}.rs`), false, ready(e));
  if (p.autoexamples !== false) for (const [name, entry] of autoEntries(join(node.dir, "examples"))) add("example", name, entry, false, true);
  return out;
};

const rel = (path: string) => path.replace(ROOT + "/", "");
const tsv = (rows: Record<string, unknown>[]): string => {
  if (!rows.length) return "";
  const header = [...new Set(rows.flatMap(row => Object.keys(row)))];
  const cell = (value: unknown) => String(value ?? "").replaceAll("\t", " ").replaceAll("\n", " ");
  return [header.join("\t"), ...rows.map(row => header.map(key => cell(row[key])).join("\t"))].join("\n") + "\n";
};
const fixed = (value: number) => Number.isFinite(value) ? Number(value.toFixed(3)) : "";

const csvRows = readFileSync(CSV, "utf8").trim().split("\n").slice(1).map(line => line.split(","));
const crates: Record<string, unknown>[] = [], units: Record<string, unknown>[] = [], overUnits: Record<string, unknown>[] = [], overDev: Record<string, unknown>[] = [];
const overFiles: Record<string, unknown>[] = [], validation: Record<string, unknown>[] = [], errors: Record<string, unknown>[] = [];
const started = Date.now();
let processed = 0;

for (const row of csvRows) {
  const manifest = row[1]!, dir = dirname(join(ROOT, manifest));
  const node = loadNode(dir);
  if (!node || (ONLY && !ONLY.has(node.name))) continue;
  processed++;
  const perLabel: Record<string, unknown> = {};
  const crate: Record<string, unknown> = {crate: node.name, manifest};
  let testTargetCounts = {integration: 0, bin: 0, example: 0};
  let testOverAny = false, devOverAny = false;
  for (const label of LABELS) {
    const graph = resolveGraph(dir, label);
    const features = graph.enabled.get(dir) ?? new Set<string>();
    const targets = targetsOf(node, features);
    if (label === "native") testTargetCounts = {integration: targets.filter(t => t.kind === "integration").length, bin: targets.filter(t => t.kind === "bin-test").length, example: targets.filter(t => t.kind === "example").length};
    const normal = node.libEntry ? await normalLib(node, label, features) : undefined;
    const normalInputs = new Set<string>(normal?.inputs.map(input => input.path) ?? []);
    let libTest: Measured | undefined, worst = {ratio: -1, unit: "", over: false}, overTestUnits = 0;
    const testOnly: Input[] = [];
    for (const target of targets) {
      const measured = await measure(target.entry, node, cfgList(label, features, target.test));
      const over = !measured.error && measured.ratio > 1;
      const entryRel = rel(target.entry);
      const record = {crate: node.name, manifest, label, unit: target.kind, target: target.name, entry: entryRel, files: measured.files, bytes: measured.bytes, lines: measured.lines, ratio: fixed(measured.ratio), binding: measured.binding, over, error: measured.error};
      units.push(record);
      if (measured.error) errors.push(record);
      if (target.kind === "lib-test") libTest = measured;
      if (target.kind === "example") continue;
      if (!measured.error && measured.ratio > worst.ratio) worst = {ratio: measured.ratio, unit: `${target.kind}:${target.name}`, over};
      if (over) {
        testOverAny = true;
        overTestUnits++;
        overUnits.push(record);
        for (const input of measured.inputs) overFiles.push({crate: node.name, label, unit: target.kind, target: target.name, file: input.path, bytes: input.bytes, lines: input.lines, kind: input.kind, notInLibNormal: !normalInputs.has(input.path)});
      }
      if (target.kind === "lib-test") for (const input of measured.inputs) if (!normalInputs.has(input.path)) testOnly.push(input);
    }
    const devRows: Record<string, unknown>[] = [];
    for (const [dir2, state] of graph.states) {
      if (dir2 === dir || !state.viaDev) continue;
      const dep = loadNode(dir2);
      if (!dep?.libEntry) continue;
      const measured = await normalLib(dep, label, graph.enabled.get(dir2) ?? new Set());
      const direct = node.specs.some(spec => spec.kind === "dev" && spec.dir === dir2 && applies(spec, label));
      const over = !measured.error && measured.ratio > 1;
      const record = {crate: node.name, manifest, label, devDep: dep.name, devDir: rel(dir2), direct, alsoNormal: state.viaNormal, files: measured.files, bytes: measured.bytes, lines: measured.lines, ratio: fixed(measured.ratio), binding: measured.binding, over, error: measured.error};
      devRows.push(record);
      units.push({crate: node.name, manifest, label, unit: "devdep-lib", target: dep.name, entry: rel(dep.libEntry), files: measured.files, bytes: measured.bytes, lines: measured.lines, ratio: fixed(measured.ratio), binding: measured.binding, over, error: measured.error});
      if (measured.error) errors.push(record);
      if (over) {
        devOverAny = true;
        overDev.push(record);
        for (const input of measured.inputs) overFiles.push({crate: node.name, label, unit: "devdep-lib", target: dep.name, file: input.path, bytes: input.bytes, lines: input.lines, kind: input.kind, notInLibNormal: ""});
      }
    }
    const devOver = devRows.filter(r => r.over).map(r => `${r.devDep}${r.direct ? "" : "(transitive)"}`);
    perLabel[`${label}_libNormalFiles`] = normal?.files ?? "";
    perLabel[`${label}_libNormalBytes`] = normal?.bytes ?? "";
    perLabel[`${label}_libNormalLines`] = normal?.lines ?? "";
    perLabel[`${label}_libTestFiles`] = libTest?.files ?? "";
    perLabel[`${label}_libTestBytes`] = libTest?.bytes ?? "";
    perLabel[`${label}_libTestLines`] = libTest?.lines ?? "";
    perLabel[`${label}_libTestRatio`] = libTest ? fixed(libTest.ratio) : "";
    perLabel[`${label}_testOnlyFiles`] = testOnly.length;
    perLabel[`${label}_testOnlyBytes`] = testOnly.reduce((sum, input) => sum + input.bytes, 0);
    perLabel[`${label}_testOnlyLines`] = testOnly.reduce((sum, input) => sum + input.lines, 0);
    perLabel[`${label}_worstTestRatio`] = worst.ratio >= 0 ? fixed(worst.ratio) : "";
    perLabel[`${label}_worstTestUnit`] = worst.unit;
    perLabel[`${label}_overTestUnits`] = overTestUnits;
    perLabel[`${label}_devCount`] = devRows.length;
    perLabel[`${label}_devOver`] = devOver.join(";");
    if (label === "native" && node.libEntry) {
      const csv = csvRows.find(r => r[1] === manifest && r[8] === "lib");
      if (csv && normal) validation.push({crate: node.name, manifest, label: "csv-compare", csvFiles: csv[2], csvBytes: csv[3], csvLines: csv[4], mine: `${normal.files}/${normal.bytes}/${normal.lines}`, match: csv[2] === String(normal.files) && csv[3] === String(normal.bytes) && csv[4] === String(normal.lines)});
    }
  }
  const hasTests = testOverAny || testTargetCounts.integration > 0 || testTargetCounts.bin > 0 || Object.keys(perLabel).some(key => key.endsWith("_testOnlyFiles") && Number(perLabel[key]) > 0);
  crates.push({...crate, hasTests, integration: testTargetCounts.integration, binTests: testTargetCounts.bin, examples: testTargetCounts.example, ...perLabel, testOverAny, devOverAny});
  if (processed % 50 === 0) console.error(`[DEBUG] processed ${processed} crates in ${Math.round((Date.now() - started) / 1000)} s`);
}

mkdirSync(GEN, {recursive: true});
const outFile = (name: string) => join(GEN, name);
writeFileSync(outFile(`c4p1-census-${LABEL}-crates.tsv`), tsv(crates));
writeFileSync(outFile(`c4p1-census-${LABEL}-units.tsv`), tsv(units));
writeFileSync(outFile(`c4p1-census-${LABEL}-overlimit-test-units.tsv`), tsv(overUnits));
writeFileSync(outFile(`c4p1-census-${LABEL}-overlimit-devdeps.tsv`), tsv(overDev));
writeFileSync(outFile(`c4p1-census-${LABEL}-overlimit-files.tsv`), tsv(overFiles));
writeFileSync(outFile(`c4p1-census-${LABEL}-validation.tsv`), tsv(validation));
writeFileSync(outFile(`c4p1-census-${LABEL}-errors.tsv`), tsv(errors));

const nativeOverUnits = overUnits.filter(u => u.label === "native");
const overCrates = new Set(overUnits.map(u => u.crate));
const overDevCrates = new Set(overDev.map(u => u.crate));
const compareRows = validation.filter(v => v.label === "csv-compare");
const summary = {
  crates: processed, units: units.length, errorUnits: errors.length,
  overTestUnitsAllLabels: overUnits.length, overTestUnitsNative: nativeOverUnits.length,
  cratesWithOverTestUnit: overCrates.size, cratesWithOverTestUnitNative: new Set(nativeOverUnits.map(u => u.crate)).size,
  overDevDepRowsAllLabels: overDev.length, overDevDepRowsNative: overDev.filter(u => u.label === "native").length, cratesWithOverDevDep: overDevCrates.size,
  csvCompare: compareRows.length, csvMismatch: compareRows.filter(v => v.match !== true).length,
  seconds: Math.round((Date.now() - started) / 1000),
};
writeFileSync(outFile(`c4p1-census-${LABEL}-summary.json`), JSON.stringify(summary, null, 2) + "\n");
console.log(JSON.stringify(summary));
const top = [...overUnits].sort((a, b) => Number(b.ratio) - Number(a.ratio)).slice(0, 15);
for (const row of top) console.log([row.crate, row.label, row.unit, row.target, row.files, row.bytes, row.lines, row.ratio].join("\t"));
