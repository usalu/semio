# Forms Resumable Try Value

## Independent Audit Closure

The later P8j public-boundary, multi-document, abandoned-input, restart, cancellation, and admission findings are closed in `📓️p8k-draw-flow-forms-audit-repair.md`. That report is authoritative for the final Draw/Flow/Forms audit-ready source state and explicitly separates executed non-Cargo gates from unexecuted Rust runtime tests.

## P8i Final Source Closure (2026-08-22)

The four P8i source-audit gaps are closed in the Forms source:

1. Scalar `value_json` no longer enters `serde_json::from_str`/`to_string` in the initiating handler. It enters the same admission-controlled session and is emitted as one replayable <=4,096-byte stage operation per hidden continuation.
2. Option and object edits no longer join the stored answer chunks or materialize a JSON array/map. The continuation loads one content chunk per turn, scans/copies <=4,096 bytes, compares only a bounded <=4,096-byte key/option token, and emits no authoritative config mutation until the compact commit. Multi-option removal repeats resumably until all duplicate values are removed. The pressed-state payload is itself scanned <=4,096 bytes per turn.
3. `setTryValues` no longer parses/clones an object map, loops every entry, or emits every operation from one handler. It uses the hidden `setTryValueStep` action to scan <=4,096 input bytes, stage one <=4,096 value chunk, verify one collision chunk, or stage one bounded entry per turn. Entry operations build an ephemeral persistent root seeded by an O(1) clone of the base root. One `CommitTryValuesBatch` swaps that root atomically, preserves unrelated base keys, and is the only authoritative mutation.
4. Collision authorization is now a replayable `VerifyTryValueChunk` operation rather than ephemeral process state. A fresh-process replay can reconstruct staged values, the prepared bulk root, and its one compact commit. Cancellation/supersession discards both value staging and bulk-root staging; epoch plus generation/cursor checks reject ABA/stale continuations; the 65th direct session admission remains an explicit `forms.try-value.busy` fault.

Every continuation uses a fixed byte/component cap and the same hidden framework action; there is no thread-local store, private executor, opaque worker batch, whole existing-value join, whole bulk-map parse, or all-events-at-once producer in the production command paths. Option/parameter identifiers are rejected before encoding when their input exceeds 512 bytes, bounding worst-case JSON escaping below the 4 KiB operation cap. Bulk keys are rejected above 4 KiB.

### Source-Authored Adversarial Coverage

- 512 KiB scalar through the production start handler, replayable binary operation encode/decode, diff, and apply.
- 40,000-entry existing option array with 256 KiB of whitespace in the pressed payload, using bounded source loading and textual membership editing.
- Existing object with a 512 KiB unrelated field plus a 512 KiB inserted object value.
- Bulk object containing two independent 512 KiB leaves, option/object leaves, and a scalar tail through scan, stage, binary operation encode/decode, diff/apply, and one atomic batch-root commit.
- Fresh-process replay of the complete bulk operation log, worker-thread session handoff, mid-flight cancellation, stale generation/epoch, 65th admission rejection, vector growth, and forced 32-chunk content-id collision/dedupe.

Each authored full-path test records the worst individual handler or operation encode/decode/diff/apply step and asserts `< 8 ms`. These Rust tests are source-authored but intentionally not reported as executed: the WFC/procedural lane still owns Cargo. Cargo/native/release/Wasm runtime evidence remains a Phase-8 integration gate, not a source-closure claim.

### Executed Non-Cargo Evidence

- `bun ./📜️script.ts verify interactivity tool-jobs --format json`: 775/775 bounded, zero batch-only, zero forbidden, zero deleted, zero failures (3.6 s).
- Focused `rustfmt --edition 2021 --check`: clean for Forms config, `set-try-value`, and `set-try-values`.
- All Forms JSON files parse with `jq empty`.
- `git diff --check -- ✏️s/🔌️plugins/📋️forms`: clean.

The older audit narrative below is retained as historical context; its “Remaining P8i Source Gaps” list is superseded by this closure section.

## P8i Source-Audit Repair (Authoritative)

The former whole-map JSON config was replaced by a schema-first per-key `FormsTryValues` representation across the Rust, TypeScript, GraphQL, JSON Schema, Proto, artifact, diff, UI, and committed fixture leaves. Config clones are persistent-root clones; a single answer update path-copies only its key path and never parses, serializes, or clones unrelated answers.

Large vector output is a chunked rope. A continuation scans/copies at most 4,096 bytes or writes at most 64 components. Final persistence is replayable event-sourced staging: each command-log operation carries at most one 4,096-byte chunk, while `CommitTryValue` carries only the key, staging id, 256-bit-plus-length content id, and chunk count. The compact commit swaps one per-key content reference, yielding one coalesced undo row. Partial progress never changes the authoritative `try_values` root. Cancellation/supersession logs bounded staging discard and leaves config unchanged.

Freshness uses a process-monotone epoch plus generation/cursor checks, so snapshot inverse/undo cannot create an ABA. Admission rejects the 65th live operation with `forms.try-value.busy`; it never evicts active work. Existing-content equality is established by one <=4 KiB comparison continuation per chunk before compact commit. A forced-content-id regression covers a 32-chunk distinct collision and identical dedupe, timing every verify step and the compact commit below 8 ms.

### Remaining P8i Source Gaps

- Option and object-parameter commands still reconstruct and parse the complete existing target value in the initiating handler. This is independent of unrelated-map size but is not bounded for an adversarially large target.
- The scalar/option/object initiating handler still parses a user-supplied `value_json` as one JSON value before the chunk-staging continuation.
- `setTryValues` remains a bulk producer and can emit all keys/chunks in one command; it has not yet been routed through the same admission-controlled per-key continuation.
- The production handler -> command-log encode -> apply timing coverage is source-authored for stage/commit primitives, but the requested real worker-hop/cancellation-storm/genesis-replay suite has not been executed because Cargo is held.

These gaps mean P8i Forms is not yet fully source-acceptable despite the interactivity declaration verifier being green.

## Outcome

`setTryValue` preserves its scalar, multi-option, object-parameter, and vector shapes while removing both hidden sources of user-sized work. Scalar, option, and object changes retain their immediate behavior. Every vector change, including an already-sized vector, starts the hidden non-palette `setTryValueStep` continuation.

The continuation now scans and copies at most 4,096 UTF-8 bytes per dispatch and materializes at most 64 new vector components per preview. It locates the target field and vector element directly in the existing JSON, preserves unrelated bytes, incrementally builds a valid replacement, and moves the completed string into the config mutation. It never parses or serializes the complete unrelated map/vector in one continuation. Missing fields, non-array targets, and malformed top-level input retain the former best-effort fallback behavior.

Continuation arguments retain generation, cumulative cursor, and target index as explicit progress. A monotone `try_values_revision` config field replaces whole-JSON equality checks: every partial preview advances it, and a stale or externally superseded session emits neither a config mutation nor another effect. Starting a newer try-value gesture still cancels pending sessions, the session registry remains capped at 64 entries, and inverse snapshots restore the revision together with the rest of the config. Rust, TypeScript, GraphQL, JSON Schema, and Proto config leaves were updated together.

## Focused Regression Coverage

- A 256,000-byte unrelated config value plus a 40,000-element existing vector proves each scan/copy slice stays within 4,096 bytes, each growth slice within 64 components, and each measured source step below 8 ms.
- A 10,001-element growth regression proves partial previews preserve unrelated suffix objects and never add more than 64 components per slice.
- Missing, non-array, and malformed target fixtures preserve best-effort semantics.
- Scalar, selected-option, and object-parameter fixtures preserve the existing heterogeneous JSON shapes.
- A stale-revision fixture proves the stale step emits no preview mutation or continuation and removes its session.

## Static Validation

`bun ./📜️script.ts verify interactivity tool-jobs --format json` passes after the post-audit repair:

- 774 command rows / 772 unique rows
- all 774 rows bounded
- 0 `BatchOnlyPendingRewrite` rows
- 0 forbidden or deleted rows
- 0 failures

The latest post-repair run on 2026-08-22 reports 775 command rows, all 775 bounded, zero batch-only/forbidden/deleted rows, and zero failures. Focused `rustfmt` completed. No Cargo command ran.

Focused `rustfmt --check`, whitespace validation, and the exact source census pass. The source census confirms the vector path enters before `try_values_map`, uses the two explicit work caps, and contains no session-wide `try_values_json_text` or parsed-map/vector round trip.

## Deferred Cargo Gates

P4 exclusively owns the Cargo lane, so the source-added timing/behavior tests were not executed and no Cargo command or lockfile mutation was performed. The exact deferred gates remain:

1. `cargo test -p semio-s-plugin-forms set_try_value --lib`
2. `cargo check -p semio-s-plugin-forms --target wasm32-wasip2`
3. `cargo clippy -p semio-s-plugin-forms --target wasm32-wasip2 -- -D warnings`
4. The master Phase 8 ActionBus/tool-job quick suite after the Cargo lane is released.
