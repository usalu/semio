import { listAmbiguous } from "./lateral-list.ts";
import { catalogEntries, loadAsset } from "./volume-lib.ts";
for (const slug of ["pregnant-person", "sleeping-person", "tour-guide"]) {
  const a = loadAsset(catalogEntries().find((x) => x.slug === slug)!.path);
  const rows = listAmbiguous(a).filter((r: any) => r.flagged);
  const vol = new Map<string, any[]>(); for (const s of a.volume.solids) (vol.get(s.bone) ?? vol.set(s.bone, []).get(s.bone)!).push(s);
  const miss = new Map<string, number>();
  let byId = 0, total = 0;
  for (const r of rows) {
    total++;
    const ids = (vol.get(r.bone) ?? []).map((s: any) => s.id);
    if (ids.includes(r.id)) byId++; else miss.set(`${r.bone}/${r.id}`, 0);
  }
  console.log(slug, "flagged", total, "volume solid with same id", byId);
  console.log(" volume ids per flagged bone:", [...new Set(rows.map((r: any) => r.bone))].map((b) => `${b}: ${(vol.get(b as string) ?? []).map((s: any) => s.id + "[" + s.kind + "]").join(",")}`).join("\n   "));
  console.log(" flagged ids without same-id volume:", [...miss.keys()].join(" "));
}
