import { writeFileSync } from "node:fs";
import { PolygonUnionError } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { catalog } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const out: { asset: string; clip: string; t: number; scale: number; input: unknown }[] = [];
const jobs = [["red-squirrel", "nibble", 0.4, 50]] as const;
for (const [id, clip, t, scale] of jobs) {
  const asset = catalog().find((a) => a.id === id)!;
  const c = asset.clips.find((x) => x.id === clip)!;
  const time = t * c.durationS; console.log(id, c.durationS, time);
  try { evaluate(asset, { kind: "clip", clip, timeS: time }, { kind: "scale", scale: { denominator: scale } }); console.log("ok", id, time); }
  catch (e) { if (e instanceof PolygonUnionError) { console.log("ERR", id, scale, e.message, e.input.map((r) => r.length)); out.push({ asset: id, clip, t: time, scale, input: e.input }); } else throw e; }
}
writeFileSync("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE/🗑️generated/final/union-inputs4.json", JSON.stringify(out));
