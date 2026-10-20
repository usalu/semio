/** 🔶️ The actor return codecs load phase of registerTests1. */
import type { ActorReturnControl, ActorReturnDrive, ActorReturnIdentity, ActorReturnOrigin, ActorReturnPageReceipt, ActorReturnResult } from "../../../🟦️.ts";
import type { TestSource } from "../🟢️shared/🟦️.ts";

/** 🔷 Registers the actor return codecs load tests of registerTests1. */
export function registerActorReturnCodecsLoadTests({ dependencies, vitest, testSource }: { readonly dependencies: Pick<typeof import("../../../../📃️page/🟦️.ts"), "createActorBytePage" | "readActorBytePage"> & Pick<typeof import("../../../🟦️.ts"), "ACTOR_RETURN_CONTROL_MAXIMUM_BYTES" | "ACTOR_RETURN_DRIVE_MAXIMUM_BYTES" | "ACTOR_RETURN_IDENTITY_MAXIMUM_BYTES" | "ACTOR_RETURN_ORIGIN_MAXIMUM_BYTES" | "ACTOR_RETURN_PAGE_RECEIPT_MAXIMUM_BYTES" | "ACTOR_RETURN_RESULT_MAXIMUM_BYTES" | "ActorReturnResultFraming" | "decodeActorReturnDrive" | "decodeActorReturnResult" | "encodeActorReturnDrive" | "encodeActorReturnResult">; readonly vitest: NonNullable<ImportMeta["vitest"]>; readonly testSource: TestSource }) {
  const { ACTOR_RETURN_CONTROL_MAXIMUM_BYTES, ACTOR_RETURN_DRIVE_MAXIMUM_BYTES, ACTOR_RETURN_IDENTITY_MAXIMUM_BYTES, ACTOR_RETURN_ORIGIN_MAXIMUM_BYTES, ACTOR_RETURN_PAGE_RECEIPT_MAXIMUM_BYTES, ACTOR_RETURN_RESULT_MAXIMUM_BYTES, ActorReturnResultFraming, createActorBytePage, decodeActorReturnDrive, decodeActorReturnResult, encodeActorReturnDrive, encodeActorReturnResult, readActorBytePage } = dependencies;
  const { it, expect, vi } = vitest;
  const hydrate = (value: unknown): ActorReturnDrive => JSON.parse(JSON.stringify(value), (key, item) => ["activationGeneration", "returnSequence", "pageSequence"].includes(key) ? BigInt(item) : item);
  const hydrateResult = (value: unknown): ActorReturnResult => JSON.parse(JSON.stringify(value), (key, item) => ["activationGeneration", "returnSequence", "pageSequence"].includes(key) ? BigInt(item) : item);
  const resultOracle = async (): Promise<(value: ActorReturnResult) => Buffer> => {
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    const moduleName = "@webassemblyjs/leb128/lib/leb.js";
    const module = await import(moduleName);
    const encode = (module.default ?? module).encodeUIntBuffer;
    const uint = (value: bigint | number): Buffer => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(BigInt(value)); return Buffer.from(encode(bytes)); };
    const origin = (value: ActorReturnOrigin): Buffer[] => [uint(value.activationGeneration), uint(value.requestSequence)];
    const identity = (value: ActorReturnIdentity): Buffer[] => [...origin(value.origin), uint(value.returnSequence)];
    const receipt = (value: ActorReturnPageReceipt): Buffer[] => [...identity(value.identity), uint(value.pageSequence), uint(value.length), Buffer.from([value.final ? 1 : 0])];
    const control = (value: ActorReturnControl): Buffer[] => [Buffer.from([fixture.controlTags[value.kind]]), ...(value.kind === "inputAck" ? receipt(value.receipt) : identity(value.identity))];
    return value => {
      const tag = Buffer.from([fixture.resultTags[value.kind]]);
      switch (value.kind) {
        case "protocolFault": return Buffer.concat([tag, Buffer.from([fixture.resultEnums.fault.indexOf(value.fault)])]);
        case "refused": return Buffer.concat([tag, ...origin(value.origin), Buffer.from([fixture.resultEnums.fault.indexOf(value.fault)])]);
        case "pending": return Buffer.concat([tag, ...identity(value.identity), Buffer.from([fixture.resultEnums.reason.indexOf(value.reason)])]);
        case "retired": return Buffer.concat([tag, ...identity(value.identity), Buffer.from([fixture.resultEnums.completion.indexOf(value.completion)])]);
        case "control": return Buffer.concat([tag, ...control(value.control), Buffer.from([fixture.resultEnums.outcome.indexOf(value.outcome), fixture.resultEnums.fault.indexOf(value.fault)])]);
        case "page": {
          const bytes = Buffer.alloc(fixture.maximumPageBytes);
          for (let block = 0; block < 64; block++) for (let word = 0; word < 8; word++) bytes.writeBigUInt64LE(Reflect.get(Reflect.get(value.page, `block${String(block).padStart(2, "0")}`), `word${word}`), block * 64 + word * 8);
          return Buffer.concat([tag, ...receipt(value.receipt), bytes]);
        }
      }
    };
  };

  it("ActorReturn codecs load natively in Node strip-only mode and preserve every shared vector", async () => {
    const { spawnSync } = await import("node:child_process");
    const source = new URL(testSource.url).href;
    const fixture = new URL("./🧪️tests/🧫️fixtures/🔣️.json", source).href;
    const page = new URL("../📃️page/🟦️.ts", source).href;
    const program = `
      import assert from "node:assert/strict";
      import { readFileSync } from "node:fs";
      import { fileURLToPath } from "node:url";
      const api = await import(${JSON.stringify(source)});
      const pageApi = await import(${JSON.stringify(page)});
      const fixture = JSON.parse(readFileSync(fileURLToPath(${JSON.stringify(fixture)}), "utf8"));
      const hydrate = value => JSON.parse(JSON.stringify(value), (key, item) => ["activationGeneration", "returnSequence", "pageSequence"].includes(key) ? BigInt(item) : item);
      for (const row of fixture.wireVectors) {
        const value = hydrate(row.value);
        const bytes = api.encodeActorReturnDrive(value);
        assert.equal(Buffer.from(bytes).toString("hex"), row.hex);
        assert.deepStrictEqual(api.decodeActorReturnDrive(bytes), value);
      }
      for (const row of fixture.resultVectors) {
        const value = hydrate(row.value);
        const bytes = api.encodeActorReturnResult(value);
        assert.equal(Buffer.from(bytes).toString("hex"), row.hex);
        assert.deepStrictEqual(api.decodeActorReturnResult(bytes), value);
      }
      for (const row of fixture.pageResultVectors) {
        const payload = Uint8Array.from({ length: row.pageLength }, (_, index) => row.pattern === "mod37plus11" ? (index * 37 + 11) % 256 : 0);
        const receipt = hydrate(row.receipt);
        const value = { kind: "page", receipt, page: pageApi.createActorBytePage(payload) };
        const bytes = api.encodeActorReturnResult(value);
        const expectedPage = Buffer.alloc(fixture.maximumPageBytes);
        expectedPage.set(payload);
        const expected = Buffer.concat([Buffer.from(row.prefixHex, "hex"), expectedPage]);
        assert.equal(bytes.length, row.wireBytes);
        assert.deepStrictEqual(Buffer.from(bytes), expected);
        const decoded = api.decodeActorReturnResult(bytes);
        assert.equal(decoded.kind, "page");
        assert.deepStrictEqual(decoded.receipt, receipt);
        assert.deepStrictEqual(pageApi.readActorBytePage(decoded.page), payload);
      }
    `;
    const environment = { ...process.env }; delete environment.NO_COLOR;
    const child = spawnSync("node", ["--experimental-strip-types", "--input-type=module", "--eval", program], { encoding: "utf8", env: environment });
    if (child.status !== 0) throw new Error(child.stderr);
    expect({ status: child.status, signal: child.signal, stdout: child.stdout }).toEqual({ status: 0, signal: null, stdout: "" });
  });

  it("ActorReturnDrive matches the shared canonical vectors and independent LEB128 bytes", async () => {
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    const { default: schema } = await import("../../../🧬️schema/🔣️.json");
    const { default: lifetimeSchema } = await import("../../../../🚪️lifetime/🧬️schema/🔣️.json");
    const { valueSchema } = await import("@semio-tech/value-ts/schema");
    const { default: pageSchema } = await import("../../../../📃️page/🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true }).addSchema(valueSchema).addSchema(lifetimeSchema).addSchema(pageSchema).addSchema(schema);
    expect(ACTOR_RETURN_ORIGIN_MAXIMUM_BYTES).toBe(fixture.maximumOriginBytes);
    expect(ACTOR_RETURN_IDENTITY_MAXIMUM_BYTES).toBe(fixture.maximumIdentityBytes);
    expect(ACTOR_RETURN_PAGE_RECEIPT_MAXIMUM_BYTES).toBe(fixture.maximumPageReceiptBytes);
    expect(ACTOR_RETURN_CONTROL_MAXIMUM_BYTES).toBe(fixture.maximumControlBytes);
    expect(ACTOR_RETURN_DRIVE_MAXIMUM_BYTES).toBe(fixture.maximumDriveBytes);
    const moduleName = "@webassemblyjs/leb128/lib/leb.js";
    const module: unknown = await import(moduleName);
    const oracle: unknown = module && typeof module === "object" ? Reflect.get(module, "default") ?? module : null;
    const encode: unknown = oracle && typeof oracle === "object" ? Reflect.get(oracle, "encodeUIntBuffer") : null;
    if (typeof encode !== "function") throw new Error("missing independent LEB128 oracle");
    for (const row of fixture.wireVectors) {
      const drive = hydrate(row.value);
      const bytes = encodeActorReturnDrive(drive);
      expect(Buffer.from(bytes).toString("hex")).toBe(row.hex);
      expect(decodeActorReturnDrive(bytes)).toEqual(drive);
      const identity = drive.kind === "execute" ? null : drive.control.kind === "inputAck" ? drive.control.receipt.identity : drive.control.identity;
      const origin = drive.kind === "execute" ? drive.origin : identity!.origin;
      const values = [origin.activationGeneration, BigInt(origin.requestSequence), ...(identity ? [identity.returnSequence] : [])];
      const tag = drive.kind === "execute" ? [fixture.driveTags.execute] : [fixture.driveTags.control, fixture.controlTags[drive.control.kind]];
      const receipt = drive.kind === "control" && drive.control.kind === "inputAck" ? drive.control.receipt : null;
      if (receipt) values.push(receipt.pageSequence, BigInt(receipt.length));
      const independent = values.map(value => { const buffer = Buffer.alloc(8); buffer.writeBigUInt64LE(value); return Buffer.from(encode(buffer)); });
      expect(Buffer.concat([Buffer.from(tag), ...independent, ...(receipt ? [Buffer.from([receipt.final ? 1 : 0])] : [])])).toEqual(Buffer.from(bytes));
      for (let length = 0; length < bytes.length; length++) expect(() => decodeActorReturnDrive(bytes.subarray(0, length))).toThrow();
      const offset = new Uint8Array(bytes.length + 3); offset.set(bytes, 2);
      expect(decodeActorReturnDrive(offset.subarray(2, bytes.length + 2))).toEqual(drive);
    }
    expect(fixture.wireVectors.at(-1)!.hex.length / 2).toBe(ACTOR_RETURN_DRIVE_MAXIMUM_BYTES);
  });

  it("ActorReturnDrive rejects malformed, noncanonical and trailing input without mutating the source", async () => {
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    for (const hex of fixture.malformedWire) {
      const bytes = Uint8Array.from(Buffer.from(hex, "hex")); const original = bytes.slice();
      expect(() => decodeActorReturnDrive(bytes)).toThrow(); expect(bytes).toEqual(original);
    }
    const valid = encodeActorReturnDrive(hydrate(fixture.wireVectors[0]!.value));
    for (const tail of [0, 1, 255]) expect(() => decodeActorReturnDrive(Uint8Array.from([...valid, tail]))).toThrow();
    expect(() => decodeActorReturnDrive(new Uint8Array(fixture.maximumDriveBytes + 1))).toThrow();
    for (const value of [null, undefined, [], {}, "000709"]) expect(() => decodeActorReturnDrive(value as Uint8Array)).toThrow();
  });

  it("ActorReturnDrive enforces exact unsigned authority, safe transport ids and final-page length", () => {
    const origin = { activationGeneration: 7n, requestSequence: 9 };
    const identity = { origin, returnSequence: 11n };
    const receipt = { identity, pageSequence: 1n, length: 3, final: true };
    const makeAck = (value: ActorReturnPageReceipt): ActorReturnDrive => ({ kind: "control", control: { kind: "inputAck", receipt: value } });
    for (const value of [0n, -1n, 0x10000000000000000n, 1, "1", null, undefined]) {
      expect(() => encodeActorReturnDrive({ kind: "execute", origin: { ...origin, activationGeneration: value as bigint } })).toThrow();
      expect(() => encodeActorReturnDrive(makeAck({ ...receipt, identity: { ...identity, returnSequence: value as bigint } }))).toThrow();
      expect(() => encodeActorReturnDrive(makeAck({ ...receipt, pageSequence: value as bigint }))).toThrow();
    }
    for (const requestSequence of [0, -1, 1.5, Number.MAX_SAFE_INTEGER + 1, Infinity, NaN, 9n, "9"]) expect(() => encodeActorReturnDrive({ kind: "execute", origin: { ...origin, requestSequence: requestSequence as number } })).toThrow();
    for (const length of [-1, 4097, 0.5, Infinity, NaN, 3n, "3"]) expect(() => encodeActorReturnDrive(makeAck({ ...receipt, length: length as number }))).toThrow();
    const invalidFinals: readonly unknown[] = [0, 1, "true", null, undefined];
    for (const final of invalidFinals) expect(() => encodeActorReturnDrive(makeAck({ ...receipt, final: final as boolean }))).toThrow();
    expect(() => encodeActorReturnDrive(makeAck({ ...receipt, length: 0, final: false }))).toThrow();
    expect(decodeActorReturnDrive(encodeActorReturnDrive(makeAck({ ...receipt, length: 0, final: true })))).toEqual(makeAck({ ...receipt, length: 0, final: true }));
    for (const kind of ["input-ack", "retired-ack", "unknown"]) expect(() => encodeActorReturnDrive({ kind: "control", control: { kind, identity } } as ActorReturnDrive)).toThrow();
  });
  return { it, resultOracle, hydrateResult, encodeActorReturnResult, expect, decodeActorReturnResult, ACTOR_RETURN_RESULT_MAXIMUM_BYTES, createActorBytePage, readActorBytePage, decodeActorReturnDrive, vi };
}
