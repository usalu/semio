/** 🪧️ Red before the repair: the lines of the explanation box of the engine as it was (a scratch copy of `S/🔨️modules` with the old wrap by 0.56 em per character, path in the first argument) measured with the Anta advance table of the live `textWidthMm`, against the frame of the box: `bun explain-old.ts <modules directory>`. Prints every line wider than the frame inside the 2 mm margins.
 * @see ../../../🧪️tests/🔤️caption-fit — the case that holds the box to its width */
import { join } from "node:path";
const [modules] = process.argv.slice(2);
const live = join(process.env.S!, "🔨️modules/📐️units/🟦️.ts");
const { explanationBox } = await import(join(modules!, "🎨️depiction/🪧️explanation/🟦️.ts"));
const { textWidthMm, ANNOTATION_TEXT_MM, LABEL_TEXT_MM } = await import(live);
const reason = { en: "The 0.15 mm gaps between the 2000 mm slats 100 mm apart are 0.003 mm on paper and close at every printable pen 0000 1111 8888 2000 2000 2000 2000 2000 2000 2000 2000 2000 2000", de: "Die Lücken von 0,15 mm zwischen den 2000 mm langen Latten im Abstand von 100 mm messen 0,003 mm auf dem Papier und schließen sich bei jeder druckbaren Feder 0000 1111 8888 2000 2000 2000 2000 2000 2000 2000 2000 2000 2000" };
const resolution = { kind: "unavailable", requested: 2000, cause: "outside-valid-range", reason, limitingFeature: { en: "slat 2000 x 8 mm", de: "Latte 2000 x 8 mm" }, limitingSizeM: 0.008, supported: [50, 100, 200], alternatives: [{ kind: "scale", denominator: 200 }] };
const asset = { symbols: [] };
let over = 0, lines = 0;
for (const language of ["en", "de"]) for (const width of [50, 60, 80]) {
  for (const line of explanationBox(asset, resolution, language, width).lines) {
    lines++;
    const w = 2 + textWidthMm(line.text, line.bold ? ANNOTATION_TEXT_MM : LABEL_TEXT_MM, line.bold);
    if (line.text.includes(" ") && w > width - 2) { over++; console.log(`${language} ${width} mm: ${w.toFixed(1)} > ${width - 2}  ${line.text}`); }
  }
}
console.log(`${lines} lines, ${over} wider than the frame`);
