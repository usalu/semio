/** 🎨️ How many paint orders of the poses of assets change between the worktree and the mirror of `ROOT` (the depth cure): per asset, authored scale, pose and view the order `prepare` paints the shapes in, and whether the evaluated rings stay the same.
 * usage: ROOT=<mirror> bun order-compare.ts used-trumpet used-classical-guitar ...
 */
import { join } from "node:path";
import { REAL, ROOT, readAsset } from "./lib.ts";

const load = async (root: string) => ({ ...(await import(join(root, "🔨️modules/🧮️evaluation/🟦️.ts"))), ...(await import(join(root, "🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts"))) });
const [base, cure] = [await load(REAL), await load(ROOT)];
let [total, changed, ringsChanged] = [0, 0, 0];
for (const id of process.argv.slice(2)) {
  const asset = readAsset(id);
  let [own, ownChanged] = [0, 0];
  const where: string[] = [];
  for (const key of Object.keys(asset.representations)) {
    if (asset.representations[key].status !== "authored") continue;
    for (const pose of asset.poses) {
      const query = { kind: "pose", pose: pose.id };
      const selector = { kind: "scale", scale: { denominator: Number(key) } };
      const [a, b] = [base.evaluate(asset, query, selector), cure.evaluate(asset, query, selector)];
      if (JSON.stringify(a.shapes) !== JSON.stringify(b.shapes)) ringsChanged++;
      for (const view of ["elevation", "front", "plan"]) {
        const k = 1000 / Number(key);
        const [p, q] = [base.prepare(a, view, base.sourceOf(a), k).map((d: any) => d.shape.id), cure.prepare(b, view, cure.sourceOf(b), k).map((d: any) => d.shape.id)];
        own++;
        if (p.join("|") !== q.join("|")) {
          ownChanged++;
          if (where.length < 4) where.push(`1:${key} ${pose.id} ${view}`);
        }
      }
    }
  }
  total += own;
  changed += ownChanged;
  console.log(`${id}: ${ownChanged} of ${own} paint orders (scale x pose x view) change${where.length ? ` (${where.join("; ")})` : ""}`);
}
console.log(`total ${changed} of ${total} paint orders change; evaluated rings differ in ${ringsChanged} states`);
