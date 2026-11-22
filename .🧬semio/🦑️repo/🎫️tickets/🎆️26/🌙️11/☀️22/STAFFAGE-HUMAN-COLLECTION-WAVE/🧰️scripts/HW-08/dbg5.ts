import { loadDoc, evaluate } from "./lib.ts";
const a = loadDoc("hiker-with-trekking-poles").asset;
const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } } as any);
for (const s of st.shapes.elevation as any[]) if (/strap|^jacket$/.test(s.shape.id)) { const xs = s.rings.flat(); console.log(s.shape.id, Math.min(...xs.map((p: any) => p.x)).toFixed(3), Math.max(...xs.map((p: any) => p.x)).toFixed(3), Math.min(...xs.map((p: any) => p.y)).toFixed(3), Math.max(...xs.map((p: any) => p.y)).toFixed(3)); }
