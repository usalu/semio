import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { exportCast } = await import(`${S}/🔨️modules/📤️export/🎭️cast/🟦️.ts`);
const manifest = JSON.parse(readFileSync(`${S}/🧪️tests/🧫️fixtures/🎭️cast-export/🔣️.json`.replace("🧪️tests/", ""), "utf8")).manifest;
const pinned = JSON.parse(readFileSync(`${S}/🧫️fixtures/🚀️export-performance/🔣️.json`, "utf8")).castDigests;
const engineIndependent = (svg: string): string => svg.replace(/data-value="([^"]*)"/gu, (_, v: string) => `data-value="${Number(Number(v).toPrecision(4))}"`);
const ids = process.argv.slice(2);
const out = mkdtempSync(join(tmpdir(), "ap-digests-"));
const result: Record<string, { frames: string[]; sha256: Record<string, string> }> = {};
try {
  const indexes = await exportCast(manifest, ids, out);
  for (const index of indexes) {
    const sha256: Record<string, string> = {};
    for (const name of readdirSync(join(out, index.cast)).sort()) sha256[name] = createHash("sha256").update(engineIndependent(readFileSync(join(out, index.cast, name), "utf8"))).digest("hex");
    result[index.cast] = { frames: index.frames, sha256 };
  }
} finally {
  rmSync(out, { recursive: true, force: true });
}
writeFileSync(process.env.DIGEST_OUT!, JSON.stringify({ result, pinned: Object.fromEntries(ids.map((id) => [id, pinned[id]])) }, null, 2));
console.log("done", Object.keys(result));
