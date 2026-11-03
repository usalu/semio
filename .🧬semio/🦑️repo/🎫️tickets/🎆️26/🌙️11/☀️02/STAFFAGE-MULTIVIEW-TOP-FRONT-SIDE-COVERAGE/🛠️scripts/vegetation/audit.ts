import { readdirSync, readFileSync, writeFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { extentOf, toleranceM, queries, authoredScales } = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const root = `${L}/🗂️catalog/🌳️vegetation`;
const only = process.argv[2];
const f = (n: number) => n.toFixed(2);
const out: string[] = [];
for (const d of readdirSync(root)) {
  if (only && d !== only) continue;
  const a = JSON.parse(readFileSync(`${root}/${d}/🔣️.json`, "utf8")).asset;
  const D = a.dimensions, tol = toleranceM(a);
  out.push(`## ${a.id} H${D.heightM} W${D.widthM} D${D.depthM} tol ${f(tol)} defaultPose ${a.presentation.defaultPose}`);
  const sels: any[] = authoredScales(a).map((den: number) => ({ label: `1:${den}`, sel: { kind: "scale", scale: { denominator: den } } }));
  for (const s of a.symbols) sels.push({ label: `sym ${s.id}`, sel: { kind: "symbol", symbol: s.id } });
  for (const { label, sel } of sels) {
    for (const { name, query } of queries(a)) {
      let st: any;
      try { st = evaluate(a, query, sel); } catch (e: any) { out.push(`  ${label} ${name}: ERR ${e.message.slice(0, 120)}`); break; }
      const e = extentOf(st.shapes.elevation), fr = extentOf(st.shapes.front), p = extentOf(st.shapes.plan);
      const isDefault = name === `pose ${a.presentation.defaultPose}`;
      const row = (x: any) => x ? `[${f(x.minX)},${f(x.maxX)}]x[${f(x.minY)},${f(x.maxY)}]` : "NONE";
      if (isDefault || name.endsWith("@0.00 s") ) out.push(`  ${label} ${name}: el ${row(e)} fr ${row(fr)} pl ${row(p)}`);
      else out.push(`  ${label} ${name}: el ${row(e)} fr ${row(fr)} pl ${row(p)}`.replace(/^/, "  ~"));
    }
  }
}
writeFileSync(`${process.argv[3] ?? "audit.txt"}`, out.join("\n"));
