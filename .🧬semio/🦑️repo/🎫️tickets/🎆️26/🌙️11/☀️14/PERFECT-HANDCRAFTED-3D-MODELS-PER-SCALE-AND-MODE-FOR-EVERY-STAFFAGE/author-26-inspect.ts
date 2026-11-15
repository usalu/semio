import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, frames, loadAsset, resolveAsset, restState, bboxOf } from "./volumes-lib.ts";

const spec = process.argv[2]!;
const entry = resolveAsset(spec);
const asset = loadAsset(entry.path);
const f = frames(asset);
const r = (x: number) => (Math.round(x * 1000) / 1000).toString();
const out: string[] = [];
out.push(`# ${asset.id} dims ${JSON.stringify(asset.dimensions)} default ${JSON.stringify(asset.presentation)}`);
out.push("## bones (parent, origin, u, v, w, length, joint)");
for (const b of asset.rig.bones) {
  const fr = f[b.id]!;
  out.push(`${b.id} <- ${b.parent} o=[${fr.o.map(r)}] u=[${fr.u.map(r)}] v=[${fr.v.map(r)}] w=[${fr.w.map(r)}] len=${b.lengthM} joint=${JSON.stringify(b.joint)} section=${JSON.stringify(b.section ?? null)} mass=${b.mass?.kg}@${b.mass?.comX},${b.mass?.comY}`);
}
out.push("## materials: " + (asset.materials ?? []).map((m: any) => `${m.id}(k${m.conductivityWmK})`).join(", "));
out.push("## regions: " + (asset.thermal?.regions ?? []).map((m: any) => `${m.id}[A=${m.areaM2},${m.kind},${m.hidden ? "hidden" : ""},layers=${(m.layers ?? []).map((l: any) => l.id + ":" + l.thicknessM).join("|")}]`).join(" "));
out.push("## emitters: " + JSON.stringify(asset.thermal?.emitters ?? []).slice(0, 1500));
out.push("## supports: " + JSON.stringify(asset.structure?.supports) + " anchors " + JSON.stringify(asset.anchors));
for (const sc of [50, 100, 200, 500]) {
  let st: any;
  try { st = restState(asset, sc); } catch (e) { out.push(`## scale ${sc} unavailable`); continue; }
  for (const view of ["elevation", "front", "plan"] as const) {
    out.push(`## s${sc} ${view}: id | bone | role | kind | material | region | bbox`);
    for (const s of st[view]) {
      const pts = s.rings.flat();
      const bb = pts.length ? bboxOf(pts) : null;
      const sh = s.shape;
      out.push(`${sh.id} | ${sh.bone} | ${sh.role} | ${sh.kind}${sh.solid ? "/" + sh.solid.kind : ""} | ${sh.material ?? ""} | ${sh.region ?? ""} | ${bb ? `x ${r(bb.minX)}..${r(bb.maxX)} y ${r(bb.minY)}..${r(bb.maxY)}` : "-"}${s.closed ? "" : " open"}${sh.tone !== undefined ? " t" + sh.tone : ""}`);
    }
  }
}
const dir = join(GENERATED, "author-26");
mkdirSync(dir, { recursive: true });
writeFileSync(join(dir, `${asset.id}.inspect.txt`), out.join("\n") + "\n");
writeFileSync(join(dir, `${asset.id}.frames.json`), JSON.stringify(f));
console.log(out.length, "lines");
