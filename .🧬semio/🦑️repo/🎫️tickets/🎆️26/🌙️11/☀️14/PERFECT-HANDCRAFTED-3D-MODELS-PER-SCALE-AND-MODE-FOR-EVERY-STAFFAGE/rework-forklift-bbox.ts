/** 📦️ World bounding box (ahead, up, left; millimetres) of every solid of one list of one scale of an asset in a pose, with its bone, so that parts can be joined by measured numbers.
 *
 * Usage: bun rework-forklift-bbox.ts <asset> --scale N [--list body|thermal|structure] [--pose ID] [--grep TEXT]
 * @see ./r3-lib.ts */
import { cli, listsIn, load, queryOf, rigAt } from "./r3-lib.ts";

const { spec, flag } = cli(["--scale", "--list", "--pose", "--grep"]);
const { asset } = load(spec, "rework-forklift-bbox.ts <asset> --scale N [--list L] [--pose ID] [--grep TEXT]");
const scale = Number(flag("--scale") ?? 50);
const list = (flag("--list") ?? "body") as "body" | "thermal" | "structure";
const rig = rigAt(asset, scale, queryOf(asset, flag("--pose"), undefined, undefined));
const lists = listsIn(asset, scale, rig.frames);
const grep = flag("--grep");
lists.solidMeshes[list].forEach((mesh, index) => {
  const solid = lists.solids[list][index]!;
  if (grep !== undefined && !grep.split(",").some((g) => `${mesh.id} ${solid.bone}`.includes(g))) return;
  const lo = [Infinity, Infinity, Infinity];
  const hi = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < mesh.world.length; i += 3) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, mesh.world[i + k]!); hi[k] = Math.max(hi[k]!, mesh.world[i + k]!); }
  const mm = (a: number[]) => a.map((x) => Math.round(x * 1000)).join(",");
  console.log(`${mesh.id} [${solid.bone}] ${solid.part ?? solid.region ?? ""} x ${mm([lo[0]!, hi[0]!])} y ${mm([lo[1]!, hi[1]!])} z ${mm([lo[2]!, hi[2]!])}`);
});
for (const [id, f] of Object.entries(rig.frames)) console.log(`# frame ${id} o ${f.o.map((x) => Math.round(x * 1000)).join(",")} u ${f.u.map((x) => x.toFixed(2)).join(",")}`);
