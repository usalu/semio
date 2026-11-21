import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { constants, copyFileSync, createReadStream, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const [log, out] = process.argv.slice(2);
assert.ok(log && out);
const rows = readFileSync(log, "utf8").split("\n");
const event = rows.findLast((line) =>
  line.startsWith('{"reason":"compiler-artifact"') &&
  line.includes("#semio-hub-stdio-office-component@") &&
  line.includes('"kind":["cdylib"]'));
if (!event) process.exit(3);

const artifact = JSON.parse(event);
const source = artifact.filenames.find((path: string) => path.endsWith("semio_hub_stdio_office_component.wasm"));
assert.ok(source && source.includes("wasm32-wasip2") && statSync(source).isFile());
const component = join(out, "stdio-office.wasm");
copyFileSync(source, component, constants.COPYFILE_EXCL | constants.COPYFILE_FICLONE);
const hash = createHash("sha256");
for await (const chunk of createReadStream(component)) hash.update(chunk);
const result = { component, byteLength: statSync(component).size, sha256: hash.digest("hex"), cargo: artifact };
writeFileSync(join(out, "stdio-office-cargo-artifact.json"), JSON.stringify(result, null, 2) + "\n", { flag: "wx" });
console.log(JSON.stringify({ component, byteLength: result.byteLength, sha256: result.sha256 }));
