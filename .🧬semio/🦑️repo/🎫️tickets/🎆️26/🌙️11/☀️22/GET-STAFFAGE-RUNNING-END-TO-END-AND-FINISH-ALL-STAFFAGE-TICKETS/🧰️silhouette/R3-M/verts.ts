/** 🔬️ The world vertices of one solid of a scale in the default pose (count, box, first vertices), to see what the kernel really builds: bun verts.ts <id|file.json> <scale> <solid id> [count] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./files.ts";

const [name, sc, id, count] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
const w = (worldSolids(asset, state.bones as never, Number(sc)) as any[]).find((x) => x.id === id);
const n = w.positions.length / 3;
const at = (i: number): number[] => [w.positions[i * 3], w.positions[i * 3 + 1], w.positions[i * 3 + 2]];
const box = [0, 1, 2].map((k) => [Math.min(...Array.from({ length: n }, (_, i) => at(i)[k]!)), Math.max(...Array.from({ length: n }, (_, i) => at(i)[k]!))].map((v) => v.toFixed(4)).join(".."));
console.log(`${id}: ${n} vertices, ${w.indices.length / 3} triangles, box x ${box[0]} y ${box[1]} z ${box[2]}`);
for (let i = 0; i < Math.min(n, Number(count ?? 8)); i++) console.log(`  ${at(i).map((v) => v!.toFixed(4)).join(", ")}`);
