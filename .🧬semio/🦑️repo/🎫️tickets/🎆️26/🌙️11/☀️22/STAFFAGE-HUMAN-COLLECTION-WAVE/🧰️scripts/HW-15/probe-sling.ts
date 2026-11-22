/** 🔭️ Probe: forward kinematics of the commuter's right arm for sling offsets. bun probe-sling.ts a tw ab f */
import { load, frames, v3, toWorld } from "./lib.ts";
const doc = await load("👤️humans", "walking-commuter");
const a = doc.asset;
const standing = a.poses.find((p: any) => p.id === "standing").bones;
const [ang, tw, ab, fa, ha] = (process.env.A ?? process.argv.slice(2).join(" ")).trim().split(/\s+/).map(Number);
const off = { ...standing, "arm-r": { angleDeg: ang, twistDeg: tw, abductionDeg: ab }, "forearm-r": { angleDeg: fa }, "hand-r": { angleDeg: ha ?? 0 } };
const f = frames(a, off);
const p = (id: string) => v3(f[id].frame.o).map((x) => x.toFixed(3)).join(", ");
const d = (id: string) => v3(f[id].frame.u).map((x) => x.toFixed(2)).join(", ");
console.log("shoulder", p("arm-r"), "u", d("arm-r"));
console.log("elbow", p("forearm-r"), "u", d("forearm-r"), "v", v3(f["forearm-r"].frame.v).map((x) => x.toFixed(2)).join(", "));
console.log("wrist", p("hand-r"), "u", d("hand-r"));
console.log("torso", p("torso"), "u", d("torso"), "hand tip", toWorld(f["hand-r"].frame, [0.095, 0, 0]).map((x) => x.toFixed(3)).join(", "));
console.log("left shoulder", p("arm-l"), "neck", p("neck"));
