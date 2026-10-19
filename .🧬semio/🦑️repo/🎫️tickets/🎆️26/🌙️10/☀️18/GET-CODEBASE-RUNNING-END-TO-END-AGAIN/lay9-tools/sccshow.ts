import { readFileSync } from "node:fs";
const sc = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const idx = Number(process.argv[3]);
const P = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/";
const s = (p: string) => p.replace(P, "").replace(/\/🟦️\.(tsx?)$/, "/#$1");
const c = sc[idx];
console.log("members:"); for (const m of c.members.map(s).sort()) console.log("  ", m);
console.log("edges (R=runtime T=type-only D=dynamic):");
for (const e of c.edges) console.log(" ", e.dynamic ? "D" : e.typeOnly ? "T" : "R", s(e.from), "->", s(e.to));
