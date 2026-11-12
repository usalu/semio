/** 🔬️ Ajv conformance and owned `validateAsset` volume rules of catalog assets: `bun validate.ts <slug>...` */
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { STAFFAGE, loadAsset, resolveAsset } from "../volume-lib.ts";

const { validateAsset } = await import(`${STAFFAGE}/🔨️modules/🗂️catalog/🟦️.ts`);
const { volumeIssues } = await import(`${STAFFAGE}/🔨️modules/🗂️catalog/🧊️volume/🟦️.ts`);
const schema = JSON.parse(readFileSync(`${STAFFAGE}/🧬️schema/🔣️.json`, "utf8"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const check = ajv.compile({ $ref: `${schema.$id}#/$defs/Asset` });
let bad = 0;
for (const slug of process.argv.slice(2)) {
  const asset = loadAsset(resolveAsset(slug).path);
  const ok = check(asset);
  const own = volumeIssues(asset);
  const all = validateAsset(asset);
  const vol = all.filter((i: any) => String(i.code).startsWith("volume-"));
  const ajvErrors = ok ? [] : (check.errors ?? []).filter((e: any) => String(e.instancePath).startsWith("/volume")).slice(0, 5);
  console.log(`${slug}: ajv ${ok ? "ok" : "asset-level errors " + (check.errors?.length ?? 0)} volumeAjv ${ajvErrors.length} volumeIssues ${own.length} validateAsset-volume ${vol.length} (all issues ${all.length})`);
  for (const e of ajvErrors) console.log("  AJV", JSON.stringify(e));
  for (const i of [...own, ...vol].slice(0, 10)) console.log("  ISSUE", JSON.stringify(i));
  if (ajvErrors.length || own.length || vol.length) bad++;
  if (!ok && ajvErrors.length === 0) console.log("  (asset-level ajv errors outside volume:", JSON.stringify((check.errors ?? []).slice(0, 2)), ")");
}
process.exit(bad === 0 ? 0 : 1);
