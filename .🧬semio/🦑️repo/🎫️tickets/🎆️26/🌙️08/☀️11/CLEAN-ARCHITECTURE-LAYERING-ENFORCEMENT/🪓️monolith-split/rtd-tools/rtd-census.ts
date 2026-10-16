// 🧮️ Census over an explicit manifest list (argv[2]: repo-relative Cargo.toml per line; argv[3]: output tag). Same counter and test-closure rules as ../tcc-tools/tcc-census.ts; writes ../🗑️generated/rtd-census-<tag>-*.tsv.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {mkdirSync, readFileSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {GEN, LABELS, ONLY, ROOT, applies, cfgList, fixed, loadNode, measure, normalLib, rel, resolveGraph, targetsOf, tsv} from "./rtd-lib.ts";

const csvRows = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean).map(m => ["", m]);
const TAG = process.argv[3]!;
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
    }
  }
  const hasTests = testOverAny || testTargetCounts.integration > 0 || testTargetCounts.bin > 0 || Object.keys(perLabel).some(key => key.endsWith("_testOnlyFiles") && Number(perLabel[key]) > 0);
  crates.push({...crate, hasTests, integration: testTargetCounts.integration, binTests: testTargetCounts.bin, examples: testTargetCounts.example, ...perLabel, testOverAny, devOverAny});
  if (processed % 50 === 0) console.error(`processed ${processed} crates in ${Math.round((Date.now() - started) / 1000)} s`);
}

mkdirSync(GEN, {recursive: true});
const outFile = (name: string) => join(GEN, name);
writeFileSync(outFile("rtd-census-" + TAG + "-crates.tsv"), tsv(crates));
writeFileSync(outFile("rtd-census-" + TAG + "-units.tsv"), tsv(units));
writeFileSync(outFile("rtd-census-" + TAG + "-overlimit-test-units.tsv"), tsv(overUnits));
writeFileSync(outFile("rtd-census-" + TAG + "-overlimit-devdeps.tsv"), tsv(overDev));
writeFileSync(outFile("rtd-census-" + TAG + "-overlimit-files.tsv"), tsv(overFiles));
writeFileSync(outFile("rtd-census-" + TAG + "-validation.tsv"), tsv(validation));
writeFileSync(outFile("rtd-census-" + TAG + "-errors.tsv"), tsv(errors));

const nativeOverUnits = overUnits.filter(u => u.label === "native");
const overCrates = new Set(overUnits.map(u => u.crate));
const overDevCrates = new Set(overDev.map(u => u.crate));
const compareRows: any[] = [];
const summary = {
  crates: processed, units: units.length, errorUnits: errors.length,
  overTestUnitsAllLabels: overUnits.length, overTestUnitsNative: nativeOverUnits.length,
  cratesWithOverTestUnit: overCrates.size, cratesWithOverTestUnitNative: new Set(nativeOverUnits.map(u => u.crate)).size,
  overDevDepRowsAllLabels: overDev.length, overDevDepRowsNative: overDev.filter(u => u.label === "native").length, cratesWithOverDevDep: overDevCrates.size,
  seconds: Math.round((Date.now() - started) / 1000),
};
writeFileSync(outFile("rtd-census-" + TAG + "-summary.json"), JSON.stringify(summary, null, 2) + "\n");
console.log(JSON.stringify(summary));
const top = [...overUnits].sort((a, b) => Number(b.ratio) - Number(a.ratio)).slice(0, 15);
for (const row of top) console.log([row.crate, row.label, row.unit, row.target, row.files, row.bytes, row.lines, row.ratio].join("\t"));

