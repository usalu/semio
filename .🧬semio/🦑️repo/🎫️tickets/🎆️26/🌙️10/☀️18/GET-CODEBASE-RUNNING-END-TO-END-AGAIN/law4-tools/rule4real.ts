import { existsSync, readFileSync, realpathSync } from "node:fs";
import { dirname, join, resolve, relative } from "node:path";
import { crateOf } from "./crateof.ts";
const root = "/Users/ueli/Documents/semio";
const fixture = JSON.parse(readFileSync(join(root, "✏️s/🔌️plugins/🧪️tests/📁fixture-placement/🧫️fixtures/🔣️.json"), "utf8"));
const parts = (f: string) => f.split(/[\\/]/u);
const accepted = (file: string): boolean => { const p = parts(file), t = p.indexOf("🧪️tests"), e = p.indexOf("🧫️fixtures"); return t >= 0 && e > t && !p.slice(0, e).some(x => ["🧬️schema", "🖼️assets"].includes(x)); };
const re = /concat!\(\s*env!\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]*🧫️fixtures[^"]*)"\s*\)/gu;
const good = (mf: string, lits: string[]) => lits.every(l => { const t = resolve(dirname(mf) + l); return existsSync(t) && accepted(realpathSync(t)); });
for (const [i, row] of fixture.manifestReaders.entries()) {
  const file = join(root, row.path); if (!existsSync(file)) { console.log(i, "NOFILE"); continue; }
  const lits = [...readFileSync(file, "utf8").matchAll(re)].map(m => m[1]!);
  const cs = crateOf(file).map(c => join(root, c));
  const cur = join(root, row.manifest);
  const curOk = existsSync(cur) && good(cur, lits);
  const okc = cs.filter(c => good(c, lits));
  const flag = curOk ? "ok " : "BAD";
  if (!curOk || process.env.ALL) console.log(i, flag, "lits", lits.length, "real", cs.length, "realOk", okc.length, "\n   file", row.path.slice(-80), "\n   cur ", row.manifest.slice(-70), "\n   real", cs.map(c => relative(root, c).slice(-90)));
}
const upd: Record<string, string> = {};
for (const row of fixture.manifestReaders) {
  const file = join(root, row.path); if (!existsSync(file)) continue;
  const lits = [...readFileSync(file, "utf8").matchAll(re)].map(m => m[1]!);
  const cur = join(root, row.manifest);
  if (existsSync(cur) && good(cur, lits)) continue;
  const cs = crateOf(file).map(c => join(root, c)).filter(c => good(c, lits));
  if (cs.length === 1) upd[row.path] = relative(root, cs[0]!);
}
require("node:fs").writeFileSync(process.env.OUT!, JSON.stringify(upd));
