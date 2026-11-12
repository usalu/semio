import { writeFileSync } from "node:fs";
import { writePdf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/📕️pdf/🟦️.ts";
import { drawingScene } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧰️.ts";
const d = process.argv[2]!;
writeFileSync(`${d}/two.pdf`, writePdf([drawingScene(), { ...drawingScene(), widthMm: 100, heightMm: 30 }]));
writeFileSync(`${d}/one.pdf`, writePdf([drawingScene()]));
