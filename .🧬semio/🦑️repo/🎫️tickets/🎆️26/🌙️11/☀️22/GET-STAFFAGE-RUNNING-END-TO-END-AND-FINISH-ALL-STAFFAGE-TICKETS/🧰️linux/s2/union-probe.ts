/** 🔺️ Counts the instants of a clip (or the poses) at which `evaluate` throws, per scale, and optionally dumps the inputs of the thrown `PolygonUnionError`.
 * usage: ROOT=<mirror> bun union-probe.ts <asset|file> <clip|poses> <t0> <t1> <step> [--dump <file>] [--scales 50,100]
 */
import { writeFileSync } from "node:fs";
import { mod, readAsset } from "./lib.ts";

const [id, clip, t0, t1, step, ...rest] = process.argv.slice(2);
const dump = rest.includes("--dump") ? rest[rest.indexOf("--dump") + 1] : undefined;
const { evaluate } = await mod("🔨️modules/🧮️evaluation/🟦️.ts");
const asset = readAsset(id!);
const scales = rest.includes("--scales") ? rest[rest.indexOf("--scales") + 1]!.split(",") : Object.keys(asset.representations).filter((k) => asset.representations[k].status === "authored");
const queries = clip === "poses" ? asset.poses.map((p: any) => ({ kind: "pose", pose: p.id })) : Array.from({ length: Math.round((Number(t1) - Number(t0)) / Number(step)) + 1 }, (_, i) => ({ kind: "clip", clip, timeS: Number((Number(t0) + i * Number(step)).toFixed(6)) }));
const inputs: unknown[] = [];
for (const scale of scales) {
  let thrown = 0;
  const at: string[] = [];
  for (const query of queries) {
    try {
      evaluate(asset, query, { kind: "scale", scale: { denominator: Number(scale) } });
    } catch (error: any) {
      thrown++;
      at.push(`${query.pose ?? query.timeS}:${error.name}:${error.polygons ?? ""}`);
      if (error.name === "PolygonUnionError") inputs.push({ name: `${asset.id} 1:${scale} ${clip} ${query.pose ?? query.timeS}`, polygons: error.input.map((ring: any[]) => ring.map((p) => [p.x, p.y])) });
      else if (thrown === 1) console.log(error.stack?.split("\n").slice(0, 4).join("\n"));
    }
  }
  console.log(`${asset.id} 1:${scale} ${clip}: ${thrown} of ${queries.length} throw${at.length ? ` (${at.slice(0, 12).join(" ")})` : ""}`);
}
if (dump !== undefined) writeFileSync(dump, JSON.stringify(inputs));
