/** 🗂️ Moves the remodel story populated scene and label tables into JSON fixtures and rewrites the two children to read them. */
import { readFileSync, writeFileSync } from "node:fs";
import { isDeepStrictEqual } from "node:util";

const F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split";
const C = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/📖️stories/🧭️coordination";
const oldDocs = await import(F + "/🗑️generated/c4p21-snap/f3-children/documents.ts");
const oldLabels = await import(F + "/🗑️generated/c4p21-snap/f3-children/labels.ts");
const en = oldLabels.remodelLabelsFor("en-US");
const de = oldLabels.remodelLabelsFor("de-DE");
if (isDeepStrictEqual(en, de)) throw new Error("locales equal");
const apply = process.argv.includes("--apply");

const docsPath = C + "/🗂️documents/🟦️.ts";
let docs = readFileSync(docsPath, "utf8");
if (docs !== readFileSync(F + "/🗑️generated/c4p21-snap/f3-children/documents.ts", "utf8")) throw new Error("documents child changed");
const dStart = "export const REMODEL_POPULATED_SCENE: RemodelScene = {\n";
const dEnd = "};\n\n/** 🌱️ The boot document";
if (docs.split(dStart).length !== 2 || docs.split(dEnd).length !== 2) throw new Error("documents anchors");
const a = docs.indexOf(dStart);
const b = docs.indexOf(dEnd);
docs = docs.slice(0, a) + "export const REMODEL_POPULATED_SCENE: RemodelScene = populatedScene as unknown as RemodelScene;\n\n/** 🌱️ The boot document" + docs.slice(b + dEnd.length);
const impAnchor = 'import type { RemodelScene } from "../📐️scene-model/🟦️.ts";\n';
if (docs.split(impAnchor).length !== 2) throw new Error("documents import anchor");
docs = docs.replace(impAnchor, impAnchor + 'import populatedScene from "./🔣️.json";\n');

const labPath = C + "/🗣️labels/🟦️.ts";
let lab = readFileSync(labPath, "utf8");
if (lab !== readFileSync(F + "/🗑️generated/c4p21-snap/f3-children/labels.ts", "utf8")) throw new Error("labels child changed");
const cut = (src: string, startNeedle: string, endNeedle: string, replacement: string): string => {
  if (src.split(startNeedle).length !== 2) throw new Error("anchor " + startNeedle);
  const s = src.indexOf(startNeedle);
  const e = src.indexOf(endNeedle, s);
  if (e < 0) throw new Error("end anchor " + endNeedle);
  return src.slice(0, s) + replacement + src.slice(e + endNeedle.length);
};
lab = cut(lab, "const REMODEL_LABELS_EN: RemodelLabels = {\n", "};\n", "const REMODEL_LABELS_EN: RemodelLabels = labels.en;\n");
lab = cut(lab, "const REMODEL_LABELS_DE: RemodelLabels = {\n", "};\n", "const REMODEL_LABELS_DE: RemodelLabels = labels.de;\n");
const labAnchor = "//#region 🔖️Labels\n";
if (lab.split(labAnchor).length !== 2) throw new Error("labels region anchor");
lab = lab.replace("/** 🗣️ The complete English and German remodel label sets and the locale lookup. */\n", '/** 🗣️ The complete English and German remodel label sets and the locale lookup. */\nimport labels from "./🔣️.json";\n\n');

const popJson = JSON.stringify(oldDocs.REMODEL_POPULATED_SCENE, null, 2) + "\n";
const labJson = JSON.stringify({ en, de }, null, 2) + "\n";
if (!isDeepStrictEqual(JSON.parse(popJson), oldDocs.REMODEL_POPULATED_SCENE)) throw new Error("populated not JSON-safe");
if (!isDeepStrictEqual(JSON.parse(labJson), { en, de })) throw new Error("labels not JSON-safe");
console.log("docs lines", docs.split("\n").length - 1, "labels lines", lab.split("\n").length - 1, "pop json", popJson.split("\n").length - 1, "label json", labJson.split("\n").length - 1);
if (apply) {
  writeFileSync(C + "/🗂️documents/🔣️.json", popJson);
  writeFileSync(C + "/🗣️labels/🔣️.json", labJson);
  writeFileSync(docsPath, docs);
  writeFileSync(labPath, lab);
  console.log("APPLIED");
}
