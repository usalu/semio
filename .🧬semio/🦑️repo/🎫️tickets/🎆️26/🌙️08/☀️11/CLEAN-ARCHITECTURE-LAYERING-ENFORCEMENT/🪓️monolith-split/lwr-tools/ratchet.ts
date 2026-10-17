import { readFileSync, writeFileSync, renameSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import fastGlob from "/Users/ueli/Documents/semio/node_modules/fast-glob";
const root = "/Users/ueli/Documents/semio";
const fx = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json");
const apply = process.argv.includes("--apply");
const roots = new Map<string, number>();
for (const manifest of fastGlob.sync("✏️s/🔌️plugins/**/Cargo.toml", { cwd: root, ignore: ["**/node_modules/**", "**/target/**", "**/dist/**"], onlyFiles: true, followSymbolicLinks: false })) {
  const parsed = Bun.TOML.parse(readFileSync(join(root, manifest), "utf8")) as { lib?: { path?: string } };
  if (!parsed.lib?.path) continue;
  const lib = resolve(dirname(join(root, manifest)), parsed.lib.path).slice(root.length + 1);
  try { roots.set(lib, readFileSync(join(root, lib), "utf8").split("\n").length); } catch { continue; }
}
const raw = readFileSync(fx, "utf8");
const ratchet = JSON.parse(raw);
const budget: number = ratchet.budget;
let removed = 0, lowered = 0, kept = 0, grew = 0;
for (const [p, pin] of Object.entries(ratchet.entries) as [string, number][]) {
  const n = roots.get(p);
  const short = p.split("/").slice(-5).join("/");
  if (n === undefined) { console.log("VANISHED-REMOVE", pin, short); delete ratchet.entries[p]; removed++; }
  else if (n <= budget) { console.log("REMOVE", pin, "->", n, short); delete ratchet.entries[p]; removed++; }
  else if (n < pin) { console.log("LOWER", pin, "->", n, short); ratchet.entries[p] = n; lowered++; }
  else if (n > pin) { console.log("GREW-NOT-TOUCHED", pin, n, short); grew++; }
  else kept++;
}
console.log({ removed, lowered, kept, grew });
if (apply) {
  const again = readFileSync(fx, "utf8");
  if (again !== raw) throw new Error("fixture changed while computing");
  const out = JSON.stringify(ratchet, null, 2) + (raw.endsWith("\n") ? "\n" : "");
  writeFileSync(fx + ".lwr-tmp", out);
  renameSync(fx + ".lwr-tmp", fx);
  console.log("written");
}
