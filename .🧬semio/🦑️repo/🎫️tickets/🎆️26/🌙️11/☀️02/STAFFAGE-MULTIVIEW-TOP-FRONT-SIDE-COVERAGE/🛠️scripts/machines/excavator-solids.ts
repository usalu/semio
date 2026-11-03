import { load, save, rep } from "./lib.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { extentOf } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

const SLEW = new Set(["upper", "counterweight", "engine", "boom", "stick", "bucket", "boom-cyl", "boom-rod", "stick-cyl", "stick-rod", "bucket-cyl", "bucket-rod"]);
const META = new Set(["id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm", "solid"]);
const r3 = (n: number) => Math.round(n * 1000) / 1000;
const ELEV_TWIN: Record<string, string[]> = { "cab-glass": ["cab-glass", "cab-glass-front", "cab-glass-pane"] };
const FORWARD_FROM: Record<string, string> = { house: "house-counterweight", "cab-block": "cab-roof", "cab-pillar": "cab-roof" };
const HEIGHT_FROM: Record<string, [string, string]> = { "hood-louvre-1": ["elevation", "hood-louvres"], "hood-louvre-2": ["elevation", "hood-louvres"], "hood-louvre-3": ["elevation", "hood-louvres"], "hood-louvre-4": ["elevation", "hood-louvres"], "hood-louvre-5": ["elevation", "hood-louvres"], "hood-louvre-6": ["elevation", "hood-louvres"], "cab-roof": ["front", "cab-block"] };
const Z_REF: [RegExp, string][] = [
  [/^boom-cyl/, "boom-cyl-barrel-right"], [/^boom-rod/, "boom-rod-right"], [/^stick-cyl/, "stick-cyl-barrel"], [/^stick-rod/, "stick-rod"],
  [/^bucket-cyl/, "bucket-cyl-barrel"], [/^bucket-rod/, "bucket-rod"], [/^boom/, "boom-body"], [/^stick/, "stick-body"], [/^bucket/, "bucket-body"],
];

const PLAN_REF: [RegExp, string[]][] = [[/^(skirt|hood|engine|house)/, ["house", "house-counterweight"]], [/^counterweight/, ["counterweight-body", "house-counterweight"]], [/^cab/, ["cab-roof", "cab-glass"]], [/^stack/, ["stack"]]];
const doc = load("excavator");
const asset = doc.asset;
const log: string[] = [];
for (const k of [50, 100, 200, 500]) {
  const r = rep(doc, k);
  const state: any = evaluate(asset, { kind: "pose", pose: "stowed" }, { kind: "scale", scale: { denominator: k } } as any);
  const ext = (view: string, id: string) => { const s = state.shapes[view].find((x: any) => x.shape.id === id); return s ? extentOf([s], true) : undefined; };
  const G = r.geometry;
  const find = (view: string, id: string) => G[view].find((s: any) => s.id === id);
  const origin = (bone: string) => state.bones[bone].origin as { x: number; y: number };
  const zRef = (id: string): [number, number] | undefined => {
    for (const view of ["front", "plan"]) { const s = find(view, id); if (s?.solid?.kind === "extruded") return [s.solid.z0, s.solid.z1]; }
    for (const [re, ref] of Z_REF) if (re.test(id)) for (const view of ["front", "plan"]) { const s = find(view, ref); if (s?.solid?.kind === "extruded") return [s.solid.z0, s.solid.z1]; }
    for (const [re, refs] of PLAN_REF) if (re.test(id)) for (const ref of refs) { const z = zOwn("plan", ref); if (z) return z; }
    return undefined;
  };
  const zOwn = (view: string, id: string): [number, number] | undefined => { const e = ext(view, id); return e ? (view === "front" ? [e.minX, e.maxX] : [e.minY, e.maxY]) : undefined; };
  const elevTwin = (id: string, bone: string) => (ELEV_TWIN[id] ?? [id]).map((i) => G.elevation.find((s: any) => s.id === i && s.bone === bone)).find(Boolean);

  const converted: Record<string, any[]> = { elevation: [], front: [], plan: [] };
  for (const view of ["elevation", "front", "plan"]) {
    G[view] = G[view].map((s: any) => {
      if (!SLEW.has(s.bone) || s.solid !== undefined || s.role === "ground") return s;
      const id = s.id;
      let z: [number, number] | undefined = view === "elevation" ? zOwn("front", id) ?? zOwn("plan", id) ?? zRef(id) : zOwn(view, id);
      if (z === undefined) z = zRef(id);
      if (z === undefined && FORWARD_FROM[id]) z = zOwn(view === "front" ? "front" : "plan", id);
      if (z === undefined) throw new Error(`${k} ${view} ${id}: no lateral range`);
      const solid = { kind: "extruded", z0: r3(z[0]), z1: r3(z[1]) };
      if (view === "elevation") { converted[view].push(id); return { ...s, solid }; }
      const twin = elevTwin(id, s.bone);
      let geometry: any;
      if (twin) {
        geometry = Object.fromEntries(Object.entries(twin).filter(([key]) => !META.has(key)));
      } else {
        const o = origin(s.bone);
        const fwd = view === "plan" ? ext("plan", id) : ext("plan", id) ?? ext("plan", FORWARD_FROM[id] ?? "");
        const hgt = view === "front" ? ext("front", id) : ext("front", id) ?? ext(HEIGHT_FROM[id]?.[0] ?? "front", HEIGHT_FROM[id]?.[1] ?? "");
        if (!fwd || !hgt) throw new Error(`${k} ${view} ${id}: cannot synthesise a profile (forward ${!!fwd}, height ${!!hgt})`);
        geometry = { kind: "rect", x: r3(fwd.minX - o.x), y: r3(hgt.minY - o.y), w: r3(fwd.maxX - fwd.minX), h: r3(hgt.maxY - hgt.minY) };
      }
      converted[view].push(`${id}${twin ? "" : "*"}`);
      const meta = Object.fromEntries(Object.entries(s).filter(([key]) => META.has(key)));
      return { ...geometry, ...meta, solid };
    });
  }
  log.push(`${k}: elevation ${converted.elevation.length} front ${converted.front.length} plan ${converted.plan.length}  (* = synthesised box profile) front: ${converted.front.join(" ")} plan: ${converted.plan.join(" ")}`);
}
save("excavator", doc);
console.log(log.join("\n"));
