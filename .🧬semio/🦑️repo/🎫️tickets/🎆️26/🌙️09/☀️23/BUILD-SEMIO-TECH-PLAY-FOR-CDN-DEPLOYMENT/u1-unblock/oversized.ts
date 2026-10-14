/** 📏️ U1: lists every workspace lib unit above the Source ceiling in the dependency closure of named root packages (canonical helper, native cfg). bun oversized.ts <metadata.json> <root-package>... */
import { readFileSync } from "node:fs";
import { dirname } from "node:path";
import { readRustSourceClosureV1 } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const [metaPath, ...roots] = process.argv.slice(2);
const meta = JSON.parse(readFileSync(metaPath, "utf8"));
const R = "/Users/ueli/Documents/semio";
const cfg = (await Bun.$`rustc --print cfg`.text()).trim().split("\n");
const pkgs = new Map<string, any>(meta.packages.map((p: any) => [p.id, p]));
const nodes = new Map<string, any>(meta.resolve.nodes.map((n: any) => [n.id, n]));
const seen = new Set<string>(); const stack = meta.packages.filter((p: any) => roots.includes(p.name)).map((p: any) => p.id);
while (stack.length) { const id = stack.pop()!; if (seen.has(id)) continue; seen.add(id); for (const d of nodes.get(id)?.deps ?? []) if (d.dep_kinds.some((k: any) => k.kind !== "dev")) stack.push(d.pkg); }
const rows: string[] = [];
for (const id of seen) {
  const p = pkgs.get(id); if (p.source) continue;
  const lib = p.targets.find((t: any) => t.kind.some((k: string) => ["lib", "rlib", "proc-macro", "cdylib"].includes(k))); if (!lib) continue;
  const features = (nodes.get(id)?.features ?? []).map((f: string) => `feature="${f}"`);
  try {
    const inputs = await readRustSourceClosureV1([lib.src_path], R, [...cfg, ...features], async path => { try { return readFileSync(path); } catch { return null; } }, { manifestDirectory: dirname(p.manifest_path), generated: () => {} });
    const lines = inputs.reduce((s, i) => s + i.activeLines, 0), bytes = inputs.reduce((s, i) => s + i.bytes, 0);
    if (lines > 4000 || bytes > 262144 || inputs.length > 32) rows.push(`${p.name}\t${inputs.length}\t${lines}\t${bytes}\t${p.manifest_path.replace(R + "/", "")}`);
  } catch (error) { rows.push(`${p.name}\tERROR\t${(error as Error).message.slice(0, 160)}`); }
}
console.log(`closure ${seen.size} packages; oversized or unreadable ${rows.length}`);
console.log(rows.sort().join("\n"));
