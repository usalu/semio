import { L, load, save } from "./lib.ts";
const { measureSuitability, DEFAULT_PRINT_PROFILE } = await import(`${L}/🔨️modules/🖨️representation/🟦️.ts`);
const clone = (x: any) => JSON.parse(JSON.stringify(x));
const limb = /^(upper-arm|arm|forearm|hand|thigh|shank|foot)(-[lr])?$/;
const VIEWS = ["elevation", "front", "plan"] as const;
const refZ = (shapes: any[]) => (shapes.find((s) => s.bone === "torso" && s.role === "body") ?? shapes.find((s) => s.bone === "pelvis" && s.role === "body"))?.z;

export function fill(slug: string, write: boolean): string[] {
  const doc = load(slug); const a = doc.asset; const log: string[] = [];
  const scales = Object.values<any>(a.representations).filter((r) => r.status === "authored").map((r) => r.denominator).sort((x, y) => x - y);
  for (const den of scales) {
    const rep = a.representations[String(den)];
    const bonesIn = (v: string) => new Set(rep.geometry[v].filter((s: any) => s.role !== "ground").map((s: any) => s.bone));
    const union = new Set([...VIEWS.flatMap((v) => [...bonesIn(v)])]);
    const feature = (0.2 * den) / 1000;
    for (const v of VIEWS) {
      const have = bonesIn(v);
      const missing = [...union].filter((b: any) => limb.test(b) && !have.has(b));
      for (const b of missing as string[]) {
        let src: any[] = []; let srcRep: any;
        for (const d2 of scales.filter((d) => d < den).reverse().concat(scales.filter((d) => d > den))) {
          const r2 = a.representations[String(d2)];
          const found = r2.geometry[v].filter((s: any) => s.bone === b && s.role === "body");
          if (found.length) { src = found; srcRep = r2; break; }
        }
        if (!src.length) { log.push(`${den}/${v}/${b}: no source`); continue; }
        const dz = (refZ(rep.geometry[v]) ?? 0) - (refZ(srcRep.geometry[v]) ?? 0);
        for (const s0 of src) {
          const s = clone(s0); delete s.hatch; delete s.penMm;
          let id = s.id; while (rep.geometry[v].some((x: any) => x.id === id)) id += "-fill"; s.id = id;
          s.z = +(s.z + dz).toFixed(3);
          if (s.kind === "capsule") { s.r0 = Math.max(s.r0, feature / 2); s.r1 = Math.max(s.r1, feature / 2); }
          if (s.kind === "ellipse") { s.rx = Math.max(s.rx, feature / 2); s.ry = Math.max(s.ry, feature / 2); }
          rep.geometry[v].push(s);
        }
        log.push(`${den}/${v}/${b}: +${src.length} from 1:${srcRep.denominator}`);
      }
    }
  }
  if (write) save(slug, doc);
  const bad: string[] = [];
  for (const den of scales) { const s = measureSuitability(a, a.representations[String(den)], den, DEFAULT_PRINT_PROFILE); if (!s.suitable) bad.push(`${den}: ${s.violations.length} print violations (${s.violations.slice(0, 2).map((x: any) => x.check + ":" + x.shapes.join("+")).join(" ")})`); }
  log.push(...bad.map((b) => "SUITABILITY " + b));
  return log;
}
if (import.meta.main) { const write = process.argv[2] === "write"; for (const s of process.argv.slice(3)) { console.log("##", s); for (const l of fill(s, write)) console.log("  ", l); } }
