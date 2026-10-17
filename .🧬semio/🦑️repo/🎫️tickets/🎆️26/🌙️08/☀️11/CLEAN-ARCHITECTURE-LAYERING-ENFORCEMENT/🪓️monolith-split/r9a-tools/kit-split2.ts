// 🔪️ Splits each snapshot kit into the part that implements methods on value types (joins the values coherence group) and the part for entity types and everything else (joins the entities group), so two smaller groups replace one.
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { parseRustSourceTree, identifierSet, rustOuterDocStart } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🌳️module-tree/🟦️.ts";
import { implHeader } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts";
const S = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/";
const apply = process.argv.includes("--apply");
const defined = (file: string): Set<string> => { const t = readFileSync(file, "utf8"); const out = new Set<string>(); for (const item of parseRustSourceTree(t).items) if (item.name && ["struct", "enum", "type", "trait"].includes(item.kind)) out.add(item.name); return out; };
const valueTypes = defined(S + "💠️values/🦀️.rs");
const entityTypes = defined(S + "🧱️entities/🦀️.rs");
const kits: [string, string, string][] = [["🕰️phases", "phases", "🧮️phases-values"], ["📋️schedule-kit", "schedule_kit", "🔩️schedule-kit-values"], ["⚓️anchors", "anchors", "🧲️anchors-values"], ["🏷️property-kit", "property_kit", "🪛️property-kit-values"], ["🖼️views", "views", "🧷️views-values"], ["📄️sheets", "sheets", "🔧️sheets-values"], ["🔥️conditions", "conditions", "🔥️conditions-values"], ["🤝️coordination", "coordination", "⚙️coordination-values"]];
for (const [dir, mod, newDir] of kits) {
  const file = S + dir + "/🦀️.rs";
  const text = readFileSync(file, "utf8");
  const tree = parseRustSourceTree(text);
  const items = tree.items.filter((item) => item.kind !== "use");
  const starts = items.map((item) => rustOuterDocStart(text, item.start));
  const src = items.map((item, i) => text.slice(starts[i]!, item.end));
  const isV = items.map((item, i) => { if (item.kind !== "impl") return false; const h = implHeader(src[i]!); return !!h?.self && valueTypes.has(h.self) && !entityTypes.has(h.self); });
  const names = items.map((item) => item.name);
  let changed = true;
  const conflicts: string[] = [];
  while (changed) {
    changed = false;
    items.forEach((_, i) => {
      if (!isV[i]) return;
      const ids = identifierSet(src[i]!);
      items.forEach((other, j) => { if (!isV[j] && names[j] && ids.has(names[j]!) && other.kind !== "impl") { isV[j] = true; changed = true; } });
    });
  }
  items.forEach((_, i) => { if (!isV[i]) return; const ids = identifierSet(src[i]!); for (const type of entityTypes) if (ids.has(type)) conflicts.push(`${items[i]!.name ?? "impl"} mentions entity type ${type}`); });
  const vLines = src.reduce((n, s, i) => n + (isV[i] ? s.split("\n").length : 0), 0);
  const eLines = src.reduce((n, s, i) => n + (!isV[i] ? s.split("\n").length : 0), 0);
  console.log(dir, "value part", vLines, "entity part", eLines, conflicts.length ? "CONFLICTS " + [...new Set(conflicts)].slice(0, 8).join("; ") : "");
  if (apply && vLines > 0) {
    const uses = tree.items.filter((item) => item.kind === "use").map((item) => text.slice(item.declStart, item.end)).join("\n");
    const vText = items.map((item, i) => (isV[i] ? src[i] : null)).filter((s): s is string => s !== null).join("\n\n");
    const doc = "//! 🧮️ The methods of the value types of the `" + mod + "` kit; they live with the value types so the entity types and the model root stay above them.\n\n";
    mkdirSync(S + newDir, { recursive: true });
    writeFileSync(S + newDir + "/🦀️.rs", doc + uses + "\n\n" + vText + "\n");
    let out = text;
    for (const index of items.map((_, i) => i).filter((i) => isV[i]).sort((a, b) => starts[b]! - starts[a]!)) out = out.slice(0, starts[index]!) + out.slice(items[index]!.end);
    writeFileSync(file, out.replace(/\n{3,}/gu, "\n\n"));
  }
}
