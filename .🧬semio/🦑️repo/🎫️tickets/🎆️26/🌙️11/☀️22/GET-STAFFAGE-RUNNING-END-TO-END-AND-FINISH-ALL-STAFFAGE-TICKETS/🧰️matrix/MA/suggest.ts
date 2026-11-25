/** 🐾️ Proposes the ops of `./fit.ts` for a quadruped or a bird from its document alone: `bun suggest.ts <id|file.json> [--skip id,id] [--only-scales 50,100] [--pairs 50:bone,100:bone] > ops/<id>.json`. For every authored scale and every closed elevation shape on a bone that a pose or clip turns (itself or an ancestor) it looks at the front and plan twins of the same id and bone that are not solids yet (flat or `followTip`) and proposes by the rest direction of the bone: the front twin becomes the prism of its own outline swept along the bone axis nearest to the viewer (`view-sweep`); the plan twin of a bone that stands (u nearer to the vertical than v) and every other twin get the op `solid`, which takes the first construction that keeps both the footprint of the elevation twin and the outline of the default pose (a standing bone tries the lateral extrusion first, every other the prism of its own outline). A revolved elevation twin is copied. `--pairs` keeps the bones the gate reports at a scale (`reported.py`). A twin that is a solid already gets the op with `\"redo\": true` (it is only rebuilt when it misses the footprint and a construction improves it); a far-side shape without an elevation shape on its bone borrows the single elevation shape of the mirror bone (`twin`, `twinBone`); every plan twin ends with a `lateral-match` to its front twin.
 * @see ../MR/suggest.ts — the proposals for people */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));

const args = process.argv.slice(2);
const flag = (name: string): string[] => {
  const i = args.indexOf(`--${name}`);
  return i < 0 ? [] : (args[i + 1] ?? "").split(",");
};
const asset = readDoc(args[0]!).doc.asset;
const skip = new Set(flag("skip"));
const onlyScales = flag("only-scales");
const pairs = new Set(flag("pairs"));
const parents = new Map<string, string | undefined>(asset.rig.bones.map((bone: any) => [bone.id, bone.parent ?? undefined]));
const moved = new Set<string>();
const visit = (node: unknown): void => {
  if (Array.isArray(node)) return node.forEach(visit);
  if (node === null || typeof node !== "object") return;
  for (const [key, value] of Object.entries(node as Record<string, unknown>)) {
    if (key === "bones" && value !== null && typeof value === "object" && !Array.isArray(value))
      for (const [bone, offset] of Object.entries(value as Record<string, any>)) if (["angleDeg", "abductionDeg", "twistDeg"].some((field) => Math.abs(offset?.[field] ?? 0) >= 1)) moved.add(bone);
    visit(value);
  }
};
visit(asset.poses);
visit(asset.clips);
const turned = (bone: string): boolean => {
  for (let at: string | undefined = bone; at !== undefined; at = parents.get(at)) if (moved.has(at)) return true;
  return false;
};
const frames = restFrames(asset.rig) as Record<string, { frame: { u: number[]; v: number[]; w: number[] } }>;
const ops: any[] = [];
for (const [scale, rep] of Object.entries(asset.representations) as [string, any][]) {
  if (rep.status !== "authored" || (onlyScales.length > 0 && !onlyScales.includes(scale))) continue;
  const mirror = (bone: string): string | undefined => {
    const other = bone.replace(/(^|-)(h|f)?(n|f)$/u, (_all, dash: string, limb: string | undefined, side: string) => `${dash}${limb ?? ""}${side === "n" ? "f" : "n"}`);
    return other !== bone && parents.has(other) ? other : undefined;
  };
  const closed = (shape: any): boolean => !(shape.kind === "poly" && shape.closed !== true);
  for (const view of ["front", "plan"] as const)
    for (const shape of rep.geometry[view]) {
      if (shape.role === "ground" || skip.has(shape.id) || !turned(shape.bone) || !closed(shape)) continue;
      if (pairs.size > 0 && !pairs.has(`${scale}:${shape.bone}`)) continue;
      let twin = rep.geometry.elevation.find((candidate: any) => candidate.id === shape.id && candidate.bone === shape.bone);
      let borrowed: { twin: string; twinBone: string } | undefined;
      if (twin === undefined && mirror(shape.bone) !== undefined && !rep.geometry.elevation.some((candidate: any) => candidate.bone === shape.bone && candidate.role !== "ground")) {
        const bodies = rep.geometry.elevation.filter((candidate: any) => candidate.bone === mirror(shape.bone) && closed(candidate) && (candidate.role ?? "body") === (shape.role ?? "body"));
        if (bodies.length === 1) (twin = bodies[0]), (borrowed = { twin: twin.id, twinBone: twin.bone });
      }
      if (twin === undefined || !closed(twin)) continue;
      const frame = frames[shape.bone]!.frame;
      const base = { scale, id: shape.id, ...(rep.geometry[view].filter((other: any) => other.id === shape.id).length > 1 ? { bone: shape.bone } : {}), ...(borrowed ?? {}) };
      if (twin.solid?.kind === "revolved") {
        if (shape.solid === undefined && borrowed === undefined) ops.push({ op: "copy", ...base, from: "elevation", to: [view] });
        continue;
      }
      const d = view === "front" ? 0 : 1;
      const lateral = Math.abs(frame.w[d]!) > Math.max(Math.abs(frame.u[d]!), Math.abs(frame.v[d]!));
      const stands = Math.abs(frame.u[1]!) > Math.abs(frame.v[1]!);
      const redo = shape.solid !== undefined ? { redo: true } : {};
      if (lateral) shape.solid === undefined && ops.push({ op: "extrude", ...base, view });
      else if (view === "plan" && stands) ops.push({ op: "solid", ...base, view, across: "front", order: ["extrude", "revolve", "view-sweep"], ...redo });
      else ops.push({ op: "solid", ...base, view, ...(view === "front" ? { across: "plan" } : {}), ...redo });
    }
  for (const shape of rep.geometry.plan) {
    if (shape.role === "ground" || !turned(shape.bone) || !closed(shape) || (pairs.size > 0 && !pairs.has(`${scale}:${shape.bone}`))) continue;
    if (rep.geometry.front.filter((other: any) => other.id === shape.id && other.bone === shape.bone).length !== 1 || rep.geometry.plan.filter((other: any) => other.id === shape.id).length !== 1) continue;
    ops.push({ op: "lateral-match", scale, id: shape.id, view: "plan" });
  }
}
console.log(JSON.stringify(ops, null, 1));
