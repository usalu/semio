import { readFileSync } from "node:fs";
import { join } from "node:path";
import { dirname, resolve } from "node:path";
import fastGlob from "/Users/ueli/Documents/semio/node_modules/fast-glob";
function pluginLibRoots(root: string): Map<string, number> {
  const roots = new Map<string, number>();
  const manifests = fastGlob.sync("✏️s/🔌️plugins/**/Cargo.toml", { cwd: root, ignore: ["**/node_modules/**", "**/target/**", "**/dist/**"], onlyFiles: true, followSymbolicLinks: false });
  for (const manifest of manifests) {
    const parsed = Bun.TOML.parse(readFileSync(join(root, manifest), "utf8")) as { lib?: { path?: string } };
    if (!parsed.lib?.path) continue;
    const lib = resolve(dirname(join(root, manifest)), parsed.lib.path).slice(root.length + 1);
    try { roots.set(lib, readFileSync(join(root, lib), "utf8").split("\n").length); } catch { continue; }
  }
  return roots;
}
const root = "/Users/ueli/Documents/semio";
const fx = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📏️lib-wiring-ratchet/🔣️.json");
const ratchet = JSON.parse(readFileSync(fx, "utf8"));
const roots = pluginLibRoots(root);
const budget = ratchet.budget;
for (const [p, n] of roots) {
  if (n <= budget) continue;
  const pin = ratchet.entries[p];
  if (pin === undefined) console.log("UNPINNED", n, p);
  else if (n > pin) console.log("GREW", pin, n, p);
}
for (const [p, pin] of Object.entries(ratchet.entries) as [string, number][]) {
  const n = roots.get(p);
  if (n === undefined) console.log("VANISHED", pin, p);
  else if (n <= budget) console.log("COMPLIES", pin, n, p);
  else if (n < pin) console.log("LOWER", pin, n, p);
}
