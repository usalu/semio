// 🚧️ Lists owner crates whose cfg gates name a feature that the owner's Cargo.toml does not declare.
import {readFileSync, existsSync} from "node:fs";
import {dirname} from "node:path";
import {execFileSync} from "node:child_process";
const ROOT = "/Users/ueli/Documents/semio/";
const gated = JSON.parse(readFileSync(process.argv[2] + "-gated.json", "utf8")) as any[];
const hits = JSON.parse(readFileSync(process.argv[2] + "-hits.json", "utf8")) as any[];
const used = new Set(hits.map(h => h.ownerCrate));
const want = new Map<string, {crate: string; features: Set<string>; dir: string}>();
for (const g of gated) {
  if (!g.gate.includes("feature")) continue;
  const feats = [...g.gate.matchAll(/feature = ([\w-]+)/g)].map(m => m[1]!);
  const dir = ROOT + dirname(dirname(g.file));
  const k = g.crate;
  let e = want.get(k);
  if (!e) want.set(k, e = {crate: g.crate, features: new Set(), dir: ""});
  for (const f of feats) e.features.add(f);
}
const manifests = new Map<string, string>();
for (const line of execFileSync("bun", [import.meta.dir + "/ctc-find.ts", ...want.keys()], {encoding: "utf8"}).split("\n").filter(Boolean)) { const [n, m] = line.split("\t") as [string, string]; manifests.set(n, m); }
let bad = 0;
for (const [name, e] of want) {
  const m = manifests.get(name);
  if (!m) continue;
  const t = Bun.TOML.parse(readFileSync(ROOT + m, "utf8")) as any;
  const declared = new Set<string>(Object.keys(t.features ?? {}));
  for (const sect of [t.dependencies, t["dev-dependencies"], t["build-dependencies"], ...Object.values(t.target ?? {}).flatMap((x: any) => [x.dependencies, x["dev-dependencies"]])]) for (const [k, v] of Object.entries(sect ?? {}) as [string, any][]) if (v && typeof v === "object" && v.optional === true) declared.add(k);
  const missing = [...e.features].filter(f => !declared.has(f));
  if (missing.length) { bad++; console.log(name + "\t" + missing.join(",") + "\t" + m); }
}
console.log(`${bad} crates with undeclared gate features`);
