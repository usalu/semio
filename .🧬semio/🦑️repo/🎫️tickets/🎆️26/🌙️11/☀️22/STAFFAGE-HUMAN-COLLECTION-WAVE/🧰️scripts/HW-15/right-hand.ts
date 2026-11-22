/** 🖐️ Replaces the right hand of `person-with-white-cane` (closed on the cane grip with the index along it) by the mirror image of her relaxed left hand: rest placement of the fifteen finger bones, the drawn hand of every view and scale and every body, thermal and structure solid of the hand bones (bone-local w negated).
 * @see ./gen-guide-dog.ts
 */
import { SCALES, VIEWS, clone } from "./lib.ts";

const FINGERS = ["thumb-metacarpal", "thumb-proximal", "thumb-distal", "index-proximal", "index-middle", "index-distal", "middle-proximal", "middle-middle", "middle-distal", "ring-proximal", "ring-middle", "ring-distal", "little-proximal", "little-middle", "little-distal"];
const LEFT = new Set(["hand-l", ...FINGERS.map((f) => `${f}-l`)]);
const RIGHT = new Set(["hand-r", ...FINGERS.map((f) => `${f}-r`)]);
const toRight = (s: string) => s.replace(/(^|-)l(?=-|$)/g, "$1r");
const flipQ = (q: number[]) => [-q[0]!, -q[1]!, q[2]!, q[3]!];
const flip3 = (p: number[]) => [p[0]!, p[1]!, -p[2]!];

/** 🪞️ A left-hand solid as the right-hand solid (bone-local w negated). */
function mirrorSolid(s: any): any {
  const o = clone(s);
  for (const k of ["id", "bone", "of", "member"]) if (typeof o[k] === "string") o[k] = toRight(o[k]);
  if (o.members) o.members = o.members.map(toRight);
  if (o.weights) o.weights = o.weights.map((w: any) => ({ ...w, bone: toRight(w.bone) }));
  if (o.p0) o.p0 = flip3(o.p0);
  if (o.p1) o.p1 = flip3(o.p1);
  if (o.sections) o.sections = o.sections.map((x: any) => ({ ...x, at: flip3(x.at), ...(x.rotation ? { rotation: flipQ(x.rotation) } : {}) }));
  if (o.transform) o.transform = { ...o.transform, ...(o.transform.translation ? { translation: flip3(o.transform.translation) } : {}), ...(o.transform.rotation ? { rotation: flipQ(o.transform.rotation) } : {}) };
  for (const k of ["axis", "origin"]) if (o[k]) o[k] = flip3(o[k]);
  if (o.kind && !["capsule", "loft", "ellipsoid", "box", "revolve"].includes(o.kind)) throw new Error(`[DEBUG] mirror ${o.kind}`);
  if (o.kind === "box" || o.kind === "extrude") throw new Error("[DEBUG] mirror box");
  return o;
}

/** 🖐️ Mirrors the left hand onto the right. */
export function mirrorRightHand(a: any): void {
  for (const f of FINGERS) {
    const l = a.rig.bones.find((b: any) => b.id === `${f}-l`), r = a.rig.bones.find((b: any) => b.id === `${f}-r`);
    Object.assign(r, { x: l.x, y: l.y, z: -(l.z ?? 0), angleDeg: l.angleDeg, abductionDeg: -(l.abductionDeg ?? 0), twistDeg: -(l.twistDeg ?? 0), lengthM: l.lengthM, joint: { ...clone(l.joint), ...(l.joint.twistMinDeg !== undefined ? { twistMinDeg: -l.joint.twistMaxDeg, twistMaxDeg: -l.joint.twistMinDeg } : {}) } });
  }
  for (const scale of SCALES) {
    const g = a.representations[scale].geometry;
    for (const view of VIEWS) {
      const lefts = g[view].filter((s: any) => LEFT.has(s.bone));
      const zr = new Map(g[view].filter((s: any) => RIGHT.has(s.bone)).map((s: any) => [s.id, s.z]));
      g[view] = g[view].filter((s: any) => !RIGHT.has(s.bone)).concat(lefts.map((s: any) => {
        const o = clone(s);
        o.id = toRight(o.id); o.bone = toRight(o.bone);
        o.z = zr.get(o.id) ?? o.z;
        if (o.solid?.kind === "extruded" && o.solid.sweep === "w") o.solid = { ...o.solid, z0: -s.solid.z1, z1: -s.solid.z0, ...(s.solid.zTip ? { zTip: [-s.solid.zTip[1], -s.solid.zTip[0]] } : {}) };
        else if (o.solid?.kind === "revolved") o.solid = { ...o.solid, axisBone: toRight(o.solid.axisBone), axisZ: -(o.solid.axisZ ?? 0) };
        else if (o.solid) throw new Error("[DEBUG] hand solid");
        else throw new Error("[DEBUG] flat hand shape");
        return o;
      }));
    }
    const vol = a.volumes[scale];
    const gone = new Set<string>();
    for (const k of ["body", "thermal", "structure"]) vol[k] = vol[k].filter((s: any) => { const d = RIGHT.has(s.bone) || RIGHT.has(s.member); if (d) gone.add(s.id); return !d; });
    for (const k of ["body", "thermal", "structure"]) vol[k] = vol[k].filter((s: any) => !(s.of && gone.has(s.of)) || false);
    const leftIds = new Set<string>();
    for (const k of ["body", "thermal", "structure"]) for (const s of vol[k]) if (LEFT.has(s.bone) || LEFT.has(s.member)) leftIds.add(s.id);
    for (const k of ["body", "thermal", "structure"]) vol[k].push(...vol[k].filter((s: any) => leftIds.has(s.id) || (s.of && leftIds.has(s.of))).map(mirrorSolid));
  }
}
