import type { CadDiff } from "../🟦️.ts";

/** 🔺️ cad.diff text facade. `CadDiff` never implements `store::ArtifactDsl` anywhere in the cad
 * plugin — the sibling `🦀️.rs` only implements `apply_to_artifact`/`MutationDiff`; this
 * facet's `📖️.grammar.semio` is registered for LSP/tooling only, the same explicitly
 * documented case as `🗒️note/…/🚪️io/🔺️diff/📝️text/🦀️.rs`. There is nothing on the Rust
 * side to wire this to — parse/print for cad diff text do not exist, by design, not by omission. */
export function parseDsl(text: string): CadDiff {
  throw new Error("cad.diff has no text codec: CadDiff never implements ArtifactDsl (grammar is registered for tooling only, never parsed)");
}
export function printDsl(value: CadDiff): string {
  throw new Error("cad.diff has no text codec: CadDiff never implements ArtifactDsl (grammar is registered for tooling only, never parsed)");
}
