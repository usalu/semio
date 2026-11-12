import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { exportCast } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";
const fx = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🎭️cast-export/🔣️.json", "utf8"));
const file = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🚀️export-performance/🔣️.json";
const text = readFileSync(file, "utf8");
const perf = JSON.parse(text);
const engineIndependent = (svg: string): string => svg.replace(/"revision": "sha256:[0-9a-f]{64}"/gu, '"revision": "sha256:0"').replace(/data-value="([^"]*)"/gu, (_, v: string) => `data-value="${Number(Number(v).toPrecision(4))}"`);
const out = mkdtempSync(join(tmpdir(), "pin-"));
const ids = Object.keys(perf.castDigests);
const indexes = await exportCast(fx.manifest, ids, out);
const digests: Record<string, unknown> = {};
for (const index of indexes) {
  const files = readdirSync(join(out, index.cast)).sort();
  const sha256: Record<string, string> = {};
  for (const f of files) sha256[f] = createHash("sha256").update(engineIndependent(readFileSync(join(out, index.cast, f), "utf8"))).digest("hex");
  digests[index.cast] = { frames: index.frames, sha256 };
}
const marker = '  "castDigests": ';
const at = text.indexOf(marker);
const block = JSON.stringify(digests, null, 2).split("\n").map((line, i) => (i === 0 ? line : "  " + line)).join("\n");
writeFileSync(file, text.slice(0, at) + marker + block + "\n}\n");
rmSync(out, { recursive: true, force: true });
