/** 🧩 layout 📡️spr WASM facade — encode/decode delegates to the plugin Rust crate. */
export function encode(value: unknown): Uint8Array {
  throw new Error("wire layout 📡️spr encode to plugin WASM");
}
export function decode(bytes: Uint8Array): unknown {
  throw new Error("wire layout 📡️spr decode to plugin WASM");
}
