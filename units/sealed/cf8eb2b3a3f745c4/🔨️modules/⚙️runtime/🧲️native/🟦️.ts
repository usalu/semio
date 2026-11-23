import { requireRuntimeMemberV1 } from "../🧭️selection/🟦️.ts";
import type { RuntimeNativeCallbackV1, RuntimeNativeLibraryV1, RuntimeNativePointerV1, RuntimeNativeReadV1, RuntimeNativeSymbolV1, RuntimeNativeSystemV1, RuntimeNativeTypeV1 } from "../📋️contract/🟦️.ts";

const NATIVE_TYPES: Readonly<Record<RuntimeNativeTypeV1, RuntimeNativeTypeV1>> = Object.freeze({ void: "void", bool: "bool", i8: "i8", u8: "u8", i16: "i16", u16: "u16", i32: "i32", u32: "u32", i64: "i64", u64: "u64", f32: "f32", f64: "f64", ptr: "ptr", cstring: "cstring" });

/** 🔌️ Opens a native library. */
export function runtimeNativeOpenV1<Symbols extends Readonly<Record<string, RuntimeNativeSymbolV1>>>(path: string, symbols: Symbols): RuntimeNativeLibraryV1<Symbols> { return requireRuntimeMemberV1("ffiOpen")(path, symbols); }
/** 🔌️ Gives typed reads at machine addresses. */
export function runtimeNativeReadV1(): RuntimeNativeReadV1 { return requireRuntimeMemberV1("ffiRead")(); }
/** 🔌️ Gives the address of a buffer view. */
export function runtimeNativePointerV1(view: ArrayBufferView): RuntimeNativePointerV1 { return requireRuntimeMemberV1("ffiPointer")(view); }
/** 🔌️ Creates a callback that foreign code can call. */
export function runtimeNativeCallbackV1(body: (...arguments_: any[]) => any, signature: RuntimeNativeSymbolV1): RuntimeNativeCallbackV1 { return requireRuntimeMemberV1("ffiCallback")(body, signature); }
/** 🔌️ Gives the whole native-binding surface as one object, for modules that bind many symbols. */
export function runtimeNativeSystemV1(): RuntimeNativeSystemV1 {
  const system: RuntimeNativeSystemV1 = {
    FFIType: NATIVE_TYPES,
    dlopen: runtimeNativeOpenV1,
    read: runtimeNativeReadV1(),
    ptr: runtimeNativePointerV1,
    toArrayBuffer: (pointer, byteOffset, byteLength) => requireRuntimeMemberV1("ffiToArrayBuffer")(pointer, byteOffset, byteLength),
    JSCallback: class { readonly pointer: RuntimeNativePointerV1; private readonly handle: RuntimeNativeCallbackV1; constructor(body: (...arguments_: any[]) => any, signature: RuntimeNativeSymbolV1) { this.handle = runtimeNativeCallbackV1(body, signature); this.pointer = this.handle.pointer; } close(): void { this.handle.close(); } },
  };
  return Object.freeze(system);
}
