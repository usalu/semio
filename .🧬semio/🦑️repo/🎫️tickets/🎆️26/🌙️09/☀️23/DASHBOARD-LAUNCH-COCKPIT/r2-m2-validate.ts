#!/usr/bin/env bun
/** M-2 helper: validate project manifests (`#/$defs/ProjectManifest`) and ticket commands (`#/$defs/TicketCommands`)
 * against the registry schema with Ajv. Usage (repository root): `bun r2-m2-validate.ts <file>…`; a file named
 * `🎮️commands.json` is a ticket commands file, every other file a project manifest. Exits 1 on any error. */
import Ajv2020 from "ajv/dist/2020.js";
import { readFileSync } from "node:fs";
import { basename } from "node:path";

const schema = JSON.parse(readFileSync("🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json", "utf8"));
const ajv = new (Ajv2020 as any)({ allErrors: true, strict: false });
ajv.addSchema(schema);
const manifest = ajv.getSchema(`${schema.$id}#/$defs/ProjectManifest`);
const tickets = ajv.getSchema(`${schema.$id}#/$defs/TicketCommands`);
let failed = false;
for (const file of Bun.argv.slice(2)) {
  const validate = basename(file) === "🎮️commands.json" ? tickets : manifest;
  const ok = validate(JSON.parse(readFileSync(file, "utf8")));
  if (!ok) failed = true;
  console.log(ok ? "valid  " : "INVALID", file, ok ? "" : JSON.stringify(validate.errors).slice(0, 600));
}
process.exit(failed ? 1 : 0);
