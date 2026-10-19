import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const OS = "🧰️framework/🛍️products/💻️os/";
const P = OS + "🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/";
const members = new Set<string>(JSON.parse(readFileSync(process.argv[3]!, "utf8")));
const s = (p: string) => p.replace(P, "").replace(/\/🟦️\.(tsx?)$/, "/#");
for (const m of r.modules) {
  if (!members.has(m.source)) continue;
  for (const d of m.dependencies) if (members.has(d.resolved) && !d.dependencyTypes.some((t: string) => t === "type-only" || t === "type-import")) console.log(s(m.source), "->", s(d.resolved), d.dynamic ? "DYN" : "");
}
