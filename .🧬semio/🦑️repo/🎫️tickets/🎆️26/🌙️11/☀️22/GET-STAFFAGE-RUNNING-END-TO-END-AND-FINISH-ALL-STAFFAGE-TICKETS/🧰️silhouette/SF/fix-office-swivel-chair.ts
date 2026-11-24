/** 💺️ Repair of `office-swivel-chair` (side elevation 41.2 mm and front elevation 16.9 mm at 1:50).
 * - 1:50 seat: the drawn seat mechanism ended 0.14 mm below and the drawn seat shell began 0.15 mm above the top of the seat plate (u 0.045 m of the bone `seat`), a hairline of paper between two parts that touch in the volume; both now meet at 0.045 m in every view. Behind it the underside of the shell rose by 1 mm over 0.17 m (loft section at v 0.17 m at u 0.058 m), a wedge of at most half a raster cell between plate and shell that the drawing and the volume resolved at different places (35.6 mm); the section stands at u 0.057 m like the one before it, the underside is flat on the plate, and the drawn underside ends at u 0.045 m.
 * - Castor hubs: the hub of a twin castor was drawn as one disc extruded across both wheels and the gap between them, where the volume has two rims and, between the wheels, only the web of the fork above 12.5 mm; every hub shape extruded over a wheel pair becomes the two rims `…-hub-r` and `…-hub-l`, each over its own wheel and the 1.3 mm it stands proud of it, at every scale and in every view that drew the single disc.
 *  bun fix-office-swivel-chair.ts [--file <asset.json>] [--dry] */
import { run } from "./lib.ts";

await run("office-swivel-chair", (asset) => {
  for (const view of ["elevation", "front", "plan"]) for (const s of asset.representations["50"].geometry[view]) {
    if (s.id === "seat-mechanism") s.points = s.points.map(([u, v]: number[]) => [u === 0.04486 ? 0.045 : u, v]);
    if (s.id === "seat-shell") s.points = s.points.map(([u, v]: number[]) => [u === 0.04515 || (u === 0.04613 && v === 0.1703) ? 0.045 : u, v]);
  }
  for (const list of ["body", "thermal"]) for (const solid of asset.volumes["50"][list]) if (solid.id === "seat-shell" && solid.sections?.[4]?.at[1] === 0.17) solid.sections[4].at[0] = 0.057;
  for (const scale of ["50", "100", "200", "500"]) for (const view of ["elevation", "front", "plan"]) {
    const list: any[] = asset.representations[scale]?.geometry?.[view] ?? [];
    for (const hub of list.filter((s) => /^castor-.*-hub$/u.test(s.id) && s.solid?.kind === "extruded" && s.solid.sweep === "w")) {
      const [left, right] = ["l", "r"].map((side) => list.find((s) => s.id === hub.id.replace(/hub$/u, `wheel-${side}`)));
      if (left?.solid === undefined || right?.solid === undefined) continue;
      const at = list.indexOf(hub);
      const rim = (side: string, z0: number, z1: number): unknown => ({ ...hub, id: `${hub.id}-${side}`, solid: { ...hub.solid, z0, z1 } });
      list.splice(at, 1, rim("r", hub.solid.z0, right.solid.z1), rim("l", left.solid.z0, hub.solid.z1));
    }
  }
});
