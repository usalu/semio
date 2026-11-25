import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { createHash } from "node:crypto";
import { readFileSync, statSync, writeFileSync } from "node:fs";
import { isMainThread, parentPort, Worker, workerData } from "node:worker_threads";
import { diagnosticPollLimitsObject, diagnosticPollLimitsTuple } from "./owned-input-poll-limits.ts";

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
const liveFixture = new URL("🗑️generated/native-note-poll-exact.json", import.meta.url);
const liveBudget = new URL("🗑️generated/native-note-poll-scalar.json", import.meta.url);
const memoryLimit = 256 * MiB;

type FixtureMode = "empty" | "minimal" | "standard" | "captured-ack" | "matched-empty";

function audit(path: string, expectedDigest: string, plugin: string, live: boolean, fixtureMode: FixtureMode, tuple: boolean, inputOnly: boolean, lifecycleTurns: boolean, transcriptPath?: string) {
  const began = performance.now();
  const elapsed = (since = began) => Number((performance.now() - since).toFixed(3));
  const log = (report: object) => parentPort?.postMessage(report);
  const stat = statSync(path);
  assert(stat.size <= (plugin === "wfc" ? 256 : 128) * MiB, "Component exceeds the diagnostic input bound");
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
  const transcript = transcriptPath ? ["owned-input-v1", `M ${selected.byteOffset - component.byteOffset} ${selected.length} ${sha256}`] : undefined;
  const record = (...fields: (number | bigint | string)[]) => {
    if (!transcript) return;
    assert(transcript.length < 65_536, "Replay transcript exceeded record bound");
    transcript.push(fields.join(" "));
  };
  const module = new WebAssembly.Module(selected);
  const compileMs = elapsed();

  let memory: WebAssembly.Memory | undefined;
  let phase = "instantiate";
  let clockReads = 0;
  const seenImports = new Set<string>();
  const exact = live ? JSON.parse(readFileSync(liveBudget, "utf8")).map(BigInt) as bigint[] : undefined;
  if (exact) assert.equal(exact.length, 19);
  const clockBinding = exact?.[3] ?? 1n;
  const micros = () => BigInt(Math.floor((performance.now() - began) * 1000));
  let call: string[] | undefined;
  const view = (pointer: number, length: number) => {
    assert(memory, "Guest used memory during instantiation");
    assert(memory.buffer.byteLength <= memoryLimit, "Guest exceeds diagnostic linear-memory bound");
    return Buffer.from(memory.buffer, pointer >>> 0, length >>> 0);
  };
  log({ stage: "imports", names: WebAssembly.Module.imports(module).map(entry => entry.module + "::" + entry.name) });
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
          const time = micros();
          out.writeUInt32LE(1, 0);
          out.writeBigUInt64LE(time, 8);
          if (call) {
            assert(call.length < 65_536, "Replay import count exceeded record bound");
            call.push(`I operation-clock-us ${args[0]} ${args[1]} ${time}`);
          }
          clockReads++;
          return;
        }
        case "wasi:clocks/monotonic-clock@0.2.0::now": {
          const time = micros() * 1000n;
          if (call) {
            assert(call.length < 65_536, "Replay import count exceeded record bound");
            call.push(`I now ${time}`);
          }
          return time;
        }
        case "wasi:clocks/wall-clock@0.2.0::now": {
          const now = BigInt(Date.now());
          const out = view(Number(args[0]), 16);
          out.fill(0);
          out.writeBigUInt64LE(now / 1000n, 0);
          out.writeUInt32LE(Number(now % 1000n) * 1_000_000, 8);
          return;
        }
        case "semio:framework/pure@1.0.0::operation-begin": {
          const out = view(Number(args[0]), 16);
          out.fill(0);
          out.writeBigUInt64LE(1n << 31n, 8);
          return;
        }
        case "semio:framework/pure@1.0.0::operation-progress":
        case "semio:framework/pure@1.0.0::operation-allocation":
        case "semio:framework/pure@1.0.0::operation-finish":
        case "semio:framework/pure@1.0.0::operation-reserve-return":
          return 0;
        case "semio:framework/pure@1.0.0::now-ms":
          return BigInt(Date.now());
        case "semio:framework/pure@1.0.0::log":
          return;
        case "wasi:random/insecure-seed@0.2.9::insecure-seed": {
          const out = view(Number(args[0]), 16);
          out.writeBigUInt64LE(0x9e3779b97f4a7c15n, 0);
          out.writeBigUInt64LE(0xbf58476d1ce4e5b9n, 8);
          return;
        }
        case "wasi:random/random@0.2.9::get-random-u64":
          return 0x94d049bb133111ebn;
        case "wasi:cli/environment@0.2.0::get-environment":
          view(Number(args[0]), 8).fill(0);
          return;
        default:
          throw Error("Unexpected " + phase + " import: " + key);
      }
    };
  }
  const guest = new WebAssembly.Instance(module, imports);
  log({ stage: "exports", names: Object.keys(guest.exports).filter(name => name.startsWith("semio_owned")).map(name => name + "/" + (guest.exports[name] as Function).length) });
  const memoryExport = guest.exports.memory;
  assert(memoryExport instanceof WebAssembly.Memory);
  memory = memoryExport;
  const invoke = (name: string, ...args: (number | bigint)[]) => {
    const fn = guest.exports["semio_owned_" + name + "_v1"];
    assert.equal(typeof fn, "function", "Missing owned export " + name);
    if (transcript) call = [];
    const result = (fn as (...values: (number | bigint)[]) => number | bigint | undefined)(...args);
    if (call) {
      record("C", name, result ?? "_", memory!.buffer.byteLength, call.length, ...args);
      for (const event of call) record(event);
      call = undefined;
    }
    return result;
  };
  const unpack = (result: number | bigint | undefined) => {
    assert.equal(typeof result, "bigint");
    const word = BigInt.asUintN(64, result as bigint);
    const pointer = Number(word & 0xffffffffn);
    const length = Number(word >> 32n);
    assert.notEqual(length, 0xffffffff, "Unframed scalar refusal " + pointer);
    assert(pointer > 0 && length > 0);
    const bytes = view(pointer, length);
    if (transcript) record("R", pointer, bytes.toString("hex"));
    return { pointer, length, bytes };
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
  const fixture = fixtures.find(({ wire, valid }: {
    wire: { Poll?: { events: { instanceOpen?: { appId: string; capabilities: unknown[] } }[] } };
    valid: boolean;
  }) => {
    if (!valid || !wire.Poll) return false;
    if (fixtureMode === "empty") return wire.Poll.events.length === 0;
    const open = wire.Poll.events[0]?.instanceOpen;
    return open?.appId === appId && open.capabilities.length === (fixtureMode === "minimal" ? 0 : 1);
  });
  assert(fixture, "No exact valid Poll fixture");
  const input = live ? JSON.parse(readFileSync(liveFixture, "utf8")) : fixture.wire;
  if (fixtureMode === "matched-empty") input.Poll.events = [];
  if (fixtureMode === "captured-ack") {
    const { activationGeneration, instanceId, requestSequence } = input.Poll.events[0].instanceOpen.request;
    input.Poll.events = [{ instanceLifecycleAck: { receipt: { kind: "captured", lifetime: { activationGeneration, instanceId, guestLifetime: "1" }, requestSequence } } }];
  }
  const limits = diagnosticPollLimitsObject(input.Poll.limits);
  input.Poll.limits = tuple ? diagnosticPollLimitsTuple(limits) : limits;
  let wire = Buffer.from(JSON.stringify(input));
  const objectLength = live ? 681 : fixtureMode === "empty" ? 284 : fixtureMode === "minimal" ? 480 : plugin === "note" ? 601 : 602;
  void objectLength;
  if (live) assert.equal(JSON.parse(wire.toString()).Poll.events[0].instanceOpen.appId, appId);

  let epoch = 0n;
  const deadline = exact?.[4] ?? 86_400_000_000n;
  const purposeDeadline = exact?.[17] ?? deadline;
  const available = [48_000_000n, 3_000_000_000n, 2_147_483_648n, 2_147_483_648n, 64n, 1_000_000_000n];
  let policy = [65_536n, BigInt(16 * MiB), BigInt(64 * MiB), BigInt(64 * MiB), 64n];
  let purposeWork = 1_000_000n;
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
    exact?.[0] ?? 1n, exact?.[1] ?? 1n, ++epoch, clockBinding, deadline, available[5],
    ...available.slice(0, 5),
    ...policy.map((value, index) => min(value, available[index])),
    min(purposeWork, available[5]), purposeDeadline, 0n,
  ];
  let bootstrapSequence = 0n;
  let revision = 0n;
  let identity: bigint[] | undefined;
  let serial = 0n;
  let inputReturnCount = 0;
  let inputCalls = 0;
  const totalInputSpent = [0n, 0n, 0n, 0n, 0n];
  let inputCallMs = 0;
  const accountFailureReceipt = (bytes: Buffer, loan: bigint[]) => {
    const offset = 8;
    assert.equal(bytes.toString("ascii", offset, offset + 4), "SAC3");
    const spent = effects(bytes, offset + 96);
    consume(spent);
    const escrowLength = bytes.readUInt32LE(offset + 140);
    const escrowPointer = bytes.readUInt32LE(offset + 136);
    const escrow = view(escrowPointer, escrowLength);
    assert.equal(escrow.toString("ascii", 0, 4), "SRE1");
    const rows = escrow.readUInt32LE(4);
    assert.equal(escrow.readBigUInt64LE(40), revision);
    assert.equal(escrow.readBigUInt64LE(32), ++revision);
    for (let index = 0; index < rows; index += 1) {
      const start = 48 + index * 184;
      const row = Array.from({ length: 23 }, (_, column) => escrow.readBigUInt64LE(start + column * 8));
      const next = row[5];
      const held = row.slice(8, 13);
      if (next <= serial) continue;
      serial = next;
      consume([held[0], held[1], held[4], held[3], held[2]]);
    }
    return { spent: describeEffects(spent), rows, loanEpoch: String(loan[2]) };
  };
  const heapStrings = () => {
    const found = new Set<string>();
    const text = Buffer.from(memory!.buffer).toString("latin1");
    for (const match of text.matchAll(/[\x20-\x7e]{6,400}/g)) found.add(match[0]);
    return found;
  };
  let baseline: Set<string> | undefined;
  let lastLoan: bigint[] = [];
  const readReceipt = (name: string, pointerArgs: number[] = [], receiveOutput?: (value: any, inputPending: boolean) => void) => {
    phase = name;
    const loan = issue();
    if (exact && name === "admit_actor_input" && inputCalls === 0) assert.deepEqual(loan, exact);
    if (pointerArgs.length) {
      assert.equal(inputReturnCount, 0);
      assert.deepEqual(view(pointerArgs[0], pointerArgs[1]), wire);
      if (transcript) record("R", pointerArgs[0], wire.toString("hex"));
    }
    if (name === "admit_actor_input" && !baseline) baseline = heapStrings();
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
    const isPoll = name === "poll";
    const isClose = name === "close_actor_output";
    assert.equal(receipt.length, offset === 8 ? 200 : isPoll ? 152 : 144);
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
    const valid = isInput ? (kind === 4 && [0, 2, 3].includes(flags)) || (kind === 7 && [0, 2].includes(flags))
      : isPoll ? kind === 5 && [0, 2, 3, 4, 6, 7].includes(flags)
      : isClose ? kind === 2 && flags <= 2 : kind === 1 && flags <= 1;
    assert(valid, "Unexpected receipt kind/flags " + kind + "/" + flags);
    const spent = effects(bytes, offset + 96);
    const scope = [...loan.slice(11, 15), loan[16]];
    assert(spent.every((value, index) => value <= scope[index]));
    consume(spent);
    const escrowLength = bytes.readUInt32LE(offset + 140);
    assert(escrowLength >= 48 && escrowLength <= 5936);
    const escrowPointer = bytes.readUInt32LE(offset + 136);
    const escrow = view(escrowPointer, escrowLength);
    if (transcript) record("R", escrowPointer, escrow.toString("hex"));
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
    if (!lifecycleTurns || ready || refusal !== 0) {
      log({ ...details, kind, flags, refusal, ready, inputReturned, spent: describeEffects(spent), sre1: { revision: Number(revision), acknowledged: Number(revision - 1n), rows, bytes: escrowLength } });
    }
    assert.equal(refusal, 0);
    if (isPoll && ready) {
      const output = view(bytes.readUInt32LE(144), bytes.readUInt32LE(148));
      receiveOutput?.(JSON.parse(output.toString("utf8")), (flags & 4) !== 0);
    }
    invoke("dealloc", receipt.pointer, receipt.length);
    if (isInput) {
      inputCalls++;
      inputCallMs += callMs;
      for (const [index, value] of spent.entries()) totalInputSpent[index] += value;
      if (kind === 7) {
        purposeWork = 100_000_000n;
        const failedEpoch = epoch;
        const probe = (probeName: string, ...head: bigint[]) => {
          lastLoan = issue();
          const raw = invoke(probeName, ...head, ...lastLoan);
          const word = BigInt.asUintN(64, raw as bigint);
          const pointer = Number(word & 0xffffffffn);
          const length = Number(word >> 32n);
          if (length === 0xffffffff) { log({ stage: probeName, refused: pointer }); return undefined; }
          const bytes = Buffer.from(view(pointer, length));
          const accounted = accountFailureReceipt(bytes, lastLoan);
          if (false) log({ stage: probeName, pointer, length, kind: bytes[4], flags: bytes[5], tag: bytes[6], a: String(bytes.readBigUInt64LE(168)), b: String(bytes.readBigUInt64LE(176)), accounted });
          const escrowAt = bytes.readUInt32LE(8 + 136), escrowLen = bytes.readUInt32LE(8 + 140);
          log({ stage: probeName + "-escrow", escrowAt, escrowLen });
          invoke("dealloc", pointer, length);
          return { pointer, length, bytes };
        };
        let published: { bytes: Buffer } | undefined;
        for (let attempt = 0; attempt < 30000; attempt += 1) {
          published = probe("publish_actor_failure", failedEpoch, 8n);
          if (attempt % 500 === 0) log({ stage: "publish-attempt", attempt, flags: published?.bytes[5] });
          if (!published || published.bytes[5] === 1) break;
        }
        const outcomeLength = published ? Number(published.bytes.readBigUInt64LE(168)) : 0;
        log({ stage: "publish-done", outcomeLength });
        for (let attempt = 0; attempt < 64 && outcomeLength > 0; attempt += 1) {
          const chunk = probe("read_actor_failure", failedEpoch, 8n);
          if (!chunk) break;
          const offset = Number(chunk.bytes.readBigUInt64LE(168));
          const size = Number(chunk.bytes.readBigUInt64LE(176));
          if (chunk.bytes[5] === 0) continue;
          const raw = invoke("borrow_actor_failure", failedEpoch, 8n, exact?.[0] ?? 1n, exact?.[1] ?? 1n, BigInt(offset), BigInt(size));
          const word = BigInt.asUintN(64, raw as bigint);
          const pointer = Number(word & 0xffffffffn);
          const length = Number(word >> 32n);
          if (length === 0xffffffff) { log({ stage: "borrow", refused: pointer }); break; }
          log({ stage: "borrow", pointer, length, hex: Buffer.from(view(pointer, length)).toString("hex") });
          break;
        }
        if (transcript && transcriptPath) writeFileSync(transcriptPath, transcript.join("\n") + "\n");
        return false;
      }
      if (false) {
        const failedEpoch = epoch;
        const probe = (probeName: string, ...head: bigint[]) => {
          try {
            const raw = invoke(probeName, ...head, ...issue());
            if (typeof raw !== "bigint") { log({ stage: probeName, raw: String(raw) }); return undefined; }
            const word = BigInt.asUintN(64, raw);
            const pointer = Number(word & 0xffffffffn);
            const length = Number(word >> 32n);
            if (length === 0xffffffff) { log({ stage: probeName, refused: pointer }); return undefined; }
            const bytes = Buffer.from(view(pointer, length));
            log({ stage: probeName, pointer, length, magic: bytes.toString("ascii", 0, 4), hex: bytes.subarray(0, Math.min(length, 400)).toString("hex"), text: bytes.toString("latin1").replace(/[^\x20-\x7e]/g, ".").slice(0, 400) });
            return { pointer, length, bytes };
          } catch (error) { log({ stage: probeName, error: String(error) }); return undefined; }
        };
        let published: { bytes: Buffer } | undefined;
        for (let attempt = 0; attempt < 64; attempt += 1) {
          published = probe("publish_actor_failure", failedEpoch, 8n);
          if (!published || published.bytes[5] === 1) break;
        }
        const outcomeLength = published ? Number(published.bytes.readBigUInt64LE(168)) : 0;
        log({ stage: "publish-done", outcomeLength, flags: published?.bytes[5] });
        for (let attempt = 0; attempt < 64 && outcomeLength > 0; attempt += 1) {
          const chunk = probe("read_actor_failure", failedEpoch, 8n);
          if (!chunk) break;
          const offset = Number(chunk.bytes.readBigUInt64LE(168));
          const size = Number(chunk.bytes.readBigUInt64LE(176));
          log({ stage: "read-chunk", flags: chunk.bytes[5], offset, size });
          if (chunk.bytes[5] === 0) continue;
          try {
            const raw = invoke("borrow_actor_failure", failedEpoch, 8n, exact?.[0] ?? 1n, exact?.[1] ?? 1n, BigInt(offset), BigInt(size));
            const word = BigInt.asUintN(64, raw as bigint);
            const pointer = Number(word & 0xffffffffn);
            const length = Number(word >> 32n);
            if (length === 0xffffffff) { log({ stage: "borrow", refused: pointer }); break; }
            const bytes = Buffer.from(view(pointer, length));
            log({ stage: "borrow", pointer, length, text: bytes.toString("utf8") });
          } catch (error) { log({ stage: "borrow", error: String(error) }); }
          break;
        }
        throw Error("probe done");
      }

      assert.equal(inputReturned, ready, "Successful passage requires exactly one terminal input return");
      assert.deepEqual(view(pointerArgs[0], pointerArgs[1]), wire);
      if (transcript) record("R", pointerArgs[0], wire.toString("hex"));
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
  if (exact) {
    assert(epoch < exact[2]);
    epoch = exact[2] - 1n;
    available.splice(0, available.length, ...exact.slice(6, 11), exact[5]);
    policy = exact.slice(11, 16);
    purposeWork = exact[16];
  }
  const inputStart = performance.now();
  const admitInput = () => {
    phase = "allocate_input";
    inputReturnCount = 0;
    const pointer = Number(invoke("alloc", wire.length));
    assert(pointer > 0);
    view(pointer, wire.length).set(wire);
    if (transcript) record("W", pointer, wire.toString("hex"));
    let inputReady = false;
    for (let attempt = 0; attempt < 64 && !inputReady; attempt += 1) inputReady = readReceipt("admit_actor_input", [pointer, wire.length]);
    assert(inputReady && inputReturnCount === 1, "Input did not reach ready and return its wire");
  };
  admitInput();
  if (lifecycleTurns) {
    const request = input.Poll.events[0].instanceOpen.request;
    let acknowledging = false;
    let inputPending = false;
    let committed = false;
    for (let turn = 0; turn < 128 && !committed; turn++) {
      let result: any;
      let ready = false;
      for (let attempt = 0; attempt < 2_048 && !ready; attempt++) {
        ready = readReceipt("poll", [], (output, pending) => { result = output.Ok ?? output; inputPending = pending; });
      }
      assert(ready && ["idle", "moreWork"].includes(result.status), "Invalid Poll result: " + JSON.stringify(result));
      const receipt = result.lifecycleReceipt ?? null;
      log({ stage: "lifecycle-turn", turn, acknowledging, status: result.status, receipt, inputPending, effects: result.effects, nextWake: result.nextWake, fuelUsed: result.fuelUsed });
      let closed = false;
      for (let attempt = 0; attempt < 2_048 && !closed; attempt++) closed = readReceipt("close_actor_output");
      assert(closed, "Poll output did not retire");
      if (acknowledging) {
        assert.equal(receipt, null);
        committed = true;
        break;
      }
      if (receipt) {
        assert.equal(receipt.kind, "captured");
        assert.equal(receipt.lifetime.activationGeneration, request.activationGeneration);
        assert.equal(receipt.lifetime.instanceId, request.instanceId);
        assert.equal(BigInt(receipt.requestSequence), BigInt(request.requestSequence));
        assert.notEqual(BigInt(receipt.lifetime.guestLifetime), 0n);
        assert(!inputPending, "Original open event remains pending after capture");
        acknowledging = true;
      } else {
        assert.equal(result.status, "moreWork");
        if (inputPending) continue;
      }
      const events = receipt ? [{ instanceLifecycleAck: { receipt: { ...receipt, requestSequence: Number(receipt.requestSequence) } } }] : [];
      wire = Buffer.from(JSON.stringify({ Poll: { ...input.Poll, events } }));
      admitInput();
    }
    assert(committed, "Exact ACK did not reach a committed turn");
  } else if (!inputOnly) {
    phase = "poll";
    const pollLoan = issue();
    const pollFn = guest.exports["semio_owned_poll_v1"] as (...values: bigint[]) => bigint;
    log({ stage: "poll-call", arity: pollFn.length, loanWords: pollLoan.length });
    const pollStart = performance.now();
    const pollResult = invoke("poll", ...pollLoan);
    const pb = unpack(pollResult).bytes;
    log({ stage: "poll-receipt", ms: elapsed(pollStart), length: pb.length, magic: pb.toString("ascii", 0, 4), kind: pb[4], flags: pb[5], refusal: pb[6], head: pb.subarray(0, 16).toString("hex"), clockReads });
  }
  if (transcript && transcriptPath) writeFileSync(transcriptPath, transcript.join("\n") + "\n");
  log({ stage: "complete", inputBytes: wire.length, inputReturnCount, inputCalls, inputCallMs: Number(inputCallMs.toFixed(3)), inputElapsedMs: elapsed(inputStart), totalInputSpent: describeEffects(totalInputSpent), sha256, elapsedMs: elapsed(), rssBytes: process.memoryUsage().rss, imports: [...seenImports] });
}

if (isMainThread) {
  const path = process.argv[2] ?? fileURLToPath(new URL(componentPath, import.meta.url));
  const expectedDigest = process.argv[3] ?? canonicalNote;
  const plugin = process.argv[4] ?? "note";
  assert(["note", "wfc"].includes(plugin));
  const live = process.argv.includes("--live-note");
  const emptyPoll = process.argv.includes("--empty-poll");
  const minimalOpen = process.argv.includes("--minimal-open");
  const capturedAck = process.argv.includes("--captured-ack-control");
  const matchedEmpty = process.argv.includes("--matched-empty-control");
  const tuple = process.argv.includes("--tuple-limits");
  const inputOnly = process.argv.includes("--input-only");
  const lifecycleTurns = process.argv.includes("--lifecycle-turns");
  assert([emptyPoll, minimalOpen, capturedAck, matchedEmpty].filter(Boolean).length <= 1);
  assert(inputOnly || !capturedAck);
  assert(!lifecycleTurns || (!inputOnly && !live && tuple && !(emptyPoll || minimalOpen || capturedAck || matchedEmpty)));
  const fixtureMode = capturedAck ? "captured-ack" : matchedEmpty ? "matched-empty" : emptyPoll ? "empty" : minimalOpen ? "minimal" : "standard";
  assert(!live || plugin === "note" && fixtureMode === "standard");
  assert(!minimalOpen || plugin === "note");
  const transcriptPath = process.env.SEMIO_OWNED_INPUT_TRANSCRIPT;
  assert(!lifecycleTurns || !transcriptPath, "The scalar transcript covers input admission only");
  const worker = new Worker(new URL(import.meta.url), { workerData: { path, expectedDigest, plugin, live, fixtureMode, tuple, inputOnly, lifecycleTurns, transcriptPath } });
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
  audit(workerData.path, workerData.expectedDigest, workerData.plugin, workerData.live, workerData.fixtureMode, workerData.tuple, workerData.inputOnly, workerData.lifecycleTurns, workerData.transcriptPath);
}
