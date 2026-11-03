import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { exportCast } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";
const STAFFAGE = resolve(import.meta.dir, "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const manifest = JSON.parse(readFileSync(join(STAFFAGE, "🧫️fixtures/🎭️cast-export/🔣️.json"), "utf8")).manifest;
await exportCast(manifest, process.argv.slice(3), process.argv[2]!, { language: "en" });
