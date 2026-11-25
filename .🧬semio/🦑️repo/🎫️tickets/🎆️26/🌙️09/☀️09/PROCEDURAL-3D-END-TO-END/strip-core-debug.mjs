import { createHash } from "node:crypto";
import { createReadStream, createWriteStream } from "node:fs";
import { copyFile, mkdir, open, readdir, stat } from "node:fs/promises";
import { basename, join } from "node:path";
import { Readable } from "node:stream";
import { pipeline } from "node:stream/promises";

const [source, output] = process.argv.slice(2);
if (!source || !output || source === output) throw new Error("strip-core-debug requires distinct input and output module directories");
await mkdir(output, { recursive: true });
const base = basename(source);
const names = await readdir(source);
await copyFile(join(source, base + ".js"), join(output, base + ".js"));

const hash = async (path) => {
  const digest = createHash("sha256");
  for await (const chunk of createReadStream(path)) digest.update(chunk);
  return digest.digest("hex");
};

for (const name of names.filter((name) => new RegExp(`^${base}\\.core[0-9]*\\.wasm$`).test(name))) {
  const input = join(source, name);
  const destination = join(output, name);
  const file = await open(input, "r");
  const { size } = await file.stat();
  const removed = [];
  let position = 0;
  const read = async (length) => {
    if (position + length > size) throw new Error("Truncated Wasm section");
    const bytes = Buffer.alloc(length);
    const { bytesRead } = await file.read(bytes, 0, length, position);
    if (bytesRead !== length) throw new Error("Incomplete Wasm section read");
    position += length;
    return bytes;
  };
  const leb = async () => {
    let value = 0;
    for (let stride = 0; stride < 35; stride += 7) {
      const byte = (await read(1))[0];
      value += (byte & 127) * 2 ** stride;
      if (value > 0xffffffff) throw new Error("Invalid Wasm u32 length");
      if (byte < 128) return value;
    }
    throw new Error("Invalid Wasm unsigned length boundary");
  };
  const copy = async function* (start, end) {
    for (let offset = start; offset < end;) {
      const bytes = Buffer.allocUnsafe(Math.min(1_048_576, end - offset));
      const { bytesRead } = await file.read(bytes, 0, bytes.length, offset);
      if (bytesRead === 0) throw new Error("Incomplete Wasm section read");
      yield bytes.subarray(0, bytesRead);
      offset += bytesRead;
    }
  };
  const retained = async function* () {
    const header = await read(8);
    if (header.toString("hex") !== "0061736d01000000") throw new Error("Expected a WebAssembly core module");
    yield header;
    while (position < size) {
      const start = position;
      const id = (await read(1))[0];
      const length = await leb();
      const end = position + length;
      if (end > size) throw new Error("Invalid Wasm section extent");
      const labelLength = id === 0 ? await leb() : 0;
      if (position + labelLength > end) throw new Error("Invalid Wasm custom section label");
      const label = id === 0 && labelLength <= 128 ? (await read(labelLength)).toString("utf8") : "";
      if (label === "name" || /^\.(?:z?debug)_/.test(label)) removed.push({ label, bytes: end - start });
      else yield* copy(start, end);
      position = end;
    }
  };
  try {
    await pipeline(Readable.from(retained(), { objectMode: false }), createWriteStream(destination, { flags: "wx" }));
  } finally {
    await file.close();
  }
  console.log("[DEBUG]", JSON.stringify({ name, sourceBytes: size, sourceSha256: await hash(input), servedBytes: (await stat(destination)).size, servedSha256: await hash(destination), removed }));
}
