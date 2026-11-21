import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { isMainThread, parentPort, Worker, workerData } from "node:worker_threads";

function length(bytes: Uint8Array, cursor: { position: number }): number {
  let value = 0;
  for (let shift = 0; shift < 35; shift += 7) {
    const byte = bytes[cursor.position++];
    if (byte === undefined) throw Error("Incomplete WebAssembly section length");
    value += (byte & 127) * 2 ** shift;
    if (byte < 128) return value;
  }
  throw Error("Invalid WebAssembly section length");
}

function* sections(bytes: Uint8Array): Generator<{ id: number; data: Uint8Array }> {
  const cursor = { position: 8 };
  while (cursor.position < bytes.length) {
    const id = bytes[cursor.position++];
    const size = length(bytes, cursor);
    const end = cursor.position + size;
    if (end > bytes.length) throw Error("WebAssembly section exceeds its container");
    yield { id, data: bytes.subarray(cursor.position, end) };
    cursor.position = end;
  }
}

function audit(path: string, exportName: string) {
  const began = performance.now();
  const bytes = readFileSync(path);
  const digest = createHash("sha256").update(bytes).digest("hex");
  if (bytes.subarray(0, 8).toString("hex") !== "0061736d0d000100") throw Error("An exact WebAssembly component is required");
  const expected = Buffer.from(exportName);
  const marker = Buffer.from("semio_owned_channel_version_v1");
  const modules = [...sections(bytes)]
    .filter(({ id, data }) => id === 1 && [...sections(data)].some(({ id, data }) => {
      const entries = Buffer.from(data.buffer, data.byteOffset, data.length);
      return id === 7 && entries.includes(expected) && entries.includes(marker);
    }));
  if (modules.length !== 1) throw Error(`Expected one owned Core guest, found ${modules.length}`);
  const module = new WebAssembly.Module(modules[0].data);
  const descriptors = WebAssembly.Module.imports(module);
  const imports: WebAssembly.Imports = Object.create(null);
  let firstImport: string | undefined;
  try {
    for (const item of descriptors) {
      if (item.kind !== "function") throw Error(`Unexpected external Core ${item.kind} ${item.module}::${item.name}`);
      const group = imports[item.module] ??= Object.create(null);
      group[item.name] = () => {
        firstImport = `${item.module}::${item.name}`;
        throw Error(firstImport);
      };
    }
    const guest = new WebAssembly.Instance(module, imports);
    const prepare = guest.exports[exportName];
    if (typeof prepare !== "function") throw Error(`Missing function ${exportName}`);
    const result = prepare();
    const memory = guest.exports.memory;
    return { digest, exportName, result: String(result), elapsedMs: Math.round(performance.now() - began), memoryBytes: memory instanceof WebAssembly.Memory ? memory.buffer.byteLength : 0 };
  } catch (error) {
    return { digest, exportName, firstImport, elapsedMs: Math.round(performance.now() - began), error: (error as Error).stack };
  }
}

if (isMainThread) {
  const path = process.argv[2];
  if (!path) throw Error("Pass an exact component path");
  const exportName = process.argv[3] ?? "semio_owned_prepare_actor_v1";
  const worker = new Worker(new URL(import.meta.url), { workerData: { path, exportName } });
  const deadline = setTimeout(() => { console.error("[DEBUG] Independent WebAssembly preparation timed out"); process.exit(1); }, 120_000);
  worker.on("message", result => {
    clearTimeout(deadline);
    console.log("[DEBUG]", JSON.stringify(result));
    process.exitCode = result.result === "1" && exportName === "semio_owned_prepare_actor_v1" ? 0 : 1;
    void worker.terminate();
  });
  worker.on("error", error => {
    clearTimeout(deadline);
    console.error("[DEBUG]", error);
    process.exitCode = 1;
  });
} else {
  parentPort?.postMessage(audit(workerData.path, workerData.exportName));
}
