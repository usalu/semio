/** 📍️ Which structure/thermal solids reach outside the body silhouette: bun author-24-outside.ts <asset> <scale> [structure|thermal] */
import { entryOf, loadSource, merged, scaleMeshes, poseState, viewMasks, distanceField, projectWorld, VIEWS } from "./volumes-lib.ts";

const [spec, sc, list = "structure"] = process.argv.slice(2);
const scale = Number(sc);
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const sm = scaleMeshes(m, scale, m.volumes[String(scale)]);
const st = poseState(m, scale);
for (const v of VIEWS) {
  const vm = viewMasks(v, st, sm.body);
  const field = distanceField(vm.volume);
  const rows: string[] = [];
  for (const mesh of (sm as any)[list] as any[]) {
    let worst = 0;
    for (let i = 0; i < mesh.world.length / 3; i++) {
      const p = projectWorld(v, [mesh.world[i * 3], mesh.world[i * 3 + 1], mesh.world[i * 3 + 2]]);
      const cx = Math.floor((p.x - vm.box.minX) / vm.cell);
      const cy = Math.floor((p.y - vm.box.minY) / vm.cell);
      if (cx < 0 || cy < 0 || cx >= vm.volume.width) continue;
      const idx = cy * vm.volume.width + cx;
      if (idx >= field.length) continue;
      worst = Math.max(worst, field[idx]!);
    }
    if (worst > 0.004) rows.push(`${mesh.id}:${(worst * 1000).toFixed(0)}mm`);
  }
  console.log(v, rows.join(" "));
}
