import { readFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { validateAsset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const doc = JSON.parse(readFileSync("/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🚶️reference-walker/🔣️.json", "utf8"));
const asset = doc.assets ? doc.assets[0] : doc.asset ?? doc;
console.log("[DEBUG] issues", JSON.stringify(validateAsset(asset)).slice(0, 400));
const s = evaluate(asset, { kind: "clip", clip: asset.clips[0].id, timeS: 0.37 });
console.log("[DEBUG] bounds", s.bounds, "contacts", s.contacts, "shapes", s.shapes.elevation.length);
