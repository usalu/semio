// 🔭️ For each manifest listed in argv[2]: every reference to the ui aggregate alias in the crate's sources over all cfg labels and all features, classified as production (normal closure), test file (only in a test closure) or inline cfg(test) region. JSON to argv[3].
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {existsSync, readFileSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {ROOT, LABELS, cfgList, loadNode, readFile, rel, targetsOf, tripleOf} from "./rtd-lib.ts";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {cfgCompiledWithoutTest, findAliasRefs, inRegions, lineOf, mask, testRegions} from "./rtd-rust.ts";

const alias = process.env.ALIAS ?? "ui_wgpu";
const tolerant = async (path: string): Promise<Uint8Array | null> => (await readFile(path)) ?? new Uint8Array(0);
const closure = async (entry: string, node: any, cfg: string[]): Promise<{path: string; kind: string}[]> => {
  try {
    const inputs = await readRustSourceClosureV1([entry], ROOT, cfg, tolerant, {manifestDirectory: node.dir, generated: () => undefined});
    return inputs.filter((i: any) => i.kind === "source").map((i: any) => ({path: i.path as string, kind: i.kind as string}));
  } catch { return []; }
};

const list = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
const out: any[] = [];
for (const m of list) {
  const dir = dirname(join(ROOT, m));
  const node = loadNode(dir);
  if (!node?.libEntry) continue;
  const allFeatures = new Set<string>(Object.keys(node.table).filter(f => f !== "default"));
  const normal = new Set<string>(), test = new Set<string>();
  for (const label of LABELS) {
    const feats = new Set<string>([...allFeatures, "default"]);
    for (const p of await closure(node.libEntry, node, cfgList(label, feats, false))) normal.add(p.path);
    for (const t of targetsOf(node, feats)) {
      if (t.kind === "example") continue;
      for (const p of await closure(t.entry, node, cfgList(label, feats, t.test))) test.add(p.path);
    }
  }
  const refs: any[] = [];
  const tokens: any[] = [];
  const files = new Set<string>([...normal, ...test]);
  const rootCfgTest = /#!\[\s*cfg\s*\(\s*test\s*\)\s*\]/.test(mask(readFileSync(node.libEntry, "utf8")));
  for (const p of files) {
    if (!p.endsWith(".rs")) continue;
    const abs = p.startsWith("/") ? p : join(ROOT, p);
    if (!existsSync(abs)) continue;
    const text = readFileSync(abs, "utf8");
    const masked = mask(text);
    const regions = testRegions(masked);
    const tk = /\b(apply_tree|publish_document)\s*\(/g;
    let tm: RegExpExecArray | null;
    while ((tm = tk.exec(masked))) { const reg = inRegions(regions, tm.index); const w = !normal.has(p) || rootCfgTest ? "testfile" : reg ? (cfgCompiledWithoutTest(reg.attr) ? "testregion-feature" : "testregion") : "normal"; let d = 0, commas = 0, j = tm.index + tm[0].length; for (; j < masked.length; j++) { const ch = masked[j]!; if (ch === "(" || ch === "[" || ch === "{") d++; else if (ch === ")" || ch === "]" || ch === "}") { if (d === 0) break; d--; } else if (ch === "," && d === 0) commas++; } const empty = masked.slice(tm.index + tm[0].length, j).trim() === ""; tokens.push({file: rel(abs), line: lineOf(masked, tm.index), token: tm[1], args: empty ? 0 : commas + 1, where: w}); }
    if (!text.includes(alias)) continue;
    const testOnlyFile = !normal.has(p) || rootCfgTest;
    for (const r of findAliasRefs(masked, alias)) {
      const reg = inRegions(regions, r.pos);
      const where = testOnlyFile ? "testfile" : reg ? (cfgCompiledWithoutTest(reg.attr) ? "testregion-feature" : "testregion") : "normal";
      refs.push({file: rel(abs), line: r.line, form: r.form, where, attr: reg?.attr, segs: r.segs, tree: r.tree, vis: r.vis, text: r.text, stmt: r.stmtStart === undefined ? undefined : [r.stmtStart, r.stmtEnd]});
    }
  }
  out.push({crate: node.name, manifest: m, features: [...allFeatures], normalFiles: normal.size, testFiles: test.size, rootCfgTest, refs, tokens});
}
writeFileSync(process.argv[3]!, JSON.stringify(out));
const tot = (w: string) => out.reduce((s, c) => s + c.refs.filter((r: any) => r.where === w).length, 0);
console.log(out.length, "crates; refs: normal", tot("normal"), "testregion", tot("testregion"), "testregion-feature", tot("testregion-feature"), "testfile", tot("testfile"));
