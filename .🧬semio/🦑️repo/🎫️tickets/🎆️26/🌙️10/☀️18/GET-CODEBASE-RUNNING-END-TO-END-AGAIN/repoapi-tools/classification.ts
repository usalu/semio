import { resolve } from "node:path";
import { classify } from "./classify.ts";
import { buildGlue } from "./glue.ts";
import { isRepoProduct, libRelOf, entryOf } from "./rules.ts";
import { TICKET, writeJson } from "./lib.ts";

const r = classify();
type Row = { target: string; name: string; class: "a" | "b" | "c"; to: string; importers: { repoProduct: number; other: number }; otherKinds?: { test: number; script: number; source: number }; testOnlyOutsideRepo?: boolean };
const rows = new Map<string, Row>();
const kindOf = (f: string): "test" | "script" | "source" => (/📜️script\.ts$/u.test(f) ? "script" : /\/🧪️tests?\/|\/🧪️testing\/|vitest\.config|🎚️config\//u.test(f) ? "test" : "source");
let currentFile = "";
const bump = (key: string, init: Row, repo: boolean): void => {
  const x = rows.get(key) ?? init; rows.set(key, x);
  if (repo) x.importers.repoProduct++;
  else { x.importers.other++; (x.otherKinds ??= { test: 0, script: 0, source: 0 })[kindOf(currentFile)]++; }
};
for (const rep of r.reports) {
  const repo = isRepoProduct(rep.file);
  currentFile = rep.file;
  for (const a of rep.a) bump(`a|${libRelOf(a.target)}|${a.name}`, { target: libRelOf(a.target), name: a.name, class: "a", to: a.to, importers: { repoProduct: 0, other: 0 } }, repo);
  for (const d of rep.dead) bump(`c|${libRelOf(d.target)}|${d.name}`, { target: libRelOf(d.target), name: d.name, class: "c", to: "delete: unused import", importers: { repoProduct: 0, other: 0 } }, repo);
  for (const o of rep.open) bump(`c|${libRelOf(o.target)}|*`, { target: libRelOf(o.target), name: "*", class: "c", to: "delete: " + o.reason + " (move the test to the owner or give it its own fixture)", importers: { repoProduct: 0, other: 0 } }, repo);
  for (const [t, names] of rep.b) for (const n of names) bump(`b|${libRelOf(t)}|${n}`, { target: libRelOf(t), name: n, class: "b", to: entryOf(libRelOf(t)).key, importers: { repoProduct: 0, other: 0 } }, repo);
}
for (const x of rows.values()) if (x.class === "b" && x.otherKinds && x.otherKinds.script === 0 && x.otherKinds.source === 0 && x.otherKinds.test > 0) x.testOnlyOutsideRepo = true;
const all = [...rows.values()].sort((x, y) => x.class.localeCompare(y.class) || x.target.localeCompare(y.target) || x.name.localeCompare(y.name));
writeJson(resolve(TICKET, "repoapi-tools/classification.json"), { generatedAt: new Date().toISOString(), counts: { a: all.filter(x => x.class === "a").length, b: all.filter(x => x.class === "b").length, c: all.filter(x => x.class === "c").length }, rows: all });
const edges = { a: 0, b: 0, c: 0 };
for (const x of all) edges[x.class] += x.importers.repoProduct + x.importers.other;
console.log("distinct symbols", all.length, "import specifiers", edges);
const dom = new Map<string, { names: number; importers: number }>();
for (const x of all.filter(y => y.class === "b")) { const d = dom.get(x.to) ?? { names: 0, importers: 0 }; dom.set(x.to, d); d.names++; d.importers += x.importers.repoProduct + x.importers.other; }
console.log([...dom].sort());
const to = new Map<string, number>();
for (const x of all.filter(y => y.class === "a")) to.set(x.to, (to.get(x.to) ?? 0) + x.importers.repoProduct + x.importers.other);
console.log([...to].sort((a, b) => b[1] - a[1]));
void buildGlue;

const testOnly = all.filter(x => x.testOnlyOutsideRepo);
console.log("b symbols whose non-repo importers are all tests:", testOnly.length, "of", all.filter(x => x.class === "b" && x.importers.other > 0).length, "consumed outside the repo product");
