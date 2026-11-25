/** 🧪️ Runs the matrix cells of ONE asset exactly as the asset process of `📊️matrix.ts` does (`runGuarded` around `runAsset`), from a frozen snapshot of the asset document, so a before and an after of the shared code compare on the same input while fixers edit the catalog.
 *
 * `bun one.ts --snapshot <asset 🔣️.json> --out <cells.json> [--suite <dir>] [--probe] [--gc]`
 *
 * - `--out`: the cells as JSON (without the seconds), the subject of the identity proof.
 * - `--suite`: the folder of the suite files to run (default: the live `🔭️multiview-coverage`); a scratch tree of the files before the repair gives the before of the same day. The catalog is given to `runAsset` the way that suite's generator gives it: the lazy lookup when the engine has one, else a map of the whole loaded catalog.
 * - `--probe`: one `[DEBUG] ` line per scale entry with resident set, heap and elapsed seconds.
 * - `--gc`: with `--probe`, a full collection before each line and the ten most frequent object types, which separates retained memory from garbage.
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/📊️matrix.ts — the generator
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { heapStats } from "bun:jsc";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const root = resolve(import.meta.dir, "../../../../../../../../..");
const suite = flag("suite") === undefined ? join(root, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🧪️tests", "🔭️multiview-coverage") : resolve(flag("suite")!);
const { catalogLookup, loadCatalog, runAsset } = (await import(join(suite, "🧰️.ts"))) as typeof import("../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts");
const { runGuarded } = (await import(join(suite, "🗃️cells.ts"))) as typeof import("../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🗃️cells.ts");

const asset = JSON.parse(readFileSync(resolve(flag("snapshot")!), "utf8")).asset;
const catalogById = catalogLookup === undefined ? new Map(loadCatalog().map((entry) => [entry.id, entry])) : catalogLookup();
const started = Date.now();
const mb = (bytes: number): string => (bytes / 1048576).toFixed(0);
const line = (label: string): void => {
  if (!args.includes("--probe")) return;
  if (args.includes("--gc")) Bun.gc(true);
  const usage = process.memoryUsage();
  const types = args.includes("--gc") ? Object.entries(heapStats().objectTypeCounts).sort((a, b) => b[1] - a[1]).slice(0, 10).map(([type, count]) => `${type} ${count}`).join(", ") : "";
  console.error(`[DEBUG] ${label} t ${((Date.now() - started) / 1000).toFixed(1)} s rss ${mb(usage.rss)} MB heapUsed ${mb(usage.heapUsed)} MB heapTotal ${mb(usage.heapTotal)} MB external ${mb(usage.external)} MB arrayBuffers ${mb(usage.arrayBuffers)} MB ${types}`);
};
const record = runGuarded(asset, (target) => runAsset(target, catalogById, (index, count, scale) => line(`entry ${index + 1}/${count} ${scale}`)));
line("end");
writeFileSync(resolve(flag("out")!), JSON.stringify(record.outcome === "done" ? record.cells : record));
console.error(`[DEBUG] ${asset.id} ${record.outcome} ${record.seconds.toFixed(1)} s`);
