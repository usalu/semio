/** Replicates the lateral-ambiguous rule per shape. Usage: bun lateral-list.ts <slug> [--all] */
import { samplePose, sampleClip } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts";
import { flattenGeometry } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { restFrames, solveChain } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { catalogEntries, loadAsset } from "./volume-lib.ts";
export function listAmbiguous(asset: any) {
  const spread = new Set<string>();
  const own = new Set<string>();
  const scan = (o: any) => { for (const [id, off] of Object.entries<any>(o ?? {})) if ((off.abductionDeg ?? 0) !== 0 || (off.twistDeg ?? 0) !== 0) own.add(id); };
  for (const p of asset.poses) scan(p.bones);
  for (const c of asset.clips) for (const k of c.keys) scan(k.bones);
  const byId = new Map<string, any>(asset.rig.bones.map((b: any) => [b.id, b]));
  for (const b of asset.rig.bones) for (let c = b; c; c = c.parent === null ? undefined : byId.get(c.parent)) if (own.has(c.id)) { spread.add(b.id); break; }
  const rest = restFrames(asset.rig) as any;
  const samples = [...asset.poses.map((p: any) => samplePose(asset, p)), ...asset.clips.flatMap((c: any) => Array.from({ length: 9 }, (_, i) => sampleClip(asset, c, (c.durationS * i) / 8)))];
  const tilts: Record<string, Map<string, number>> = {};
  for (const v of ["elevation", "front", "plan"]) {
    const normal = v === "elevation" ? [0, 0, 1] : v === "front" ? [1, 0, 0] : [0, 1, 0];
    const t = new Map<string, number>();
    for (const s of samples) {
      const now = solveChain(asset.rig, s.offsets, s.root) as any;
      for (const id of Object.keys(now)) {
        const r = rest[id].frame, n = now[id].frame;
        const dw = (a: number[]) => a[0] * normal[0] + a[1] * normal[1] + a[2] * normal[2];
        const carried = [0, 1, 2].map((ax) => n.u[ax] * dw(r.u) + n.v[ax] * dw(r.v) + n.w[ax] * dw(r.w));
        const cos = Math.min(1, Math.max(-1, carried[0] * normal[0] + carried[1] * normal[1] + carried[2] * normal[2]));
        t.set(id, Math.max(t.get(id) ?? 0, Math.acos(cos)));
      }
    }
    tilts[v] = t;
  }
  const rows: any[] = [];
  for (const [scale, rep] of Object.entries<any>(asset.representations)) {
    if (!rep.geometry) continue;
    for (const v of ["elevation", "front", "plan"]) rep.geometry[v].forEach((shape: any, index: number) => {
      if (shape.solid !== undefined || shape.role === "ground" || !spread.has(shape.bone) || shape.kind === "member") return;
      const theta = tilts[v]!.get(shape.bone) ?? 0;
      const rings = flattenGeometry(shape).rings.flat();
      const extent = rings.length === 0 ? 0 : Math.max(Math.max(...rings.map((p: any) => p.x)) - Math.min(...rings.map((p: any) => p.x)), Math.max(...rings.map((p: any) => p.y)) - Math.min(...rings.map((p: any) => p.y)));
      const errorMm = (extent * Math.sin(theta) * 1000) / Number(scale);
      rows.push({ scale, view: v, index, id: shape.id, kind: shape.kind, bone: shape.bone, role: shape.role, thetaDeg: (theta * 180) / Math.PI, errorMm, flagged: errorMm > 0.2 });
    });
  }
  return rows;
}
if (import.meta.main) {
  const slug = process.argv[2]!;
  const rows = listAmbiguous(loadAsset(catalogEntries().find((x) => x.slug === slug)!.path));
  const all = process.argv.includes("--all");
  for (const r of rows) if (all || r.flagged) console.log(`${r.scale}\t${r.view}\t${r.index}\t${r.bone}\t${r.id}\t${r.kind}\t${r.role}\t${r.thetaDeg.toFixed(1)}\t${r.errorMm.toFixed(2)}${r.flagged ? "" : "\t(below)"}`);
  console.log("flagged", rows.filter((r) => r.flagged).length, "non-solid on spread bones", rows.length);
}
