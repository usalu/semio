import { readFileSync } from "node:fs";
import { assertPdfPainting } from "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser/🔮️oracle.mjs";
const [a, b] = process.argv.slice(2);
const x = await assertPdfPainting(readFileSync(a), 1), y = await assertPdfPainting(readFileSync(b), 1);
console.log(x.width, x.height, x.digest.slice(0, 12), y.width, y.height, y.digest.slice(0, 12));
