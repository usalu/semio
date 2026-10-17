/** 🔎 Deep-equality check of the remodel story data before and after the fixture move. */
import { isDeepStrictEqual } from "node:util";

const F = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split";
const C = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/📖️stories/🧭️coordination";
const oldDocs = await import(F + "/🗑️generated/c4p21-snap/f3-children/documents.ts");
const oldLabels = await import(F + "/🗑️generated/c4p21-snap/f3-children/labels.ts");
const newDocs = await import(C + "/🗂️documents/🟦️.ts");
const newLabels = await import(C + "/🗣️labels/🟦️.ts");
const rows: [string, boolean][] = [
  ["populated scene deep-equal", isDeepStrictEqual(oldDocs.REMODEL_POPULATED_SCENE, newDocs.REMODEL_POPULATED_SCENE)],
  ["populated scene key order", JSON.stringify(oldDocs.REMODEL_POPULATED_SCENE) === JSON.stringify(newDocs.REMODEL_POPULATED_SCENE)],
  ["empty scene deep-equal", isDeepStrictEqual(oldDocs.REMODEL_EMPTY_SCENE, newDocs.REMODEL_EMPTY_SCENE)],
  ["labels en", isDeepStrictEqual(oldLabels.remodelLabelsFor("en-US"), newLabels.remodelLabelsFor("en-US"))],
  ["labels de", isDeepStrictEqual(oldLabels.remodelLabelsFor("de-DE"), newLabels.remodelLabelsFor("de-DE"))],
  ["labels key order en", JSON.stringify(oldLabels.remodelLabelsFor("en")) === JSON.stringify(newLabels.remodelLabelsFor("en"))],
  ["labels key order de", JSON.stringify(oldLabels.remodelLabelsFor("de")) === JSON.stringify(newLabels.remodelLabelsFor("de"))],
];
for (const [name, ok] of rows) console.log(ok ? "OK  " : "FAIL", name);
process.exit(rows.every(([, ok]) => ok) ? 0 : 1);
