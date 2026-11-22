/** 🎈️ Author 14 float check: groups the solids of a list by touching world bounding boxes (tolerance 1 mm) and prints every group beyond the largest, i.e. parts that certainly float; boxes over-connect, so a silent result is necessary, not sufficient; `--fine` also asks for a vertex of one solid inside the box of the other (crossing bars without such a vertex are then reported and judged by eye).
 *
 * @see ./rework-brief.md */
import { cli, load, rigAt, queryOf, listsIn } from "./r3-lib.ts";

const { spec, flag, has } = cli(["--scale", "--list", "--pose", "--tol"]);
const { asset } = load(spec, "rework-14-float.ts <asset> [--scale N] [--list body] [--pose ID]");
const tol = Number(flag("--tol") ?? 0.001);
const inside = (world: Float64Array, b: number[]): boolean => {
  for (let k = 0; k < world.length; k += 3) if ([0, 1, 2].every((a) => world[k + a]! >= b[a]! - tol && world[k + a]! <= b[a + 3]! + tol)) return true;
  return false;
};
for (const scale of flag("--scale") ? [Number(flag("--scale"))] : [50, 100, 200, 500]) {
  for (const list of (flag("--list") ? [flag("--list")!] : ["body", "thermal", "structure"]) as ("body" | "thermal" | "structure")[]) {
    const rig = rigAt(asset, scale, queryOf(asset, flag("--pose"), undefined, undefined));
    const lists = listsIn(asset, scale, rig.frames);
    const boxes = lists[list].map((m) => {
      const bb = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
      for (let k = 0; k < m.world.length; k += 3) for (let a = 0; a < 3; a++) {
        bb[a] = Math.min(bb[a]!, m.world[k + a]!);
        bb[a + 3] = Math.max(bb[a + 3]!, m.world[k + a]!);
      }
      return bb;
    });
    const parent = boxes.map((_, i) => i);
    const find = (i: number): number => (parent[i] === i ? i : (parent[i] = find(parent[i]!)));
    for (let i = 0; i < boxes.length; i++) for (let j = i + 1; j < boxes.length; j++) {
      const [a, b] = [boxes[i]!, boxes[j]!];
      if (![0, 1, 2].every((k) => a[k]! <= b[k + 3]! + tol && b[k]! <= a[k + 3]! + tol)) continue;
      if (has("--fine") && !inside(lists[list][i]!.world, b) && !inside(lists[list][j]!.world, a)) continue;
      parent[find(i)] = find(j);
    }
    const groups = new Map<number, string[]>();
    boxes.forEach((_, i) => groups.set(find(i), [...(groups.get(find(i)) ?? []), String(lists.solids[list][i]!.id)]));
    const sorted = [...groups.values()].sort((a, b) => b.length - a.length);
    console.log(`${spec} 1:${scale} ${list}: ${boxes.length} solids, ${sorted.length} group(s)${sorted.slice(1).map((g) => " | " + g.join(",")).join("")}${lists.errors.length ? " ERRORS " + lists.errors.join(";") : ""}`);
  }
}
