import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, readdirSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { exportCast } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";
import { featureWidthMm } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/📐️measure/🟦️.ts";

const path = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🚀️export-performance/🔣️.json";
const casts = JSON.parse(readFileSync("C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🎭️cast-export/🔣️.json", "utf8")).manifest;
const fixture = JSON.parse(readFileSync(path, "utf8"));
const engineIndependent = (svg: string): string => svg.replace(/data-value="([^"]*)"/gu, (_, v: string) => `data-value="${Number(Number(v).toPrecision(4))}"`);

for (const shape of fixture.featureWidth) shape.referenceMm = featureWidthMm(shape.rings, shape.minFeatureMm);

const ids = Object.keys(fixture.castDigests);
const out = mkdtempSync(join(tmpdir(), "staffage-regen-"));
try {
  const indexes = await exportCast(casts, ids, out);
  for (const index of indexes) {
    const sha256: Record<string, string> = {};
    for (const name of readdirSync(join(out, index.cast)).sort()) sha256[name] = createHash("sha256").update(engineIndependent(readFileSync(join(out, index.cast, name), "utf8"))).digest("hex");
    fixture.castDigests[index.cast] = { frames: index.frames, sha256 };
  }
} finally {
  rmSync(out, { recursive: true, force: true });
}
const temporary = `${path}.tmp`;
writeFileSync(temporary, JSON.stringify(fixture, null, 2) + "\n", "utf8");
renameSync(temporary, path);
console.log("regenerated", ids.length, "casts and", fixture.featureWidth.length, "shapes");
