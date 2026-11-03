import { mkdtempSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { exportCast } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";

const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures";
const perf = JSON.parse(readFileSync(`${S}/🚀️export-performance/🔣️.json`, "utf8"));
const casts = JSON.parse(readFileSync(`${S}/🎭️cast-export/🔣️.json`, "utf8")).manifest;
const ids = Object.keys(perf.castDigests);
const on = mkdtempSync(join(tmpdir(), "depth-on-"));
const off = mkdtempSync(join(tmpdir(), "depth-off-"));
delete process.env.SEMIO_DEPTH_OFF;
await exportCast(casts, ids, on);
process.env.SEMIO_DEPTH_OFF = "1";
await exportCast(casts, ids, off);
const lines: string[] = [];
let changed = 0;
for (const id of ids)
  for (const name of readdirSync(join(on, id))) {
    const same = readFileSync(join(on, id, name), "utf8") === readFileSync(join(off, id, name), "utf8");
    if (!same) { changed++; lines.push(`CHANGED ${id}/${name}`); }
  }
lines.push(`casts ${ids.length}, files changed by depth ordering: ${changed}`);
writeFileSync(process.env.FW_LOG!, lines.join("\n") + "\n");
