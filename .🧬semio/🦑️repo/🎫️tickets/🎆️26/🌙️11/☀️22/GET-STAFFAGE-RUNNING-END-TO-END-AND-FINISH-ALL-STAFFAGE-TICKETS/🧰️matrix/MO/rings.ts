/** 🧿️ The evaluated rings of the drawn shapes of one view whose id contains a substring, in the default pose, with the ops applied in memory unless `--no-ops`: bun rings.ts <id> <scale> <view> <id-substring> [--no-ops] */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { applyAll } from "./apply.ts";
import { catalogFile, fileOf, opsFile } from "./files.ts";
const [name, sc, view, sub] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
if (!process.argv.includes("--no-ops") && !name!.endsWith(".json") && (await Bun.file(opsFile(name!)).exists())) applyAll(asset, JSON.parse(await Bun.file(opsFile(name!)).text()));
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
for (const s of state.shapes[view!]) if (String(s.shape.id).includes(sub!)) for (const ring of s.rings) console.log(s.shape.id, ring.length, ring.slice(0, 40).map((p: any) => `(${p.x.toFixed(4)},${p.y.toFixed(4)})`).join(" "));
