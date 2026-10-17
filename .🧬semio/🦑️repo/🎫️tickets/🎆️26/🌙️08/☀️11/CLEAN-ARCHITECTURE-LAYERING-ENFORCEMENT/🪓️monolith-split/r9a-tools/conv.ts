// 🔭️ Runs only the mutation-family conversion on the artifact lib root (no writes) and prints what converted and what stays by hand.
import { readFileSync, existsSync } from "node:fs";
import { resolve, dirname } from "node:path";
import { convertMutationLeaves } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🧬️mutation-families/🟦️.ts";
const root = "/Users/ueli/Documents/semio/" + process.argv[2]!;
const source = readFileSync(root, "utf8");
const r = convertMutationLeaves(root, source, (p) => (existsSync(p) ? readFileSync(p, "utf8") : null));
console.log("aggregates", r.aggregates.map((a) => `${a.name}:${a.variants.length}`));
console.log("converted", r.converted.length, "hand", r.hand.length, "edits", r.edits.size, "creates", r.creates.size);
for (const n of r.notes.slice(0, 60)) console.log("note:", n.slice(0, 300));
for (const h of r.hand.slice(0, 80)) console.log("hand:", h.payload, h.reason.slice(0, 200));
const want = process.env.SHOW;
if (want) for (const [file, text] of [...r.edits, ...r.creates]) if (file.includes(want)) { console.log("=== " + file.slice(-90)); console.log(text.split("\n").slice(0, 40).join("\n")); }
if (process.env.RESIDUAL) {
  const { readdirSync, statSync } = await import("node:fs");
  const base = root.replace(/🦀️\.rs$/u, "") + "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/";
  const out: string[] = [];
  for (const leaf of readdirSync(base)) {
    const dir = base + leaf;
    if (!statSync(dir).isDirectory()) continue;
    for (const sub of readdirSync(dir)) {
      const file = `${dir}/${sub}/🦀️.rs`;
      if (!existsSync(file)) continue;
      const text = r.edits.get(file) ?? r.creates.get(file) ?? readFileSync(file, "utf8");
      const code = text.split("\n").filter((line) => !line.trim().startsWith("//")).join("\n");
      if (/\bModelMutation\b/u.test(code)) out.push(`${leaf}/${sub} edited=${r.edits.has(file)}`);
    }
  }
  console.log("RESIDUAL " + out.length);
  for (const line of out) console.log("RES " + line);
}
