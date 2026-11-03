import { catalog, authoredScales, sweep, consistency } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const ids = (process.env.IDS ?? "").split(",").filter(Boolean);
for (const a of catalog()) {
  if (!["machinery","vehicles"].includes(a.category)) continue;
  if (ids.length && !ids.includes(a.id)) continue;
  for (const s of authoredScales(a)) {
    for (const f of [...sweep(a, s), ...consistency(a, s)]) console.log(`[${f.check}] ${f.message}`);
  }
}
console.log("done");
