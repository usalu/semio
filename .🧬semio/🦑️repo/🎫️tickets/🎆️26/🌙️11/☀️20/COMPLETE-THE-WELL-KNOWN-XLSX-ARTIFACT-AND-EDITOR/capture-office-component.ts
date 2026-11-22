import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { closeSync, constants, copyFileSync, createReadStream, existsSync, mkdirSync, openSync, readSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const [log, out, terminal] = process.argv.slice(2);
assert.ok(log && out);
const isOfficeComponent = (line: string) =>
  line.startsWith('{"reason":"compiler-artifact"') &&
  line.includes("#semio-hub-stdio-office-component@") &&
  line.includes('"kind":["cdylib"]');

const handle = openSync(log, "r");
const block = Buffer.alloc(128 * 1024);
const decoder = new TextDecoder();
let offset = 0;
let frame = "";
let event: string | undefined;
try {
  while (!event) {
    const count = readSync(handle, block, 0, block.byteLength, offset);
    if (count === 0) {
      if (!terminal || existsSync(terminal)) {
        event = isOfficeComponent(frame) ? frame : undefined;
        break;
      }
      await Bun.sleep(1000);
      continue;
    }
    offset += count;
    const lines = (frame + decoder.decode(block.subarray(0, count), { stream: true })).split("\n");
    frame = lines.pop()!;
    event = lines.find(isOfficeComponent);
  }
} finally {
  closeSync(handle);
}
if (!event) process.exit(3);
mkdirSync(out, { recursive: true });

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
