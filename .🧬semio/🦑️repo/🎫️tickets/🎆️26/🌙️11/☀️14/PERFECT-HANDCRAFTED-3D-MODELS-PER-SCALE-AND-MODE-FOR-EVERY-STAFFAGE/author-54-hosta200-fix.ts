import { loftOf, openCtx, saveLists, superellipseSection, type Json } from "./kit-lib.ts";

const c = openCtx("vegetation/hosta-clump", 200);
const body: Json[] = c.vol.body;
const emitters = c.vol.thermal.filter((s: Json) => s.tags?.includes("emitter"));
const thermal: Json[] = [];
for (const b of body) {
  const cy = b.transform.translation[1];
  const cx = Math.abs(b.transform.translation[0]);
  const region = cy > 0.2 && cx < 0.3 ? "blades-inner" : "blades-outer";
  thermal.push({ id: `th-${b.id}`, of: b.id, region });
}
if (!thermal.some((s) => s.region === "blades-inner")) thermal[thermal.length - 1].region = "blades-inner";
if (!thermal.some((s) => s.region === "blades-outer")) thermal[0].region = "blades-outer";
const frames = c.state.frames;
for (const b of c.raw.rig.bones.filter((x: Json) => /^petiole-/.test(x.id))) {
  const f = frames[b.id]!;
  const L = b.lengthM;
  const p = (s: number): [number, number, number] => [f.o[0] + f.u[0] * s, f.o[1] + f.u[1] * s, f.o[2] + f.u[2] * s];
  const cap = loftOf({ id: `th-${b.id}`, bone: "root-plate", material: "hosta-petiole", tone: 0.3 }, [superellipseSection(p(0), 0.017, 0.017, 2), superellipseSection(p(L), 0.016, 0.016, 2)]);
  thermal.push({ ...cap, region: "petioles" });
}
thermal.push(...emitters);
const st: Json[] = c.vol.structure;
const k = st.findIndex((s) => s.id === "member-root-plate");
st[k] = { ...loftOf({ id: "member-root-plate", bone: "root-plate", material: "hosta-petiole", tone: 0.5 }, [superellipseSection([-0.03, 0.06, 0], 0.017, 0.017, 2), superellipseSection([0.03, 0.06, 0], 0.017, 0.017, 2)]), member: "root-plate", part: "member" };
saveLists(c, { thermal, structure: st });
console.log(thermal.length, "thermal");
