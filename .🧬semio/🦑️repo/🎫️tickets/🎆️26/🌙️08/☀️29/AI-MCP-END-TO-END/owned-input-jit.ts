import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { createHash } from "node:crypto";
import { readFileSync, statSync } from "node:fs";
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


const canonicalNote = "a745c114e0f7e685ad6af51c1c658b6bd6cca590f5249c641d5b78642163c2e3";
const componentPath = "🗑️generated/m-canon/browser/prototype/components/6d8a4f7ae2584b94ed7f1643/📦️component.wasm";
const fixturesPath = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🔁️persistent-local/📊️actor-input.json";
const MiB = 1024 * 1024;
const wallLimitMs = 60_000;
const memoryLimit = 256 * MiB;

function audit(path: string, expectedDigest: string, plugin: string) {
  const began = performance.now();
  const elapsed = (since = began) => Number((performance.now() - since).toFixed(3));
  const log = (report: object) => parentPort?.postMessage(report);
  const stat = statSync(path);
  assert(stat.size < 128 * MiB, "Component exceeds the diagnostic input bound");
  const component = readFileSync(path);
  const sha256 = createHash("sha256").update(component).digest("hex");
  assert.equal(sha256, expectedDigest);
  assert.equal(component.subarray(0, 8).toString("hex"), "0061736d0d000100");
  const markers = ["semio_owned_channel_version_v1", "semio_owned_admit_actor_input_v1"].map(name => Buffer.from(name));
  const matches = [...sections(component)].filter(({ id, data }) => id === 1 && [...sections(data)].some(({ id, data }) => {
    const exports = Buffer.from(data.buffer, data.byteOffset, data.length);
    return id === 7 && markers.every(marker => exports.includes(marker));
  }));
  assert.equal(matches.length, 1);
  const selected = matches[0].data;
  const module = new WebAssembly.Module(selected);
  const compileMs = elapsed();

  let memory: WebAssembly.Memory | undefined;
  let phase = "instantiate";
  let clockReads = 0;
  const seenImports = new Set<string>();
  const clockBinding = 1n;
  const micros = () => BigInt(Math.floor((performance.now() - began) * 1000));
  const view = (pointer: number, length: number) => {
    assert(memory, "Guest used memory during instantiation");
    assert(memory.buffer.byteLength <= memoryLimit, "Guest exceeds diagnostic linear-memory bound");
    return Buffer.from(memory.buffer, pointer >>> 0, length >>> 0);
  };
  const imports: WebAssembly.Imports = Object.create(null);
  for (const entry of WebAssembly.Module.imports(module)) {
    assert.equal(entry.kind, "function");
    const key = entry.module + "::" + entry.name;
    const group = imports[entry.module] ??= Object.create(null);
    group[entry.name] = (...args: (number | bigint)[]) => {
      seenImports.add(key);
      if (phase === "prepare" || phase === "instantiate") throw Error("Unexpected " + phase + " import: " + key);
      switch (key) {
        case "semio:framework/pure@1.0.0::operation-clock-us": {
          const out = view(Number(args[1]), 16);
          out.fill(0);
          assert.equal(args[0], clockBinding);
          out.writeUInt32LE(1, 0);
          out.writeBigUInt64LE(micros(), 8);
          clockReads++;
          return;
        }
        case "wasi:clocks/monotonic-clock@0.2.0::now":
          return micros() * 1000n;
        default:
          throw Error("Unexpected " + phase + " import: " + key);
      }
    };
  }
  const guest = new WebAssembly.Instance(module, imports);
  const memoryExport = guest.exports.memory;
  assert(memoryExport instanceof WebAssembly.Memory);
  memory = memoryExport;
  const invoke = (name: string, ...args: (number | bigint)[]) => {
    const fn = guest.exports["semio_owned_" + name + "_v1"];
    assert.equal(typeof fn, "function", "Missing owned export " + name);
    return (fn as (...values: (number | bigint)[]) => number | bigint | undefined)(...args);
  };
  const unpack = (result: number | bigint | undefined) => {
    assert.equal(typeof result, "bigint");
    const word = BigInt.asUintN(64, result as bigint);
    const pointer = Number(word & 0xffffffffn);
    const length = Number(word >> 32n);
    assert.notEqual(length, 0xffffffff, "Unframed scalar refusal " + pointer);
    assert(pointer > 0 && length > 0);
    return { pointer, length, bytes: view(pointer, length) };
  };
  phase = "prepare";
  const prepareStart = performance.now();
  const hasPrepare = typeof guest.exports.semio_owned_prepare_actor_v1 === "function";
  if (hasPrepare) assert.equal(invoke("prepare_actor"), 1);
  const prepareMs = elapsed(prepareStart);
  phase = "channel";
  const channel = unpack(invoke("channel_version"));
  const channelVersion = JSON.parse(channel.bytes.toString());
  assert.equal(channelVersion, 25);
  invoke("dealloc", channel.pointer, channel.length);
  log({ stage: "prepared", sha256, componentBytes: stat.size, coreBytes: selected.length, channelVersion, hasPrepare, compileMs, prepareMs, linearMemoryBytes: memory.buffer.byteLength });

  const fixtures = JSON.parse(readFileSync(fixturesPath, "utf8"));
  const appId = plugin === "note" ? "s.note.note@1/*#editor" : "s.wfc.bitmap@1/*#editor";
  const fixture = fixtures.find(({ wire, valid }: { wire: { Poll?: { events: { instanceOpen?: { appId: string; capabilities: unknown[] } }[] } }; valid: boolean }) => valid && wire.Poll?.events[0]?.instanceOpen?.appId === appId && wire.Poll.events[0].instanceOpen?.capabilities.length === 1);
  assert(fixture, "No exact valid InstanceOpen fixture");
  const wire = Buffer.from(JSON.stringify(fixture.wire));
  assert.equal(wire.length, plugin === "note" ? 601 : 602);

  let epoch = 0n;
  const deadline = 86_400_000_000n;
  const available = [48_000_000n, 3_000_000_000n, 2_147_483_648n, 2_147_483_648n, 64n, 1_000_000_000n];
  const policy = [65_536n, BigInt(16 * MiB), BigInt(64 * MiB), BigInt(64 * MiB), 64n];
  const effects = (bytes: Buffer, offset: number) =>
    Array.from({ length: 5 }, (_, index) => bytes.readBigUInt64LE(offset + index * 8));
  const describeEffects = (values: bigint[]) => ({
    items: Number(values[0]), copyBytes: Number(values[1]), capacityBytes: Number(values[2]), releaseBytes: Number(values[3]), work: Number(values[4]),
  });
  const consume = (used: bigint[]) => {
    for (const [index, value] of used.entries()) {
      const stockIndex = index === 4 ? 5 : index;
      assert(value >= 0n && value <= available[stockIndex], "Diagnostic currency exhausted");
      available[stockIndex] -= value;
    }
  };
  const min = (a: bigint, b: bigint) => a < b ? a : b;
  const issue = () => [
    1n, 1n, ++epoch, clockBinding, deadline, available[5],
    ...available.slice(0, 5),
    ...policy.map((value, index) => min(value, available[index])),
    min(1_000_000n, available[5]), deadline, 0n,
  ];
  let bootstrapSequence = 0n;
  let revision = 0n;
  let identity: bigint[] | undefined;
  let serial = 0n;
  let inputReturnCount = 0;
  let inputCalls = 0;
  const totalInputSpent = [0n, 0n, 0n, 0n, 0n];
  let inputCallMs = 0;
  const readReceipt = (name: string, pointerArgs: number[] = []) => {
    phase = name;
    const loan = issue();
    if (pointerArgs.length) {
      assert.equal(inputReturnCount, 0);
      assert.deepEqual(view(pointerArgs[0], pointerArgs[1]), wire);
    }
    const start = performance.now();
    const clockBefore = clockReads;
    const result = invoke(name, ...pointerArgs, ...loan);
    const callMs = elapsed(start);
    const receipt = unpack(result);
    const { bytes } = receipt;
    const magic = bytes.toString("ascii", 0, 4);
    const details = {
      stage: name,
      epoch: Number(epoch),
      magic,
      callMs,
      clockReads: clockReads - clockBefore,
      memoryBytes: memory.buffer.byteLength,
    };
    if (magic === "SAB1") {
      assert.equal(name, "admit_actor_context");
      assert.equal(receipt.length, 304);
      assert.deepEqual([...bytes.subarray(4, 8)], [1, 0, 48, 1]);
      assert.deepEqual(Array.from({ length: 19 }, (_, index) => bytes.readBigUInt64LE(8 + index * 8)), loan);
      const sequence = bytes.readBigUInt64LE(280);
      const returned = bytes.readBigUInt64LE(288);
      assert.equal(returned, bootstrapSequence);
      assert.equal(sequence, ++bootstrapSequence);
      const status = bytes[296];
      const bootPhase = bytes[297];
      const refusal = bytes.readUInt32LE(300);
      assert(status < 2 && refusal === 0, "Boot refused: " + refusal);
      const actual = effects(bytes, 160);
      consume(actual);
      consume(effects(bytes, 240));
      invoke("dealloc", receipt.pointer, receipt.length);
      log({ ...details, sequence: Number(sequence), status, bootPhase, spent: describeEffects(actual) });
      return false;
    }

    const offset = magic === "SBT1" ? 8 : 0;
    assert.equal(receipt.length, offset === 8 ? 200 : 144);
    if (offset) {
      assert.equal(revision, 0n);
      assert.deepEqual([...bytes.subarray(4, 8)], [1, 0, 200, 0]);
      assert.equal(bytes.readBigUInt64LE(152), bootstrapSequence);
      assert.deepEqual(effects(bytes, 160), [1n, 2n, 0n, 0n, 1n]);
    }
    assert.equal(bytes.toString("ascii", offset, offset + 4), "SAC3");
    assert.deepEqual(Array.from({ length: 11 }, (_, index) => bytes.readBigUInt64LE(offset + 8 + index * 8)), loan.slice(0, 11));
    const kind = bytes[offset + 4];
    const flags = bytes[offset + 5];
    const refusal = bytes[offset + 6];
    const isInput = name === "admit_actor_input";
    const ready = (flags & 1) !== 0;
    const inputReturned = isInput && (flags & 2) !== 0;
    assert(isInput ? (kind === 4 && [0, 2, 3].includes(flags)) || (kind === 7 && [0, 2].includes(flags)) : kind === 1 && flags <= 1, "Unexpected receipt kind/flags " + kind + "/" + flags);
    const spent = effects(bytes, offset + 96);
    const scope = [...loan.slice(11, 15), loan[16]];
    assert(spent.every((value, index) => value <= scope[index]));
    consume(spent);
    const escrowLength = bytes.readUInt32LE(offset + 140);
    assert(escrowLength >= 48 && escrowLength <= 5936);
    const escrow = view(bytes.readUInt32LE(offset + 136), escrowLength);
    assert.equal(escrow.toString("ascii", 0, 4), "SRE1");
    const rows = escrow.readUInt32LE(4);
    assert(rows <= 32);
    assert.equal(escrowLength, 48 + rows * 184);
    const source = [8, 16, 24].map(index => escrow.readBigUInt64LE(index));
    assert(source.every(word => word > 0n));
    if (identity) assert.deepEqual(source, identity);
    else identity = source;
    assert.equal(escrow.readBigUInt64LE(40), revision);
    assert.equal(escrow.readBigUInt64LE(32), ++revision);
    let previous = 0n;
    for (let index = 0; index < rows; index += 1) {
      const start = 48 + index * 184;
      const row = Array.from({ length: 23 }, (_, index) => escrow.readBigUInt64LE(start + index * 8));
      assert.deepEqual(row.slice(0, 3), source);
      const [, , , destination, , next, origin, status] = row;
      assert(destination > 0n && next > previous && origin > 0n && origin <= epoch);
      assert(status === 1n || status === 2n);
      previous = next;
      const held = row.slice(8, 13);
      assert(held.every((held, index) => status === 2n ? row[13 + index] + row[18 + index] === held : row[13 + index] + row[18 + index] <= held));
      if (next <= serial) continue;
      serial = next;
      consume([held[0], held[1], held[4], held[3], held[2]]);
    }
    log({ ...details, kind, flags, refusal, ready, inputReturned, spent: describeEffects(spent), sre1: { revision: Number(revision), acknowledged: Number(revision - 1n), rows, bytes: escrowLength } });
    assert.equal(refusal, 0);
    invoke("dealloc", receipt.pointer, receipt.length);
    if (isInput) {
      inputCalls++;
      inputCallMs += callMs;
      for (const [index, value] of spent.entries()) totalInputSpent[index] += value;
      assert.notEqual(kind, 7, "Input published a semantic failure");
      assert.equal(inputReturned, ready, "Successful passage requires exactly one terminal input return");
      assert.deepEqual(view(pointerArgs[0], pointerArgs[1]), wire);
      if (inputReturned) {
        invoke("dealloc", pointerArgs[0], pointerArgs[1]);
        inputReturnCount++;
      }
    }
    return ready;
  };

  let bootReady = false;
  for (let attempt = 0; attempt < 64 && !bootReady; attempt += 1) bootReady = readReceipt("admit_actor_context");
  assert(bootReady, "Boot did not reach ready within 64 receipts");
  phase = "allocate_input";
  const pointer = Number(invoke("alloc", wire.length));
  assert(pointer > 0);
  view(pointer, wire.length).set(wire);
  const inputStart = performance.now();
  let inputReady = false;
  for (let attempt = 0; attempt < 64 && !inputReady; attempt += 1) inputReady = readReceipt("admit_actor_input", [pointer, wire.length]);
  assert(inputReady && inputReturnCount === 1, "Input did not reach ready and return its wire");
  log({ stage: "complete", inputBytes: wire.length, inputReturnCount, inputCalls, inputCallMs: Number(inputCallMs.toFixed(3)), inputElapsedMs: elapsed(inputStart), totalInputSpent: describeEffects(totalInputSpent), sha256, elapsedMs: elapsed(), rssBytes: process.memoryUsage().rss, imports: [...seenImports] });
}

if (isMainThread) {
  const path = process.argv[2] ?? fileURLToPath(new URL(componentPath, import.meta.url));
  const expectedDigest = process.argv[3] ?? canonicalNote;
  const plugin = process.argv[4] ?? "note";
  assert(["note", "wfc"].includes(plugin));
  const worker = new Worker(new URL(import.meta.url), { workerData: { path, expectedDigest, plugin } });
  const deadline = setTimeout(() => {
    console.error("[DEBUG] Owned Input diagnostic exceeded its wall-clock bound");
    process.exit(1);
  }, wallLimitMs);
  worker.on("message", result => {
    console.log("[DEBUG]", JSON.stringify(result));
    if (result.stage === "complete") {
      clearTimeout(deadline);
      void worker.terminate();
    }
  });
  worker.on("error", error => {
    clearTimeout(deadline);
    console.error("[DEBUG]", error);
    process.exitCode = 1;
  });
} else {
  audit(workerData.path, workerData.expectedDigest, workerData.plugin);
}
