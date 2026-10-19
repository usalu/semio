import { existsSync, readFileSync, realpathSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const root = "/Users/ueli/Documents/semio";
const fixture = JSON.parse(readFileSync(join(root, "✏️s/🔌️plugins/🧪️tests/📁fixture-placement/🧫️fixtures/🔣️.json"), "utf8"));
const parts = (f: string) => f.split(/[\\/]/u);
const accepted = (file: string): boolean => { const p = parts(file), t = p.indexOf("🧪️tests"), e = p.indexOf("🧫️fixtures"); return t >= 0 && e > t && !p.slice(0, e).some(x => ["🧬️schema", "🖼️assets"].includes(x)); };
console.log(fixture.manifestReaders.length, "readers");
let i = 0;
for (const row of fixture.manifestReaders) {
  const file = join(root, row.path), manifest = join(root, row.manifest);
  if (!existsSync(manifest) || !existsSync(file)) { console.log("NOFILE", row.path, existsSync(manifest), existsSync(file)); continue; }
  for (const m of readFileSync(file, "utf8").matchAll(/concat!\(\s*env!\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"]*🧫️fixtures[^"]*)"\s*\)/gu)) {
    const target = resolve(dirname(manifest) + m[1]!);
    const ok = existsSync(target) && accepted(realpathSync(target));
    if (!ok || process.env.ALL) console.log(i, ok ? "ok" : existsSync(target) ? "UNPLACED" : "MISSING", row.path.replace("✏️s/🔌️plugins/", "P/"), "\n    manifest:", row.manifest.replace("✏️s/🔌️plugins/", "P/"), "\n    literal:", m[1]);
  }
  i++;
}
