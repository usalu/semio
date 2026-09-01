/** 📡️ cad.mutation op-binary facade. Real codec: `encode_op`/`decode_op` in the sibling
 * `🦀️component.rs`, wrapping `protocol::OpBinary for CadMutation` (callable natively from Rust
 * today). Not wired here — see the 📝️text facade's doc comment for why: no stateless codec-call
 * WIT export on `world actor` (poll/jobs/checkpoint/describe only, per the B1 world-collapse);
 * needs one plus a TS host loader. See 📓️wasm-facade-wiring.md. */
export function encode(value: unknown): Uint8Array {
  throw new Error("cad.mutation encode: no WASM codec-call export exists (world actor only exports poll/jobs/checkpoint/describe); wire once one is added");
}
export function decode(bytes: Uint8Array): unknown {
  throw new Error("cad.mutation decode: no WASM codec-call export exists (world actor only exports poll/jobs/checkpoint/describe); wire once one is added");
}
