/** 🛏️ Repair of `single-bed` (the guard did not end within 900 s): the 56 rubber caps of the slats of 1:50 were boxes with all twelve edges rounded at 5 mm, 2028 triangles each, 113 568 of the 132 004 triangles of the body, all of them projecting onto each other in the front elevation (the caps of a row stand behind one another). The sharp reference of the guard puts every triangle of the body into one path, and librsvg needs more than 300 s for that view (measured: `oracle-timing.ts`, `oracle-solids.ts`; the owned raster needs 2.6 s). A cap becomes what the posts and rails of the same bed already are: the prism of its rounded section (50 x 30 mm, 5 mm corners in three steps) over its 25 mm depth, 60 triangles; the edges of the two end faces, below the smallest feature of 1:50 (7.5 mm), are no longer rounded. Body and thermal list.
 *  bun fix-single-bed.ts [--file <asset.json>] [--dry] */
import { restate, run } from "./lib.ts";

const round = (n: number): number => Number(n.toFixed(5));

await run("single-bed", (asset) => {
  for (const list of ["body", "thermal"]) for (const solid of asset.volumes["50"][list]) {
    if (!/^(th-)?cap-/u.test(solid.id) || solid.kind !== "box" || solid.cornerRadius === undefined) continue;
    const [sx, sy, sz] = solid.size as number[], [cx, cy, cz] = solid.transform.translation as number[], r = solid.cornerRadius as number;
    const profile: number[][] = [];
    for (const [qx, qy, start] of [[1, -1, -90], [1, 1, 0], [-1, 1, 90], [-1, -1, 180]] as const) for (let k = 0; k <= 3; k++) {
      const a = ((start + k * 30) * Math.PI) / 180;
      profile.push([round(cx! + qx * (sx! / 2 - r) + r * Math.cos(a)), round(cy! + qy * (sy! / 2 - r) + r * Math.sin(a))]);
    }
    const { id, bone, material, role, tone, repeat, size: _size, cornerRadius: _radius, transform: _transform, kind: _kind, ...rest } = solid;
    restate(solid, { id, kind: "extrude", bone, material, role, tone, profile, plane: "xy", from: round(cz! - sz! / 2), to: round(cz! + sz! / 2), repeat, ...rest });
  }
});
