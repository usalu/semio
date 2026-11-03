import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const a = catalog().find((x) => x.id === "date-palm")!;
const st: any = evaluate(a, { kind: "pose", pose: "storm" } as any, { kind: "scale", scale: { denominator: 50 } } as any);
for (const v of ["front", "plan"]) {
  const rows = st.shapes[v].filter((s: any) => s.shape.role !== "ground").map((s: any) => { const xs = s.rings.flat().map((p: any) => (v === "front" ? p.x : p.y)); return [s.shape.id, s.shape.bone, Math.min(...xs), Math.max(...xs)] as const; });
  rows.sort((p: any, q: any) => p[2] - q[2]);
  console.log(v, "MIN", rows.slice(0, 4).map((r: any) => `${r[0]}@${r[1]} ${r[2].toFixed(2)}`).join(" | "));
  rows.sort((p: any, q: any) => q[3] - p[3]);
  console.log(v, "MAX", rows.slice(0, 4).map((r: any) => `${r[0]}@${r[1]} ${r[3].toFixed(2)}`).join(" | "));
}
