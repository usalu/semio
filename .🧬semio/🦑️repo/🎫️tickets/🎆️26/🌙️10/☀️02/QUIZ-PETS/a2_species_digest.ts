/** 🗒️ Read-only digest of the architecture species for work package A2: size, gait, clips and repertoire of every species document, the minimal second-round members derived from them, and the lines of the file an anchored edit can hold on to. */
import { readFileSync, readdirSync } from "node:fs";
import { join, resolve } from "node:path";

const ROOT = resolve(import.meta.dir, "../../../../../../..");
const PETS = join(ROOT, "🎓️teaching", "🏛️architecture", "🐾️pets");

for (const entry of readdirSync(PETS, { withFileTypes: true })) {
  if (!entry.isDirectory()) continue;
  const path = join(PETS, entry.name, "🔣️.json");
  let text: string;
  try {
    text = readFileSync(path, "utf8");
  } catch {
    continue;
  }
  const species = JSON.parse(text);
  const lines = text.split("\n");
  const find = (needle: string) => lines.findIndex((line) => line.includes(needle)) + 1;
  console.log(`== ${entry.name} id=${species.id} size=${species.size.width}x${species.size.height} gait=${species.locomotion.gait} hover=${species.locomotion.hover ?? "-"} bytes=${text.length} lines=${lines.length} crlf=${text.includes("\r\n")}`);
  console.log(`   keys: ${Object.keys(species).join(" ")}`);
  console.log(`   clips: ${species.clips.map((clip: { id: string; loop: boolean; seconds: number }) => `${clip.id}${clip.loop ? "~" : ""}`).join(" ")}`);
  console.log(`   repertoire: ${JSON.stringify(species.repertoire)}`);
  console.log(`   repertoire line ${find('"repertoire"')}: ${JSON.stringify(lines[find('"repertoire"') - 1]?.slice(0, 80))}`);
  console.log(`   locomotion line ${find('"locomotion"')}: ${JSON.stringify(lines[find('"locomotion"') - 1]?.slice(0, 120))}`);
  console.log(`   temperament line ${find('"temperament"')}: ${JSON.stringify(lines[find('"temperament"') - 1]?.slice(0, 120))}`);
  console.log(`   tail: ${JSON.stringify(lines.slice(-4).join("\n").slice(-200))}`);
}
