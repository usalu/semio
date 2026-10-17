// 🧾️ Lists the names the sequence editor children provide to the root glob (pub items, pub uses, pub mods) and the private uses of the root.
import { readFileSync } from "node:fs";
const L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree";
const { parseRustSourceTree, useLeafNames } = await import(`${L}/🌳️module-tree/🟦️.ts`);
const E = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor";
const kids: Record<string, string> = { base: "🧱️base", host: "🖥️host", retained: "🧵️retained", persistent: "🔁️persistent", import: "🎞️import", app: "🎹️app" };
for (const [name, dir] of Object.entries(kids)) {
  const text = readFileSync(`${E}/${dir}/🦀️.rs`, "utf8");
  const tree = parseRustSourceTree(text);
  const rows: string[] = [];
  for (const item of tree.items) {
    if (item.kind === "use") { if (item.visibility !== "private") rows.push(`${item.visibility} use ${useLeafNames(text.slice(item.declStart, item.end)).join(",")}`); continue; }
    if (item.name) rows.push(`${item.visibility} ${item.kind} ${item.name}`);
    else rows.push(`(${item.kind}) ${text.slice(item.declStart, item.declStart + 70).replace(/\n/gu, " ")}`);
  }
  for (const m of tree.modules) rows.push(`${m.visibility} mod ${m.name}`);
  console.log(`== ${name}\n` + rows.map((r) => "  " + r).join("\n"));
}
