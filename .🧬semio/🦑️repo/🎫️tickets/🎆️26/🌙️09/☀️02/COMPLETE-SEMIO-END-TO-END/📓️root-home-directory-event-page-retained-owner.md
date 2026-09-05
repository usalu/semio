# Home Directory Event-Page Retained Owner

## Outcome

Home now has a typed `applyDirectoryEventPage` retained command that parses the shared canonical `DirectoryEventPageV1`, validates its receipt, applies an exact authority/frontier transition, advances across invisible raw holes, and emits at most one config replacement. The projection and its session-binding, authorization-generation, and receipt resume authority are persisted together.

This slice owns the Home publication boundary only. It does not claim that the hub endpoint or the React/native shell page-fetch loops exist yet.

## Contract

- first page or changed session/authorization authority must start at raw frontier zero and replaces the prior projection;
- a page under the same authority must start exactly at the persisted projection cursor;
- successful application sets the projection cursor to `throughSeqInclusive`, even when the page contains no visible events;
- exact receipt replay is an idempotent no-op;
- stale-frontier, nonzero rebootstrap, malformed canonical JSON, forged receipt, unsafe generation, and corrupt projection state are rejected;
- the retained Store preparation admits the typed replacement only on the config lane, seals one forward/inverse edit against the exact base authority, and owns cancellation plus terminal cleanup;
- all Home config schema leaves now declare the complete projection and resume-authority fields.

## TDD Evidence

- RED: the independent source law validated the shared page schema and SHA-256 vector, then failed exactly because the Home command/retained owner did not exist.
- GREEN: `@semio-tech/space-plugin:home-directory-event-page-owner-check` passes 19 checks using AJV, Node SHA-256, schema-leaf parity, retained-fixture validation, and hostile source substitutions.
- JSON fixtures, schemas, and project configuration parse successfully; owned diffs pass `git diff --check`.
- Native exact law `editor::home::commands::apply_directory_event_page::tests::sealed_page_replaces_projection_once_and_rejects_races` reached the build stage twice but did not reach discovery or its assertion. The latest exact receipt is `🗑️generated/root-home-directory-event-page-owner/exact-cargo-laws-wNjcct/00`; Cargo exited 101 because concurrent Stdio taxonomy source references an absent Semio Drawing DXF R12 deserializer. The earlier receipt `exact-cargo-laws-etKTPq/00` stopped at an absent PDF `replace-page-text` mutation. Both are outside the Home packet, so no native result is claimed.

## Permanent Gates

- `@semio-tech/space-plugin:home-directory-event-page-owner-check`
- `@semio-tech/space-plugin:home-directory-event-page-owner-native-check`
- launch entries `⚖️gate🏠️directory-event-page-owner` and `⚖️gate🏠️directory-event-page-owner🦀️native`

## Files

- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📬️apply-directory-event-page/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🧫️retained-command-limits/`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/📋️project.json`
- `.vscode/🧩️launch.seed.jsonc`

## Next Slice

Implement the authenticated hub event-page endpoint, then replace both shells' stream-from-zero bootstrap with one-page fetch, real retained-action ACK, and resume-from-`throughSeqInclusive` ordering before opening the live socket.
