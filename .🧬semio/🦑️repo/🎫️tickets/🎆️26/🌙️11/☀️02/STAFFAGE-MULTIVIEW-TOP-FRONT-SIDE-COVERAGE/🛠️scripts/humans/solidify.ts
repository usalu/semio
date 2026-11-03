import { L, load, save } from "./lib.ts";
const { validateAsset } = await import(`${L}/🔨️modules/🗂️catalog/🟦️.ts`);
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { extentOf } = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const write = process.argv[2] === "write";
const r5 = (n: number) => +n.toFixed(5);
const snap = (a0: any) => { const a = JSON.parse(JSON.stringify(a0)); const out: any = {}; for (const r of Object.values<any>(a.representations)) { if (r.status !== "authored") continue; const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: r.denominator } }); for (const v of ["elevation", "front", "plan"]) out[`${r.denominator}/${v}`] = extentOf(st.shapes[v]); } return out; };
const localHalfWidth = (s: any): number => {
  if (s.kind === "capsule") return Math.max(s.r0, s.r1);
  if (s.kind === "ellipse") return Math.min(s.rx, s.ry);
  if (s.kind === "poly") { const ys = s.points.map((p: number[]) => p[0]); return (Math.max(...ys) - Math.min(...ys)) / 2; }
  return 0.04;
};
for (const slug of process.argv.slice(3)) {
  const d = load(slug); const a = d.asset; const before = snap(a);
  const bone = (id: string) => a.rig.bones.find((b: any) => b.id === id); const worldZ = (id: string): number => { let z = 0; for (let b = bone(id); b; b = b.parent ? bone(b.parent) : undefined) z += b.z ?? 0; return z; };
  const issues = validateAsset(a).filter((i: any) => i.code === "lateral-ambiguous");
  let n = 0, skipped = 0;
  for (const iss of issues) {
    const b = bone(iss.data.bone); const L0 = b.lengthM;
    for (const band of iss.data.bands as string[]) {
      const [den, view] = [band.split(" ")[0]!.slice(2), band.split(" ")[1]!];
      const rep = a.representations[den]; const shapes: any[] = rep.geometry[view];
      for (const id of iss.data.shapes as string[]) {
        const s = shapes.find((x) => x.id === id && x.bone === iss.data.bone); if (!s || s.solid) continue;
        const side = iss.data.bone.endsWith("-l") ? 1 : -1;
        const planCap = rep.geometry.plan.find((x: any) => x.bone === iss.data.bone && x.kind === "capsule");
        const zc = (planCap?.y0 ?? side * 0.15);
        if (view === "elevation") { const t = Math.max(0.02, planCap ? Math.max(planCap.r0, planCap.r1) : 0.04); s.solid = { kind: "extruded", sweep: "w", z0: r5(zc - t), z1: r5(zc + t) }; n++; continue; }
        if (process.env.SKIPRE && new RegExp(process.env.SKIPRE).test(iss.data.bone)) { skipped++; continue; }
        if (s.role !== "body") { skipped++; shapes.splice(shapes.indexOf(s), 1); continue; }
        const rr = planCap ? [planCap.r0, planCap.r1] : (() => { const h = localHalfWidth(s); return [h, h * 0.85]; })();
        const w = view === "front" ? { x0: 0, x1: 0 } : {};
        for (const k of ["kind", "x0", "y0", "x1", "y1", "r0", "r1", "rx", "ry", "cx", "cy", "points", "closed", "followTip", "d", "rotationDeg", "x", "y", "w", "h", "r"]) delete (s as any)[k];
        Object.assign(s, { kind: "poly", closed: true, points: [[0, 0], [0, r5(rr[0])], [r5(L0), r5(rr[1])], [r5(L0), 0]], solid: { kind: "revolved", axisBone: iss.data.bone, axis: "u", axisX: 0, axisZ: r5(zc - worldZ(iss.data.bone)), mirror: true } }); void w;
        n++;
      }
    }
  }
  const after = snap(a); let worst = 0; const rows: string[] = [];
  for (const k of Object.keys(before)) for (const e of ["minX", "maxX", "minY", "maxY"]) { const dd = Math.abs(before[k][e] - after[k][e]); worst = Math.max(worst, dd); if (dd > 0.015) rows.push(`${k}.${e}=${dd.toFixed(3)}`); }
  const left = validateAsset(a).filter((i: any) => i.code === "lateral-ambiguous").length;
  console.log(slug, "converted", n, "skipped details", skipped, "default-pose extent change max", worst.toFixed(3), rows.slice(0, 5).join(" "), "remaining lateral-ambiguous", left, write ? "WRITE" : "dry");
  if (write) save(slug, d);
}
