import { catalog, consistency, sweep } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const [, , scale, ...ids] = process.argv;
for (const a of catalog().filter((x) => ids.includes(x.id))) { const c = consistency(a, +scale), s = sweep(a, +scale); console.log(a.id, scale, "consistency", c.length, c.map((f) => f.message).join(" | ").slice(0, 400), "sweep", s.length, s.map((f) => f.message).join(" | ").slice(0, 300)); }
