import { loftOf, openCtx, saveLists, superellipseSection, featureM, type Json } from "./kit-lib.ts";

const scale = Number(process.argv[2]);
const c = openCtx("vegetation/hosta-clump", scale);
const geo = c.raw.representations[String(scale)].geometry;
const F = featureM(scale);
const regionOf = (id: string): string[] => {
  const sh = geo.elevation.find((s: Json) => s.id === id);
  return sh?.regions ?? (sh?.region ? [sh.region] : []);
};
const old: Json[] = c.vol.thermal;
const emitters = old.filter((s) => s.tags?.includes("emitter"));
const thermal: Json[] = [];
const hidden = new Set(c.m.thermal.regions.filter((r: Json) => r.hidden !== undefined).map((r: Json) => r.id));
for (const b of c.vol.body)
  for (const region of regionOf(b.id)) {
    const id = `th-${b.id}${regionOf(b.id).length > 1 ? `-${region}` : ""}`;
    if (hidden.has(region)) {
      const own = JSON.parse(JSON.stringify(b));
      thermal.push({ ...own, id, region });
    } else thermal.push({ id, of: b.id, region });
  }
thermal.push(...emitters);
const st: Json[] = c.vol.structure;
const k = st.findIndex((s) => s.id === "member-root-plate");
if (k >= 0) {
  const r = Math.max(F * 0.8, 0.0042);
  st[k] = { ...loftOf({ id: "member-root-plate", bone: "root-plate", material: "hosta-petiole", tone: 0.5 }, [superellipseSection([-0.012, 0.075, 0], r, r, 2), superellipseSection([0.012, 0.075, 0], r, r, 2)]), member: "root-plate", part: "member" };
}
saveLists(c, { thermal, structure: st });
console.log(thermal.length, "thermal solids");
