/** 🔎️ Names the parts of one scale of the Scots pine whose outline in a view covers a paper point.
 * Usage: bun r3-scots-pine-probe.ts <scale> <view> <x> <y> */
import { lobeOutline, tubePolygons, type View } from "./r3-scots-pine-kit.ts";
import { models } from "./r3-scots-pine-model.ts";
const [scale, view, x, y] = [Number(process.argv[2]), process.argv[3] as View, Number(process.argv[4]), Number(process.argv[5])];
const m = models.find((k) => k.scale === scale)!;
const inside = (ring: { x: number; y: number }[]): boolean => {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a.y > y !== b.y > y && x < ((b.x - a.x) * (y - a.y)) / (b.y - a.y) + a.x) hit = !hit;
  }
  return hit;
};
console.log(`[DEBUG] 1:${scale} ${view} ${x} ${y}: lobes ${m.lobes.filter((l) => inside(lobeOutline(view, l, 48))).map((l) => l.id).join(" ") || "none"}; wood ${m.wood.filter((t) => tubePolygons(view, t).some(inside)).map((t) => t.id).join(" ") || "none"}`);
