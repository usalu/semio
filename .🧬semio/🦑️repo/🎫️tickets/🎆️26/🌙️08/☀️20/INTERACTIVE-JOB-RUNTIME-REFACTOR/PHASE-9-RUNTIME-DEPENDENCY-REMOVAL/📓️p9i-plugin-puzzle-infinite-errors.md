# Phase 9i — Owned Plugin, Puzzle, and Infinite Errors

## Scope

This packet removes direct `thiserror` ownership from the assigned plugin surfaces, Puzzle, and Infinite without changing error text, conversion behavior, or source chaining. It also repairs the assigned OS-kernel SPR lib-test seams after the mutation and wire primitives became synchronous.

## Error ownership

The following direct package dependencies were removed only after their Rust source and manifest census reached zero:

- `semio-s-plugin-animate`
- `semio-s-plugin-sequence`
- `semio-s-plugin-architect`
- `semio-s-plugin-lowpoly`
- `semio-s-plugin-reasoning-mindmap`
- `semio-s-plugin-layout`
- `semio-s-plugin-norm`
- `semio-s-plugin-imperative`
- `semio-s-plugin-trinity`
- `semio-s-plugin-trinity-jack-shell`
- `semio-s-plugin-dag`
- `semio-s-plugin-draw-fsm`
- `semio-s-plugin-puzzle`
- `semio-framework-os-infinite`

Each former derive now has an owned `Display` and `Error` implementation. Former `#[from]` variants have explicit `From` implementations. Former named `#[source]` fields return the wrapped error directly. Former transparent variants preserve transparent semantics by delegating `Display` and `Error::source` to the inner error rather than introducing a new source layer.

The final scoped census command searched `thiserror`, `#[error]`, `#[from]`, and `#[source]` across every assigned Rust source and `Cargo.toml`; it returned no matches.

## Puzzle and Infinite

Puzzle now owns `Puzzle3dError` and the transparent `Puzzle5dError` delegation. Infinite now owns `DagError`, `NormalPortError`, `UndirectedGraphError`, and `CanvasError`. Exact messages, debug formatting, JSON conversions, and JSON source behavior are preserved.

Evidence:

- `cargo check -p semio-framework-os-infinite --lib` with the isolated Phase 7 target: passed.
- `cargo test -p semio-framework-os-infinite --lib board_fill`: compilation is blocked outside this conversion by 15 existing directed-DAG test errors (`vcs` is not linked, old `MutationOutcome` methods, and stale async `ArtifactStore` construction). The production library itself compiles.
- Puzzle's authoritative Nx gate remains queued behind the shared stdio/plugin-runtime compiler wall; the two converted source files and manifest have a zero census.

## SPR lib-test de-async repair

The assigned `📡️spr/🧪️testkit` fixture implementations now match the synchronous `MutationDiff`, `Mutation`, `OpText`, and `DiffAlgebra` traits. Stale awaits were removed from synchronous mutation outcomes, messages, policy probes, hybrid timestamps, and test chains. The assigned `📡️spr/📜️history` message tests no longer await synchronous `ByteWriter`, `ByteReader`, or byte-vector operations.

The follow-up umbrella packet also makes the VCS collection apply/inverse/diff helpers genuinely synchronous, aligns their `Patchable` fixtures, removes stale IO test awaits on synchronous keys/grammar/iterators/registry validation, and removes the channel fixture's stale hybrid-timestamp await. This restores the SPR command collection-law tests to concrete mutation/diff values instead of discarded or inspected futures.

Evidence:

- Filtered `cargo test -p semio-framework-os-kernel --lib --no-run --message-format short` diagnostics for `spr/testkit`, `spr/history`, IO, SPR command, VCS, and SPR channel: zero.
- `cargo check -p semio-framework-os-kernel --lib --message-format short`: passed.
- The umbrella lib-test compilation now has 232 errors outside the assigned files: DSL 174, inference 27, Store 26, SPR materialize 3, SPR facade 1, and pack CLI 1.

## Plugin gates

- `CARGO_TARGET_DIR=…/🧪️target-plugin-errors bun nx run @semio-tech/draw-fsm:test-quick`: passed, 26/26.
- `cargo check -p semio-s-plugin-architect --lib` reaches the shared stdio dependency and is blocked by two unrelated PDF declaration errors where `resolve_ready` wraps an already-synchronous `ArtifactDeclarationBuilder<DeclarationReady>`.
- Standalone `rustfmt --check` parsed the changed large component files but reported broad pre-existing formatting differences; it is not recorded as a passing formatting gate.

No stdio, PDF, renderer glue, plugin framework, plugin host, WFC, or Energy source was edited by this packet.
