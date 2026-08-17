# Cross-ticket integration log

This ticket shares `🔌️plugin/🦀️component.rs`, `📡️spr/🧵️channel/🦀️component.rs`, `🏪️store/🦀️component.rs`, `🚪️io/🦀️component.rs`, `🛂️manifest/🦀️component.rs` and `🔣️taxonomy.json` with at least two other live sessions. Everything here was verified against start commit `7ad8955884` before being attributed.

## Concurrent tickets observed

| Ticket | Evidence | What it changed under us |
|---|---|---|
| `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS` | auto-commit `3140b01d2c` message; new `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️component.rs` | `store::document_codec` → `Result<Option<_>, _>`; `FormatDescriptor::primary_mime/primary_extension` → plural `mimes`/`extensions`; stricter alternative/checkpoint validation in `🏪️store`; conflict-rejecting IO registry |
| `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET` | `🔣️taxonomy.json` `_surfaceComment` + schemaVersion 5; new window-kit region in `🔌️plugin/🦀️component.rs`; `AppCommand::{OpenArtifact,SetDefaultApp,ClearDefaultApp}` | `AppDefinition` gained `role: AppRole` + `dialect: ArtifactDialect`; `CHANNEL_VERSION` 9 → 10 |

## Adaptations this ticket made (ours to make — call sites we own)

1. `🔌️plugin/🖥️host/🦀️component.rs:90` — `let Ok(Some(codec)) = store::document_codec(schema)`.
2. `🔌️plugin/🦀️component.rs` `formats()` — claims every `mime` and every `extension` now that both are plural. Strictly more correct for a conflict-rejecting registry than the old primary-only claim.
3. `🏪️store/🔄️sync/🦀️component.rs:389` — `origin: MutationOrigin::Owner` on the remote-ingest `HistoryOpMeta`.

## The channel-version golden, fixed for good

Two separate tickets each broke `app_command_fixture_corpus_matches_golden_hex_and_round_trips` by bumping `CHANNEL_VERSION` (8 → 9 here, 9 → 10 there), because the `Hello`/`Welcome` fixtures encoded the **live constant** — so every wire bump silently rewrote its own golden's expected bytes.

Fixed by separating the two concerns:
- the corpus entries now carry a **literal** version (`1`), so the goldens pin the codec's bytes and never move again;
- the constant gets its own pin, `🧫️fixtures/📡️channel/channel-version.json`, asserted by `channel_version_matches_the_shared_cross_language_pin` in Rust and `pins APP_CHANNEL_VERSION against the shared cross-language channel version` in TypeScript.

**This caught a live bug**: Rust was at `CHANNEL_VERSION = 10` while TypeScript's `APP_CHANNEL_VERSION` still read `8` — the two hosts would have disagreed on the wire version at runtime, and nothing in the tree would have said so. TS is now 10 and both sides are pinned to the shared file.

## Open cross-ticket blocker (not ours to fix)

`AppDefinition` gained `role`/`dialect`, and the construction site in `build_definition` (`🔌️plugin/🦀️component.rs` ~4665) has not been updated by its owning ticket, so `cargo check -p semio-framework-plugin` fails there. The values are that ticket's semantics (a surface's viewer/editor role and its bound dialect), so this ticket deliberately does not invent them. Lane W1-B was told to leave it, add only the exhaustive-match arms for the three new `AppCommand` variants inside its own lease, and report honestly if a gate cannot run because of it.
