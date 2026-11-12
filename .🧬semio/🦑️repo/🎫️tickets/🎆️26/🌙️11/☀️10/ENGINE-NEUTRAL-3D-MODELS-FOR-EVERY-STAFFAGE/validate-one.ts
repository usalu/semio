import validator from "gltf-validator";
import { readFileSync } from "node:fs";
import { encodeGlb } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb/🟦️.ts";
const name = process.argv[2] ?? "rig-full";
const v = JSON.parse(readFileSync(new URL(`../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/📤️export/🧊️glb/🧫️fixtures/🧊️${name}/🔣️.json`, import.meta.url), "utf8"));
const r = await validator.validateBytes(encodeGlb(v.document, v.options, { report() {}, cancelled: () => false }), { uri: "x.glb" });
console.log(JSON.stringify(r.issues, null, 1));
