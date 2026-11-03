import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const ids = process.argv.slice(2);
for (const a of catalog().filter((x: any) => ids.includes(x.id)) as any[]) {
  const by = new Map<string, number>(); let n = 0, solids = 0, flat = 0;
  for (const [sc, r] of Object.entries<any>(a.representations)) {
    if (r.status !== "authored" && !r.geometry) continue;
    for (const v of ["front", "plan"]) for (const s of r.geometry?.[v] ?? []) { if (s.solid) solids++; else flat++; if (s.followTip) { n++; const k = `${v}:${s.bone}`; by.set(k, (by.get(k) ?? 0) + 1); } }
  }
  console.log(a.id, "followTip", n, "solids", solids, "flat", flat);
  if (process.env.V) console.log("   ", [...by].map(([k, v]) => `${k}x${v}`).join(" "));
}
