/** 🔭️ Probe: frames of the standing-adult arms in a pose: bun probe-sign.ts <pose> */
import { load, frames, v3 } from "./lib.ts";
const a = (await load("👤️humans", "standing-adult")).asset;
const pose = a.poses.find((p: any) => p.id === (process.argv[2] ?? "relaxed")).bones;
const f = frames(a, pose);
const s = (x: any) => v3(x).map((q) => q.toFixed(3)).join(",");
for (const id of ["torso", "neck", "head", "arm-r", "forearm-r", "hand-r", "index-proximal-r", "hand-l"]) console.log(id, "o", s(f[id].frame.o), "u", s(f[id].frame.u), "v", s(f[id].frame.v), "w", s(f[id].frame.w));
for (const b of a.rig.bones) if (["arm-r", "forearm-r", "hand-r", "clavicle-r", "arm-l"].includes(b.id)) console.log(JSON.stringify({ ...b, title: undefined }));
