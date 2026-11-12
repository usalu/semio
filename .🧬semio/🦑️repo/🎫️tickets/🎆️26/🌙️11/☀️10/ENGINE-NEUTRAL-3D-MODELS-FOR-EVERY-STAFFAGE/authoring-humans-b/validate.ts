/** ✅️ Ajv schema conformance of the asset plus the owned `validateAsset` volume rules (`volume-*`), and the count of all other findings. Usage: bun validate.ts <slug>... */
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { validateAsset } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { resolveAsset, STAFFAGE } from "../volume-lib.ts";

const schema = JSON.parse(readFileSync(`${STAFFAGE}/🧬️schema/🔣️.json`, "utf8"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const check = ajv.compile({ $ref: `${schema.$id}#/$defs/Asset` });
let bad = 0;
for (const slug of process.argv.slice(2)) {
  const asset = JSON.parse(readFileSync(resolveAsset(slug).path, "utf8")).asset;
  const ok = check(asset);
  const issues = validateAsset(asset);
  const vol = issues.filter((i: any) => String(i.code).startsWith("volume-"));
  const other = issues.filter((i: any) => !String(i.code).startsWith("volume-"));
  console.log(`${slug}: schema ${ok ? "ok" : "FAIL " + JSON.stringify((check.errors ?? []).slice(0, 5))}, volume issues ${vol.length}, other validateAsset findings ${other.length}${other.length ? " (" + [...new Set(other.map((i: any) => i.code))].join(",") + ")" : ""}`);
  for (const i of vol.slice(0, 20)) console.log("   ", i.code, i.path, i.message?.en?.slice(0, 140));
  if (!ok || vol.length) bad++;
}
process.exit(bad ? 1 : 0);
