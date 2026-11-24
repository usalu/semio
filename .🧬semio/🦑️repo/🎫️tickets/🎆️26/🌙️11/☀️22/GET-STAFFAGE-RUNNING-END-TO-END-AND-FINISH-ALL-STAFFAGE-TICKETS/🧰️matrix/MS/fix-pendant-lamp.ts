/** 💡️ The multiview repair of `pendant-lamp` (idempotent, the last step of its chain): the cord, the lampholder, the shade and the bulb hang from the cord bone, which the pendulum poses turn by 8 degrees, so every filled front shape on these bones becomes a `revolved` solid whose profile is the side-elevation shape of the same id (the lamp is a body of revolution about its cord), and the plan disc of the shade becomes the revolved shade profile. The rose is fixed to the ceiling and stays as drawn; the open detail strokes stay flat sheets on their bone.
 *  bun fix-pendant-lamp.ts [--in file] [--out file] */
import { load, save } from "./lib.ts";

const loaded = await load("pendant-lamp");
const TURNING = new Set(["cord", "holder", "shade", "bulb"]);
const GEOMETRY = ["kind", "d", "points", "closed", "x", "y", "w", "h", "r"];
let changed = 0;
for (const [scale, rep] of Object.entries<any>(loaded.asset.representations)) {
  if (rep.geometry === undefined) continue;
  const twin = (id: string): any => {
    const found = rep.geometry.elevation.find((shape: any) => shape.id === id);
    if (found === undefined) throw new Error(`1:${scale}: no elevation shape "${id}"`);
    return found;
  };
  const solidOf = (shape: any, profile: any): any => {
    const out: any = {};
    for (const [key, value] of Object.entries(shape)) if (!GEOMETRY.includes(key) && key !== "solid") out[key] = value;
    for (const key of GEOMETRY) if (profile[key] !== undefined) out[key] = profile[key];
    out.solid = { kind: "revolved", axisBone: shape.bone, axis: "u", axisX: 0, mirror: true };
    return out;
  };
  rep.geometry.front = rep.geometry.front.map((shape: any) => {
    if (!TURNING.has(shape.bone) || shape.filled !== true) return shape;
    const next = solidOf(shape, twin(shape.id));
    if (JSON.stringify(next) !== JSON.stringify(shape)) changed++;
    return next;
  });
  rep.geometry.plan = rep.geometry.plan.map((shape: any) => {
    if (shape.id !== "plan-shade") return shape;
    const next = solidOf(shape, twin("shade"));
    if (JSON.stringify(next) !== JSON.stringify(shape)) changed++;
    return next;
  });
}
console.log(`pendant-lamp: ${changed} shapes changed, file ${(await save(loaded)) ? "written" : "unchanged"}`);
