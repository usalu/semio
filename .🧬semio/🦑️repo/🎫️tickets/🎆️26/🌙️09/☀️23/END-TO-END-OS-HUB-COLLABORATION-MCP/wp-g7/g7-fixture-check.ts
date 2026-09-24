#!/usr/bin/env bun
/** 🧪️ G7 third-party check: AJV (draft-07) validates the new/changed law fixtures against their schemas, cross-file $refs included. */
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { join, dirname, resolve } from "node:path";

const MCP = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp";
const schemas = ["🏠️workspace/🧬️schema/🔣️.json", "💡️inference/🧬️schema/🔣️.json", "🗿️artifact/🧬️schema/🔣️.json"];
const ajv = new Ajv({ strict: false, allErrors: true });
const byFile = new Map<string, string>();
for (const path of schemas) {
  const document = JSON.parse(readFileSync(join(MCP, path), "utf8"));
  ajv.addSchema(document);
  byFile.set(join(MCP, path), document.$id);
}
const fixtures = ["🏠️workspace/🧫️fixtures/⏱️compile-cancellation-law.json", "💡️inference/🧫️fixtures/⏱️binding-cancellation-law.json", "🗿️artifact/🧫️fixtures/⏱️create-cancellation-law.json"];
let failed = 0;
for (const path of fixtures) {
  const full = join(MCP, path);
  const fixture = JSON.parse(readFileSync(full, "utf8"));
  const [file, pointer] = fixture.$schema.split("#");
  const ref = `${byFile.get(resolve(dirname(full), file))}#${pointer}`;
  const validate = ajv.getSchema(ref);
  if (!validate) throw new Error(`no schema at ${ref}`);
  const ok = validate(fixture);
  console.log(`${ok ? "PASS" : "FAIL"} ${path} ${ok ? "" : JSON.stringify(validate.errors)}`);
  if (!ok) failed++;
  const bad = { ...fixture, cases: [{ ...fixture.cases[0], expected: "NOT-A-STATUS" }] };
  const rejected = !validate(bad);
  console.log(`${rejected ? "PASS" : "FAIL"} ${path} rejects an unknown expected status`);
  if (!rejected) failed++;
}
process.exit(failed ? 1 : 0);
