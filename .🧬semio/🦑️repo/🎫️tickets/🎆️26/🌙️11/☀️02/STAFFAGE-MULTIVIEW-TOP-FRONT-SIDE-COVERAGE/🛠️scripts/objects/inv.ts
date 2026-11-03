import { readdirSync, readFileSync } from "node:fs";
import { ROOT } from "./patch.ts";
for (const cat of ["🪑️objects", "🛋️furniture"]) for (const slug of readdirSync(`${ROOT}/${cat}`)) {
  const text = readFileSync(`${ROOT}/${cat}/${slug}/🔣️.json`, "utf8");
  const a = JSON.parse(text).asset;
  const sup = a.anchors.filter((x: any) => x.kind === "contact" || x.kind === "support");
  if (sup.length === 0) { console.log(slug, "no contact anchors"); continue; }
  console.log(`## ${slug}  supports=${JSON.stringify(a.structure?.supports)}`);
  for (const s of sup) {
    const refs = (text.match(new RegExp(`"${s.id}"`, "g")) ?? []).length;
    console.log(`  ${s.id} bone=${s.bone} (${s.x},${s.y}) kind=${s.kind} support=${s.support} fp=${s.footprintLengthM}x${s.footprintWidthM} refs=${refs} z=${s.z}`);
  }
  console.log("  poses contacts:", [...new Set(a.poses.map((p: any) => JSON.stringify(p.contacts)))].join(" ; "));
  console.log("  clips contacts:", [...new Set(a.clips.flatMap((c: any) => (c.contacts ?? []).map((k: any) => k.anchor)))].join(","));
}
