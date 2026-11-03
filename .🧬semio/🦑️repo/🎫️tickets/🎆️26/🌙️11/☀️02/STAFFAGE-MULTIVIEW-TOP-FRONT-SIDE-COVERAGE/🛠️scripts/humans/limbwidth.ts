import { readdirSync, readFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { queries } = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const dir = `${L}/🗂️catalog/👤️humans`;
const lim = /^(arm|forearm|upper-arm|hand|thigh|shank|foot|toe)-[lr]$/;
const lat = Number(process.argv[2] ?? 0.16);
for (const slug of readdirSync(dir).sort()) {
  const a = JSON.parse(readFileSync(`${dir}/${slug}/🔣️.json`, "utf8")).asset;
  const res: string[] = [];
  for (const [k, r] of Object.entries<any>(a.representations)) {
    if (r.status !== "authored") continue;
    const worst: Record<string, [number, string]> = {};
    for (const { name, query } of queries(a)) {
      const st = evaluate(a, query, { kind: "scale", scale: { denominator: r.denominator } });
      for (const v of ["front", "plan"]) for (const s of st.shapes[v]) {
        if (!lim.test(s.shape.bone) || s.shape.role === "ground") continue;
        const p = s.rings.flat(); if (!p.length) continue;
        const c = v === "front" ? "x" : "y"; const ys = p.map((q: any) => q[c]);
        const w = Math.max(...ys) - Math.min(...ys);
        const key = `${v}/${s.shape.bone}`;
        if (w > lat && (!worst[key] || w > worst[key][0])) worst[key] = [w, name];
      }
    }
    const e = Object.entries(worst).map(([k2, [w, n]]) => `${k2}=${w.toFixed(2)}@${n}`);
    if (e.length) res.push(`${k}: ${e.join(", ")}`);
  }
  console.log(slug, "\n   " + res.join("\n   "));
}
