import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { Glob } from "bun";
import Ajv from "ajv";

const repo = resolve(import.meta.dir, "../../../../../../..");
const owner = resolve(repo, "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any");
const schema = resolve(owner, "🧬️schema");
const [{ parseDin18599Snapshot }, { parseDin18599Diff }, { parseDin18599Mutation }] = await Promise.all([
  import(pathToFileURL(resolve(schema, "📸️snapshot/🟦️.ts")).href),
  import(pathToFileURL(resolve(schema, "🔺️diff/🟦️.ts")).href),
  import(pathToFileURL(resolve(schema, "🧬️mutations/🟦️.ts")).href),
]);
const snapshotSchema = await Bun.file(resolve(schema, "📸️snapshot/🔣️.json")).json();
const diffSchema = await Bun.file(resolve(schema, "🔺️diff/🔣️.json")).json();
const ajv = new Ajv({ strict: false });
ajv.addSchema(snapshotSchema);
const validateDiff = ajv.compile(diffSchema);
const snapshots = new Glob("**/*.json");
const fixtureRoot = resolve(owner, "🧪️tests/🧫️fixtures/🧬️mutations");
let snapshotsTested = 0;
let diffsTested = 0;
let mutationsTested = 0;
let expectedWireRefusals = 0;

for (const file of snapshots.scanSync(fixtureRoot)) {
  const payload = await Bun.file(resolve(fixtureRoot, file)).json();
  if (file.endsWith(".before.json") || file.endsWith(".after.json")) {
    parseDin18599Snapshot(payload);
    snapshotsTested++;
  } else if (file.endsWith(".diff.json")) {
    parseDin18599Diff(payload);
    if (!validateDiff(payload)) throw Error(file + " " + JSON.stringify(validateDiff.errors));
    diffsTested++;
  } else if (file.endsWith(".mutation.json")) {
    try {
      parseDin18599Mutation(payload);
      mutationsTested++;
    } catch (error) {
      if (!file.includes("/🚫rule/")) throw Error(file, { cause: error });
      expectedWireRefusals++;
    }
  }
}
console.log("[DEBUG] DIN V 18599 strict wire fixture checks", { snapshotsTested, diffsTested, mutationsTested, expectedWireRefusals });
