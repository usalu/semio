import { catalog, authoredScales, missingViews, sweep, consistency } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const ids = (process.env.IDS ?? "").split(",").filter(Boolean);
const cats = new Set(["machinery","vehicles"]);
for (const a of catalog()) {
  if (!cats.has(a.category)) continue;
  if (ids.length && !ids.includes(a.id)) continue;
  const d = a.dimensions;
  console.log(`## ${a.id} dims h${d.heightM} w${d.widthM} d${d.depthM} poses=${a.poses.map(p=>p.id)} clips=${a.clips.map(c=>c.id+":"+c.durationS)} default=${a.presentation.defaultPose}`);
  for (const [k, r] of Object.entries<any>(a.representations)) {
    if (r.status === "authored") console.log(`  ${k} authored miss=[${missingViews(a, r.denominator)}] n=${r.geometry.elevation?.length}/${r.geometry.front?.length}/${r.geometry.plan?.length}`);
    else console.log(`  ${k} UNAVAILABLE alt=${JSON.stringify(r.alternatives)}`);
  }
  for (const s of a.symbols ?? []) console.log(`  symbol ${s.id} den=${s.denominators} views=${Object.entries(s.geometry).map(([k,v]:any)=>k+":"+v.length)}`);
}
