// 🧪️ stale-g: real-resolver probe (graph-free), one fresh process per phase like the generation verifier. usage: bun resolver-probe.mjs            (driver)
//   bun resolver-probe.mjs compute <root> <rows.json> <roster.json>   |   bun resolver-probe.mjs check <root> <rows.json>
// Driver: per edit step builds a fixture workspace, computes the walk + observations in process A, applies the edit, asks process B whether the observations still hold, process C for the fresh walk, esbuild as the independent bundle graph.
import {mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync, symlinkSync, readFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname, join} from "node:path";
import {createRequire} from "node:module";
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const [mode, root, rowsFile, rosterFile] = process.argv.slice(2);
const write = (r, files) => { for (const [p, t] of Object.entries(files)) { if (t === null) { rmSync(join(r, p), {force: true}); continue; } mkdirSync(dirname(join(r, p)), {recursive: true}); writeFileSync(join(r, p), t); } };
if (mode === "compute" || mode === "check") {
  process.chdir("/Users/ueli/Documents/semio");
  const inference = await import(L + "🕸️graph/📥️inference/🟨️.mjs"); await inference.libraryBootstrap;
  const obs = await import(L + "👁️observation/🟨️.mjs"), si = await import(L + "🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
  if (mode === "compute") {
    const r = await obs.observeDetachedV1(async () => { const scripts = inference.createScriptInputCache({workspaceRoot: root}); return (await si.relativeScriptInputs([join(root, "tools/entry.ts")], root, scripts)).map((p) => p.replace("{workspaceRoot}/", "")).sort(); });
    writeFileSync(rowsFile, JSON.stringify({rows: r.rows, layers: r.layers.map((l) => [...l])}));
    writeFileSync(rosterFile, JSON.stringify(r.value));
  } else {
    inference.prepareSourceAnalysesV1();
    const {rows, layers} = JSON.parse(readFileSync(rowsFile, "utf8"));
    console.log(JSON.stringify({holds: obs.observationRowsHoldV1(rows) && layers.every((l) => obs.observationLayerHoldsV1(l))}));
  }
  process.exit(0);
}
const esbuild = createRequire("/Users/ueli/Documents/semio/package.json")("esbuild");
const base = {
  "tools/entry.ts": 'import "./a.ts";\nimport "@fix/pkg/feature";\nimport "@alias/x";\n',
  "tools/a.ts": 'export const a = 1;\n',
  "packages/pkg/package.json": JSON.stringify({name: "@fix/pkg", version: "1.0.0", exports: {"./feature": "./f1.ts"}}),
  "packages/pkg/f1.ts": 'export const f1 = 1;\n', "packages/pkg/f2.ts": 'export const f2 = 2;\n',
  "libs/x.ts": 'export const x = 1;\n', "libs2/x.ts": 'export const x2 = 2;\n',
  "tsconfig.json": JSON.stringify({compilerOptions: {baseUrl: ".", paths: {"@alias/*": ["./libs/*"]}}}),
  "package.json": JSON.stringify({name: "fixture", private: true}),
};
const steps = [
  ["transitive import of a NEW file added to a.ts", {"tools/a.ts": 'import "../libs2/x.ts";\nexport const a = 1;\n'}],
  ["workspace package exports repointed (f1 -> f2)", {"packages/pkg/package.json": JSON.stringify({name: "@fix/pkg", version: "1.0.0", exports: {"./feature": "./f2.ts"}})}],
  ["tsconfig paths repointed (libs -> libs2)", {"tsconfig.json": JSON.stringify({compilerOptions: {baseUrl: ".", paths: {"@alias/*": ["./libs2/*"]}}})}],
  ["unrelated file added (outside the closure)", {"libs2/unrelated.ts": 'export const u = 1;\n'}],
];
const run = (args) => { const p = Bun.spawnSync(["bun", import.meta.path, ...args], {cwd: "/Users/ueli/Documents/semio", stdout: "pipe", stderr: "pipe"}); if (p.exitCode !== 0) throw Error(String(p.stderr)); return String(p.stdout).trim(); };
const strip = (l) => l.filter((f) => !f.endsWith("package.json") && !f.endsWith("tsconfig.json"));
const table = [];
for (const [name, edit] of [["(baseline)", {}], ...steps]) {
  const r = mkdtempSync(join(realpathSync(tmpdir()), "stale-g-")); try {
    write(r, base); mkdirSync(join(r, "node_modules/@fix"), {recursive: true}); symlinkSync(join(r, "packages/pkg"), join(r, "node_modules/@fix/pkg"));
    const rows = join(r, "rows.json"), before = join(r, "before.json"), after = join(r, "after.json"), tmpRows = join(tmpdir(), "stale-g-rows-" + process.pid + ".json");
    run(["compute", r, tmpRows, before]);
    write(r, edit);
    const holds = JSON.parse(run(["check", r, tmpRows])).holds;
    run(["compute", r, tmpRows + ".2", after]); rmSync(tmpRows + ".2", {force: true}); rmSync(tmpRows, {force: true});
    const b = JSON.parse(readFileSync(before, "utf8")), a = JSON.parse(readFileSync(after, "utf8"));
    const truth = await esbuild.build({entryPoints: [join(r, "tools/entry.ts")], absWorkingDir: r, bundle: true, write: false, platform: "node", format: "esm", metafile: true, logLevel: "silent", outfile: "out.js"}).then((x) => Object.keys(x.metafile.inputs).map((p) => p.replace(/^\.\//u, "")).sort());
    const moved = JSON.stringify(b) !== JSON.stringify(a);
    table.push({step: name, rowsHold: holds, rosterMoved: moved, walkEqualsEsbuild: JSON.stringify(strip(a)) === JSON.stringify(truth), SILENT_STALE: holds && moved, walk: strip(a).length});
  } finally { rmSync(r, {recursive: true, force: true}); }
}
console.table(table);
process.exit(0);
