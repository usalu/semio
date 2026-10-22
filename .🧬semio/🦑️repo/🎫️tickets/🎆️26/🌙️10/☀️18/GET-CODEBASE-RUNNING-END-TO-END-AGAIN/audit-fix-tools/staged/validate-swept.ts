import Ajv from "ajv";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

/** Validates every probed manifest (swept copy when the sweep changed it, live file otherwise) against the patched project schema. usage: bun validate-swept.ts <patched-schema> <probe-dir> <sweep-overlay> */
const REPO = process.env.AUDIT_FIX_REPO ?? "/Users/ueli/Documents/semio";
const [schemaPath, probeDirectory, sweptDirectory] = process.argv.slice(2);
if (!schemaPath || !probeDirectory || !sweptDirectory) throw new Error("refused: schema, probe directory and sweep overlay required");
const ajv = new Ajv({ strict: false, allErrors: false });
const validate = ajv.compile(JSON.parse(readFileSync(schemaPath, "utf8")));
const paths = readdirSync(probeDirectory).filter(name => /^old-\d+\.json$/u.test(name)).flatMap(name => (JSON.parse(readFileSync(join(probeDirectory, name), "utf8")) as { rows: { path: string }[] }).rows.map(row => row.path));
if (paths.length === 0) throw new Error("refused: empty probe table");
let invalid = 0;
for (const path of paths) {
  const swept = join(sweptDirectory, path), file = existsSync(swept) ? swept : join(REPO, path);
  if (!validate(JSON.parse(readFileSync(file, "utf8")))) { invalid++; if (invalid <= 15) console.log("invalid", path, JSON.stringify(validate.errors?.[0]).slice(0, 220)); }
}
console.log(JSON.stringify({ manifests: paths.length, invalid }));
if (invalid > 0) process.exitCode = 1;
