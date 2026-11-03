import { catalog, authoredScales, sweep, consistency } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const [, , ...ids] = process.argv;
for (const a of catalog().filter((x) => ids.includes(x.id))) for (const d of authoredScales(a)) {
  const s = sweep(a, d), c = consistency(a, d);
  if (s.length || c.length) { console.log(`${a.id} 1:${d} sweep ${s.length} consistency ${c.length}`); for (const f of [...s, ...c].slice(0, 4)) console.log("   ", f.check, f.message.slice(0, 210)); }
}
console.log("done");
