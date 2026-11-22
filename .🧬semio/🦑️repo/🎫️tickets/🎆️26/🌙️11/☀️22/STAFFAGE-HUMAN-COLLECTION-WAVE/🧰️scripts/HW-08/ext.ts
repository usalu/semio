import { loadDoc, evaluate } from "./lib.ts";
const [slug, sc, view, n] = process.argv.slice(2);
const a = loadDoc(slug!).asset;
const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } } as any);
const rows = (st.shapes as any)[view!].map((s: any) => { const xs = s.rings.flat().map((p: any) => p.x); return [s.shape.id, Math.min(...xs), Math.max(...xs)]; });
rows.sort((p: any, q: any) => p[1] - q[1]); console.log(rows.slice(0, Number(n ?? 4)).map((r: any) => `${r[0]} ${r[1].toFixed(3)}`).join(" | "));
rows.sort((p: any, q: any) => q[2] - p[2]); console.log(rows.slice(0, Number(n ?? 4)).map((r: any) => `${r[0]} ${r[2].toFixed(3)}`).join(" | "));
