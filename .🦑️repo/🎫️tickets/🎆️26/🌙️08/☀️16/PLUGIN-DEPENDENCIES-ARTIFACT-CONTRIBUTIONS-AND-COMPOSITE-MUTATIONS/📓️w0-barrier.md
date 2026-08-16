# Wave 0 barrier — combined-tree verification

Run serially by the coordinator after all four W0 lanes reported. Start commit `7ad8955884`; the tree also absorbed auto-commit `3140b01d2c` (2026-08-16 02:17, the FULL-STDIO ticket's "Restructure glTF/OBJ/semio schemas … plugin builder, store, host, and I/O module event contract handling") mid-wave.

## Fixes the barrier required (coordinator, cross-lease)

| Fix | File | Why |
|---|---|---|
| `origin: MutationOrigin::Owner` on the remote-ingest `HistoryOpMeta` literal | `🏪️store/🔄️sync/🦀️component.rs:389` | W0-A flagged it as outside its lease; a remote-ingested envelope carries no provenance of its own, so it records an owner edit. |
| `"Hello"` and `"Welcome"` golden hex `0008…` → `0009…` | `📡️spr/🧵️channel/🦀️component.rs:1358,1394` | Both fixtures encode the live `CHANNEL_VERSION`, which W0-B bumped 8 → 9. The goldens pin the encoding, so a version bump legitimately rewrites them — and the failure proved the pin works. |
| `store::document_codec(schema)` → `let Ok(Some(codec)) = …` | `🔌️plugin/🖥️host/🦀️component.rs:90` | The concurrent ticket changed the document-codec registry to return `Result<Option<_>, DocumentCodecRegistryError>`. |
| `FormatDescriptor` claim construction now walks `mimes`/`extensions` | `🔌️plugin/🦀️component.rs:2692` | The concurrent ticket replaced `primary_mime()`/`primary_extension()` with plural `Vec<String>` fields. Claiming **every** mime and extension is also strictly more correct for a conflict-rejecting registry than claiming only the first. |

## Gate results

| Gate | Result |
|---|---|
| `cargo test -p semio-framework-os-kernel --lib` | **890 passed / 3 failed** — all three external (below). All 7 composite-law tests and both transaction-vector tests pass; the descriptor-fingerprint golden pin is unchanged. |
| `cargo test -p semio-framework --lib` | **128 passed / 1 failed** — the one failure is external (below). All 18 new manifest/version/graph tests pass. |
| `cargo check -p semio-framework-plugin -p semio-framework-plugin-host` | **clean** (warnings only) after the two adaptation fixes above. |
| TS `bunx vitest run` in `💻️os/📦️packages/🟦️typescript` | **244 passed / 4 failed**; `-t AppChannelCodec` → **116 passed, 0 failed**. The 4 failures are environmental: missing generated wire fixtures under `🏪️store/🔄️sync/…/🧫️fixtures/📡️wire/` and a missing `pkg/semio_framework_os.js` wasm build. |
| `cargo check -p semio-s-plugin-flow -p semio-s-plugin-cad -p semio-s-plugin-cad-aec-building` | **BLOCKED — not achievable at this barrier.** |

## External failures (attributed, not ours)

1. `os_io::tests::io_registry_rejects_a_conflicting_key…` and `io::tests::…` (same test, both crates) — `🚪️io/🦀️component.rs`, the concurrent ticket's conflict-rejecting IO registry work. No lane here touched `🚪️io`.
2. `os_dsl::fixture_sweep::m5_cross_artifact_rejection::all_non_stdio_grammars_reject_each_others_shipped_fixtures` — falls out of that ticket's glTF/OBJ/semio schema restructure.
3. `os_store::component::tests::switch_to_an_alternative_whose_pinned_checkpoint_is_missing_is_rejected` — a **new** stricter alternative/checkpoint validation ("alternative … has an invalid checkpoint reference") that does not exist in `git show HEAD:` and appeared in the worktree after commit `3140b01d2c`; it now rejects an older test fixture. Written by the concurrent session, not by any lane here.
4. **Guest builds blocked**: every plugin crate depends on `semio-s-plugin-stdio`, whose `📇️registry/🦀️component.rs` is a brand-new file (created 02:24, staged-added, never committed) currently mid-write by the concurrent ticket — 5 compile errors in it (`matches!(*state, "…")` against `String`, `?` converting `PluginAssemblyError` into `ArtifactDefinitionError`). Nothing in that file belongs to this ticket.

**Honest statement:** this ticket's four spines compile and pass their own tests; the sample *guest component* build could not be demonstrated at this barrier because the shared stdio crate is mid-refactor by another live session. That gate is deferred to the W1 barrier and must be re-run then — it is NOT claimed as passing.

## Verdict

Wave 0 closed. The spine is usable by Wave 1: `semio-framework-os-kernel`, `semio-framework`, `semio-framework-plugin` and `semio-framework-plugin-host` all compile against each other, which is everything W1-A/B/C need.
