import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, readdirSync, writeFileSync, mkdirSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { exportCast } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";
const R = `${process.cwd()}/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/`;
const perf = JSON.parse(readFileSync(`${R}🚀️export-performance/🔣️.json`, "utf8")).castDigests;
const manifest = JSON.parse(readFileSync(`${R}🎭️cast-export/🔣️.json`, "utf8")).manifest;
const keep = process.argv[2];
const out = mkdtempSync(join(tmpdir(), "perf-"));
const ev = (s: string) => s.replace(/"revision": "sha256:[0-9a-f]{64}"/gu, '"revision": "sha256:0"').replace(/data-value="([^"]*)"/gu, (_, v: string) => `data-value="${Number(Number(v).toPrecision(4))}"`);
const ids = Object.keys(perf);
await exportCast(manifest, ids, out);
for (const id of ids) {
  const files = readdirSync(join(out, id)).sort();
  const now: Record<string, string> = {};
  for (const f of files) now[f] = createHash("sha256").update(ev(readFileSync(join(out, id, f), "utf8"))).digest("hex");
  const bad = files.filter((f) => now[f] !== perf[id].sha256[f]);
  console.log(id, "files", files.length, "pinned", Object.keys(perf[id].sha256).length, "mismatch", bad.length, bad.slice(0, 4).join(","));
  if (keep) { mkdirSync(join(keep, id), { recursive: true }); for (const f of files) writeFileSync(join(keep, id, f), readFileSync(join(out, id, f))); }
  perf[id].now = now;
}
if (keep) writeFileSync(join(keep, "now.json"), JSON.stringify(perf));
