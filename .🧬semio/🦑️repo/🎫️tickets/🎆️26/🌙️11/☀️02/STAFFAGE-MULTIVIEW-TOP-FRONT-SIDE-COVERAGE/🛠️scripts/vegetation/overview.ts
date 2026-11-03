import { readdirSync, readFileSync } from "node:fs";
const root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation";
for (const d of readdirSync(root)) {
  const a = JSON.parse(readFileSync(`${root}/${d}/🔣️.json`, "utf8")).asset;
  console.log(`## ${a.id} dims ${JSON.stringify(a.dimensions)} bones ${a.rig.bones.length} poses ${a.poses.map((p:any)=>p.id)} clips ${a.clips.map((c:any)=>c.id+":"+c.durationS)}`);
  for (const [k, r] of Object.entries<any>(a.representations)) {
    const g = r.geometry; console.log(`  ${k} ${r.status} ${r.status==="authored"?`range ${r.validRange.minDenominator}-${r.validRange.maxDenominator} el ${g.elevation?.length} fr ${g.front?.length} pl ${g.plan?.length} sections ${JSON.stringify(r.sections??null)}`:JSON.stringify(r.reason??r).slice(0,200)+" alts "+JSON.stringify(r.alternatives)}`);
  }
  for (const s of a.symbols) console.log(`  symbol ${s.id} ${s.semantics} ${s.units} den ${s.denominators} el ${s.geometry.elevation?.length} fr ${s.geometry.front?.length} pl ${s.geometry.plan?.length}`);
  const ab = JSON.stringify(a).match(/abductionDeg/g)?.length ?? 0; console.log("  abduction mentions", ab, "bones with z?", a.rig.bones.filter((b:any)=>b.z!==undefined).length);
}
