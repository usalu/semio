import { ID, bands, boxes, load, save, stateAt, three, transformShape, type Aff } from "./lib.ts";
const [id, write] = [process.argv[2]!, process.argv[3] === "write"];
if (["agave", "date-palm", "hosta-clump", "miscanthus-grass"].includes(id)) { console.log("[DEBUG] skip " + id); process.exit(0); }
const x = load(id); const a = x.asset; const D = a.dimensions;
const LEAFY = /leaf|leaves|needle|blade|frond|plume|curtain|crown|canopy/i;
const log = (m: string) => console.log(`[DEBUG] ${id} ${m}`);
const keep = /^(root|trunk|stem|culm|post|pollard|leader$|ld$)/;
const geomOf = (den: number) => a.representations[String(den)].geometry;
const ext = (den: number) => { const st = stateAt(a, den); return { e: three.extentOf(st.shapes.elevation), f: three.extentOf(st.shapes.front), p: three.extentOf(st.shapes.plan), st }; };
const shiftBone = (den: number, bone: string, dx: number, dy: number) => { for (const s of geomOf(den).plan) if (s.bone === bone) transformShape(s, { sx: 1, tx: dx, sy: 1, ty: dy }); };
for (const den of bands(a)) {
  const thr = Math.max(0.12, 0.03 * Math.max(D.heightM, D.widthM, D.depthM) / 3);
  const st = stateAt(a, den); const bx = boxes(st);
  for (const bone of Object.keys(bx.plan)) {
    const plans = geomOf(den).plan.filter((s: any) => s.bone === bone);
    if (!/^(fol|curt|cap|crown)/.test(bone) && !plans.every((s: any) => LEAFY.test(s.region ?? (s.regions ?? []).join(" ")))) continue;
    const e = bx.elevation[bone], f = bx.front[bone], p = bx.plan[bone]; if (!e || !f) continue;
    const dx = (e[0] + e[1]) / 2 - (p[0] + p[1]) / 2, dy = (f[0] + f[1]) / 2 - (p[2] + p[3]) / 2;
    const ax = Math.abs(dx) > thr ? dx : 0, ay = Math.abs(dy) > thr ? dy : 0;
    if (ax || ay) { log(`1:${den} shift plan ${bone} by (${ax.toFixed(2)}, ${ay.toFixed(2)})`); shiftBone(den, bone, ax, ay); }
  }
  for (let pass = 0; pass < 4; pass++) {
    const { e, f, p } = ext(den);
    const tl = (nom: number, m: number) => Math.abs(m - nom) > 0.005 * nom;
    const fix = (view: "front" | "plan", axis: "x" | "y", lo: number, hi: number, tLo: number, tHi: number, all = false) => {
      const sc = (tHi - tLo) / (hi - lo); const aff: Aff = { ...ID }; const sk = axis === "x" ? "sx" : "sy", tk = axis === "x" ? "tx" : "ty";
      aff[sk] = sc; aff[tk] = tLo - lo * sc;
      for (const s of geomOf(den)[view]) if (s.role !== "ground" && (all || !keep.test(s.bone))) transformShape(s, aff);
      log(`1:${den} ${view}.${axis} [${lo.toFixed(2)},${hi.toFixed(2)}] -> [${tLo.toFixed(2)},${tHi.toFixed(2)}]`);
    };
    let changed = false;
    if (tl(D.depthM, f.maxX - f.minX)) { const c = (f.minX + f.maxX) / 2; fix("front", "x", f.minX, f.maxX, c - D.depthM / 2, c + D.depthM / 2); changed = true; }
    if (tl(D.heightM, f.maxY - f.minY)) { fix("front", "y", f.minY, f.maxY, 0, D.heightM, true); changed = true; }
    const e2 = ext(den); const f2 = e2.f, p2 = e2.p;
    const ec = (e.minX + e.maxX) / 2, fc = (f2.minX + f2.maxX) / 2;
    if (tl(D.widthM, p2.maxX - p2.minX) || Math.abs(p2.minX - (ec - D.widthM / 2)) > 0.005 * D.widthM) { fix("plan", "x", p2.minX, p2.maxX, ec - D.widthM / 2, ec + D.widthM / 2); changed = true; }
    if (tl(D.depthM, p2.maxY - p2.minY) || Math.abs(p2.minY - (fc - D.depthM / 2)) > 0.005 * D.depthM) { fix("plan", "y", p2.minY, p2.maxY, fc - D.depthM / 2, fc + D.depthM / 2); changed = true; }
    if (!changed) break;
  }
  const { e, f, p } = ext(den);
  log(`1:${den} final el ${(e.maxX - e.minX).toFixed(3)}x${(e.maxY - e.minY).toFixed(3)} fr ${(f.maxX - f.minX).toFixed(3)}x${(f.maxY - f.minY).toFixed(3)} pl ${(p.maxX - p.minX).toFixed(3)}x${(p.maxY - p.minY).toFixed(3)} | dims ${D.widthM}x${D.depthM}x${D.heightM}`);
}
if (write) { save(x); log("written"); }
