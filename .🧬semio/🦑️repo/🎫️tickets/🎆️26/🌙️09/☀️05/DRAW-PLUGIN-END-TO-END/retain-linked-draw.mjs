import { createHash } from "node:crypto";
import { constants } from "node:fs";
import { mkdir, open, rename, stat } from "node:fs/promises";
import { dirname, join } from "node:path";

const [root, log, destination] = process.argv.slice(2);
const lanes = join(root, ".cache", "b", "r", "m", "u", "l");
const componentName = "semio_hub_draw_component.wasm";
const pattern = new Bun.Glob("*/t-*/wasm32-wasip2/prototype/" + componentName);
const started = (await stat(log)).birthtimeMs;
const deadline = Date.now() + 6 * 60 * 60 * 1000;
const signature = Buffer.from("0061736d0d000100", "hex");

async function findLink() {
  const candidates = [];
  for await (const file of pattern.scan({ cwd: lanes })) {
    const path = join(lanes, file);
    const status = await stat(path).catch(() => undefined);
    if (status?.isFile() && status.mtimeMs >= started && status.size > signature.length) {
      candidates.push({ path, modified: status.mtimeMs });
    }
  }
  return candidates.sort((left, right) => right.modified - left.modified)[0]?.path;
}

async function retain(source) {
  const artifact = await open(source, constants.O_RDONLY | (constants.O_NOFOLLOW ?? 0));
  try {
    const before = await artifact.stat();
    const header = Buffer.alloc(signature.length);
    await artifact.read(header, 0, header.length, 0);
    if (!header.equals(signature)) throw new Error("Cargo output lacks the WASI component signature");

    await mkdir(dirname(destination), { recursive: true });
    const temporary = destination + "." + process.pid + ".part";
    const copy = await open(temporary, "wx");
    const hash = createHash("sha256");
    let bytes = 0;
    try {
      const buffer = Buffer.alloc(1024 * 1024);
      while (bytes < before.size) {
        const { bytesRead } = await artifact.read(buffer, 0, Math.min(buffer.length, before.size - bytes), bytes);
        if (bytesRead === 0) throw new Error("Cargo output ended during the authenticated copy");
        for (let written = 0; written < bytesRead;) {
          const { bytesWritten } = await copy.write(buffer, written, bytesRead - written, bytes + written);
          if (bytesWritten === 0) throw new Error("Could not advance the retained component file");
          written += bytesWritten;
        }
        hash.update(buffer.subarray(0, bytesRead));
        bytes += bytesRead;
      }
    } finally {
      await copy.close();
    }
    const after = await artifact.stat();
    if (bytes !== before.size || before.size !== after.size || before.mtimeMs !== after.mtimeMs) {
      throw new Error("Cargo output moved during the authenticated copy");
    }
    await rename(temporary, destination);
    console.log(JSON.stringify({ source, destination, bytes, sha256: hash.digest("hex") }));
  } finally {
    await artifact.close();
  }
}

let logOffset = 0;
let seenFinish = false;
let previous = "";
while (Date.now() < deadline) {
  const file = Bun.file(log);
  const end = file.size;
  if (end > logOffset) {
    const text = previous + await file.slice(logOffset, end).text();
    seenFinish ||= /Finished `prototype` profile/.test(text);
    if (!seenFinish && /repository:command failed with exit code/.test(text)) {
      throw new Error("Cargo ended before linking Draw");
    }
    previous = text.slice(-100);
    logOffset = end;
  }
  if (seenFinish) {
    const source = await findLink();
    if (source) {
      await retain(source);
      process.exit(0);
    }
  }
  await Bun.sleep(2000);
}
throw new Error("Timed out waiting for Draw Cargo linking");
