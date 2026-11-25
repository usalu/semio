/** 🗓️ Plans the full matrix run as requests of one memory class each: `bun plan.ts --out <run folder> --plan <file> [--retry-failed] [--minutes <most minutes of one request>] [--assets a,b]`.
 *
 * Reads the catalog one document at a time, leaves out the assets that have a current record in the run folder (the same test as the generator: version, id and digest) and writes one line per request to the plan file: `<memory MB> <jobs> <per-asset limit s> <asset ids separated by commas>`. The assets of a class go largest document first and a request holds at most `--minutes` of estimated work, so a request that is refused or cut costs little and gives its memory back soon.
 * The class follows the category and the size of the document (trees by size, people and animals, rigid assets); its memory and seconds are the figures of the report.
 *
 * @see ../../../multiview-memory.md — the measurements behind the classes
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/📊️matrix.ts — the generator
 */
import { writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { headOf, pendingAssets, type AssetHead } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🗃️cells.ts";
import { mapCatalog } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const out = resolve(flag("out")!);
const minutes = Number(flag("minutes") ?? 30);
const only = flag("assets")?.split(",");

/** 📏️ The classes of the final run, heaviest first: which assets they take, the resident memory of one asset process (measured, see the report), processes at a time, the limit of one asset in seconds, and the estimated seconds per megabyte of document on the loaded host (people and trees measured since the landing of 17:48). */
type Class = { readonly name: string; readonly takes: (head: Sized) => boolean; readonly memory: number; readonly jobs: number; readonly limitS: number; readonly secondsPerMb: number };
const BIG_TREE = (head: Sized): boolean => head.category === "vegetation" && head.bytes >= 6e6;
const MID_TREE = (head: Sized): boolean => head.category === "vegetation" && head.bytes >= 1.5e6 && head.bytes < 6e6;
const SMALL_TREE = (head: Sized): boolean => head.category === "vegetation" && head.bytes < 1.5e6;
const LIVING = (head: Sized): boolean => head.category === "humans" || head.category === "animals";
const CLASSES: readonly Class[] = [
  { name: "big-tree", takes: BIG_TREE, memory: 2500, jobs: 1, limitS: 21600, secondsPerMb: 60 },
  { name: "mid-tree", takes: MID_TREE, memory: 1600, jobs: 2, limitS: 14400, secondsPerMb: 120 },
  { name: "small-tree", takes: SMALL_TREE, memory: 1000, jobs: 3, limitS: 43200, secondsPerMb: 300 },
  { name: "living", takes: LIVING, memory: 600, jobs: 3, limitS: 43200, secondsPerMb: 170 },
  { name: "rigid", takes: (head) => !LIVING(head) && head.category !== "vegetation", memory: 350, jobs: 4, limitS: 7200, secondsPerMb: 70 },
];

type Sized = AssetHead & { readonly bytes: number };
const heads = mapCatalog((asset): Sized => ({ ...headOf(asset), bytes: JSON.stringify(asset).length }), only);
const lines: string[] = [];
const pending = pendingAssets(out, heads, args.includes("--retry-failed"));
for (const found of CLASSES) {
  const mine = pending.filter(found.takes).sort((a, b) => b.bytes - a.bytes);
  let request: Sized[] = [];
  let seconds = 0;
  const print = (): void => {
    if (request.length > 0) lines.push(`${found.memory * Math.min(found.jobs, request.length) + 100} ${Math.min(found.jobs, request.length)} ${found.limitS} ${request.map((head) => head.id).join(",")}`);
    request = [];
    seconds = 0;
  };
  for (const head of mine) {
    const cost = ((head.bytes / 1e6) * found.secondsPerMb) / found.jobs;
    if (request.length >= found.jobs && seconds + cost > minutes * 60) print();
    request.push(head);
    seconds += cost;
  }
  print();
}
    request.push(head);
    seconds += cost;
  }
  print();
}
writeFileSync(resolve(flag("plan")!), lines.map((line) => `${line}\n`).join(""));
console.error(`[DEBUG] ${pending.length} of ${heads.length} assets to run, ${heads.length - pending.length} current`);
