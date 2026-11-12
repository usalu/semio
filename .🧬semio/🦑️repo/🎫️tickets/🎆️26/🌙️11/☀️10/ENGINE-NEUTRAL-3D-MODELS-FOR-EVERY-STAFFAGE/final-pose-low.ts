import { readFileSync } from "node:fs";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [path, pose, scale] = process.argv.slice(2);
const a = JSON.parse(readFileSync(path!, "utf8")).asset;
const st: any = evaluate(a, { kind: "pose", pose: pose! }, { kind: "scale", scale: { denominator: Number(scale) } });
const list = st.shapes.elevation.filter((e: any) => e.shape.role !== "ground").map((e: any) => { const pts = e.rings.flat(); return { id: e.shape.id, minY: Math.min(...pts.map((p: any) => p.y)) }; }).sort((x: any, y: any) => x.minY - y.minY).slice(0, 8);
console.log(list.map((x: any) => x.id + " " + x.minY.toFixed(3)).join("\n"));
