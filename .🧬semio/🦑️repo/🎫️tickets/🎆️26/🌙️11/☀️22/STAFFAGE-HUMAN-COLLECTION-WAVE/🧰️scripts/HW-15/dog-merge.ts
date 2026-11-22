/** 🐕️ Merges the labrador-dog asset into a human asset as a passenger: every bone, anchor, shape, solid, region, material, emitter and reference of the dog under the prefix `dog-`, its root hung below the human pelvis on a carrier of two slides and a hinge (`dog-lead-x`, `dog-lead-y`, `dog-lead`) that the generator drives so the dog walks on its own paws; front and plan shapes move by the rest offset of the carrier.
 * @see ./gen-guide-dog.ts
 */
import { SCALES, VIEWS, clone } from "./lib.ts";

export const P = (s: string) => `dog-${s}`;
export const DOG = "guide-dog";

/** 📍️ Rest placement of the dog pelvis in the world of the human rest pose: ahead, up, left. */
export type Rest = { x: number; y: number; z: number };

const renameBones = (o: Record<string, any> | undefined) => o && Object.fromEntries(Object.entries(o).map(([k, v]) => [P(k), v]));

/** 🧩️ The carrier bones under the human pelvis (rest angle 90°, origin at `pelvisY`) that place the dog pelvis at `rest`. */
export function carrier(pelvisY: number, rest: Rest): any[] {
  return [
    { id: P("lead-x"), parent: "pelvis", title: { en: "Guide dog carrier, forward slide", de: "Führhundträger, Längsschlitten" }, x: Math.round((rest.y - pelvisY) * 1e5) / 1e5, y: -rest.x, z: rest.z, angleDeg: 0, lengthM: 0.05, joint: { type: "prismatic", axisDeg: 90, minM: -0.8, maxM: 0.8 } },
    { id: P("lead-y"), parent: P("lead-x"), title: { en: "Guide dog carrier, vertical slide", de: "Führhundträger, Höhenschlitten" }, x: 0, y: 0, angleDeg: 0, lengthM: 0.05, joint: { type: "prismatic", axisDeg: 0, minM: -0.6, maxM: 0.6 } },
    { id: P("lead"), parent: P("lead-y"), title: { en: "Guide dog carrier, pitch", de: "Führhundträger, Neigung" }, x: 0, y: 0, angleDeg: -90, lengthM: 0.05, joint: { type: "revolute", minDeg: -60, maxDeg: 60 } },
  ];
}

/** 🎨️ Authored paint order of the dog against the handler: behind her in the side elevation and the plan, in front of her in the front elevation (the dog walks ahead). */
export const Z_SHIFT: Record<string, number> = { elevation: -20, plan: -20, front: 20 };

/** 🔤️ Renames every dog reference of a solid. */
function solid(s: any): any {
  const o = clone(s);
  o.id = P(o.id);
  if (o.bone) o.bone = P(o.bone);
  if (o.of) o.of = P(o.of);
  if (o.material) o.material = P(o.material);
  if (o.region) o.region = P(o.region);
  if (o.member) o.member = P(o.member);
  if (o.members) o.members = o.members.map(P);
  if (o.anchor) o.anchor = P(o.anchor);
  if (o.weights) o.weights = o.weights.map((w: any) => ({ ...w, bone: P(w.bone) }));
  if (o.tags) o.tags = o.tags.map((x: string) => (x === "emitter" || x === "glyph" ? x : P(x)));
  return o;
}

/** ✏️ Renames a dog shape and moves front and plan shapes by the rest offset. */
function shape(s: any, view: string, rest: Rest, pens: Record<string, number | null>): any {
  const o = clone(s);
  if (o.penMm === undefined && pens[o.role] != null) o.penMm = pens[o.role];
  o.id = P(o.id);
  o.bone = P(o.bone);
  if (o.region) o.region = P(o.region);
  if (o.regions) o.regions = o.regions.map(P);
  if (o.material) o.material = P(o.material);
  if (o.hatch) o.hatch = P(o.hatch);
  if (o.solid) throw new Error("[DEBUG] dog shape with solid");
  o.z = Math.round((o.z + Z_SHIFT[view]!) * 1e4) / 1e4;
  const [dx, dy] = view === "front" ? [rest.z, rest.y] : view === "plan" ? [rest.x, rest.z] : [0, 0];
  if (o.kind === "poly") o.points = o.points.map(([x, y]: number[]) => [Math.round((x! + dx) * 1e6) / 1e6, Math.round((y! + dy) * 1e6) / 1e6]);
  else if (o.kind === "capsule") Object.assign(o, { x0: o.x0 + dx, y0: o.y0 + dy, x1: o.x1 + dx, y1: o.y1 + dy });
  else throw new Error(`[DEBUG] dog shape kind ${o.kind}`);
  return o;
}

/** 🐕️ Adds the dog to the human asset `a` with its pelvis resting at `rest`; returns the dog asset for sampling. */
export function mergeDog(a: any, dogAsset: any, rest: Rest): void {
  const d = dogAsset;
  const pelvisY = a.rig.bones.find((b: any) => b.id === "pelvis").y;
  a.rig.bones.push(...carrier(pelvisY, rest));
  for (const b of d.rig.bones) a.rig.bones.push({ ...clone(b), id: P(b.id), parent: b.parent === null ? P("lead") : P(b.parent), title: { en: `Guide dog: ${b.title.en}`, de: `Führhund: ${b.title.de}` } });
  for (const an of d.anchors) a.anchors.push({ ...clone(an), id: P(an.id), bone: P(an.bone), title: { en: `Guide dog: ${an.title.en}`, de: `Führhund: ${an.title.de}` } });
  for (const m of d.materials) a.materials.push({ ...clone(m), id: P(m.id), title: { en: `Guide dog: ${m.title.en}`, de: `Führhund: ${m.title.de}` } });
  const refs = new Set(a.provenance.references.map((r: any) => r.id));
  for (const r of d.provenance.references) if (!refs.has(r.id)) a.provenance.references.push(clone(r));
  for (const r of d.thermal.regions) a.thermal.regions.push({ ...clone(r), id: P(r.id), material: P(r.material), title: { en: `Guide dog: ${r.title.en}`, de: `Führhund: ${r.title.de}` }, layers: r.layers.map((l: any) => ({ ...clone(l), material: P(l.material) })), body: r.body ? { ...r.body, subject: DOG } : undefined });
  for (const e of d.thermal.emitters) a.thermal.emitters.push({ ...clone(e), id: P(e.id), anchor: P(e.anchor), anchors: e.anchors?.map(P), region: e.region ? P(e.region) : undefined, title: { en: `Guide dog: ${e.title.en}`, de: `Führhund: ${e.title.de}` } });
  const resp = clone(d.thermal.respiration);
  resp.noseEmitter = resp.noseEmitter && P(resp.noseEmitter);
  resp.mouthEmitter = resp.mouthEmitter && P(resp.mouthEmitter);
  resp.motion = resp.motion.map((m: any) => ({ ...m, bone: P(m.bone) }));
  a.thermal.passengers = [{ id: DOG, title: { en: "Guide dog, a 30 kg Labrador Retriever", de: "Führhund, ein 30 kg schwerer Labrador Retriever" }, physiology: clone(d.thermal.body), respiration: resp }];
  for (const scale of SCALES) {
    const rep = a.representations[scale], dr = d.representations[scale];
    for (const view of VIEWS) rep.geometry[view].push(...dr.geometry[view].map((s: any) => shape(s, view, rest, dr.pens)));
    for (const h of dr.hatches ?? []) rep.hatches.push({ ...clone(h), id: P(h.id) });
    const vol = a.volumes[scale], dv = d.volumes[scale];
    for (const k of ["body", "thermal", "structure"]) vol[k].push(...dv[k].map(solid));
    vol.decisions.push(...dv.decisions.map((x: any) => ({ en: `Guide dog: ${x.en}`, de: `Führhund: ${x.de}` })));
  }
}

/** 🎚️ Dog bone offsets of a labrador pose or clip sample under the prefix. */
export function dogOffsets(offsets: Record<string, any>): Record<string, any> {
  const out: Record<string, any> = {};
  for (const [k, v] of Object.entries(offsets)) {
    const o: any = {};
    for (const key of ["angleDeg", "abductionDeg", "twistDeg"]) if (v[key]) o[key] = Math.round(v[key] * 1e4) / 1e4;
    if (Object.keys(o).length) out[P(k)] = o;
  }
  return out;
}

export { renameBones };
