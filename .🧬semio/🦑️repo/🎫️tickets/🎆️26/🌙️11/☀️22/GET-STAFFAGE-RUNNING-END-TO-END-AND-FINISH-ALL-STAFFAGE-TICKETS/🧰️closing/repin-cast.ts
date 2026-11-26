/** 📌️ Regenerates the `castDigests` of `🧫️fixtures/🚀️export-performance` from the live engine, with the digest function of the case `🚀️export-performance`: `bun repin-cast.ts <out.json>`.
 *
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🚀️export-performance/🟦️.ts — the pinned case
 */
import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { exportCast } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";
const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const manifest = JSON.parse(readFileSync(`${S}/🧫️fixtures/🎭️cast-export/🔣️.json`, "utf8")).manifest;
const pinned = JSON.parse(readFileSync(`${S}/🧫️fixtures/🚀️export-performance/🔣️.json`, "utf8")).castDigests as Record<string, unknown>;
const engineIndependent = (svg: string): string => svg.replace(/"revision": "sha256:[0-9a-f]{64}"/gu, '"revision": "sha256:0"').replace(/data-value="([^"]*)"/gu, (_, v: string) => `data-value="${Number(Number(v).toPrecision(4))}"`);
const out = mkdtempSync(join(tmpdir(), "staffage-repin-"));
try {
  const ids = Object.keys(pinned);
  const indexes = await exportCast(manifest, ids, out);
  const result: Record<string, { frames: string[]; sha256: Record<string, string> }> = {};
  for (const index of indexes) {
    const files = readdirSync(join(out, index.cast)).sort();
    result[index.cast] = { frames: index.frames, sha256: Object.fromEntries(files.map((name) => [name, createHash("sha256").update(engineIndependent(readFileSync(join(out, index.cast, name), "utf8"))).digest("hex")])) };
  }
  writeFileSync(process.argv[2]!, JSON.stringify(result));
} finally {
  rmSync(out, { recursive: true, force: true });
}
