/** ✅️ Ajv conformance of the whole asset plus the owned `validateAsset` volume rules for machinery assets.
 * Usage: bun authoring-machinery-validate.ts <slug>... ; writes 🗑️generated/authoring-machinery/<slug>.validate.json */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { GENERATED, STAFFAGE, loadAsset, resolveAsset } from "./volume-lib.ts";

const schema = JSON.parse(readFileSync(join(STAFFAGE, "🧬️schema", "🔣️.json"), "utf8"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const check = ajv.compile({ $ref: `${schema.$id}#/$defs/Asset` });
let bad = 0;
for (const slug of process.argv.slice(2)) {
  const asset = loadAsset(resolveAsset(slug).path);
  const ok = check(asset);
  const schemaErrors = ok ? [] : (check.errors ?? []).map((e) => `${e.instancePath} ${e.message}`);
  const volumeSchemaErrors = schemaErrors.filter((e) => e.startsWith("/volume"));
  const issues = validateAsset(asset) as readonly { path: string; code: string }[];
  const volumeIssues = issues.filter((i) => i.code.startsWith("volume-") || i.path.startsWith("volume"));
  const others = issues.filter((i) => !volumeIssues.includes(i));
  const result = { slug, hasVolume: asset.volume !== undefined, solids: asset.volume?.solids?.length ?? 0, ajvOk: ok, volumeSchemaErrors, otherSchemaErrors: schemaErrors.length - volumeSchemaErrors.length, volumeIssues, otherIssueCodes: [...new Set(others.map((i) => i.code))] };
  writeFileSync(join(GENERATED, "authoring-machinery", `${slug}.validate.json`), JSON.stringify(result, null, 1));
  if (!result.hasVolume || volumeSchemaErrors.length > 0 || volumeIssues.length > 0) bad++;
  console.log(`${slug}: solids ${result.solids} volumeSchemaErrors ${volumeSchemaErrors.length} volumeIssues ${volumeIssues.length} (other ajv ${result.otherSchemaErrors}, other issues ${others.length} ${JSON.stringify(result.otherIssueCodes)})${volumeIssues.length ? " " + JSON.stringify(volumeIssues) : ""}${volumeSchemaErrors.length ? " " + JSON.stringify(volumeSchemaErrors.slice(0, 5)) : ""}`);
}
process.exit(bad ? 1 : 0);
