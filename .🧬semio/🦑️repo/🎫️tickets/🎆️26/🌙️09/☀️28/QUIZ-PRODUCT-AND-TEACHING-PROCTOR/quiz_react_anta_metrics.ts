/** 📏️ Calibrates the radar label width estimate: prints every printable Latin-1 (and a few extra) character whose Anta
 * advance width exceeds `estimateTextWidth`, and the overall ratio for the demand quiz labels. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import opentype from "opentype.js";
import { estimateTextWidth } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🕸️radar/🟦️.tsx";

const root = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..", "🧰️framework", "🔨️modules", "🖼️assets", "🔤️fonts", "🚀️anta");
const fonts = ["🏛️latin", "➕️latin-ext", "🧮️math", "🔣️symbols"].map((subset) => {
  const bytes = readFileSync(join(root, subset, "📖️regular", "🔤️outline.ttf"));
  return opentype.parse(bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength));
});
const advance = (char: string): number | undefined => {
  const font = fonts.find((candidate) => candidate.charToGlyphIndex(char) !== 0);
  return font === undefined ? undefined : (font.charToGlyph(char).advanceWidth ?? 0) / font.unitsPerEm;
};
const codes = [...Array.from({ length: 95 }, (_, index) => 32 + index), ...Array.from({ length: 96 }, (_, index) => 160 + index), 0x2013, 0x2014, 0x20ac, 0x2010, 0x2212, 0x00b7, 0x2026];
const short: string[] = [];
for (const code of codes) {
  const char = String.fromCodePoint(code);
  const real = advance(char);
  if (real === undefined) continue;
  const guessed = estimateTextWidth(char, 1);
  if (guessed < real) short.push(`${JSON.stringify(char)} U+${code.toString(16)} anta=${real.toFixed(3)} estimate=${guessed.toFixed(3)}`);
}
console.log(short.length === 0 ? "no character is underestimated" : short.join("\n"));
