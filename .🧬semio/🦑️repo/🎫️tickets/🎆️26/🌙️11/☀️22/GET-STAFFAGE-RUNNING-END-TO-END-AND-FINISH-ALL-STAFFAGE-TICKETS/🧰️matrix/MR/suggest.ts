/** 🧭️ Proposes the ops of `fit.ts` for an asset from its document alone (nothing is evaluated): `bun suggest.ts <id|file.json> [--all] > ops/<id>.json`. For every authored scale and every bone that a pose or clip turns (itself or an ancestor), it looks at the shapes that have a twin of the same id in the elevation and, by default, are the only shape of their bone in both views (the hard tier of the matrix; `--all` takes every twin), and proposes by construction: a revolved elevation twin is copied to a front or plan twin that is not the same body (`copy`); a front twin that is flat or swept along v is fitted to the elevation outline (`front-sweep`; `extrude` on a bone that points at the viewer, marked for review); a flat plan twin of a limb becomes a revolved solid (`plan-revolve`), of another part an op marked `"review": true` for the fixer to decide (`plan-revolve` for round parts, `extrude` with `"view": "plan"` for prisms, or leave a part that only nods). Each op carries `why`; `fit.ts` ignores it. The gate decides which of the proposals are needed: keep the ops of the bones it reports, drop the others. */
import { join } from "node:path";
import { S, readDoc } from "./lib.ts";

const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));

const args = process.argv.slice(2);
const asset = readDoc(args[0]!).doc.asset;
const all = args.includes("--all");
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
const frames = restFrames(asset.rig) as Record<string, { frame: { u: number[]; w: number[] } }>;
const edgeOn = (bone: string): boolean => Math.abs(frames[bone]!.frame.u[1]! * frames[bone]!.frame.w[2]! - frames[bone]!.frame.w[1]! * frames[bone]!.frame.u[2]!) < 0.2;
const LIMB = /(^|-)(arm|forearm|upper-arm|thigh|shank|leg|calf|finger|tail)(-|$)/u;
const same = (a: any, b: any): boolean => JSON.stringify([a.kind, a.points, a.solid]) === JSON.stringify([b.kind, b.points, b.solid]);
const ops: any[] = [];
for (const [scale, rep] of Object.entries(asset.representations) as [string, any][]) {
  if (rep.status !== "authored") continue;
  const count = (view: string, bone: string): number => rep.geometry[view].filter((shape: any) => shape.bone === bone && shape.role !== "ground").length;
  for (const twin of rep.geometry.elevation) {
    if (twin.role === "ground" || !turned(twin.bone)) continue;
    const single = (view: string): boolean => all || (count("elevation", twin.bone) === 1 && count(view, twin.bone) === 1);
    const front = rep.geometry.front.find((shape: any) => shape.id === twin.id && shape.bone === twin.bone);
    const plan = rep.geometry.plan.find((shape: any) => shape.id === twin.id && shape.bone === twin.bone);
    const base = { scale, id: twin.id, ...(rep.geometry.elevation.filter((shape: any) => shape.id === twin.id).length > 1 ? { bone: twin.bone } : {}) };
    if (twin.solid?.kind === "revolved") {
      const to = [front !== undefined && single("front") && !same(twin, front) ? "front" : undefined, plan !== undefined && single("plan") && !same(twin, plan) ? "plan" : undefined].filter(Boolean);
      if (to.length > 0) ops.push({ op: "copy", ...base, from: "elevation", to, why: "the elevation twin is a revolved body; the same body in every view agrees at every pose" });
      continue;
    }
    if (twin.kind === "poly" && twin.closed !== true) continue;
    if (front !== undefined && single("front")) {
      if (front.solid === undefined && edgeOn(twin.bone)) ops.push({ op: "extrude", ...base, view: "front", review: true, why: "flat front shape on a bone that points at the viewer and turns: the elevation outline swept laterally over the width the front shape has (the front then draws the hull of that prism: look at it)" });
      else if (front.solid === undefined) ops.push({ op: "front-sweep", ...base, why: "flat front shape on a bone that turns: it becomes a prism swept along v, fitted to the elevation outline" });
      else if (front.solid.kind === "extruded" && front.solid.sweep === "v") ops.push({ op: "front-sweep", ...base, why: "refit of the u-range and the v-span to the elevation outline (changes nothing when they already agree)" });
    }
    if (plan !== undefined && single("plan")) {
      if (plan.solid === undefined) ops.push({ op: "plan-revolve", ...base, ...(LIMB.test(twin.bone) ? {} : { review: true }), why: LIMB.test(twin.bone) ? "flat plan shape of a limb that turns: revolved about the bone axis from the elevation and front twins" : "flat plan shape on a bone that turns: decide between plan-revolve (round part), extrude with view plan (prism) or no op (a part that only nods a few degrees)" });
      else if (plan.solid.kind === "revolved") ops.push({ op: "plan-revolve", ...base, why: "refit of the axis and radius to the elevation and front twins (changes nothing when they already agree)" });
    }
  }
}
console.log(JSON.stringify(ops, null, 2));
