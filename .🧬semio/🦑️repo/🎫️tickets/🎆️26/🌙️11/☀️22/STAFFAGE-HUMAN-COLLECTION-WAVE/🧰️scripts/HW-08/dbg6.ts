import { loadDoc, boneExtents, solidBoxes } from "./lib.ts";
const a = loadDoc("standing-adult").asset;
for (const b of ["thigh-l", "thigh-r", "torso", "foot-l"]) console.log(b, JSON.stringify(boneExtents(a, "50", b)));
const bx = solidBoxes(a, "relaxed", 50, (id) => /thigh|foot|torso/.test(id)); for (const [k, v] of Object.entries(bx)) console.log(k, JSON.stringify(v));
