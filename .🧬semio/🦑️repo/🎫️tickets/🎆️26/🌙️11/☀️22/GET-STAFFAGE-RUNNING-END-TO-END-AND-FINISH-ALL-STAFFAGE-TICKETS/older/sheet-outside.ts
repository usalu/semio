/** 📑️ Writes the multiview sheet of one asset as SVG and PDF and lists what the PDF draws beyond the figure boxes of the SVG: `bun sheet-outside.ts <category-folder>/<asset> <view> <first|third> <out-dir>` (the listing is `sheet-outside.py`, run in the documents oracle set). */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../../../../..");
const staffage = join(root, "🎓️teaching/🏛️architecture/🧍️staffage");
const { defaultDepictionOptions } = await import(join(staffage, "🔨️modules/🎨️depiction/🟦️.ts"));
const { resolveSheet } = await import(join(staffage, "🔨️modules/📤️export/📑️sheet/🟦️.ts"));
const { writePdf } = await import(join(staffage, "🔨️modules/📤️export/📕️pdf/🟦️.ts"));
const { writeSvg } = await import(join(staffage, "🔨️modules/📤️export/🖼️svg/🟦️.ts"));
const { authoredScales } = await import(join(staffage, "🧪️tests/🧊️three-views/🧰️.ts"));
const { python } = await import(join(staffage, "🧪️tests/🧰️support/🐍️.ts"));
const [path, view, projection, out] = process.argv.slice(2);
const { asset } = JSON.parse(readFileSync(join(staffage, "🗂️catalog", path!, "🔣️.json"), "utf8"));
const denominator = authoredScales(asset)[0];
const { view: _view, viewpoint: _viewpoint, scale: _scale, representation: _representation, ...options } = defaultDepictionOptions(view, "en", denominator);
const { sheet } = resolveSheet(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", denominator }, view, options, projection, "en");
mkdirSync(out!, { recursive: true });
writeFileSync(join(out!, "sheet.svg"), writeSvg(sheet));
writeFileSync(join(out!, "sheet.pdf"), writePdf([sheet]));
console.log(python(join(import.meta.dir, "sheet-outside.py"), "documents", [join(out!, "sheet.svg"), join(out!, "sheet.pdf")]));
