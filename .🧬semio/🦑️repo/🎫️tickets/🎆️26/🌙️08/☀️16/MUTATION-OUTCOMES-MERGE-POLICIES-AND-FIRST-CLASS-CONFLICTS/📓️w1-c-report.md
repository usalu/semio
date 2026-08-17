# Lane 1-C (channel) — C8 report

## Tag table (Rust ↔ TS, identical ordinals)

| Enum | Variant | Tag | Fields (declaration order) |
|---|---|---|---|
| `AppCommand` | `SetMergePolicy` / `setMergePolicy` | 30 | `seq: u64, policy: u8` |
| `AppCommand` | `ResolveConflict` / `resolveConflict` | 31 | `seq: u64, conflict_id: String, resolution: u8` |
| `AppCommand` | `ReadConflicts` / `readConflicts` | 32 | `seq: u64` |
| `AppFrame` | `MergeReport` | 23 | `in_reply_to: Option<u64>, report: Vec<u8>` |
| `AppFrame` | `Conflicts` | 24 | `in_reply_to: Option<u64>, conflicts: Vec<u8>` |
| `AppFrame` | `Invocation` (tag 2, unchanged) | 2 | `in_reply_to, output, diagnostics, ui_scope, history_patch, +messages: Vec<u8>` (trailing) |
| `AppFrame` | `Error` (tag 13, unchanged) | 13 | `in_reply_to, fault, +report: Vec<u8>` (trailing) |
| `📡️wire::ApplyOutcome` | `Rejected` (tag 2, unchanged) | 2 | `reason: String, +messages: Vec<u8>` (trailing, opaque packed blob) |

`CHANNEL_VERSION` bumped 10 → 11 (Rust `component.rs:20`, JSON pin `channel-version.json`). TS `APP_CHANNEL_VERSION` constant lives in `AppChannelClient` (2-C's lease) — left untouched per handoff; their test will fail against the new pin until they bump it, as expected.

## Fixtures baked (hex-JSON, existing format)

- `🧫️fixtures/📡️channel/app-command-merge.json` — `SetMergePolicy`, `ResolveConflict`, `ReadConflicts` (3 vectors)
- `🧫️fixtures/📡️channel/app-frame-merge.json` — `MergeReport`, `Conflicts`, `Invocation` (extended), `Error` (extended) (4 vectors)
- `🧫️fixtures/📡️channel/channel-version.json` — pin updated to 11

Asserted on **both** sides:
- Rust: new test `channel_merge_fixtures_match_shared_cross_language_json_vectors` (next to `channel_opening_fixtures_match_shared_cross_language_json_vectors`, `component.rs`), plus the full-corpus tests (`app_command_fixture_corpus_matches_golden_hex_and_round_trips`, `app_frame_fixture_corpus_matches_golden_hex_and_round_trips`) and per-variant round-trip tests (`🔖️Merge` sub-regions under `AppCommand`/`AppFrame`).
- TS (`🟦️component.ts`, `AppChannelCodec` describe block): new test `"matches the shared cross-language merge fixture vectors, byte-for-byte"` (next to the opening/transaction ones), plus updated `sampleCommands`/`sampleFrames` round-trip arrays and the two "tags every variant" tests (now `...ReadConflicts=32` / `...Conflicts=24`).

## Test counts (real, both run to completion)

**Rust** (`cargo test -p semio-framework-os-kernel --lib -- os_spr::channel`): **69 passed; 0 failed; 0 ignored; 866 filtered out.** `cargo check -p semio-framework-os-kernel`: **0 errors** (9 pre-existing dead-code warnings, none mine). Crate-wide compile was transiently blocked ~15 min by peer lanes' in-flight C6/C7 work (unrelated files); resolved once they landed. Raw output in `🧪️w1-c-cargo.txt`.

**TS** (`bunx vitest run --config 🧪️vitest.config.ts` from `📦️packages/🟦️typescript` — equivalent to `bun nx run @semio-tech/framework-os:test`, invoked directly because 500+ concurrent sibling-lane `nx run @semio-tech/framework-os:test` processes were starving the nx-orchestrated path, confirmed via `ps aux`): **306 passed; 6 failed (3 distinct, doubled by duplicate file matching); 312 total.** Every `AppChannelCodec` test I own passes: round-trips, both "tags every variant" tests, the full golden-hex corpus test, and the transaction/opening/**new merge** fixture-vector tests. Raw output in `🧪️w1-c-ts.txt`.

The 3 distinct TS failures, none in my code:
1. `AppChannelCodec > pins APP_CHANNEL_VERSION against the shared cross-language channel version` — expects 11, gets 10. Exactly the documented handoff: `APP_CHANNEL_VERSION` lives in `AppChannelClient` (2-C's lease); I bumped only the JSON pin + Rust constant per my brief ("update the pin, do not delete the assertion"). Resolves once 2-C bumps their constant.
2. `AppChannelClient > command() allocates an incrementing seq...` — `TypeError: Cannot read properties of undefined (reading 'length')` in `writeBytes(frame.Invocation.messages)`. Root cause: `🟦️component.ts:3257`, inside the `AppChannelClient` describe block (2-C's lease), constructs `{ Invocation: { in_reply_to, output, diagnostics, ui_scope, history_patch } }` without the new trailing `messages` field. One-line fix (`messages: []`), out of my lease — reported, not edited.
3. `workflow > matches the Rust plan_workflow across shared fixtures decoded via wasm` — pre-existing, unrelated: `Cannot find module '.../🖥️host/📦️packages/🦀️rust/pkg/semio_framework_os.js'` (missing wasm build artifact). Nothing to do with C8.

## Blockers / out-of-lease items (reported, not edited)

1. `🟦️component.ts:3257` — `AppChannelClient` describe block (2-C's lease) constructs `AppFrame::Invocation` without the new `messages` field; runtime `TypeError` (see above). Needs `messages: []`.
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:1287` and `:1632` (feature = `sync`, one native + one wasm32-gated copy of `handle_ack`) and one `#[cfg(test)]` construction near line 2445 — all match/construct `ApplyOutcome::Rejected { reason }` without the new `messages` field. Not hit by the default `cargo check -p semio-framework-os-kernel` (feature-gated, confirmed via `cargo check --features sync`), so not blocking my acceptance gate, but will break a `--features sync` or wasm32 build. Trivial one-line fixes (`{ reason, .. }` / add `messages: Vec::new()`), outside my lease (unowned file, not in the lease table) — left untouched per "stop and report."

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📡️wire/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🟦️component.ts` (`AppChannelCodec` region + its tests only)
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/channel-version.json`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/app-command-merge.json` (new)
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📡️channel/app-frame-merge.json` (new)
