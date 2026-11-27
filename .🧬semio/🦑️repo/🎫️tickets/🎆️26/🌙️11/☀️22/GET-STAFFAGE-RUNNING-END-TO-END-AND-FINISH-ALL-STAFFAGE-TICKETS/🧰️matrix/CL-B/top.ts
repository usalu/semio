/** 🔝️ Names what sets the upper and the left edge of a scene in both languages: top.ts <asset id or file> <requested denominator> <mode> <view> <pose>. Per language it lists the items that reach the smallest y and the smallest x of the scene (text boxes as the canvas reserves them), and the items whose distance to the first `shape:` path differs between the languages (those are the items a language moves).
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🧱️canvas/🟦️.ts
 */
import { existsSync, readFileSync } from "node:fs";
import { drawOptions, loadAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts";
import { resolveViews } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔭️multiview/🟦️.ts";
import { textWidthMm } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📐️units/🟦️.ts";

const [target, requestedText, mode, view, pose] = process.argv.slice(2) as [string, string, any, any, string];
const requested = Number(requestedText);
const asset = existsSync(target) ? JSON.parse(readFileSync(target, "utf8")).asset : loadAsset(target)!;
type Row = { id: string; minX: number; minY: number };
const rowsOf = (language: "en" | "de"): Row[] => {
  const drawing = resolveViews(asset, { kind: "pose", pose }, { kind: "scale", denominator: requested }, mode, drawOptions(mode, requested, false, language)).views[view as "elevation"];
  if (drawing.kind === "unavailable") throw new Error("unavailable");
  return drawing.scene.layers.flatMap((layer: any) => layer.items.map((item: any): Row => {
    if (item.kind === "text") {
      const w = textWidthMm(item.text, item.sizeMm, item.weight === "bold") * 1.04;
      return { id: `${item.id} "${item.text}"`, minX: item.anchor === "start" ? item.x : item.anchor === "end" ? item.x - w : item.x - w / 2, minY: item.y - item.sizeMm * 0.8 };
    }
    const points = item.commands.filter((c: any) => c.op !== "Z");
    return { id: item.id, minX: Math.min(...points.map((c: any) => c.x)), minY: Math.min(...points.map((c: any) => c.y)) };
  }));
};
const en = rowsOf("en"), de = rowsOf("de");
for (const [name, rows] of [["en", en], ["de", de]] as const) {
  const top = Math.min(...rows.map((r) => r.minY)), left = Math.min(...rows.map((r) => r.minX));
  console.log(`${name}: top ${top} set by ${rows.filter((r) => r.minY - top < 1e-9).map((r) => `${r.id} (${r.minY})`).join("; ")}`);
  console.log(`${name}: left ${left} set by ${rows.filter((r) => r.minX - left < 1e-9).map((r) => `${r.id} (${r.minX})`).join("; ")}`);
}
const reference = (rows: Row[]): Row => rows.find((r) => r.id.startsWith("shape:"))!;
const a = reference(en), b = reference(de);
const byId = new Map(de.map((r) => [r.id.split(' "')[0]!, r]));
for (const r of en) {
  const other = byId.get(r.id.split(' "')[0]!);
  if (other === undefined) continue;
  const dy = r.minY - a.minY - (other.minY - b.minY);
  if (dy !== 0) console.log(`moves in y by ${dy} mm against the shapes: ${r.id}: en ${r.minY - a.minY} de ${other.minY - b.minY}`);
}
