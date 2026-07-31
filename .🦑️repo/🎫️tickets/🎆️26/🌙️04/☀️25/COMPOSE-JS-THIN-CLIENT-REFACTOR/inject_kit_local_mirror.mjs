import fs from "node:fs";

const indexPath = "c:/git/compose/compose/js/index.ts";
const regionPath = "c:/git/compose/.repo/🎫️/26/04/25/COMPOSE-JS-THIN-CLIENT-REFACTOR/kit_local_mirror_region.txt";

const index = fs.readFileSync(indexPath, "utf8");
const region = fs.readFileSync(regionPath, "utf8");

const start = index.indexOf("/** 🧱️Normalizes plain kit DTOs into the JS [`Kit`] entity wrapper. */");
const end = index.indexOf("\n\n/**\n * Binary asset storage contract.", start);
if (start < 0 || end < 0) {
  throw new Error(`inject markers not found: start=${start} end=${end}`);
}

const out = index.slice(0, start) + region + index.slice(end);
fs.writeFileSync(indexPath, out);
console.log("injected", regionPath, "at", start, "len", region.length);
