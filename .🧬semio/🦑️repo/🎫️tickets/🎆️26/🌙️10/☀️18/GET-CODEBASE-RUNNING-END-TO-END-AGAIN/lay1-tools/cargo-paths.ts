import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
const root = "/Users/ueli/Documents/semio";
const skip = new Set(["node_modules", "target", "pkg", "dist", "📤️dist", ".git", "🗑️generated", ".cache", "bazel-out", "temp", "🎫️tickets"]);
let bad = 0, checked = 0;
const walk = (d: string) => {
  let es; try { es = readdirSync(d, { withFileTypes: true }); } catch { return; }
  for (const e of es) {
    if (skip.has(e.name)) continue;
    const p = join(d, e.name);
    if (e.isDirectory()) walk(p);
    else if (e.name === "Cargo.toml") {
      const t = readFileSync(p, "utf8");
      for (const m of t.matchAll(/path\s*=\s*"([^"]+)"/gu)) { checked++; const target = resolve(dirname(p), m[1]!); if (!existsSync(target)) { bad++; if (bad <= 40) console.log("MISSING", p.slice(root.length + 1), "->", m[1]); } }
      const mem = /members\s*=\s*\[([^\]]*)\]/su.exec(t);
      if (mem) for (const m of mem[1]!.matchAll(/"([^"]+)"/gu)) { if (/[*]/u.test(m[1]!)) continue; checked++; const target = resolve(dirname(p), m[1]!); if (!existsSync(target)) { bad++; if (bad <= 40) console.log("MISSING member", p.slice(root.length + 1), "->", m[1]); } }
    }
  }
};
for (const r of ["✏️s", "🧰️framework", "🌎️hub", "♻️mit-bestand", "🎓️teaching"]) walk(join(root, r));
for (const f of ["Cargo.toml"]) { const p = join(root, f); if (existsSync(p)) { const t = readFileSync(p, "utf8"); for (const m of t.matchAll(/(?:path\s*=|")\s*"?([^"\s,\]]+)"/gu)) {} } }
console.log(`cargo path/member entries checked ${checked}, missing ${bad}`);
