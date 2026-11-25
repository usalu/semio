/** 🗣️ The `language-geometry` check of the matrix, laid open for one asset: renders every pose (or the subjects named by `--pose a,b`) at every authored band in the seven modes in English and German exactly as the matrix does and prints, per scale, pose, view and mode whose shape paths differ, the count of shape items of either language and the first items that only one language has or whose commands differ.
 *  bun langgeo.ts <id | file.json> [--scale 50] [--pose a,b] [--items 6]
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts — `renderSubject` */
import { parseArgs } from "node:util";
import { S, fileOf } from "./files.ts";
const T = `${S}/🧪️tests/🔭️multiview-coverage`;
const { resolveViews } = await import(`${S}/🔨️modules/🔭️multiview/🟦️.ts`);
const { MODE_LIST, VIEWPOINT_LIST, authoredOf, drawOptions } = await import(`${T}/🧰️.ts`);
const { values, positionals } = parseArgs({ args: process.argv.slice(2), options: { scale: { type: "string" }, pose: { type: "string" }, items: { type: "string" } }, allowPositionals: true });
const asset = (await Bun.file(fileOf(positionals[0]!)).json()).asset;
const bands = values.scale ? [values.scale] : Object.keys(asset.representations).filter((key) => authoredOf(asset, key) !== undefined);
const poses: string[] = values.pose ? values.pose.split(",") : asset.poses.map((p: any) => p.id);
const limit = Number(values.items ?? 6);
const itemsOf = (scene: any): Map<string, string> => new Map(scene.layers.flatMap((layer: any) => layer.items.flatMap((item: any) => (item.kind === "path" && item.id.startsWith("shape:") ? [[item.id, JSON.stringify(item.commands)]] : []))));
let cells = 0;
for (const key of bands)
  for (const pose of poses)
    for (const mode of MODE_LIST) {
      const source = { kind: "scale", denominator: Number(key) };
      const render = (language: string) => resolveViews(asset, { kind: "pose", pose }, source, mode, drawOptions(mode, Number(key), false, language));
      const en = render("en"), de = render("de");
      for (const view of VIEWPOINT_LIST) {
        if (en.views[view].kind === "unavailable" || de.views[view].kind === "unavailable") continue;
        const a = itemsOf(en.views[view].scene), b = itemsOf(de.views[view].scene);
        const order = JSON.stringify([...a.keys()]) === JSON.stringify([...b.keys()]);
        const differing = [...new Set([...a.keys(), ...b.keys()])].filter((id) => a.get(id) !== b.get(id));
        if (differing.length === 0 && order) continue;
        cells++;
        console.log(`1:${key} pose ${pose} ${view} ${mode}: en ${a.size} shape items, de ${b.size}; ${differing.length} differ${order ? "" : "; other order"}`);
        for (const id of differing.slice(0, limit)) console.log(`      ${id}\n         en ${(a.get(id) ?? "absent").slice(0, 260)}\n         de ${(b.get(id) ?? "absent").slice(0, 260)}`);
      }
    }
console.log(`${asset.id}: ${cells} (scale, pose, view, mode) differ between the languages`);
