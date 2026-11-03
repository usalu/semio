import { load } from "./lib.ts";
const [id, scale, view] = process.argv.slice(2);
const a = load(id).asset;
for (const s of a.representations[scale].geometry[view]) console.log(s.id, s.bone, s.kind, JSON.stringify(s.points ?? [s.x, s.y, s.w, s.h, s.cx, s.cy, s.rx, s.ry]).slice(0, 120));
