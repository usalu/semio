/** 🗺️ Proposes the ops of `./fit.ts` that make every part on a turning bone a solid in the views where it is still a flat sheet: `bun plan.ts <id|file.json> [--bones a,b] [--ids a,b] [--scales 50,100] [--single] [--refit]`. Nothing is evaluated. For every authored scale and every closed elevation shape on a bone that a pose or clip turns (itself or an ancestor), with its twins of the same id and bone: a revolved elevation twin is copied to a front or plan twin that is not the same body (`copy`); a flat front or plan twin becomes a prism that keeps its outline (`prism`); a flat elevation shape on a bone that is spread or twisted (itself or an ancestor, `abductionDeg` or `twistDeg`) and has a front or plan twin becomes a solid swept laterally (`elevation-solid`, placed before the prisms that read it). `--single` keeps the shapes that are alone on their bone in two views (the hard tier), `--bones` and `--ids` restrict, `--refit` adds the refits of MR for twins that are solids already (`front-sweep`, `plan-revolve`). Each op carries `why`; `fit.ts` ignores it. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const args = process.argv.slice(2);
const list = (name: string): string[] | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]!.split(","));
const asset = readDoc(args[0]!).doc.asset;
const parents = new Map<string, string | undefined>(asset.rig.bones.map((bone: any) => [bone.id, bone.parent ?? undefined]));
const moved = new Set<string>();
const spread = new Set<string>();
const visit = (node: unknown): void => {
  if (Array.isArray(node)) return node.forEach(visit);
  if (node === null || typeof node !== "object") return;
  for (const [key, value] of Object.entries(node as Record<string, unknown>)) {
    if (key === "bones" && value !== null && typeof value === "object" && !Array.isArray(value))
      for (const [bone, offset] of Object.entries(value as Record<string, any>)) {
        if (["angleDeg", "abductionDeg", "twistDeg"].some((field) => Math.abs(offset?.[field] ?? 0) >= 1)) moved.add(bone);
        if (["abductionDeg", "twistDeg"].some((field) => Math.abs(offset?.[field] ?? 0) >= 1)) spread.add(bone);
      }
    visit(value);
  }
};
visit(asset.poses);
visit(asset.clips);
const inherits = (set: Set<string>, bone: string): boolean => {
  for (let at: string | undefined = bone; at !== undefined; at = parents.get(at)) if (set.has(at)) return true;
  return false;
};
const frames = restFrames(asset.rig) as Record<string, { frame: { w: number[] } }>;
const closed = (shape: any): boolean => (shape.kind === "poly" ? shape.closed === true : ["rect", "ellipse", "circle", "capsule"].includes(shape.kind));
const same = (a: any, b: any): boolean => JSON.stringify([a.kind, a.points, a.solid]) === JSON.stringify([b.kind, b.points, b.solid]);
const [bones, ids, scales] = [list("bones"), list("ids"), list("scales")];
const ops: any[] = [];
for (const [scale, rep] of Object.entries(asset.representations) as [string, any][]) {
  if (rep.status !== "authored" || (scales !== undefined && !scales.includes(scale))) continue;
  const count = (view: string, bone: string): number => rep.geometry[view].filter((shape: any) => shape.bone === bone && shape.role !== "ground").length;
  for (const twin of rep.geometry.elevation) {
    if (twin.role === "ground" || !inherits(moved, twin.bone) || !closed(twin)) continue;
    if ((bones !== undefined && !bones.includes(twin.bone)) || (ids !== undefined && !ids.includes(twin.id))) continue;
    const front = rep.geometry.front.find((shape: any) => shape.id === twin.id && shape.bone === twin.bone);
    const plan = rep.geometry.plan.find((shape: any) => shape.id === twin.id && shape.bone === twin.bone);
    if (front === undefined && plan === undefined) continue;
    const single = (view: string): boolean => !args.includes("--single") || (count("elevation", twin.bone) === 1 && count(view, twin.bone) === 1);
    const base = { scale, id: twin.id, ...(rep.geometry.elevation.filter((shape: any) => shape.id === twin.id).length > 1 ? { bone: twin.bone } : {}) };
    if (twin.solid?.kind === "revolved") {
      const to = [front !== undefined && single("front") && !same(twin, front) ? "front" : undefined, plan !== undefined && single("plan") && !same(twin, plan) ? "plan" : undefined].filter(Boolean);
      if (to.length > 0) ops.push({ op: "copy", ...base, from: "elevation", to, why: `revolved elevation twin on ${twin.bone}: one body in every view` });
      continue;
    }
    if (twin.solid === undefined && inherits(spread, twin.bone) && frames[twin.bone]!.frame.w[2]! >= 0.99 && (single("front") || single("plan"))) ops.push({ op: "elevation-solid", ...base, why: `flat elevation shape on ${twin.bone}, which poses spread or twist` });
    for (const [view, shape] of [["front", front], ["plan", plan]] as const) {
      if (shape === undefined || !single(view)) continue;
      if (shape.solid === undefined) ops.push({ op: "prism", ...base, view, why: `flat ${view} shape on ${twin.bone}, which turns${shape.followTip === true ? " (followTip)" : ""}` });
      else if (args.includes("--refit") && view === "front" && shape.solid.kind === "extruded" && shape.solid.sweep === "v") ops.push({ op: "front-sweep", ...base, why: "refit of the u-range and the v-span to the elevation outline" });
      else if (args.includes("--refit") && view === "plan" && shape.solid.kind === "revolved") ops.push({ op: "plan-revolve", ...base, why: "refit of the axis and radius to the elevation and front twins" });
    }
  }
}
console.log(JSON.stringify(ops, null, 1));
