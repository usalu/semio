import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { exportCast } = await import(pathToFileURL(resolve(S, "🔨️modules/📤️export/🎭️cast/🟦️.ts")).href);
const perf = resolve(S, "🧫️fixtures/🚀️export-performance/🔣️.json");
const cast = JSON.parse(readFileSync(resolve(S, "🧫️fixtures/🎭️cast-export/🔣️.json"), "utf8"));
const doc = JSON.parse(readFileSync(perf, "utf8"));
const ids = process.argv.slice(2);
const eng = (svg: string) => svg.replace(/data-value="([^"]*)"/gu, (_: string, v: string) => `data-value="${Number(Number(v).toPrecision(4))}"`);
const out = mkdtempSync(join(tmpdir(), "pin-"));
const indexes = await exportCast(cast.manifest, ids, out);
for (const index of indexes) {
  const files = readdirSync(join(out, index.cast)).sort();
  const sha: Record<string, string> = {};
  for (const f of files) sha[f] = createHash("sha256").update(eng(readFileSync(join(out, index.cast, f), "utf8"))).digest("hex");
  const old = doc.castDigests[index.cast];
  const moved = files.filter((f) => old.sha256[f] !== sha[f]);
  console.log(index.cast, "moved", moved.length, "of", files.length);
  doc.castDigests[index.cast] = { frames: index.frames, sha256: sha };
}
rmSync(out, { recursive: true, force: true });
if (process.env.WRITE) writeFileSync(perf, JSON.stringify(doc, null, 2) + "\n");
