import { L, load, save } from "./lib.ts";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const WITHIN = (m: number, n: number) => Math.abs(m - n) <= Math.max(0.02 * n, 0.015);

export function shift(shape: any, view: "front" | "plan", delta: number): boolean {
  if (shape.solid?.kind === "extruded") { shape.solid.z0 = +(shape.solid.z0 + delta).toFixed(5); shape.solid.z1 = +(shape.solid.z1 + delta).toFixed(5); return true; }
  if (shape.solid?.kind === "revolved") { shape.solid.axisZ = +((shape.solid.axisZ ?? 0) + delta).toFixed(5); return true; }
  const r = (n: number) => +n.toFixed(5);
  const front = view === "front";
  switch (shape.kind) {
    case "poly": for (const p of shape.points) p[front ? 0 : 1] = r(p[front ? 0 : 1] + delta); return true;
    case "ellipse": if (front) shape.cx = r(shape.cx + delta); else shape.cy = r(shape.cy + delta); return true;
    case "capsule": if (front) { shape.x0 = r(shape.x0 + delta); shape.x1 = r(shape.x1 + delta); } else { shape.y0 = r(shape.y0 + delta); shape.y1 = r(shape.y1 + delta); } return true;
    case "rect": if (front) shape.x = r(shape.x + delta); else shape.y = r(shape.y + delta); return true;
    default: return false;
  }
}

export function fixLateral(slug: string, only?: { scales?: number[]; views?: ("front" | "plan")[] }): void {
  const doc = load(slug); const a = doc.asset; const nominal = a.dimensions.depthM;
  for (const rep of Object.values<any>(a.representations)) {
    if (rep.status !== "authored") continue;
    if (only?.scales && !only.scales.includes(rep.denominator)) continue;
    for (const view of (only?.views ?? ["front", "plan"]) as ("front" | "plan")[]) {
      for (let it = 0; it < 4; it++) {
        const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: rep.denominator } });
        const rows = st.shapes[view].filter((s: any) => s.shape.role !== "ground").map((s: any) => { const p = s.rings.flat().map((q: any) => (view === "front" ? q.x : q.y)); return { id: s.shape.id, lo: Math.min(...p), hi: Math.max(...p) }; });
        const lo = Math.min(...rows.map((r: any) => r.lo)), hi = Math.max(...rows.map((r: any) => r.hi));
        const measured = hi - lo;
        if (WITHIN(measured, nominal) && it > 0 || Math.abs(measured - nominal) < 0.002) break;
        if (WITHIN(measured, nominal) && it === 0) break;
        const E = measured - nominal;
        const asym = Math.abs(hi + lo);
        const dHi = asym > 0.02 ? (hi > -lo ? E : 0) : E / 2;
        const dLo = asym > 0.02 ? (hi > -lo ? 0 : E) : E / 2;
        const edge = (ids: string[]) => ids.map((id) => rep.geometry[view].find((s: any) => s.id === id)).filter(Boolean);
        const hiIds = rows.filter((r: any) => Math.abs(r.hi - hi) < 0.003).map((r: any) => r.id);
        const loIds = rows.filter((r: any) => Math.abs(r.lo - lo) < 0.003).map((r: any) => r.id);
        const done: string[] = [];
        const chainOf = (s: any): any[] => {
          const m = /^(upper-arm|arm|forearm|hand)-([lr])$/.exec(s.bone) ?? /^(thigh|shank|foot|toe)-([lr])$/.exec(s.bone);
          if (!m) return [s];
          const family = /arm|hand/.test(m[1]!) ? /^(upper-arm|arm|forearm|hand)-/ : /^(thigh|shank|foot|toe)-/;
          return rep.geometry[view].filter((x: any) => family.test(x.bone) && x.bone.endsWith("-" + m[2]) && x.role !== "ground");
        };
        const moved = new Set<any>();
        const apply = (ids: string[], delta: number) => { for (const s of edge(ids)) for (const c of chainOf(s)) { if (moved.has(c)) continue; moved.add(c); done.push(shift(c, view, delta) ? `${c.id}:${delta > 0 ? "+" : ""}${delta.toFixed(3)}` : `${c.id}:SKIP`); } };
        if (dHi !== 0) apply(hiIds, -dHi);
        if (dLo !== 0) apply(loIds, dLo);
        console.log(slug, rep.denominator, view, `it${it} measured ${measured.toFixed(3)} nominal ${nominal}`, done.join(" "));
      }
    }
  }
  save(slug, doc);
}
if (import.meta.main) fixLateral(process.argv[2]!);
