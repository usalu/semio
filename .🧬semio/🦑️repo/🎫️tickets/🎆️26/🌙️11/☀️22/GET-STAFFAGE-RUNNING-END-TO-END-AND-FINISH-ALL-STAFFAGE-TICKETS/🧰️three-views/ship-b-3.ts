/** 🚢️ Ships the three-views ops of a person of HW-10 … HW-18: appends them to the repair ops of the person (`HW-1x/repair/<slug>.json`, the last step of its generator chain) and checks that the whole list, applied to the untouched copy, gives the proven candidate. The catalog file is then written by the chain's own `REP-B/apply.ts <slug>`.
 *
 * Usage: `bun ship-b-3.ts <slug> <ops json> [<ops json> …] --before <untouched copy> --cand <proven candidate>`
 * @see ../../STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/REP-B/ops.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { applyOps, readOps, writeOps, type Op } from "../../STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/REP-B/ops.ts";
import { catalogFile, opsFile } from "../../STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/REP-B/people.ts";

const args = process.argv.slice(2);
const option = (name: string): string => args[args.indexOf(name) + 1]!;
const slug = args[0]!;
const files = args.slice(1, args.indexOf("--before"));
const added = files.flatMap((file) => JSON.parse(readFileSync(file, "utf8")) as Op[]);
if (readFileSync(catalogFile(slug), "utf8") !== readFileSync(option("--before"), "utf8")) throw new Error(`${slug}: the catalog file changed since the untouched copy was taken`);
const existing = await readOps(opsFile(slug));
const all = [...existing, ...added];
const document = JSON.parse(readFileSync(option("--before"), "utf8"));
applyOps(document.asset, all);
const candidate = JSON.parse(readFileSync(option("--cand"), "utf8"));
if (JSON.stringify(document) !== JSON.stringify(candidate)) throw new Error(`${slug}: the whole ops list on the untouched copy does not give the candidate`);
await writeOps(opsFile(slug), all);
writeFileSync(`${import.meta.dir}/ops-3/${slug}.json`, `[\n${added.map((op) => JSON.stringify(op)).join(",\n")}\n]\n`);
console.log(`${slug}: ${added.length} ops appended to ${existing.length}`);
