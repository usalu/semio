/** 🚶️ Probe: exports the cast `walker-line` of the cast-export fixture against a catalog root and prints the digest the export-performance suite pins for each frame. `bun walker-digest.ts <catalog root> <out dir>` */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { exportCast } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";

const [catalog, out] = process.argv.slice(2);
const root = "🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures";
const manifest = JSON.parse(readFileSync(join(root, "🎭️cast-export/🔣️.json"), "utf8")).manifest;
const pinned = JSON.parse(readFileSync(join(root, "🚀️export-performance/🔣️.json"), "utf8")).castDigests["walker-line"].sha256 as Record<string, string>;
const engineIndependent = (svg: string): string => svg.replace(/"revision": "sha256:[0-9a-f]{64}"/gu, '"revision": "sha256:0"').replace(/data-value="([^"]*)"/gu, (_, v: string) => `data-value="${Number(Number(v).toPrecision(4))}"`);
const [index] = await exportCast(manifest, ["walker-line"], out!, { catalog: catalog! });
const names = [...new Set(index!.frames)].sort();
console.log(names.map((name) => `${name} ${createHash("sha256").update(engineIndependent(readFileSync(join(out!, "walker-line", name), "utf8"))).digest("hex") === pinned[name] ? "= pin" : "DIFFERS"}`).join("\n"));
