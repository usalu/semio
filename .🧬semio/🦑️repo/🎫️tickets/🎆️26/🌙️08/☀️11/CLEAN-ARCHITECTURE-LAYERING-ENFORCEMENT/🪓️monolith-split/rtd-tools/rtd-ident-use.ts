// 🔎️ Where is a dependency identifier used in a crate's compiled sources (all labels, all features, lib and tests)? argv: manifest dir (repo-relative), ident regex. Prints "<kind> <file>:<line>" per use.
// @see ../RESULT-RENDERER-TEST-DEPS.md
import {existsSync, readFileSync} from "node:fs";
import {join} from "node:path";
import {LABELS, ROOT, cfgList, loadNode, readFile, rel, targetsOf} from "./rtd-lib.ts";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import {cfgCompiledWithoutTest, inRegions, lineOf, mask, testRegions} from "./rtd-rust.ts";
const tolerant = async (p: string) => (await readFile(p)) ?? new Uint8Array(0);
const node = loadNode(join(ROOT, process.argv[2]!))!;
const rx = new RegExp(`(?<![A-Za-z0-9_:.$])(?:${process.argv[3]})\\s*::`, "g");
const feats = new Set<string>([...Object.keys(node.table), "default"]);
const normal = new Set<string>(), test = new Set<string>();
const clo = async (entry: string, cfg: string[]) => { try { return (await readRustSourceClosureV1([entry], ROOT, cfg, tolerant, {manifestDirectory: node.dir, generated: () => undefined}) as any[]).filter(i => i.kind === "source").map(i => i.path as string); } catch { return []; } };
for (const label of LABELS) { for (const p of await clo(node.libEntry!, cfgList(label, feats, false))) normal.add(p); for (const t of targetsOf(node, feats)) if (t.kind !== "example") for (const p of await clo(t.entry, cfgList(label, feats, t.test))) test.add(p); }
let n = 0;
for (const p of new Set([...normal, ...test])) {
  const abs = p.startsWith("/") ? p : join(ROOT, p);
  if (!existsSync(abs)) continue;
  const m = mask(readFileSync(abs, "utf8"));
  const regs = testRegions(m);
  for (const x of m.matchAll(rx)) { const reg = inRegions(regs, x.index!); const kind = !normal.has(p) ? "test" : reg ? (cfgCompiledWithoutTest(reg.attr) ? "prod(feature)" : "test") : "prod"; console.log(`${kind} ${rel(abs).split("/").slice(-3).join("/")}:${lineOf(m, x.index!)}`); n++; }
}
console.log(n, "uses");
