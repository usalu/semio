/** 🧪 Round 3 tied canopy: runs the product cross-scale measurement of 1:200 against 1:100 on the candidate document with solids of the coarser body left out, to find what drives the plan distance. Usage: bun r3-tied-canopy-probe2.ts */
import { readFileSync } from "node:fs";

const mod = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts");
const doc = JSON.parse(readFileSync("🗑️generated/r3-tied-canopy/🔣️.json", "utf8"));
const drop = (scale: string, ids: string[]) => (a: any) => { a.volumes[scale].body = a.volumes[scale].body.filter((s: any) => !ids.includes(s.id)); };
const variants: Record<string, (a: any) => void> = {
  asis: () => {},
  "no-walls": (a) => { drop("100", ["wall-core", "wall-insulation", "plate-beam", "plate-strut"])(a); drop("200", ["wall-core", "wall-outer"])(a); },
  "no-glass": (a) => { drop("100", ["glass-0", "glass-1", "glass-2", "glass-3", "joint-0", "joint-1", "joint-2", "notch-seal"])(a); drop("200", ["glass-roof"])(a); },
  "only-glass": (a) => { a.volumes["100"].body = a.volumes["100"].body.filter((s: any) => /glass|joint|notch/.test(s.id)); a.volumes["200"].body = a.volumes["200"].body.filter((s: any) => /glass/.test(s.id)); },
  "only-wall": (a) => { a.volumes["100"].body = a.volumes["100"].body.filter((s: any) => /wall|plate/.test(s.id)); a.volumes["200"].body = a.volumes["200"].body.filter((s: any) => /wall/.test(s.id)); },
};
for (const [name, edit] of Object.entries(variants)) {
  const a = structuredClone(doc.asset);
  edit(a);
  const out = mod.measureVolumes(a, { scale: 200 as never, rules: ["volumes-cross-scale"] as never });
  console.log(name, out.filter((m: any) => /plan/.test(m.name) && !/2D/.test(m.name)).map((m: any) => `${m.name}=${Number(m.value).toFixed(4)}`).join(" "));
}
