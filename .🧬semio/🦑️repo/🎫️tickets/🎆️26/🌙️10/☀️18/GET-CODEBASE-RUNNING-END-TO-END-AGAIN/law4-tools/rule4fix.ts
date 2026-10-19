import { existsSync, readFileSync, realpathSync, readdirSync } from "node:fs";
import { dirname, join, resolve, relative, basename } from "node:path";
const root = "/Users/ueli/Documents/semio";
const fixture = JSON.parse(readFileSync(join(root, "✏️s/🔌️plugins/🧪️tests/📁fixture-placement/🧫️fixtures/🔣️.json"), "utf8"));
const parts = (f: string) => f.split(/[\\/]/u);
const accepted = (file: string): boolean => { const p = parts(file), t = p.indexOf("🧪️tests"), e = p.indexOf("🧫️fixtures"); return t >= 0 && e > t && !p.slice(0, e).some(x => ["🧬️schema", "🖼️assets"].includes(x)); };
const re = /concat!\(\s*env!\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]*🧫️fixtures[^"]*)"\s*\)/gu;
for (const row of fixture.manifestReaders) {
  const file = join(root, row.path), manifest = join(root, row.manifest);
  if (!existsSync(file)) { console.log("NOFILE", row.path); continue; }
  const lits = [...readFileSync(file, "utf8").matchAll(re)].map(m => m[1]!);
  const good = (mf: string) => lits.every(l => { const t = resolve(dirname(mf) + l); return existsSync(t) && accepted(realpathSync(t)); });
  if (existsSync(manifest) && good(manifest)) continue;
  const cands: string[] = [];
  let d = dirname(file);
  while (d.length > root.length) {
    for (const c of [join(d, "Cargo.toml"), join(d, "📦️packages/🦀️rust/Cargo.toml")]) if (existsSync(c)) cands.push(c);
    d = dirname(d);
  }
  const ok = cands.filter(good);
  console.log(row.path.replace("✏️s/🔌️plugins/", "P/").slice(-110), "\n   lits", lits.length, "cands", cands.length, "ok", ok.map(c => relative(root, c).replace("✏️s/🔌️plugins/", "P/").slice(-100)));
}
