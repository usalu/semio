import { readFileSync } from "node:fs";
import { unionPolygons, ringArea } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
const d = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
for (const c of d) { const r = unionPolygons([c.low, c.high, ...c.sweeps]); console.log("rings", r.length, r.map((x: any) => x.length + ":" + ringArea(x).toExponential(2)).join(" ")); }
