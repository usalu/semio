import { existsSync, readFileSync } from "node:fs";
import { decodeCargoProvenanceV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
const o = decodeCargoProvenanceV1(readFileSync(process.argv[2]!, "utf8"));
let art = 0, have = 0, dep = 0, depHave = 0, inputs = 0, inHave = 0;
for (const u of o.units) { for (const a of u.artifacts) { art++; if (existsSync(a.path)) have++; } for (const d of u.depInfo) { dep++; if (existsSync(d.path)) depHave++; } for (const i of u.inputs) { inputs++; if (existsSync(i.path)) inHave++; } }
console.log({ units: o.units.length, art, have, dep, depHave, inputs, inHave, build: o.buildDirectory, args: o.args.join(" ") });
console.log(o.units[0]!.artifacts[0]!.path, Object.keys(o.units[0]!.message));
