import { readFile } from "node:fs/promises";
import { exportCast } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";
const [manifest, id, out] = process.argv.slice(2);
try { await exportCast(JSON.parse(await readFile(manifest!, "utf8")), [id!], out!, { language: "en" }); } catch (e) { console.log((e as Error).stack); }
