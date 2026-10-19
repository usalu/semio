import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
const root = "/Users/ueli/Documents/semio";
const skip = new Set(["node_modules", "target", "pkg", "dist", "📤️dist", ".git", "🗑️generated", ".cache", "bazel-out", "temp"]);
const needles = process.argv.slice(2);
const walk = (d: string) => {
  let es; try { es = readdirSync(d, { withFileTypes: true }); } catch { return; }
  for (const e of es) {
    if (skip.has(e.name)) continue;
    const p = join(d, e.name);
    if (e.isDirectory()) walk(p);
    else if (/\.(ts|tsx|mts|cts|json|jsonc|mjs|cjs|js|rs|toml|go|md|py|sh|yml|yaml|bzl|bazel|lock)$/u.test(e.name) || e.name === "Cargo.lock" || e.name === "BUILD") {
      let t: string; try { t = readFileSync(p, "utf8"); } catch { continue; }
      for (const n of needles) { const i = t.indexOf(n); if (i >= 0) { const ln = t.slice(0, i).split("\n").length; console.log(`${p.slice(root.length + 1)}:${ln}  [${n.slice(-30)}]`); break; } }
    }
  }
};
for (const r of ["✏️s", "🧰️framework", "🌎️hub", "♻️mit-bestand", "🎓️teaching", ".🧬semio/🦑️repo/🔨️modules", ".vscode", ".github", ".devcontainer"]) walk(join(root, r));
for (const f of ["Cargo.toml", "package.json", "nx.json", "MODULE.bazel", "go.work", "tsconfig.json", "pyproject.toml"]) { try { const t = readFileSync(join(root, f), "utf8"); for (const n of needles) if (t.includes(n)) console.log(`${f}  [${n.slice(-30)}]`); } catch {} }
