import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, readdirSync, writeFileSync, mkdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { exportCast } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";
const fx = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🎭️cast-export/🔣️.json", "utf8"));
const perf = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🚀️export-performance/🔣️.json", "utf8"));
const engineIndependent = (svg: string): string => svg.replace(/data-value="([^"]*)"/gu, (_, v: string) => `data-value="${Number(Number(v).toPrecision(4))}"`);
const out = mkdtempSync(join(tmpdir(), "perf-"));
const ids = Object.keys(perf.castDigests);
await exportCast(fx.manifest, ids, out);
const keep = process.argv[2]!;
for (const id of ids) {
  const pinned = perf.castDigests[id];
  const files = readdirSync(join(out, id)).sort();
  const bad = files.filter((f) => pinned.sha256[f] === undefined || createHash("sha256").update(engineIndependent(readFileSync(join(out, id, f), "utf8"))).digest("hex") !== pinned.sha256[f]);
  console.log(id, "files", files.length, "pinned", Object.keys(pinned.sha256).length, "mismatch", bad.length, bad.slice(0, 3).join(","));
  if (bad.length) { mkdirSync(join(keep, id), { recursive: true }); for (const f of bad.slice(0, 2)) writeFileSync(join(keep, id, f), readFileSync(join(out, id, f))); }
}
