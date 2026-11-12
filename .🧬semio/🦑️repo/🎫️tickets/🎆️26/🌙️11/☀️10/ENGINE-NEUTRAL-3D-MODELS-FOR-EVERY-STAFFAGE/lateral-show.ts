import { catalogEntries, loadAsset } from "./volume-lib.ts";
const [slug, bone, scale = "50", full] = process.argv.slice(2);
const a = loadAsset(catalogEntries().find((x) => x.slug === slug)!.path);
for (const v of ["elevation", "front", "plan"]) a.representations[scale].geometry[v].forEach((s: any, i: number) => { if (s.bone === bone) console.log(v, i, JSON.stringify(s).slice(0, Number(full ?? 400))); });
console.log("BONE", JSON.stringify(a.rig.bones.find((b: any) => b.id === bone)));
console.log("VOLUME", a.volume.solids.filter((s: any) => s.bone === bone).map((s: any) => JSON.stringify(s).slice(0, 200)).join("\n"));
