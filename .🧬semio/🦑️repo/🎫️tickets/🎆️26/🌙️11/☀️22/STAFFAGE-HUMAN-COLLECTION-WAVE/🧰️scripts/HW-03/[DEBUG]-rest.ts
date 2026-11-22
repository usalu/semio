import { framesOf } from "./ik.ts";
const a = (await Bun.file("../../🗑️generated/HW-03/doctor.json").json()).asset;
const f = framesOf(a, {});
for (const k of ["torso","hand-r","hand-l","forearm-l","neck","head"]) console.log(k, JSON.stringify(f[k].frame));
