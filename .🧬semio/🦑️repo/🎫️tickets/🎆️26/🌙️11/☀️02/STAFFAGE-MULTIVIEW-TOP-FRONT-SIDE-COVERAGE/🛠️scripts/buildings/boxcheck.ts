import { readFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const T = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/👤️humans/worker-carrying-box/🔣️.json`, "utf8")).asset;
let worst = 0; let worstName = "";
const rows: string[] = [];
for (const { name, query } of T.queries(a)) {
  const st = evaluate(a, query, { kind: "scale", scale: { denominator: 50 } });
  const box = st.shapes.elevation.find((s: any) => s.shape.id === "box");
  const p = box.rings.flat(); const bx = [Math.min(...p.map((q: any) => q.x)), Math.max(...p.map((q: any) => q.x))], by = [Math.min(...p.map((q: any) => q.y)), Math.max(...p.map((q: any) => q.y))];
  const bf = st.shapes.front.find((s: any) => s.shape.id === "box").rings.flat(); const bz = [Math.min(...bf.map((q: any) => q.x)), Math.max(...bf.map((q: any) => q.x))];
  for (const id of ["grip-l", "grip-r"]) {
    const g = st.anchors[id];
    const dx = Math.max(bx[0] - g.at.x, 0, g.at.x - bx[1]), dy = Math.max(by[0] - g.at.y, 0, g.at.y - by[1]);
    const side = id === "grip-l" ? bz[1] : bz[0];
    const dz = Math.abs(g.z - side);
    const d = Math.hypot(dx, dy, dz);
    if (d > worst) { worst = d; worstName = `${name} ${id} dx${dx.toFixed(3)} dy${dy.toFixed(3)} dz${dz.toFixed(3)} z${g.z.toFixed(3)} side${side.toFixed(3)}`; }
    if (d > 0.06) rows.push(`${name} ${id} d=${d.toFixed(3)} (dx${dx.toFixed(3)} dy${dy.toFixed(3)} dz${dz.toFixed(3)})`);
  }
}
console.log("worst", worst.toFixed(3), worstName, "bad instants", rows.length);
for (const r of rows.slice(0, 12)) console.log("  ", r);
