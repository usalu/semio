import { readFileSync, writeFileSync } from "node:fs";
const file = process.argv[2]!;
let t = readFileSync(file, "utf8");
const sub = (from: string, to: string): void => {
  if (!t.includes(from)) throw new Error(`missing: ${from.slice(0, 70)}`);
  t = t.replace(from, to);
};
sub('    if (!(Math.abs(entry.heightM - asset.dimensions.heightM) <= HEIGHT_TOLERANCE * asset.dimensions.heightM)) mismatch("heightM");\n    if (!(Math.abs(entry.widthM - asset.dimensions.widthM) <= WIDTH_TOLERANCE * asset.dimensions.widthM)) mismatch("widthM");',
  '    for (const field of ["heightM", "widthM"] as const) if (!(Math.abs(entry[field] - asset.dimensions[field]) <= MANIFEST_DIMENSION_TOLERANCE_M)) add(`${path}.${field}`, "manifest-dimension-mismatch", `Entry "${entry.id}" declares ${field} ${entry[field]} m but the asset file says ${asset.dimensions[field]} m; the asset file is the single source of truth (tolerance 1 mm).`, `Eintrag "${entry.id}" nennt ${field} ${entry[field]} m, die Asset-Datei aber ${asset.dimensions[field]} m; die Asset-Datei ist die einzige Quelle der Wahrheit (Toleranz 1 mm).`);');
const start = t.indexOf("/** 🎯️ Allowed deviation of manifest");
const end = t.indexOf("export const WIDTH_TOLERANCE = 0.1;\n") + "export const WIDTH_TOLERANCE = 0.1;\n".length;
if (start < 0) throw new Error("tol");
t = t.slice(0, start) + "/** 🎯️ Allowed deviation in metres of the manifest height and width from the asset file, whose `dimensions` are the single source of truth (1 mm). */\nexport const MANIFEST_DIMENSION_TOLERANCE_M = 0.001;\n" + t.slice(end);
writeFileSync(file, t);
