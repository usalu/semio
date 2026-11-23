import { RuntimeCapabilityMissingV1, runtimeMessageV1 } from "../../../🗣️messages/🟦️.ts";
import type { RuntimeNativeCallbackV1, RuntimeNativeLibraryV1, RuntimeNativePointerV1, RuntimeNativeReadV1, RuntimeNativeSymbolV1, RuntimeNativeSystemV1, RuntimeNativeTypeV1 } from "../../../📋️contract/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

const protocol = schema.definitions.Protocol.const;
const rows = protocol.types as Readonly<Record<string, Readonly<{ kind: string; bits?: number; signed?: boolean }>>>;

/** 🔩️ The seven primitives of the Node native core: libraries and symbols, one integer-class call, buffer addresses and raw memory, all as exact 64-bit words. */
export interface NodeNativeCoreV1 {
  open(path: string): bigint;
  symbol(handle: bigint, name: string): bigint;
  close(handle: bigint): bigint;
  call(address: bigint, ...words: bigint[]): bigint;
  addressOf(view: ArrayBufferView): bigint;
  peek(address: bigint, length: bigint): ArrayBuffer;
  view(address: bigint, length: bigint): ArrayBuffer;
}

function refuse(id: string, parameters: Readonly<Record<string, string | number>> = {}): TypeError { return new TypeError(runtimeMessageV1(id, parameters)); }

/** 🧱️ Looks up the declared width and class of a native type. */
export function nodeNativeTypeV1(type: string): Readonly<{ kind: string; bits?: number; signed?: boolean }> {
  const row = rows[type];
  if (row === undefined) throw refuse("nativeType", { type });
  return row;
}

/** 📥️ Turns one JavaScript argument into the 64-bit word the callee reads; buffers that must outlive the call are pushed to `keep`. */
export function nodeNativeWordV1(core: Pick<NodeNativeCoreV1, "addressOf">, type: string, value: unknown, keep: unknown[]): bigint {
  const row = nodeNativeTypeV1(type);
  if (row.kind === "bool") return value ? 1n : 0n;
  if (row.kind === "int") return integerWord(type, value);
  if (row.kind === "pointer" || row.kind === "string") {
    if (value === null || value === undefined) return 0n;
    if (typeof value === "string" && row.kind === "string") { const bytes = Buffer.from(`${value}\0`, "utf8"); keep.push(bytes); return core.addressOf(bytes); }
    if (typeof value === "number") return Number.isFinite(value) && value >= 0 ? BigInt.asUintN(64, BigInt(Math.trunc(value))) : 0n;
    if (typeof value === "bigint") return BigInt.asUintN(64, value);
    if (ArrayBuffer.isView(value)) { keep.push(value); return core.addressOf(value); }
    if (value instanceof ArrayBuffer) { const view = new Uint8Array(value); keep.push(view); return core.addressOf(view); }
    throw refuse("nativeArgument", { type, kind: typeof value });
  }
  throw refuse("nativeFloat", { type });
}

function integerWord(type: string, value: unknown): bigint {
  if (typeof value === "bigint") return BigInt.asUintN(64, value);
  if (typeof value === "boolean") return value ? 1n : 0n;
  if (typeof value === "number") return Number.isFinite(value) ? BigInt.asUintN(64, BigInt(Math.trunc(value))) : 0n;
  if (value === null || value === undefined) return 0n;
  throw refuse("nativeArgument", { type, kind: typeof value });
}

/** 📤️ Turns the 64-bit return register into the JavaScript value of the declared type, truncated and sign-extended exactly like the Bun binding. */
export function nodeNativeResultV1(core: Pick<NodeNativeCoreV1, "peek">, type: string, word: bigint): unknown {
  const row = nodeNativeTypeV1(type);
  if (row.kind === "void") return undefined;
  if (row.kind === "bool") return (word & 0xffn) !== 0n;
  if (row.kind === "int") {
    const bits = row.bits!, narrowed = row.signed ? BigInt.asIntN(bits, word) : BigInt.asUintN(bits, word);
    return bits === 64 ? narrowed : Number(narrowed);
  }
  if (row.kind === "pointer") return word === 0n ? null : Number(word);
  if (row.kind === "string") return word === 0n ? null : cstringAt(core, word);
  throw refuse("nativeFloat", { type });
}

function cstringAt(core: Pick<NodeNativeCoreV1, "peek">, address: bigint): string {
  const bytes: number[] = [];
  for (let offset = 0n; offset < BigInt(protocol.cstringMaximumBytes); offset++) {
    const byte = new Uint8Array(core.peek(address + offset, 1n))[0]!;
    if (byte === 0) return Buffer.from(bytes).toString("utf8");
    bytes.push(byte);
  }
  throw refuse("nativeLength", { limit: protocol.cstringMaximumBytes });
}

function reader(core: Pick<NodeNativeCoreV1, "peek">): RuntimeNativeReadV1 {
  const at = (width: number, pointer: RuntimeNativePointerV1, offset = 0): DataView => new DataView(core.peek(BigInt(pointer) + BigInt(offset), BigInt(width)));
  const methods: RuntimeNativeReadV1 = {
    i8: (pointer, offset) => at(1, pointer, offset).getInt8(0),
    u8: (pointer, offset) => at(1, pointer, offset).getUint8(0),
    i16: (pointer, offset) => at(2, pointer, offset).getInt16(0, true),
    u16: (pointer, offset) => at(2, pointer, offset).getUint16(0, true),
    i32: (pointer, offset) => at(4, pointer, offset).getInt32(0, true),
    u32: (pointer, offset) => at(4, pointer, offset).getUint32(0, true),
    i64: (pointer, offset) => at(8, pointer, offset).getBigInt64(0, true),
    u64: (pointer, offset) => at(8, pointer, offset).getBigUint64(0, true),
    ptr: (pointer, offset) => Number(at(8, pointer, offset).getBigUint64(0, true)),
  };
  return Object.freeze(methods);
}

function bind(core: NodeNativeCoreV1, address: bigint, symbol: RuntimeNativeSymbolV1, alive: () => void): (...arguments_: unknown[]) => unknown {
  return (...arguments_) => {
    alive();
    const keep: unknown[] = [], words = symbol.args.map((type, index) => nodeNativeWordV1(core, type, arguments_[index], keep));
    const word = core.call(address, ...words);
    keep.length = 0;
    return nodeNativeResultV1(core, symbol.returns, word);
  };
}

/** 🔌️ Projects a native core onto the `nativeSystemV1` contract: the same library, symbol, memory and pointer surface the Bun binding offers, with integer-class symbols of at most eight arguments. */
export function createNodeNativeSystemV1(core: NodeNativeCoreV1): RuntimeNativeSystemV1 {
  const types = Object.freeze(Object.fromEntries(Object.keys(rows).map(name => [name, name])) as Record<RuntimeNativeTypeV1, RuntimeNativeTypeV1>);
  const dlopen = <Symbols extends Readonly<Record<string, RuntimeNativeSymbolV1>>>(path: string, symbols: Symbols): RuntimeNativeLibraryV1<Symbols> => {
    for (const [name, symbol] of Object.entries(symbols)) {
      if (symbol.args.length > protocol.words) throw refuse("nativeArity", { name, limit: protocol.words });
      for (const type of [...symbol.args, symbol.returns]) if (nodeNativeTypeV1(type).kind === "float") throw refuse("nativeFloat", { type });
    }
    const handle = core.open(path);
    let closed = false;
    const alive = (): void => { if (closed) throw refuse("nativeClosed", { path }); };
    const bound: Record<string, (...arguments_: unknown[]) => unknown> = {};
    try {
      for (const [name, symbol] of Object.entries(symbols)) {
        let address: bigint;
        try { address = core.symbol(handle, name); } catch { throw new TypeError(runtimeMessageV1("nativeSymbol", { name, path })); }
        bound[name] = bind(core, address, symbol, alive);
      }
    } catch (error) { core.close(handle); throw error; }
    return { symbols: Object.freeze(bound) as RuntimeNativeLibraryV1<Symbols>["symbols"], close: () => { if (closed) return; closed = true; core.close(handle); } };
  };
  const callbacks = class {
    readonly pointer: RuntimeNativePointerV1 = 0;
    constructor(_body: (...arguments_: any[]) => any, _signature: RuntimeNativeSymbolV1) { throw new RuntimeCapabilityMissingV1("ffi", "node", runtimeMessageV1("reasonNativeCallback", {})); }
    close(): void {}
  } satisfies new (body: (...arguments_: any[]) => any, signature: RuntimeNativeSymbolV1) => RuntimeNativeCallbackV1;
  const system: RuntimeNativeSystemV1 = {
    FFIType: types,
    dlopen,
    read: reader(core),
    ptr: view => Number(core.addressOf(view)),
    toArrayBuffer: (pointer, byteOffset = 0, byteLength) => {
      if (byteLength === undefined) throw refuse("nativeByteLength");
      return core.view(BigInt(pointer) + BigInt(byteOffset), BigInt(byteLength));
    },
    JSCallback: callbacks,
  };
  return Object.freeze(system);
}
