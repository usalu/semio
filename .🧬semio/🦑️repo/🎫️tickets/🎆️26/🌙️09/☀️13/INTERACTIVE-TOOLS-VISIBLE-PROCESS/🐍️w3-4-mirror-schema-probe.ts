/** 🧾️ W3-4 mirrors: validates every committed remodeling snapshot, diff and new-kind mutation payload against the
 *  handcrafted JSON Schema mirrors with the third-party `ajv` validator. */
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";

const any = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any";
const read = (path: string): Record<string, unknown> => JSON.parse(readFileSync(path, "utf8"));
const walk = (dir: string): string[] => readdirSync(dir).flatMap((name) => (statSync(join(dir, name)).isDirectory() ? walk(join(dir, name)) : [join(dir, name)]));
const ajv = new Ajv({ strict: false, validateFormats: false, allErrors: false });
const root = read(`${any}/🧬️schema/🔣️.json`);
ajv.addSchema(root);
const snapshot = ajv.compile(read(`${any}/🧬️schema/📸️snapshot/🔣️.json`));
const artifact = ajv.getSchema(root.$id as string)!;
const diff = ajv.compile(read(`${any}/🧬️schema/🔺️diff/🔣️.json`));
const files = walk(`${any}/🧫️fixtures`);
const report: string[] = [];
let failures = 0;

function check(label: string, validate: ReturnType<typeof ajv.compile>, paths: string[], project: (document: Record<string, unknown>) => unknown = (document) => document): void {
  let invalid = 0;
  for (const path of paths) {
    if (validate(project(read(path)))) continue;
    invalid += 1;
    report.push(`  INVALID ${path.split("🧫️fixtures/")[1]}: ${JSON.stringify(validate.errors?.[0])}`);
  }
  failures += invalid;
  report.push(`${label}: validated=${paths.length} invalid=${invalid}`);
}

const snapshots = files.filter((path) => path.includes("/📸️snapshot/") || /🏁️commit-reconstruction\/(⬅️before|➡️after)\.json$/.test(path));
check("snapshot documents vs 📸️snapshot/🔣️.json", snapshot, snapshots);
check("snapshot documents vs artifact 🔣️.json", artifact, snapshots);
check("diff documents vs 🔺️diff/🔣️.json", diff, files.filter((path) => path.endsWith("/🔺️diff/🔣️.json")));
for (const kind of ["📦append-content", "🔪remove-content", "🏁commit-reconstruction"]) {
  const payload = ajv.compile(read(`${any}/🧬️schema/🧬️mutations/${kind}/🧬️schema/🔣️.json`));
  const paths = files.filter((path) => path.includes(`/🧬️mutations/${kind}/`) && path.endsWith("/🦠️mutation/🔣️.json"));
  if (kind === "🏁commit-reconstruction") paths.push(`${any}/🧫️fixtures/🏁️commit-reconstruction/🦠️mutation.json`);
  check(`${kind} payloads vs its 🧬️schema/🔣️.json`, payload, paths, ({ mutation: _tag, ...rest }) => rest);
}
process.stdout.write(`${report.join("\n")}\n`);
process.exit(failures === 0 ? 0 : 1);
