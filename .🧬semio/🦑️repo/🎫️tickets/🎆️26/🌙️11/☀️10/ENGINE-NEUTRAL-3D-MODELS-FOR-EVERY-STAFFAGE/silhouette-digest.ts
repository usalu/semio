import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { exportCast } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";
import { fixture } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🧰️.ts";
const engineIndependent = (svg: string): string => svg.replace(/"revision": "sha256:[0-9a-f]{64}"/gu, '"revision": "sha256:0"').replace(/data-value="([^"]*)"/gu, (_, v: string) => `data-value="${Number(Number(v).toPrecision(4))}"`);
const castVectors = fixture<{ manifest: any }>("🎭️cast-export");
const out = mkdtempSync(join(tmpdir(), "perf-"));
const ids = Object.keys(castVectors.manifest.casts).filter((k) => castVectors.manifest.casts[k].asset === "panel-radiator");
await exportCast(castVectors.manifest, ids, out);
const r: Record<string, Record<string, string>> = {};
for (const id of ids) { r[id] = {}; for (const n of readdirSync(join(out, id)).sort()) r[id][n] = createHash("sha256").update(engineIndependent(readFileSync(join(out, id, n), "utf8"))).digest("hex"); }
console.log(JSON.stringify(r));
rmSync(out, { recursive: true, force: true });
