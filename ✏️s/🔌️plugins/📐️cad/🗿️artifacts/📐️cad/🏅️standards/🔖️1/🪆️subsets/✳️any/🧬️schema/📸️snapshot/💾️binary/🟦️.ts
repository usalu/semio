import type { CadSnapshot } from "../🟦️.ts";

/** 🎒️ cad.cad snapshot binary facade. Real codec: `encode`/`decode` in the sibling
 * `🦀️.rs`, wrapping `store::ArtifactPack for CadSnapshot` (callable natively from Rust
 * today). Not wired here — see the 📝️text facade's doc comment for why: no stateless codec-call
 * WIT export on `world actor` (poll/jobs/checkpoint/describe only, per the B1 world-collapse);
 * needs one plus a TS host loader. See 📓️wasm-facade-wiring.md. */
export function encode(value: CadSnapshot): Uint8Array {
  throw new Error("cad.cad encode: no WASM codec-call export exists (world actor only exports poll/jobs/checkpoint/describe); wire once one is added");
}
export function decode(bytes: Uint8Array): CadSnapshot {
  throw new Error("cad.cad decode: no WASM codec-call export exists (world actor only exports poll/jobs/checkpoint/describe); wire once one is added");
}
