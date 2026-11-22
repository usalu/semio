/** 🔬️ Probe of the chisel helpers: tessellates a box and an extrude and prints their projected outlines. */
import { frames, meshOf, meshStats, outline } from "./author-lib.ts";
const rig = { kind: "mechanical", root: "a", bones: [{ id: "a", parent: null, title: { en: "a", de: "a" }, x: 0, y: 0, angleDeg: 30, lengthM: 1, joint: { type: "fixed" } }] };
const f = frames(rig);
console.log(JSON.stringify(f));
const q = { chordM: 0.002, minSegments: 12, maxSegments: 64 };
const m = meshOf({ id: "b", bone: "a", kind: "extrude", plane: "zx", profile: [[-0.1, 0], [0.1, 0], [0.1, 0.3], [-0.1, 0.3]], holes: [[[-0.02, 0.1], [0.02, 0.1], [0.02, 0.14], [-0.02, 0.14]]], from: -0.05, to: 0.05 }, f.a!, q);
console.log(meshStats(m));
for (const v of ["elevation", "front", "plan"] as const) console.log(v, JSON.stringify(outline([m], v, f.a!, 0.001)));
