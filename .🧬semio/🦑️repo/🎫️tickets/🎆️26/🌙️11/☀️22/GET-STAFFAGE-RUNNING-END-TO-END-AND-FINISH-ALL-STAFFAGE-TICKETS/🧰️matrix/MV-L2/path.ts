/** 🧵️ The path items the scene draws for one shape: `bun path.ts <id|file.json> <elevation|front|plan> <pose> <denominator> <shape id>` prints every `shape:<id>` and `outline:<id>:<n>` item with its layer and commands in paper millimetres, beside the evaluated rings of the shape in millimetres of paper.
 * @see ./paper.ts — which shape sets the paper box */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";

const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const { resolveViews } = await import(join(S, "🔨️modules", "🔭️multiview", "🟦️.ts"));
const [id, view, pose, denominator, shape] = process.argv.slice(2) as [string, "elevation" | "front" | "plan", string, string, string];
const asset = JSON.parse(readFileSync(fileOf(id), "utf8")).asset;
const multi = resolveViews(asset, { kind: "pose", pose }, { kind: "scale", denominator: Number(denominator) }, "light-line", engine.drawOptions("light-line", Number(denominator)));
const drawing = multi.views[view];
const k = drawing.scene.metadata.mmPerMetre;
console.log(`1:${denominator} ${view} ${pose}: ${k} mm per metre; metadata ${JSON.stringify(drawing.scene.metadata).slice(0, 600)}`);
for (const evaluated of drawing.state.shapes[view]) if (evaluated.shape.id === shape) console.log(`evaluated ${evaluated.rings.map((ring: any[]) => ring.map((p) => `${(p.x * k).toFixed(4)},${(p.y * k).toFixed(4)}`).join(" ")).join(" | ")}`);
for (const layer of drawing.scene.layers) for (const item of layer.items) if (item.kind === "path" && (item.id === `shape:${shape}` || item.id.startsWith(`outline:${shape}:`))) console.log(`${layer.id} ${item.id} stroke ${JSON.stringify(item.stroke)} fill ${JSON.stringify(item.fill)}: ${item.commands.map((c: any) => (c.op === "Z" ? "Z" : `${c.op}${c.x.toFixed(4)},${c.y.toFixed(4)}`)).join(" ")}`);
