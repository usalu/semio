import { readdirSync, readFileSync, writeFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { extentOf, authoredScales } = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const root = `${L}/🗂️catalog/🌳️vegetation`;
const only = process.argv[2] ?? "all";
const out: string[] = []; let bad = 0;
for (const d of readdirSync(root)) {
  if (only !== "all" && d !== only) continue;
  const a = JSON.parse(readFileSync(`${root}/${d}/🔣️.json`, "utf8")).asset; const D = a.dimensions;
  const sels: any[] = authoredScales(a).map((n: number) => [`1:${n}`, { kind: "scale", scale: { denominator: n } }]);
  for (const s of a.symbols) sels.push([`sym:${s.id}`, { kind: "symbol", symbol: s.id }]);
  for (const [label, sel] of sels) {
    const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, sel);
    const e = extentOf(st.shapes.elevation), f = extentOf(st.shapes.front), p = extentOf(st.shapes.plan);
    const chk = (name: string, m: number | undefined, nom: number) => { if (m === undefined) { bad++; out.push(`${d} ${label} ${name} MISSING`); return; } const t = Math.max(0.02 * nom, 0.015); if (Math.abs(m - nom) > t) { bad++; out.push(`${d} ${label} ${name} ${m.toFixed(3)} vs ${nom} (tol ${t.toFixed(3)})`); } };
    const nominal = label.startsWith("sym:") && a.symbols.find((s: any) => `sym:${s.id}` === label).semantics === "nominal";
    if (nominal) { if (!e || !f || !p) { bad++; out.push(`${d} ${label} missing view el${!!e} fr${!!f} pl${!!p}`); } continue; }
    chk("el-width", e && e.maxX - e.minX, D.widthM); chk("el-height", e && e.maxY - e.minY, D.heightM);
    chk("fr-depth", f && f.maxX - f.minX, D.depthM); chk("fr-height", f && f.maxY - f.minY, D.heightM);
    chk("pl-width", p && p.maxX - p.minX, D.widthM); chk("pl-depth", p && p.maxY - p.minY, D.depthM);
  }
}
writeFileSync(process.argv[3] ?? "audit4.txt", out.join("\n") + `\nTOTAL ${bad}\n`);
