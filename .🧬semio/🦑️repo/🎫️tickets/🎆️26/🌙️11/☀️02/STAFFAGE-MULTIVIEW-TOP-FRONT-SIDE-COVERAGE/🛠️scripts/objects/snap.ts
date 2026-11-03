const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { queries } = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const { readFileSync, writeFileSync } = await import("node:fs");
const [slug, cat, ids, out] = process.argv.slice(2);
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/${cat}/${slug}/🔣️.json`, "utf8")).asset;
const want = new Set(ids.split(","));
const res: Record<string, string> = {};
for (const k of Object.keys(a.representations)) {
  if (a.representations[k].status !== "authored") continue;
  for (const { name, query } of queries(a)) {
    const st = evaluate(a, query, { kind: "scale", scale: { denominator: +k } });
    for (const v of ["elevation", "front", "plan"]) for (const s of st.shapes[v]) if (want.has(s.shape.id)) { const p = s.rings.flat(); res[`${k}/${name}/${v}/${s.shape.id}`] = [Math.min(...p.map((q: any) => q.x)), Math.max(...p.map((q: any) => q.x)), Math.min(...p.map((q: any) => q.y)), Math.max(...p.map((q: any) => q.y))].map((n: number) => n.toFixed(3)).join(" "); }
  }
}
writeFileSync(out, JSON.stringify(res, null, 1));
console.log(Object.keys(res).length);
