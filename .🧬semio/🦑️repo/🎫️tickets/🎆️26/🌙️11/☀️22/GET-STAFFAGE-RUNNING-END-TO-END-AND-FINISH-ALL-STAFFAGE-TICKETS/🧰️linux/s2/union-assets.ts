/** 🔺️ Reproduces the open `PolygonUnionError` requests on in-memory variants of two catalog people (nothing is written to the catalog) over the subjects of the multiview gate, and dumps the failing union inputs.
 * - `pregnant-person` 1:50 front `shank-r`: the profile point the fixer moved by 0.4 mm (`unlevel`) is put back to `[0.1217, 0.0509]`, so one profile edge runs exactly along the bone again (request "multiview MH-4").
 * - `tour-guide` 1:50 plan `hand-r`: the elevation hand (traced outline, `sweep: "w"`) is copied into the plan (request "multiview MH-6").
 * usage: ROOT=<mirror> bun union-assets.ts [--dump <file>] [--scales 50,100,200,500]
 */
import { writeFileSync } from "node:fs";
import { mod, readAsset } from "./lib.ts";

const args = process.argv.slice(2);
const dump = args.includes("--dump") ? args[args.indexOf("--dump") + 1] : undefined;
const scales = (args.includes("--scales") ? args[args.indexOf("--scales") + 1]! : "50").split(",").map(Number);
const { evaluate } = await mod("🔨️modules/🧮️evaluation/🟦️.ts");
const engine = await mod("🧪️tests/🔭️multiview-coverage/🧰️.ts");

const pregnant = readAsset("pregnant-person");
for (const key of Object.keys(pregnant.representations)) {
  const shank = pregnant.representations[key].geometry?.front?.find((s: any) => s.id === "shank-r" && s.solid?.sweep === "v");
  const point = shank?.points?.find((p: number[]) => p[0] === 0.1217 && p[1] === 0.0505);
  if (point !== undefined) point[1] = 0.0509;
}
const guide = readAsset("tour-guide");
const hand = guide.representations["50"].geometry.elevation.find((s: any) => s.id === "hand-r");
guide.representations["50"].geometry.plan = [...guide.representations["50"].geometry.plan.filter((s: any) => s.id !== "hand-r"), structuredClone(hand)];

const inputs: unknown[] = [];
for (const [label, asset] of [["pregnant-person with the level shank edge", pregnant], ["tour-guide with the elevation hand in the plan", guide]] as const)
  for (const scale of scales) {
    if (asset.representations[String(scale)]?.status !== "authored") continue;
    const list = engine.subjects(asset, scale);
    const failed: string[] = [];
    const started = performance.now();
    for (const subject of list) {
      try {
        evaluate(asset, subject.query, engine.scaleSelector(scale));
      } catch (error: any) {
        failed.push(`${subject.id} [${error.name} ${error.polygons ?? ""}]`);
        if (error.name === "PolygonUnionError") inputs.push({ name: `${label}, 1:${scale}, ${subject.id}`, polygons: error.input.map((ring: any[]) => ring.map((p) => [p.x, p.y])) });
      }
    }
    console.log(`${label} 1:${scale}: ${failed.length} of ${list.length} subjects throw, ${((performance.now() - started) / 1000).toFixed(1)} s`);
    for (const line of failed) console.log(`  ${line}`);
  }
if (dump !== undefined) writeFileSync(dump, JSON.stringify(inputs));
