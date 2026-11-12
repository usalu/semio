/** Ajv schema conformance and owned validateAsset of catalog assets. Usage: bun validate.ts <slug>... */
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { validateAsset } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { volumeIssues } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volume/🟦️.ts";
import { STAFFAGE, loadAsset, resolveAsset } from "../volume-lib.ts";

const schema = JSON.parse(readFileSync(resolve(STAFFAGE, "🧬️schema/🔣️.json"), "utf8"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const check = ajv.compile({ $ref: `${schema.$id}#/$defs/Asset` });
let bad = 0;
for (const slug of process.argv.slice(2)) {
  const asset = loadAsset(resolveAsset(slug).path);
  const ok = check(asset);
  const schemaErrors = ok ? [] : (check.errors ?? []).map((e) => `${e.instancePath} ${e.message}`);
  const vol = volumeIssues(asset);
  const all = validateAsset(asset);
  const other = all.filter((i) => !i.code.startsWith("volume-"));
  console.log(`${slug}: ajv ${ok ? "ok" : "FAIL " + schemaErrors.slice(0, 5).join("; ")} | volume rules ${vol.length} | other validateAsset ${other.length}`);
  for (const i of vol.slice(0, 20)) console.log("  ", i.code, i.path, (i.message as any).en ?? i.message);
  for (const i of other.slice(0, 5)) console.log("   other:", i.code, i.path);
  if (!ok || vol.length > 0) bad++;
}
process.exit(bad ? 1 : 0);
