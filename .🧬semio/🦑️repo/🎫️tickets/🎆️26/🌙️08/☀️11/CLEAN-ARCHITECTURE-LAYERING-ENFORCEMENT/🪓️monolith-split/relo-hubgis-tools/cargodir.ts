/** 🧭️ Static Cargo direction scan: every path dependency of the touched manifests points to the same or a lower layer. */
import { readFileSync, existsSync, realpathSync } from "node:fs";
import { dirname, join, relative } from "node:path";
const R = "/Users/ueli/Documents/semio/";
const layer = (p: string): [number, string] => {
  const r = relative(R, existsSync(p) ? realpathSync(p) : p);
  if (r.startsWith("🧰️framework/🛍️products/")) return [1, "os-product"];
  if (r.startsWith("🧰️framework/")) return [0, "framework"];
  if (r.startsWith("✏️s/")) return [2, "plugin"];
  if (/^(🌎️hub|♻️mit-bestand|🏢️semio-tech)/u.test(r)) return [3, "product"];
  return [9, "?"];
};
const ws = (Bun.TOML.parse(readFileSync(R + "🌎️hub/Cargo.toml", "utf8")) as any).workspace.dependencies;
let bad = 0;
for (const m of process.argv.slice(2)) {
  const man = Bun.TOML.parse(readFileSync(R + m, "utf8")) as any;
  const base = dirname(R + m);
  const [L] = layer(base);
  let count = 0;
  for (const sect of ["dependencies", "dev-dependencies"]) {
    for (const [name, spec] of Object.entries<any>(man[sect] ?? {})) {
      let d: string | undefined;
      if (spec && typeof spec === "object" && spec.workspace) d = ws[name]?.path ? join(R + "🌎️hub", ws[name].path) : undefined;
      else if (spec && typeof spec === "object" && spec.path) d = join(base, spec.path);
      if (!d) continue;
      count++;
      const [dl, dn] = layer(d);
      if (dl > L) { bad++; console.log("VIOLATION", m, sect, name, dn, relative(R, d)); }
      if (!existsSync(join(d, "Cargo.toml"))) { bad++; console.log("MISSING", m, sect, name, relative(R, d)); }
    }
  }
  console.log("ok", m, "layer", L, "path deps", count);
}
console.log("violations", bad);
