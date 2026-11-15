import { entryOf, loadSource, merged } from "./volumes-lib.ts";
const [spec, sc] = process.argv.slice(2);
const mod: any = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts");
const { entry, asset } = entryOf(spec!);
const m: any = merged(asset, loadSource(entry)!);
const run = (a: any) => { const r = mod.measureVolumes(a, { scale: Number(sc), rules: ["volumes-silhouette"] }); return Object.fromEntries(r.filter((x: any) => x.name.startsWith("iou:") || x.name.startsWith("p95M")).map((x: any) => [x.name.replace("elevation", "e").replace("p95M:e", "p95e"), +x.value.toFixed(3)])); };
const variants: Record<string, (c: any) => void> = {
  base: () => {},
  "no unfilled 2D elev": (c) => { const g = c.representations[sc!].geometry.elevation; c.representations[sc!].geometry.elevation = g.filter((s: any) => s.filled !== false); },
  "no tyres 2D": (c) => { const g = c.representations[sc!].geometry.elevation; c.representations[sc!].geometry.elevation = g.filter((s: any) => !s.id.startsWith("tyre")); },
  "no tyres 3D": (c) => { for (const l of ["body"]) c.volumes[sc!][l] = c.volumes[sc!][l].filter((s: any) => !s.id.includes("tyre")); },
  "no tyres both": (c) => { const g = c.representations[sc!].geometry.elevation; c.representations[sc!].geometry.elevation = g.filter((s: any) => !s.id.startsWith("tyre")); c.volumes[sc!].body = c.volumes[sc!].body.filter((s: any) => !s.id.includes("tyre")); },
  "no frame tubes 3D": (c) => { c.volumes[sc!].body = c.volumes[sc!].body.filter((s: any) => s.bone !== "frame"); },
  "no frame both": (c) => { c.volumes[sc!].body = c.volumes[sc!].body.filter((s: any) => s.bone !== "frame"); const g = c.representations[sc!].geometry.elevation; c.representations[sc!].geometry.elevation = g.filter((s: any) => s.bone !== "frame"); },
  "no wheels both": (c) => { const f = (s: any) => !String(s.bone).includes("wheel"); c.volumes[sc!].body = c.volumes[sc!].body.filter(f); const g = c.representations[sc!].geometry.elevation; c.representations[sc!].geometry.elevation = g.filter(f); },
};
for (const [k, f] of Object.entries(variants)) { const c = structuredClone(m); f(c); console.log(k.padEnd(22), JSON.stringify(run(c))); }
