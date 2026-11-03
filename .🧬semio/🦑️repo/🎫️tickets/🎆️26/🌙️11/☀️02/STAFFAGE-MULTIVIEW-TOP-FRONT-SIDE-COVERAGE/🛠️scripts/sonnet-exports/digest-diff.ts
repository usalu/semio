import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const NEW = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const OLD = join(tmpdir(), "idx", "🎓️teaching/🏛️architecture/🧍️staffage").replaceAll("\\", "/");
const exporter = async (root: string) => (await import(pathToFileURL(`${root}/🔨️modules/📤️export/🎭️cast/🟦️.ts`).href)) as { exportCast: (m: unknown, ids: string[], out: string, o?: unknown) => Promise<unknown> };
const vectors = JSON.parse(readFileSync(`${NEW}/🧫️fixtures/🚀️export-performance/🔣️.json`, "utf8")) as { castDigests: Record<string, { frames: string[]; sha256: Record<string, string> }> };
const independent = (svg: string): string => svg.replace(/data-value="([^"]*)"/gu, (_, v: string) => `data-value="${Number(Number(v).toPrecision(4))}"`);
const casts = JSON.parse(readFileSync(`${NEW}/🧫️fixtures/🎭️cast-export/🔣️.json`, "utf8")).manifest as { version: 1; casts: Record<string, unknown> };
const ids = Object.keys(vectors.castDigests);
const manifest = { version: 1, casts: Object.fromEntries(ids.map((id) => [id, casts.casts[id]])) };

const strip = (svg: string): string =>
  svg
    .replace(/<title>[^<]*<\/title>\n?/u, "")
    .replace(/<desc>[^<]*<\/desc>\n?/u, "")
    .replace(/ (?:view-title|viewpoint-title|fidelity-title)-(?:en|de)="[^"]*"/gu, "")
    .replace(/ fidelity="[^"]*"/u, "")
    .replace(/ omitted-values="[^"]*"/u, "");

const a = mkdtempSync(join(tmpdir(), "cast-old-"));
const b = mkdtempSync(join(tmpdir(), "cast-new-"));
await (await exporter(OLD)).exportCast(manifest, ids, a, { catalog: `${OLD}/🗂️catalog` });
await (await exporter(NEW)).exportCast(manifest, ids, b, { catalog: `${NEW}/🗂️catalog` });
let same = 0;
let pinnedByOld = 0;
let total = 0;
const different: string[] = [];
for (const id of ids)
  for (const frame of vectors.castDigests[id]!.frames) {
    total += 1;
    if (createHash("sha256").update(independent(readFileSync(join(a, id, frame), "utf8"))).digest("hex") === vectors.castDigests[id]!.sha256[frame]) pinnedByOld += 1;
    const left = strip(readFileSync(join(a, id, frame), "utf8"));
    const right = strip(readFileSync(join(b, id, frame), "utf8"));
    if (left === right) same += 1;
    else {
      let at = 0;
      while (left[at] === right[at]) at += 1;
      different.push(`${id}/${frame} at ${at}: old ${JSON.stringify(left.slice(at - 60, at + 120))} new ${JSON.stringify(right.slice(at - 60, at + 120))}`);
    }
  }
console.log(`old (index) frames equal to the pinned digests: ${pinnedByOld} of ${total}`);
console.log(`identical after removing title, desc, fidelity attributes: ${same}; different: ${different.length}`);
for (const line of different.slice(0, 6)) console.log(line);
console.log(readdirSync(a).length, readdirSync(b).length);
rmSync(a, { recursive: true, force: true });
rmSync(b, { recursive: true, force: true });
