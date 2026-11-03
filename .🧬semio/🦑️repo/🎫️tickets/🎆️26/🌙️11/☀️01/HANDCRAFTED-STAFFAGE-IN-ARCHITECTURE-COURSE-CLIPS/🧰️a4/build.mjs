import fs from "fs";
import { toaster } from "./toaster.mjs";
import { vacuum } from "./vacuum.mjs";
import { heater } from "./heater.mjs";
const S = "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances";
const only = process.argv.slice(2);
const mods = { toaster: () => toaster, "vacuum-cleaner": () => vacuum, "electric-fan-heater": () => heater };
for (const id of (only.length ? only : Object.keys(mods))) {
  const asset = mods[id]();
  const doc = { $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset };
  fs.mkdirSync(`${S}/${id}`, { recursive: true });
  fs.writeFileSync(`${S}/${id}/🔣️.json`, JSON.stringify(doc, null, 2) + "\n");
  console.log("wrote", id);
}
