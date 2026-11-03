import { catalog, authoredScales } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { validateAsset } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
const [, , ...ids] = process.argv;
console.log("| asset | authored bands (el/fr/pl shapes) | unavailable (cause, alternatives) | poses | clips | spread bones | validator now |\n|---|---|---|---|---|---|---|");
for (const id of ids) {
  const a = catalog().find((x) => x.id === id)!;
  const bands = authoredScales(a).map((d) => { const g = (a.representations as any)[d].geometry; return `1:${d} (${g.elevation.length}/${g.front.length}/${g.plan.length})`; }).join(", ");
  const un = Object.values(a.representations as any).filter((r: any) => r.status === "unavailable").map((r: any) => `1:${r.denominator} ${r.cause} -> ${r.alternatives.map((x: any) => x.kind === "scale" ? "1:" + x.denominator : x.kind + ":" + (x.asset ?? x.symbol)).join("+")}`).join("; ") || "-";
  const spread = new Set<string>();
  for (const p of a.poses) for (const [b, o] of Object.entries<any>(p.bones ?? {})) if (o.abductionDeg || o.twistDeg) spread.add(b);
  for (const c of a.clips) for (const k of c.keys) for (const [b, o] of Object.entries<any>(k.bones ?? {})) if (o.abductionDeg || o.twistDeg) spread.add(b);
  const iss: any[] = (validateAsset as any)(a); const by = new Map<string, number>(); for (const i of iss) by.set(i.code, (by.get(i.code) ?? 0) + 1);
  console.log(`| ${id} | ${bands} | ${un} | ${a.poses.length} | ${a.clips.length} | ${[...spread].join(" ") || "-"} | ${[...by].map(([k, v]) => `${k} x${v}`).join(", ") || "0 issues"} |`);
}
