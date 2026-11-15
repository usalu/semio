import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, frames, loadAsset, resolveAsset, restState, bboxOf } from "./volumes-lib.ts";

const slug = process.argv[2]!;
const scale = Number(process.argv[3] ?? 50);
const entry = resolveAsset(slug);
const asset = loadAsset(entry.path);
const f = frames(asset);
const out: string[] = [];
const r = (x: number) => (Math.round(x * 1000) / 1000).toString();
out.push(`# ${asset.id} dims ${JSON.stringify(asset.dimensions)} pose ${asset.presentation?.defaultPose}`);
for (const b of asset.rig.bones) {
  const fr = f[b.id]!;
  out.push(`${b.id} <- ${b.parent} o=[${fr.o.map(r)}] u=[${fr.u.map(r)}] v=[${fr.v.map(r)}] w=[${fr.w.map(r)}] len=${b.lengthM} joint=${JSON.stringify(b.joint)} mass=${b.mass?.kg}@${b.mass?.comX},${b.mass?.comY}`);
}
out.push("## materials: " + (asset.materials ?? []).map((m: any) => m.id).join(", "));
for (const g of asset.thermal?.regions ?? []) out.push(`region ${g.id} area=${g.areaM2} layers=${(g.layers ?? []).map((l: any) => l.id + ":" + l.thicknessM + ":" + l.material).join("|")} hidden=${g.hidden ?? ""} bones=${g.bones ?? ""}`);
out.push("emitters " + JSON.stringify(asset.thermal?.emitters ?? []));
out.push("structure " + JSON.stringify(asset.structure ?? {}).slice(0, 1500));
const st = restState(asset, scale);
for (const view of ["elevation", "front", "plan"] as const) {
  out.push(`## ${view} (paper coords) per shape: id | bone | role | kind | material | region | bbox`);
  for (const s of st[view]) {
    const pts = s.rings.flat();
    const bb = pts.length ? bboxOf(pts) : null;
    const sh = s.shape;
    out.push(`${sh.id} | ${sh.bone} | ${sh.role} | ${sh.kind}${sh.solid ? "/" + sh.solid.kind : ""} | ${sh.material ?? ""} | ${sh.region ?? sh.regions ?? ""} | ${bb ? `x ${r(bb.minX)}..${r(bb.maxX)} y ${r(bb.minY)}..${r(bb.maxY)}` : "-"}${s.closed ? "" : " open"}${sh.tone !== undefined ? " t" + sh.tone : ""}`);
  }
}
mkdirSync(join(GENERATED, "author-03"), { recursive: true });
writeFileSync(join(GENERATED, "author-03", `${asset.id}.s${scale}.inspect.txt`), out.join("\n") + "\n");
writeFileSync(join(GENERATED, "author-03", `${asset.id}.frames.json`), JSON.stringify(f));
console.log(out.join("\n"));
