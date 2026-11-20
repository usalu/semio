/** 🧪 Round 3 tied canopy: runs the product silhouette measurement of one scale on the candidate document with small variations of the front drawing, to find which shape the slender strut follows. Usage: bun r3-tied-canopy-probe.ts <scale> */
import { readFileSync } from "node:fs";

const mod = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts");
const scale = process.argv[2] ?? "500";
const doc = JSON.parse(readFileSync("🗑️generated/r3-tied-canopy/🔣️.json", "utf8"));
const variants: Record<string, (a: any) => void> = {
  asis: () => {},
  "no-front-strut": (a) => { a.representations[scale].geometry.front = a.representations[scale].geometry.front.filter((s: any) => s.id !== "strut"); },
  "strut-first": (a) => { const f = a.representations[scale].geometry.front; const i = f.findIndex((s: any) => s.id === "strut"); f.unshift(...f.splice(i, 1)); },
  ...Object.fromEntries([2.33, 2.40, 2.43, 2.44, 2.6, 3.5].map((y) => [`strut-from-${y}`, (a: any) => { const s = a.representations[scale].geometry.front.find((s: any) => s.id === "strut"); if (s) s.points[0][1] = y; }])),
  "no-wall": (a) => { a.representations[scale].geometry.front = a.representations[scale].geometry.front.filter((s: any) => s.id !== "wall"); },
};
for (const [name, edit] of Object.entries(variants)) {
  const a = structuredClone(doc.asset);
  edit(a);
  const out = mod.measureVolumes(a, { scale: Number(scale) as never, rules: ["volumes-silhouette"] as never });
  console.log(name, out.filter((m: any) => /front/.test(m.name)).map((m: any) => `${m.name}=${Number(m.value).toFixed(4)}`).join(" "));
}
