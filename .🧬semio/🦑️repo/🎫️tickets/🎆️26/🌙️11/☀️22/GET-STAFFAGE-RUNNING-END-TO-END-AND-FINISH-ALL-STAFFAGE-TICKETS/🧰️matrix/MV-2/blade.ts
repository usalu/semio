/** 🌽️ Sets the outlets of arched blades onto the blade: `bun blade.ts <id|file.json> --ops ops.json [--match ^tp-]`. A transpiration anchor authored on the chord of its leaf bone (`y` 0, no `z`) hangs below the arch the blade is drawn as. For every matching anchor whose bone draws, at the finest authored scale, an elevation shape with an `extruded` `sweep: "w"` solid, the anchor keeps its `x` along the bone and gets the `y` of the middle of that profile at `x` and the `z` of the middle of the lateral span (measured from the rest origin of the bone, as the solid states it). Written as `anchor` ops for `apply.ts`.
 * @see ./apply.ts */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { flattenGeometry } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const asset = readDoc(args[0]!).doc.asset;
const match = new RegExp(flag("match") ?? "^tp-");
const rest = restFrames(asset.rig) as Record<string, { frame: { o: number[] } }>;
const finest = Object.keys(asset.representations).filter((key) => asset.representations[key].geometry !== undefined).sort((a, b) => Number(a) - Number(b))[0]!;
const round = (n: number): number => Math.round(n * 1e4) / 1e4;
const ops: any[] = [];
for (const anchor of asset.anchors) {
  if (!match.test(anchor.id)) continue;
  const blade = asset.representations[finest].geometry.elevation.find((shape: any) => shape.bone === anchor.bone && shape.filled === true && shape.solid?.kind === "extruded" && shape.solid.sweep === "w");
  if (blade === undefined) continue;
  const cuts: number[] = [];
  for (const ring of flattenGeometry(blade).rings)
    for (let i = 0; i < ring.length; i++) {
      const [p, q] = [ring[i], ring[(i + 1) % ring.length]];
      if ((p.x - anchor.x) * (q.x - anchor.x) > 0 || p.x === q.x) continue;
      cuts.push(p.y + ((q.y - p.y) * (anchor.x - p.x)) / (q.x - p.x));
    }
  if (cuts.length < 2) continue;
  const set = { bone: anchor.bone, x: anchor.x, y: round((Math.min(...cuts) + Math.max(...cuts)) / 2), z: round((blade.solid.z0 + blade.solid.z1) / 2 - rest[anchor.bone]!.frame.o[2]!) };
  console.log(`${anchor.id} on ${anchor.bone}: (${anchor.x}, ${anchor.y}, ${anchor.z ?? "-"}) -> (${set.x}, ${set.y}, ${set.z}) on "${blade.id}" of 1:${finest}`);
  ops.push({ op: "anchor", id: anchor.id, set });
}
const file = flag("ops")!;
const known: any[] = existsSync(file) ? JSON.parse(readFileSync(file, "utf8")) : [];
const next = [...known.filter((op) => op.op !== "anchor" || !ops.some((mine) => mine.id === op.id)), ...ops];
writeFileSync(file, `[\n${next.map((op) => `  ${JSON.stringify(op)}`).join(",\n")}\n]\n`);
console.log(`${asset.id}: ${ops.length} anchor ops written, ${next.length} ops in ${file}`);
