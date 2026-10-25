import { readFileSync, existsSync } from "node:fs";
import { dirname, join, resolve, relative } from "node:path";
import { parse } from "/Users/ueli/Documents/semio/node_modules/smol-toml/dist/index.js";
const root = "/Users/ueli/Documents/semio";
const [, , crate, truthFile, outFile, product] = process.argv;
const truth = JSON.parse(readFileSync(truthFile, "utf8"));
const manifests: string[] = product ? truth[product].rust.find((e: any) => e.name === crate).users : truth.literal.find((e: any) => e.ecosystem === "rust" && e.name === crate).oracleConflictUsers;
const ident = crate.replaceAll("-", "_");
const pat = new RegExp(`\\b${ident}\\b`);
const out: Record<string, { files: number; uses: string[] }> = {};
const codeLine = (text: string): boolean => text.split("\n").some((line) => !/^\s*(\/\/|\*)/.test(line) && pat.test(line));
function sources(manifest: string): string[] {
  const t: any = parse(readFileSync(manifest, "utf8"));
  const lib = t.lib?.path ?? "src/lib.rs";
  const seen = new Set<string>(), queue = [resolve(dirname(manifest), lib)];
  for (const b of (t.bin ?? [])) if (b.path) queue.push(resolve(dirname(manifest), b.path));
  while (queue.length) {
    const f = queue.pop()!;
    if (seen.has(f) || !existsSync(f)) continue;
    seen.add(f);
    const text = readFileSync(f, "utf8");
    for (const m of text.matchAll(/#\[path\s*=\s*"([^"]+)"\]/g)) queue.push(resolve(dirname(f), m[1]!));
    for (const m of text.matchAll(/^\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z0-9_]+)\s*;/gm)) { for (const c of [join(dirname(f), `${m[1]}.rs`), join(dirname(f), m[1]!, "mod.rs")]) if (existsSync(c)) queue.push(c); }
  }
  return [...seen];
}
for (const mf of manifests) {
  const abs = join(root, mf);
  try {
    const files = sources(abs);
    const uses: string[] = [];
    for (const f of files) { const text = readFileSync(f, "utf8"); const cfgTest = f.includes("🧪️tests"); if (codeLine(text) && !cfgTest) uses.push(relative(root, f).slice(-60)); }
    out[mf] = { files: files.length, uses };
  } catch (e) { out[mf] = { files: -1, uses: [String(e).slice(0, 80)] }; }
}
import { writeFileSync } from "node:fs";
writeFileSync(outFile, JSON.stringify(out));
const used = Object.entries(out).filter(([, v]) => v.uses.length > 0).length, err = Object.values(out).filter((v) => v.files < 0).length;
console.log("crates", manifests.length, "used", used, "errors", err);
