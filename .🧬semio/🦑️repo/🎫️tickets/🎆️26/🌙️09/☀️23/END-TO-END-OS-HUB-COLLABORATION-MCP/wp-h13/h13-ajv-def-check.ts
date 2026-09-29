/** ✅️ H13: validates a staged fixture against one `$defs` export of a staged schema with strict Ajv (draft-07).
 * usage: bun h13-ajv-def-check.ts <schema> <def> <fixture> */
import { readFileSync } from "node:fs";
import Ajv from "/Users/ueli/Documents/semio/node_modules/ajv/dist/ajv.js";

const schema = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const ajv = new (Ajv as any)({ strict: true, allErrors: true });
ajv.addSchema(schema);
const validate = ajv.getSchema(`${schema.$id}#/$defs/${process.argv[3]}`)!;
const valid = validate(JSON.parse(readFileSync(process.argv[4]!, "utf8")));
console.log(valid ? "VALID" : `INVALID ${JSON.stringify(validate.errors)}`);
process.exit(valid ? 0 : 1);
