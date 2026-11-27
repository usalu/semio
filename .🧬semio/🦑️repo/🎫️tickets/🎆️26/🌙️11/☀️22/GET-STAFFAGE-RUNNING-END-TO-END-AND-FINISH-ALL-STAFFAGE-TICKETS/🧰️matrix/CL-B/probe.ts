/** 🔬️ Names the shapes whose geometry differs between the English and the German drawing of one asset: probe.ts <asset id or file> <requested denominator> [mode] [view]. For every pose and every mode (or the one given) it resolves the three views in both languages as the matrix does (`renderSubject` of `T/🧰️.ts`) and prints, per differing `shape:` path, the first differing command.
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts
 */
import { existsSync, readFileSync } from "node:fs";
import { MODE_LIST, VIEWPOINT_LIST, drawOptions, loadAsset, subjects } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts";
import { resolveViews } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔭️multiview/🟦️.ts";

const [target, requestedText, onlyMode, onlyView] = process.argv.slice(2);
const requested = Number(requestedText);
const asset = existsSync(target!) ? JSON.parse(readFileSync(target!, "utf8")).asset : loadAsset(target!)!;
const pathsOf = (scene: any): Map<string, string[]> => {
  const map = new Map<string, string[]>();
  for (const layer of scene.layers) for (const item of layer.items) if (item.kind === "path" && item.id.startsWith("shape:")) map.set(item.id, [...(map.get(item.id) ?? []), JSON.stringify(item.commands)]);
  return map;
};
let differing = 0;
for (const subject of subjects(asset, requested).filter((s) => s.languages.length > 1))
  for (const mode of MODE_LIST.filter((m) => onlyMode === undefined || m === onlyMode)) {
    const en = resolveViews(asset, subject.query, { kind: "scale", denominator: requested }, mode, drawOptions(mode, requested, false, "en"));
    const de = resolveViews(asset, subject.query, { kind: "scale", denominator: requested }, mode, drawOptions(mode, requested, false, "de"));
    for (const view of VIEWPOINT_LIST.filter((v) => onlyView === undefined || v === onlyView)) {
      const a = en.views[view], b = de.views[view];
      if (a.kind === "unavailable" || b.kind === "unavailable") continue;
      const pa = pathsOf(a.scene), pb = pathsOf(b.scene);
      const orderA = JSON.stringify([...pa.keys()]), orderB = JSON.stringify([...pb.keys()]);
      const ids = new Set([...pa.keys(), ...pb.keys()]);
      const bad = [...ids].filter((id) => JSON.stringify(pa.get(id)) !== JSON.stringify(pb.get(id)));
      if (bad.length === 0 && orderA === orderB) continue;
      differing++;
      console.log(`  origin en ${JSON.stringify(a.scene.metadata.origin)} de ${JSON.stringify(b.scene.metadata.origin)}`);
      for (const [name, scene] of [["en", a.scene], ["de", b.scene]] as const) {
        const texts = scene.layers.flatMap((layer: any) => layer.items.filter((item: any) => item.kind === "text").map((item: any) => ({ id: item.id, anchor: item.anchor, x: item.x, y: item.y, text: item.text })));
        const left = [...texts].sort((p: any, q: any) => (p.anchor === "start" ? p.x : -1e9 + p.x) - (q.anchor === "start" ? q.x : -1e9 + q.x)).slice(0, 4);
        console.log(`  ${name} texts not anchored at their start, then the leftmost: ${JSON.stringify(left)}`);
        console.log(`  ${name} legends: ${JSON.stringify(scene.legends.map((l: any) => [l.quantity, l.min, l.max, l.unit]))}`);
      }
      console.log(`${subject.id} | ${mode} | ${view}: ${bad.length} of ${ids.size} shape paths differ${orderA === orderB ? "" : ", ORDER differs"}; scene en ${a.scene.widthMm}x${a.scene.heightMm} de ${b.scene.widthMm}x${b.scene.heightMm}`);
      for (const id of bad.slice(0, 12)) {
        const x = pa.get(id) ?? [], y = pb.get(id) ?? [];
        console.log(`  ${id}: en ${x.length} path(s), de ${y.length} path(s)`);
        for (let i = 0; i < Math.max(x.length, y.length); i++) {
          if (x[i] === y[i]) continue;
          const ca = x[i] === undefined ? [] : JSON.parse(x[i]!), cb = y[i] === undefined ? [] : JSON.parse(y[i]!);
          const at = ca.findIndex((c: unknown, k: number) => JSON.stringify(c) !== JSON.stringify(cb[k]));
          console.log(`    path ${i}: en ${ca.length} commands, de ${cb.length}; first difference at ${at}: en ${JSON.stringify(ca[at])} de ${JSON.stringify(cb[at])}`);
        }
      }
    }
  }
console.log(`differing cells: ${differing}`);
