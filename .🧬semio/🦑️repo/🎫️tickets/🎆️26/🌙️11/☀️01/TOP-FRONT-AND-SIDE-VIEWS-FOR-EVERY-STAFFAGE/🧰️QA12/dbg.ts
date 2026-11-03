import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery";
const asset = JSON.parse(readFileSync(`${S}/concrete-mixer-truck/🔣️.json`, "utf8")).asset;
const g = asset.representations["100"].geometry;
const old: any = {}; for (const v of ["front","plan"]) old[v] = g[v].filter((s: any) => s.id==="drum"||s.id==="drum-belly").map((s:any)=>s);
const prof = [[0, 0], [0.05, 0.3681], [0.14, 0.5999], [0.28, 0.8119], [0.45, 0.9699], [0.65, 1.0759], [0.9, 1.12], [3, 1.12], [5.4, 0.5], [5.4, 0], [0, 0]];
for (const v of ["front", "plan"] as const) {
  g[v] = g[v].filter((s: any) => s.id !== "drum" && s.id !== "drum-belly" && s.id !== "drum-seam");
  g[v].push({ id: "drum", bone: "drum", role: "body", z: 2.0, filled: true, tone: 0.3, kind: "poly", points: prof, closed: true, region: "drum", material: "drum-paint", solid: { kind: "revolved", axisBone: "drum", axisX: 1.9, axisZ: 0, mirror: true } });
}
const st0 = evaluate(asset, { kind: "pose", pose: "transit-agitating" }, { kind: "scale", scale: { denominator: 100 } } as any);
for (const v of ["front","plan"] as const) { const s = st0.shapes[v].find((x:any)=>x.shape.id==="drum"||x.shape.id==="drum-belly")!; const xs = s.rings.flat().map((p: any) => p.x), ys = s.rings.flat().map((p: any) => p.y); console.log("[DEBUG] envelope", v, Math.min(...xs).toFixed(3), Math.max(...xs).toFixed(3), Math.min(...ys).toFixed(3), Math.max(...ys).toFixed(3)); }
const st = evaluate(asset, { kind: "pose", pose: "transit-agitating" }, { kind: "scale", scale: { denominator: 100 } } as any);
for (const v of ["front", "plan"] as const) { const s = st.shapes[v].find((x: any) => x.shape.id === "drum")!; const xs = s.rings.flat().map((p: any) => p.x), ys = s.rings.flat().map((p: any) => p.y); console.log("[DEBUG]", v, s.rings.length, Math.min(...xs).toFixed(2), Math.max(...xs).toFixed(2), Math.min(...ys).toFixed(2), Math.max(...ys).toFixed(2)); }
