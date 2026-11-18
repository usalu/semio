/** 🚩 The drawn shapes that leave the range of the other view (the culprits of a spread finding). bun spread-out.ts <asset> <scale> <pose:ID|clip:ID:T> */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { toleranceM } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { loadAsset } from "./humansa-lib.ts";
const [name, scale, q] = process.argv.slice(2);
const a = await loadAsset(name);
const p = q.split(":");
const query: any = p[0] === "pose" ? { kind: "pose", pose: p[1] } : { kind: "clip", clip: p[1], timeS: Number(p[2]) };
const st = evaluate(a, query, { kind: "scale", scale: { denominator: Number(scale) } }) as any;
const tol = toleranceM(a);
const rows = (v: string) => st.shapes[v].filter((s: any) => s.shape.role !== "ground").map((s: any) => { const pts = s.rings.flat(); const xs = pts.map((t: any) => t.x), ys = pts.map((t: any) => t.y); return { id: s.shape.id, bone: s.shape.bone, solid: s.shape.solid !== undefined, x0: Math.min(...xs), x1: Math.max(...xs), y0: Math.min(...ys), y1: Math.max(...ys) }; });
const R = { elevation: rows("elevation"), front: rows("front"), plan: rows("plan") };
const ext = (r: any[], ax: string) => [Math.min(...r.map((t) => t[ax + "0"])), Math.max(...r.map((t) => t[ax + "1"]))];
const f = (r: any) => `${String(r.id).padEnd(26)} ${String(r.bone).padEnd(14)} ${r.solid ? "S" : " "} x[${r.x0.toFixed(3)},${r.x1.toFixed(3)}] y[${r.y0.toFixed(3)},${r.y1.toFixed(3)}]`;
const check = (label: string, view: string, ax: string, ref: number[], other: string) => {
  const hit = R[view as keyof typeof R].filter((r: any) => r[ax + "0"] < ref[0] - tol + 1e-9 || r[ax + "1"] > ref[1] + tol - 1e-9);
  const lo = R[view as keyof typeof R].filter((r: any) => r[ax + "0"] < ref[0] - tol * 0.5), hi = R[view as keyof typeof R].filter((r: any) => r[ax + "1"] > ref[1] + tol * 0.5);
  console.log(`${label}: ${view} ${ax} vs ${other} [${ref[0].toFixed(3)},${ref[1].toFixed(3)}] own [${ext(R[view as keyof typeof R], ax).map((n) => n.toFixed(3))}] tol ${tol.toFixed(3)}`);
  for (const r of [...new Set([...lo, ...hi])]) console.log("    " + f(r));
};
check("1", "plan", "x", ext(R.elevation, "x"), "elevation");
check("1b", "elevation", "x", ext(R.plan, "x"), "plan");
check("2", "front", "y", ext(R.elevation, "y"), "elevation");
check("2b", "elevation", "y", ext(R.front, "y"), "front");
check("3", "plan", "y", ext(R.front, "x"), "front x");
check("3b", "front", "x", ext(R.plan, "y"), "plan y");
