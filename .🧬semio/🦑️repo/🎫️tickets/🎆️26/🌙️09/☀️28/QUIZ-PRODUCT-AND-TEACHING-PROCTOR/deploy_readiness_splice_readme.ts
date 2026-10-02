/** ✂️ Deploy-readiness helper: replaces the `## Deploy` section of the site README (everything from that heading to the
 * end of the file) with the section authored in `argv[2]`, leaving every other section as its owner wrote it. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const readme = join(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/❓️quiz/README.md");
const text = readFileSync(readme, "utf8");
const section = readFileSync(process.argv[2]!, "utf8");
const marker = "\n## Deploy\n";
if (text.split(marker).length !== 2) throw new Error("the README does not have exactly one Deploy section");
writeFileSync(readme, text.slice(0, text.indexOf(marker) + 1) + section);
console.log(`[DEBUG] ${readme}: ${text.length} -> ${text.indexOf(marker) + 1 + section.length} characters`);
