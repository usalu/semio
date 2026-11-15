import { readFileSync, writeFileSync } from "node:fs";
import { worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const a = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const m = (w: any) => ({ positions: [...w.positions], indices: [...w.indices] });
const body = (d: number) => worldSolids(a, evaluate(a, { kind: "pose", pose: "stand" }, { kind: "scale", scale: { denominator: d } }).bones as never, d, "body").map(m);
const cases = [{ name: "b50", meshes: body(50), cell: 0.04 }, { name: "x", meshes: body(200), other: body(50), view: "elevation" }];
writeFileSync(process.argv[3]!, JSON.stringify({ cases }));
