/** 🔎️ Read-only probe of C1b: which directory names the taxonomy already accepts under a `🔨️modules` folder, for a home of the drawing rules beside `🖌️depiction`. Run from the repository root: `bun TK/c1b_module_names.ts`. */
import { loadTaxonomy, semanticDirectoryKindId } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";

const V = "️";
const taxonomy: any = loadTaxonomy();
const candidates = ["🖍drawing", "🧱elements", "✏drawing", "🎨painting", "🖼scenery", "🎭scene", "🎬staging", "🖍strokes", "📐geometry", "🧩parts", "🖌brushes", "🎨paint", "🖼picture", "📏rules", "🧱primitives", "🖍sketch", "🏞scenery", "🎪stage", "🪞lifting", "🧰gear"];
for (const raw of candidates) {
  const glyph = [...raw][0]!;
  const name = `${glyph}${V}${raw.slice(glyph.length)}`;
  console.log(name.padEnd(20), String(semanticDirectoryKindId(name, taxonomy, { parentKindId: "modules" })));
}
