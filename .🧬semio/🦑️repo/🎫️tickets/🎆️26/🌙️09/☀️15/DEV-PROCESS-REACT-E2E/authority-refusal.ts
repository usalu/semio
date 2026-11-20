import assert from "node:assert/strict";
import { statSync } from "node:fs";
import { dirname, resolve } from "node:path";
import Ajv from "ajv";

const [workerFile, transportSchemaFile, invalidAuthority] = process.argv.slice(2).map(path => resolve(path));
const transportSchema = await Bun.file(transportSchemaFile).json();
const authoritySchema = await Bun.file(resolve(dirname(workerFile), "../../../../../../../../🏗️build/🧬️schema/📥️authority.json")).json();
const ajv = new Ajv({ strict: false });
const authority = await Bun.file(invalidAuthority).json();
assert.equal(ajv.validate(authoritySchema, authority), false);
assert.deepEqual(ajv.errors?.map(error => error.instancePath), ["/graph"]);

const file = statSync(invalidAuthority, { bigint: true });
const stamp = [file.dev, file.ino, file.mode, file.nlink, file.size, file.mtimeNs, file.ctimeNs].join(":");
const credit = { version: 1, maximumReplyBytes: 8192 };
const request = {
  files: [{ path: invalidAuthority, kind: "authority", stamp }],
  imageRoot: dirname(invalidAuthority),
  root: process.cwd(),
  remainingBytes: Number(file.size),
};
const worker = Bun.spawn({
  cmd: [process.execPath, "-e", `import { runCapturedBuildSourceV1 } from ${JSON.stringify(workerFile)}; await runCapturedBuildSourceV1();`],
  stdin: "pipe",
  stdout: "pipe",
  stderr: "inherit",
});
worker.stdin.write([credit, request, null].map(frame => JSON.stringify(frame)).join("\n") + "\n");
await worker.stdin.end();
const [output, exit] = await Promise.all([new Response(worker.stdout).text(), worker.exited]);
assert.equal(exit, 1);
const failure = JSON.parse(output);
assert.equal(ajv.validate({ definitions: transportSchema.definitions, $ref: "#/definitions/ReadFailure" }, failure), true);
assert.equal(failure.failure.message, "Closed authenticated closure graph authority required");
console.log("[DEBUG] Closure worker preserves the graph authentication refusal");
