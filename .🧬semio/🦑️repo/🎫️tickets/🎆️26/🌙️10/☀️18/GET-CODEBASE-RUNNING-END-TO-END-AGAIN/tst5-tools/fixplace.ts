import { readdirSync, realpathSync } from "node:fs";
import { join, resolve } from "node:path";
import fixture from "../../../../../../../../✏️s/🔌️plugins/🧪️tests/📁fixture-placement/🧫️fixtures/🔣️.json";
const plugins = "/Users/ueli/Documents/semio/✏️s/🔌️plugins";
const parts = (f: string) => f.split(/[\\/]/u);
const opaque = new Set(["node_modules", "target", "dist", ".git", ".nx", "🗑️generated", "🤖️generated"]);
const walk = (d: string): string[] => readdirSync(d, { withFileTypes: true }).flatMap(e => e.isDirectory() && !opaque.has(e.name) ? walk(join(d, e.name)) : e.isFile() ? [join(d, e.name)] : []);
const accepted = (file: string) => { const p = parts(file), t = p.indexOf("🧪️tests"), x = p.indexOf("🧫️fixtures"); return t >= 0 && x > t && !p.slice(0, x).some(q => ["🧬️schema", "🖼️assets"].includes(q)); };
for (const o of fixture.owners) for (const f of walk(join(plugins, o))) if (parts(f).includes("🧫️fixtures") && !accepted(realpathSync(f))) console.log(f.slice(plugins.length + 1));
