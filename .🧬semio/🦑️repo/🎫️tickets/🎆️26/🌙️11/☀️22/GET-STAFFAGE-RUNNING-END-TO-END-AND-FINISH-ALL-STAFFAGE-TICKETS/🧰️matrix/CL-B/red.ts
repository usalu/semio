/** 🔴️ The red of the arrow rule shown on the former `arrowOutline` (copied here as `former`): the heights of the outline of one arrow at several places of the sheet, former against current. The former outline took its direction back from the two sheet points `tail` and `head`, so its heights change in the last bit with the horizontal place; the current one is built from the direction and the length. red.ts prints the count of places whose heights differ from those at x = 0.
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/📐️geometry/🟦️.ts
 */
import { add, arrowOutline, length, perp, scale, sub, unit, type Vec } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/📐️geometry/🟦️.ts";

function former(tail: Vec, head: Vec, shaft: number, headLength: number, headWidth: number): Vec[] {
  const d = sub(head, tail);
  const l = length(d);
  const u = unit(d);
  const n = perp(u);
  const hl = Math.min(headLength, l * 0.6);
  const base = sub(head, scale(u, hl));
  const hw = headWidth * (hl / headLength);
  return [add(tail, scale(n, shaft / 2)), add(base, scale(n, shaft / 2)), add(base, scale(n, hw / 2)), head, sub(base, scale(n, hw / 2)), sub(base, scale(n, shaft / 2)), sub(tail, scale(n, shaft / 2))];
}
const places = Array.from({ length: 2000 }, (_, i) => 40 + i * 0.0817);
for (const direction of [{ x: 0.6, y: 0.8 }, { x: -0.28, y: 0.96 }, { x: 5 / 13, y: -12 / 13 }, { x: -0.8, y: -0.6 }]) {
  const reach = 11.3;
  const old = (x: number): string => JSON.stringify(former(sub({ x, y: 16.2 }, scale(direction, reach)), { x, y: 16.2 }, 0.3, 1.8, 1.3).map((p) => p.y));
  const now = (x: number): string => JSON.stringify(arrowOutline({ x, y: 16.2 }, direction, reach, 0.3, 1.8, 1.3).map((p) => p.y));
  const drift = Math.max(...places.flatMap((x) => { const a = former(sub({ x, y: 16.2 }, scale(direction, reach)), { x, y: 16.2 }, 0.3, 1.8, 1.3), b = arrowOutline({ x, y: 16.2 }, direction, reach, 0.3, 1.8, 1.3); return a.map((p, i) => Math.hypot(p.x - b[i]!.x, p.y - b[i]!.y)); }));
  console.log(`direction ${JSON.stringify(direction)}: places of ${places.length} whose heights differ from x = 0: former ${places.filter((x) => old(x) !== old(0)).length}, current ${places.filter((x) => now(x) !== now(0)).length}; largest distance former to current ${drift} mm`);
}
