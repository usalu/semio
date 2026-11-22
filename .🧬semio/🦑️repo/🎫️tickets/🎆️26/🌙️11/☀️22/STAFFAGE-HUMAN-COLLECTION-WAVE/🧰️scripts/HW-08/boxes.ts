import { loadDoc, solidBoxes } from "./lib.ts";
const [slug, poseId, scale, filter] = process.argv.slice(2);
const a = loadDoc(slug!).asset;
for (const [id, b] of Object.entries(solidBoxes(a, poseId!, Number(scale), (id) => !filter || new RegExp(filter).test(id)))) console.log(id.padEnd(28), JSON.stringify(b));
