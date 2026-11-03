/** 🧪 Writes the `featureWidth` and `castDigests` keys of the export-performance fixture.
 * `featureWidth` references come from the PRE-optimisation measure module (`bun gen_fixture.ts <orig-measure.ts>`), `castDigests` from the current exporter on the cast-export fixture manifest. */
import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { exportCast } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";

const STAFFAGE = resolve(import.meta.dir, "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const FIXTURE = join(STAFFAGE, "🧫️fixtures/🚀️export-performance/🔣️.json");
const original = (await import(pathToFileURL(resolve(process.argv[2]!)).href)) as { featureWidthMm: (rings: { x: number; y: number }[][], minFeatureMm: number) => number };

const engineIndependent = (svg: string): string => svg.replace(/data-value="([^"]*)"/gu, (_, v: string) => `data-value="${Number(Number(v).toPrecision(4))}"`);

type P = { x: number; y: number };
const rect = (x0: number, y0: number, x1: number, y1: number): P[] => [{ x: x0, y: y0 }, { x: x1, y: y0 }, { x: x1, y: y1 }, { x: x0, y: y1 }];
const polygon = (cx: number, cy: number, r: number, n: number): P[] => Array.from({ length: n }, (_, i) => ({ x: cx + r * Math.cos((2 * Math.PI * i) / n), y: cy + r * Math.sin((2 * Math.PI * i) / n) }));
const comb = (): P[] => {
  const ring: P[] = [{ x: 0, y: 0 }, { x: 12, y: 0 }];
  for (let i = 3; i >= 0; i--) ring.push({ x: 1 + i * 3.5 + 0.4, y: 2 }, { x: 1 + i * 3.5 + 0.4, y: 7 }, { x: 1 + i * 3.5, y: 7 }, { x: 1 + i * 3.5, y: 2 });
  ring.push({ x: 0, y: 2 });
  return ring;
};
const shapes = [
  { name: "rectangle-10x2", rings: [rect(0, 0, 10, 2)], minFeatureMm: 0.3, analyticMm: 2 },
  { name: "bar-40x0.5", rings: [rect(0, 0, 40, 0.5)], minFeatureMm: 0.3, analyticMm: 0.5 },
  { name: "square-6", rings: [rect(0, 0, 6, 6)], minFeatureMm: 0.3, analyticMm: 6 },
  { name: "l-shape", rings: [[{ x: 0, y: 0 }, { x: 8, y: 0 }, { x: 8, y: 1.5 }, { x: 1.5, y: 1.5 }, { x: 1.5, y: 8 }, { x: 0, y: 8 }]], minFeatureMm: 0.3, analyticMm: 1.5 },
  { name: "disk-r5", rings: [polygon(0, 0, 5, 64)], minFeatureMm: 0.3, analyticMm: 10 },
  { name: "annulus-width-1", rings: [polygon(0, 0, 6, 96), polygon(0, 0, 5, 96)], minFeatureMm: 0.3, analyticMm: 1 },
  { name: "comb-thin-teeth", rings: [comb()], minFeatureMm: 0.5, analyticMm: null },
  { name: "large-capped-raster", rings: [rect(0, 0, 120, 120), rect(30, 30, 31, 90).slice().reverse()], minFeatureMm: 0.3, analyticMm: 30 },
];
const featureWidth = shapes.map((shape) => ({ ...shape, referenceMm: original.featureWidthMm(shape.rings, shape.minFeatureMm) }));
for (const row of featureWidth) console.log(row.name, row.referenceMm);

const manifest = JSON.parse(readFileSync(join(STAFFAGE, "🧫️fixtures/🎭️cast-export/🔣️.json"), "utf8")).manifest;
const ids = ["walker-line", "walker-thermal", "radiator-transient", "radiator-rated"];
const out = mkdtempSync(join(tmpdir(), "staffage-digest-"));
const castDigests: Record<string, { frames: string[]; sha256: Record<string, string> }> = {};
try {
  const indexes = await exportCast(manifest, ids, out, { language: "en" });
  for (const index of indexes) {
    const sha256: Record<string, string> = {};
    for (const name of readdirSync(join(out, index.cast)).sort()) sha256[name] = createHash("sha256").update(engineIndependent(readFileSync(join(out, index.cast, name), "utf8"))).digest("hex");
    castDigests[index.cast] = { frames: index.frames as string[], sha256 };
  }
} finally {
  rmSync(out, { recursive: true, force: true });
}
const fixture = JSON.parse(readFileSync(FIXTURE, "utf8"));
fixture.featureWidth = featureWidth;
fixture.castDigests = castDigests;
writeFileSync(FIXTURE, `${JSON.stringify(fixture, null, 2)}\n`);
console.log("wrote", Object.keys(castDigests).map((id) => `${id}:${Object.keys(castDigests[id]!.sha256).length}`).join(" "));
