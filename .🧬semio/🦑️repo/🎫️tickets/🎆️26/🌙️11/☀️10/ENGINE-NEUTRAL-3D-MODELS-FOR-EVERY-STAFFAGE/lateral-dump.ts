import { catalogEntries, loadAsset } from "./volume-lib.ts";
const [slug, bone, scale = "50"] = process.argv.slice(2);
const a = loadAsset(catalogEntries().find((x) => x.slug === slug)!.path);
for (const v of ["elevation", "front", "plan"]) {
  const list = a.representations[scale].geometry[v];
  list.forEach((s: any, i: number) => { if (!bone || s.bone === bone) console.log(v, i, s.id, s.kind, s.role ?? "", JSON.stringify(s.solid ?? null), s.kind === "capsule" ? JSON.stringify([s.x1,s.y1,s.x2,s.y2,s.r].map((x:any)=>x)) : ""); });
}
