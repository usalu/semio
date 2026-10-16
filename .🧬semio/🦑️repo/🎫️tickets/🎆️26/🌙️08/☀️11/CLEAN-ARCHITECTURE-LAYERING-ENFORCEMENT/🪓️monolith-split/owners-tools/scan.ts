/** 🔎 Lists Cargo packages whose directory is neither an owner root nor an owner nativeRoot. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { spawnSync } from "node:child_process";
const root = process.cwd();
const reg = JSON.parse(readFileSync(join(root, "📋️build-owners.json"), "utf8"));
const roots = new Map<string, string>();
for (const o of reg.owners) for (const r of [o.root, ...(o.nativeRoots ?? [])]) roots.set(r, o.name);
const out = spawnSync("git", ["-c", "core.quotepath=off", "ls-files", "-co", "--exclude-standard", "-z", "--", "**/Cargo.toml", "Cargo.toml"], { cwd: root, maxBuffer: 1 << 28 }).stdout.toString("utf8");
const found: string[] = [];
for (const f of out.split("\0")) {
  if (!f) continue;
  let text = "";
  try { text = readFileSync(join(root, f), "utf8"); } catch { continue; }
  if (/^\[package\]/m.test(text)) found.push(dirname(f));
}
const missing = found.filter((d) => !roots.has(d) && !d.startsWith(".🧬semio/")).sort();
console.log(JSON.stringify({ packages: found.length, owners: reg.owners.length, missing }, null, 1));
