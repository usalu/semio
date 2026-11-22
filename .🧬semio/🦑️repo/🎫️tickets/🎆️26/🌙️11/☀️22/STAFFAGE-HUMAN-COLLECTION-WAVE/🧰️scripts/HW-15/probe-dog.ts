/** 🔭️ Probe: world positions (ungrounded rest root) of the handle tip, left shoulder, wrist and dog pelvis in a pose of the guide-dog asset: bun probe-dog.ts <pose> */
import { CATALOG, frames, toWorld, v3 } from "./lib.ts";
import { join } from "node:path";
const a = (await Bun.file(join(CATALOG, "👤️humans/person-with-guide-dog/🔣️.json")).json()).asset;
const p = a.poses.find((x: any) => x.id === (process.argv[2] ?? "standing-with-guide-dog"));
const f = frames(a, p.bones);
const s = (x: any) => v3(x).map((q) => q.toFixed(3)).join(",");
const L = a.rig.bones.find((b: any) => b.id === "dog-harness-handle").lengthM;
console.log("tip", s(toWorld(f["dog-harness-handle"].frame, [L, 0, 0])), "start", s(f["dog-harness-handle"].frame.o));
for (const id of ["pelvis", "arm-l", "forearm-l", "hand-l", "dog-pelvis", "dog-thorax"]) console.log(id, s(f[id].frame.o), "u", s(f[id].frame.u));
console.log(JSON.stringify({ arm: p.bones["arm-l"], fore: p.bones["forearm-l"], hand: p.bones["hand-l"], lead: [p.bones["dog-lead-x"], p.bones["dog-lead-y"], p.bones["dog-lead"]] }));
