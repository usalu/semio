import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
const ROOT = process.cwd();
const skip = new Set(["node_modules", "dist", "target", "temp", "pkg", "storybook-static", "🤖️generated", "🗑️generated", "🎫️tickets", ".git", ".cache", ".🧬semio", "bazel-out"]);
export const t0 = performance.now();
export function walkManifests(): string[] {
  const out: string[] = [];
  const walk = (rel: string) => {
    let entries; try { entries = readdirSync(join(ROOT, rel), { withFileTypes: true }); } catch { return; }
    for (const e of entries) {
      if (e.isDirectory() && !e.isSymbolicLink()) { if (!skip.has(e.name) && !e.name.startsWith(".")) walk(rel ? rel + "/" + e.name : e.name); }
      else if (e.isFile() && e.name === "Cargo.toml") out.push(rel ? rel + "/" + e.name : e.name);
    }
  };
  walk("");
  return out;
}
if (import.meta.main) {
  const m = walkManifests();
  console.log("manifests", m.length, Math.round(performance.now() - t0), "ms");
  const rows: any[] = [];
  for (const f of m) { const t = readFileSync(join(ROOT, f), "utf8"); if (!t.includes("semio.playground")) continue; let toml: any; try { toml = Bun.TOML.parse(t); } catch (e) { console.log("parse fail", f); continue; } for (const r of toml.package?.metadata?.semio?.playground ?? []) rows.push({ ...r, f }); }
  console.log(rows.length, JSON.stringify(rows[0]), new Set(rows.map(r => r.variant)).size);
  console.log(JSON.stringify(rows.find(r => r.variant === "s")));
}
