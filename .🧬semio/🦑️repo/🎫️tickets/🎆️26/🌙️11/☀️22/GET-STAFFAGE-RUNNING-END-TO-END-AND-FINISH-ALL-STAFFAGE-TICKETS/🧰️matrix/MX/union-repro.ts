/** 🧨️ Reproduces the `PolygonUnionError` of the requests "multiview MH-4 — prism swept along v" and "multiview MH-6 — prism swept along w": `bun union-repro.ts <out.json>` puts the filed shapes back on copies of `pregnant-person` (the level profile edge of the front `shank-r`) and `tour-guide` (the elevation hand copied into the plan), evaluates every subject of 1:50 and writes the polygons of every union that throws. */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const pregnant = readDoc("pregnant-person").doc.asset;
const shank = pregnant.representations["50"].geometry.front.find((shape: any) => shape.id === "shank-r");
shank.points = shank.points.map(([x, y]: [number, number]) => (x === 0.1217 ? [x, 0.0509] : [x, y]));
const guide = readDoc("tour-guide").doc.asset;
const geometry = guide.representations["50"].geometry;
geometry.plan = [...geometry.plan.filter((shape: any) => shape.id !== "hand-r"), structuredClone(geometry.elevation.find((shape: any) => shape.id === "hand-r"))];
const found: { asset: string; subject: string; message: string; polygons: number[][][] }[] = [];
for (const asset of [pregnant, guide]) {
  const all = engine.subjects(asset, 50) as { id: string; query: unknown }[];
  let ok = 0;
  for (const subject of all) {
    try {
      evaluate(asset, subject.query, engine.scaleSelector(50));
      ok++;
    } catch (error) {
      const input = (error as { input?: { x: number; y: number }[][] }).input ?? [];
      found.push({ asset: asset.id, subject: subject.id, message: (error as Error).message.slice(0, 120), polygons: input.map((ring) => ring.map((p) => [p.x, p.y])) });
      console.log(`[DEBUG] ${asset.id} ${subject.id}: ${(error as Error).message.slice(0, 120)}`);
    }
  }
  console.log(`[DEBUG] ${asset.id}: ${ok} of ${all.length} subjects evaluate`);
}
writeFileSync(process.argv[2]!, JSON.stringify(found));
