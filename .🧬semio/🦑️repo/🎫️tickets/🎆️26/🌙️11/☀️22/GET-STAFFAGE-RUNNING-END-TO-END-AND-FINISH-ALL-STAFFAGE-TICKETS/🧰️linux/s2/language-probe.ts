/** 🌍️ Where the German drawing of an asset has other shape geometry than the English one: per pose, view and mode the `shape:` path items that differ, and by how much.
 * With `--label A-A` every declared section of the asset is named by that letter in both languages, in memory only (the cure tried on a copy).
 * usage: ROOT=<mirror> bun language-probe.ts storage-crate-stack 50 [mode] [--label A-A]
 */
import { mod, readAsset } from "./lib.ts";

const engine = await mod("🧪️tests/🔭️multiview-coverage/🧰️.ts");
const { resolveViews } = await mod("🔨️modules/🔭️multiview/🟦️.ts");
const argv = process.argv.slice(2);
const label = argv.includes("--label") ? argv.splice(argv.indexOf("--label"), 2)[1] : undefined;
const [id, scale, only] = argv;
const asset = readAsset(id!);
if (label !== undefined) for (const representation of Object.values<any>(asset.representations)) for (const section of Object.values<any>(representation.sections ?? {})) section.label = { en: label, de: label };
const denominator = Number(scale);
const items = (scene: any): Map<string, any[]> => {
  const found = new Map<string, any[]>();
  const order: string[] = [];
  for (const layer of scene.layers) for (const item of layer.items) if (item.kind === "path" && item.id.startsWith("shape:")) (found.set(item.id, [...(found.get(item.id) ?? []), item.commands]), order.push(item.id));
  (found as any).order = order;
  return found;
};
let cells = 0;
let differing = 0;
for (const pose of asset.poses)
  for (const mode of only === undefined ? engine.MODE_LIST : [only]) {
    const draw = (language: string) => resolveViews(asset, { kind: "pose", pose: pose.id }, { kind: "scale", denominator }, mode, engine.drawOptions(mode, denominator, false, language));
    const [en, de] = [draw("en"), draw("de")];
    for (const view of engine.VIEWPOINT_LIST) {
      if (en.views[view].kind === "unavailable" || de.views[view].kind === "unavailable") continue;
      cells++;
      const [a, b] = [items(en.views[view].scene), items(de.views[view].scene)];
      const [orderA, orderB] = [(a as any).order as string[], (b as any).order as string[]];
      if (JSON.stringify([...a]) === JSON.stringify([...b]) && orderA.join() === orderB.join()) continue;
      differing++;
      const notes: string[] = [];
      if (orderA.join() !== orderB.join()) notes.push(`item order differs (${orderA.length} against ${orderB.length} items; first difference at ${orderA.findIndex((x, i) => x !== orderB[i])}: ${orderA[orderA.findIndex((x, i) => x !== orderB[i])]} against ${orderB[orderA.findIndex((x, i) => x !== orderB[i])]})`);
      let worst = 0;
      let changed = 0;
      for (const [key, list] of a) {
        const other = b.get(key);
        if (other === undefined) {
          notes.push(`${key} only in en`);
          continue;
        }
        if (JSON.stringify(list) === JSON.stringify(other)) continue;
        changed++;
        const [p, q] = [list.flat(), other.flat()];
        if (p.length !== q.length) notes.push(`${key}: ${p.length} against ${q.length} commands`);
        else p.forEach((c: any, i: number) => { if (c.op !== "Z") worst = Math.max(worst, Math.abs(c.x - q[i].x), Math.abs(c.y - q[i].y)); });
      }
      const sizes = `scene ${en.views[view].scene.widthMm.toFixed(3)} x ${en.views[view].scene.heightMm.toFixed(3)} mm (en) against ${de.views[view].scene.widthMm.toFixed(3)} x ${de.views[view].scene.heightMm.toFixed(3)} mm (de)`;
      if (differing <= 12) console.log(`pose ${pose.id} ${view} ${mode}: ${changed} of ${a.size} shape items differ, largest coordinate difference ${worst} mm; ${sizes}; ${notes.slice(0, 3).join("; ")}`);
    }
  }
console.log(`${id} 1:${scale}: ${differing} of ${cells} pose x view x mode drawings differ between en and de`);
