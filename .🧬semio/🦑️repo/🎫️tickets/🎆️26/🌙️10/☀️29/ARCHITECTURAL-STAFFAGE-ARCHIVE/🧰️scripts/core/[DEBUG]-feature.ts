import { featureWidthMm } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/📐️measure/🟦️.ts";
const rect = (w: number, h: number) => [[{ x: 0, y: 0 }, { x: w, y: 0 }, { x: w, y: h }, { x: 0, y: h }]];
for (const [w, h] of [[1.35, 0.35], [10, 0.5], [10, 1], [3, 3], [1, 0.2]] as const) console.log("[DEBUG]", w, h, featureWidthMm(rect(w, h), 0.2));
const disk = Array.from({ length: 200 }, (_, i) => ({ x: 2 + 2 * Math.cos((i / 200) * 2 * Math.PI), y: 2 + 2 * Math.sin((i / 200) * 2 * Math.PI) }));
console.log("[DEBUG] disk d=4", featureWidthMm([disk], 0.2));
