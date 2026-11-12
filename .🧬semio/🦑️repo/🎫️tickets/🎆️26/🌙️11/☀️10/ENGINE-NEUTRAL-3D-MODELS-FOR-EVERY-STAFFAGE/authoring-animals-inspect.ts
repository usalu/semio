import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, frames, loadAsset, resolveAsset, restState, bboxOf } from "./volume-lib.ts";

const slug = process.argv[2]!;
const entry = resolveAsset(slug);
const asset = loadAsset(entry.path);
const f = frames(asset);
const out: string[] = [];
const r = (x: number) => (Math.round(x * 1000) / 1000).toString();
out.push(`# ${asset.id} dims ${JSON.stringify(asset.dimensions)}`);
out.push("## bones (parent, origin, u, v, w, length, joint)");
for (const b of asset.rig.bones) {
  const fr = f[b.id]!;
  out.push(`${b.id} <- ${b.parent} o=[${fr.o.map(r)}] u=[${fr.u.map(r)}] v=[${fr.v.map(r)}] w=[${fr.w.map(r)}] len=${b.lengthM} joint=${JSON.stringify(b.joint)}`);
}
out.push("## materials: " + (asset.materials ?? []).map((m: any) => m.id).join(", "));
out.push("## regions: " + (asset.thermal?.regions ?? []).map((m: any) => m.id + "(" + (m.bones ?? "") + ")").join(", "));
const st = restState(asset, 50);
for (const view of ["elevation", "front", "plan"] as const) {
  out.push(`## ${view} world bbox (paper coords) per shape: id bone role kind material region bbox`);
  for (const s of st[view]) {
    const pts = s.rings.flat();
    const bb = pts.length ? bboxOf(pts) : null;
    const sh = s.shape;
    out.push(`${sh.id} | ${sh.bone} | ${sh.role} | ${sh.kind}${sh.solid ? "/" + sh.solid.kind : ""} | ${sh.material ?? ""} | ${sh.region ?? ""} | ${bb ? `x ${r(bb.minX)}..${r(bb.maxX)} y ${r(bb.minY)}..${r(bb.maxY)}` : "-"}${s.closed ? "" : " open"}${sh.tone !== undefined ? " t" + sh.tone : ""}`);
  }
}
mkdirSync(join(GENERATED, "authoring-animals"), { recursive: true });
writeFileSync(join(GENERATED, "authoring-animals", `${asset.id}.inspect.txt`), out.join("\n") + "\n");
writeFileSync(join(GENERATED, "authoring-animals", `${asset.id}.frames.json`), JSON.stringify(f));
