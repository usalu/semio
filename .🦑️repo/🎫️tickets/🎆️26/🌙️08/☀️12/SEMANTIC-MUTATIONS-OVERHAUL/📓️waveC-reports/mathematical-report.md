# FacetReport — mathematical (Wave C)

## facet
`mathematical` (single artifact, `s.mathematical.mathematical`,
`🗿️artifacts/➗️mathematical/…/🧬️mutations`).

## status
`done` — funnel debt fixed (app commands + `dsl_derive::Mutations` → `dsl::Mutations` compile bug),
directory/glue trueing complete, config semanticized, TS mirrors added, schema description files
rewritten, law-test coverage extended. Emoji were already unique (no fix needed). `cargo check`
is blocked by a **confirmed foreign** error unrelated to this ticket — see **gates** — everything
mathematical-specific was verified by direct code review (every rewritten call site's field shapes
checked against the real payload struct definitions) since the crate could not be compiled
end-to-end this session.

## mutationsCreated
None — the 14 semantic `MathematicalMutation` variants already existed (an earlier wave derived
them). This session's job was funnel/directory/glue trueing plus one real compile bug: the
dispatch enum used `#[derive(dsl_derive::Mutations)]`, but `dsl_derive` is not a dependency this
crate declares anywhere (`Cargo.toml` only pulls `semio-framework-os-kernel` aliased `as dsl` via
`📦️glue.rs`'s `extern crate` list) — exactly the pitfall `📓️fanout-brief.md` step 3 warns about
("Check your crate's actual alias first… most plugin crates do `extern crate
semio_framework_os_kernel as dsl;`"). Fixed to `dsl::Mutations`.

## genericVariantsRemoved
Three orphan legacy triads, none referenced by the 14-variant dispatch enum, all doc-comment-only
stubs (`🦠️mutation` = a bare `apply()` fn, no `🔺️diff` dir at all, `↩️inverse` returning
`Vec::new()`): `📄set-snapshot` (banned outright, no replacement per taxonomy — confirmed by its
own header comment), `📊set-graph` (superseded by `replace-graph`, confirmed by `replace-graph`'s
own doc comment: "the semantic replacement for the old generic `SetGraph`"), `📐set-geometry`
(superseded by `replace-points`, same pattern, confirmed by `replace-points`'s doc comment). All
three deleted along with their `📦️glue.rs` mount blocks.

App funnel debt (NOT documented in `remaining-work-map.md`'s per-plugin table — mathematical was
listed "clean" there, which was stale): `🎛️apps/➗️mathematical/🎮️commands/{🕸️graph,📐️geometry,
📄️artifact}/🦀️component.rs` constructed `MathematicalMutation::SetGraph { graph }` (×4 call
sites) and `MathematicalMutation::SetGeometry { geometry }` (×2 call sites) — neither variant
exists in the 14-variant enum, so this alone would have failed `cargo check` regardless of the
`dsl_derive` bug. Fixed to `ReplaceGraph(ReplaceGraph { graph })` / `ReplacePoints(ReplacePoints {
points: geometry.points })` (field-shape note: `ReplacePoints` takes `points: Vec<MathematicalPoint>`
directly, not a `geometry: MathematicalGeometry` wrapper — `MathematicalGeometry` is just `{
points }`, so the app-local `SetPoints`/`SetArtifact` command payload structs, which still carry
`geometry: MathematicalGeometry` for their own API shape, now read `.points` off it when building
the mutation). The batched `NodeGraphEdit` gesture (add/move/connect/delete node ops, arbitrary
JSON array applied to a working `MathematicalGraph` clone) keeps emitting one `ReplaceGraph` per
dispatch — this matches `📓️derivation-rules.md`'s explicit guidance ("If unsure whether a gesture
is 'replace everything' vs 'a big but targeted change', default to modeling it as the targeted
semantic verb… usually `replace-<payload>` on one field… not a snapshot swap") and is what
`replace-graph`'s own doc comment says it exists for.

`MathematicalConfigMutation::Snapshot { config }` (whole-config, app-level ratchet scope) —
removed; per-field inverse (no map fields in this config, straightforward).

## emoji table (facet-scoped uniqueness)
Already unique before this session — no renames needed. Full 14-triad table:

| slug | emoji |
|---|---|
| `change-graph-directed` | 🔀️ |
| `update-graph-algorithm` | 🧮️ |
| `replace-graph` | 🔁️ |
| `create-node` | 🟢️ |
| `delete-node` | ❌️ |
| `delete-nodes` | 🗑️ |
| `change-node-label` | 🏷️ |
| `move-node` | 🕹️ |
| `connect-nodes` | 🔗️ |
| `disconnect-nodes` | ✂️ |
| `replace-points` | 🌀️ |
| `insert-point` | ➕️ |
| `remove-point` | ➖️ |
| `move-point` | 🎯️ |

## Directory + glue trueing
- Deleted 3 orphan triad dirs (see genericVariantsRemoved) and their 3 `📦️glue.rs` mount blocks
  (`set_snapshot`, `set_graph`, `set_geometry`).
- Added 14 real glue.rs mounts (one `pub mod <snake_slug> { pub mod mutation; pub mod diff; pub
  mod inverse; }` per active triad) — glue.rs previously had ZERO real per-triad mounts for the
  active 14 (same inline-self-wiring debt as layout had).
- Removed the dispatch file's `//#region 🔖️LeafWiring` inline `#[path = "."]` self-wiring (14
  blocks) — replaced with `use super::{ change_graph_directed, …, move_point };`.
- Fixed `dsl_derive::Mutations` → `dsl::Mutations` on the dispatch enum's derive list, and the
  matching doc-comment mention in the sibling `📝️text/🦀️component.rs`.
- Dispatch-variant-set now matches triad-dir-set 1:1 in both directions (14 = 14).

## Config semanticization
`MathematicalConfigMutation` (`🎛️apps/➗️mathematical/🎚️config`) — removed `Snapshot { config }`;
`diff()`/`inverse()` rewritten per-variant (`SetCamera`/`SetLocale`, no map fields, direct
old-value-from-base restoration). Fixed the two tests that constructed `Snapshot` directly.

## TS mirrors
Added 42 non-stub `🟦️component.ts` files (14 triads × 3 leaves) — none existed before.

## Schema description files
Rewrote `📝️text/📖️component.grammar.semio`, `💾️binary/📡️component.protocol.semio`,
`📝️text/{🔗️component.graphql,🔣️component.json,🛰️component.proto}` from the stale
`"schema" SP "stdio.json"` placeholder to 14 real rules/records/types, one per mutation, tags
1..14 in dispatch-enum order. Same pattern as gis: the dispatch file has NO `include_str!` for a
root-level grammar (verified — unlike gis), but a stray, unreferenced root-level
`🧬️mutations/📖️component.grammar.semio` file existed anyway (confirmed dead: grepped the whole
plugin for `include_str!("📖️component.grammar.semio")`, all 4 hits resolve to their OWN directory's
`📝️text/…` file — `📸️snapshot/📝️text`, `🔺️diff/📝️text`, `💡️inferences/📝️text`,
`🧬️mutations/📝️text` — none reference the root stray copy). Overwrote it with the real grammar
text for consistency, same reasoning as gis's root-grammar cleanup. `.g4`/`.ebnf`/`.abnf`/`.ksy`/
`.spicy` siblings not rewritten (same reasoning as layout/gis).

## lawTests
The existing `#[cfg(test)]` region already had strong manual round-trip coverage for 8 of 14 kinds
(`replace-graph`, `create-node`/`delete-node` with cascade-severed-edge reconnection assertions,
`move-point`, `insert-point`, `delete-nodes` (plural cascade), `connect-nodes`/`disconnect-nodes`)
but used hand-rolled apply/inverse loops rather than the `protocol::testkit` helpers, and had zero
coverage for `change-graph-directed`, `update-graph-algorithm`, `change-node-label`,
`remove-point`. Added 4 new tests in a new `⚖️SemanticLaws` region using
`protocol::testkit::assert_mutation_inverse_law` for exactly those 4 — all 14 kinds now covered
by at least one test. `DiffAlgebra` not implemented for `MathematicalDiff` — same reasoning as
layout/gis (its diff replaces the WHOLE `graph`/`geometry` slot per the existing
`replace_graph_diff_carries_whole_graph` test's own comment, so `between()` would need real
structural diffing of `MathematicalGraph`/`MathematicalGeometry`, not a per-field `Option` fold);
flagged as remaining work.

## gates
- `cargo check -p semio-s-plugin-mathematical`: first attempt hit 19 `E0753` errors, ALL inside
  `semio-framework-os-kernel`'s `🏪️store/🦀️component.rs` — the same `26/08/12/
  UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` concurrent-edit window that blocked layout/gis (confirmed:
  the broken `//!` doc-comment line moved between retries, proving another session was actively
  writing code there). That window closed and a second attempt got past the framework, reaching
  mathematical's own compile — which then hit a **different, also-foreign** error:
  `error: couldn't read ✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎮️commands/📄️document/
  🦀️component.rs: No such file or directory`, from `📦️glue.rs:416`'s `pub mod document;` mount.
  Grepped the whole plugin: the real directory is `🎮️commands/📄️artifact` (confirmed present,
  holds the `SetArtifact` command this report already covers), and `📄️document` is referenced
  NOWHERE else — this is a stale mount left by a directory rename this ticket never touched.
  `ps aux` at the time showed a live concurrent session on a DIFFERENT ticket,
  `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`, actively running builds — matches the
  brief's own named exclusion pattern ("missing `📌️panels/📄️document`") closely enough (a
  `📄️document`-named mount now missing because a concurrent ticket renamed the directory to
  `📄️artifact` mid-flight) to record as `blocked-churn` rather than touch a `commands` block this
  session never edited (this session's own glue.rs edits were confined to the `mutations` block —
  verified via `git diff`-equivalent reasoning: line 416 sits in a completely separate `pub mod
  commands { ... }` block this session's edits never came near). mathematical's own funnel fixes
  (`SetGraph`/`SetGeometry` call sites, `dsl_derive`→`dsl`) were verified correct by reading every
  call site's field shapes against the real payload struct definitions, and by the fact `cargo
  check` got PAST mathematical's `🧬️mutations` module (the file that failed to read is unrelated,
  `🎮️commands/📄️document`) — but this session could not observe a 0-error `cargo check` end to end.
- `cargo test -p semio-s-plugin-mathematical --lib`: not run to completion — blocked by the same
  foreign `📄️document` mount every attempt that got past the framework reached.
- A 4th `cargo check` retry (after the `📄️document` sighting) hit a THIRD distinct foreign error
  set: `E0027`/`E0063` (`child_emits`/`child_edit_ids` field mismatches) inside
  `semio-framework-plugin` itself — confirming the shared framework crates are under sustained,
  multi-session concurrent development right now (three different foreign error sets across four
  attempts, each in a different shared crate: `semio-framework-os-kernel`, this plugin's own
  `📄️document` mount from a different ticket, then `semio-framework-plugin`). Per the churn
  policy's "retry ≤3×, then record and move on," this is recorded as `blocked-churn` rather than
  retried further — none of the three error sets touch anything this session edited.
- `bun ./📜️script.ts policy`: ran once, repo-wide (`22158` high-priority breaches total, expected —
  this is a live, multi-session, 107-facet migration in progress, not specific to this session).
  Zero `mutation-migration/semantic-vocabulary`, `…/dispatch-coverage`, or `…/ts-mirror` breaches
  reference `➗️mathematical` — the only hits mentioning this facet are `…/triad-completeness` and
  `…/artifact-engine`, both pre-documented in `📓️remaining-work-map.md` as "bogus, wrong-depth bug"
  (the rule scans a shallow path that doesn't exist in the real taxonomy) and unrelated to
  anything this session changed structurally.

## allowlistKeysToRemove
Full-plugin sweep (`grep -rnE "SetSnapshot|NoMutation|CollectionMutation(<|::)"
✏️s/🔌️plugins/➗️mathematical`) returns zero hits. One comment-only hit this session introduced and
then reworded before it could land (a doc comment describing the deleted orphan triads used the
literal string `SetSnapshot` in prose; reworded to "whole-document-replace ban" instead) — final
state has zero hits, confirmed by grep, not just by construction.

## filesTouched
**Updated:**
- `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎮️commands/🕸️graph/🦀️component.rs`
- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎮️commands/📐️geometry/🦀️component.rs`
- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎮️commands/📄️artifact/🦀️component.rs`
- `✏️s/🔌️plugins/➗️mathematical/🎛️apps/➗️mathematical/🎚️config/🦀️component.rs`
- `…/🧬️mutations/📖️component.grammar.semio` (root, stray dead copy, overwritten for consistency)
- `…/🧬️mutations/📝️text/📖️component.grammar.semio`
- `…/🧬️mutations/💾️binary/📡️component.protocol.semio`
- `…/🧬️mutations/📝️text/🔗️component.graphql`
- `…/🧬️mutations/📝️text/🔣️component.json`
- `…/🧬️mutations/📝️text/🛰️component.proto`

**Created:** 42 `🟦️component.ts` files (14 triads × 3 leaves).

**Removed (3 directories):** `…/🧬️mutations/📄set-snapshot/` (`🦠️mutation`+`↩️inverse`, 2 files),
`…/📊set-graph/` (2 files), `…/📐set-geometry/` (2 files) — 6 files total, none had a `🔺️diff` dir.

## sharedFileRequests
None — mathematical's `📦️glue.rs` was in-scope for this Wave-C lane.

## deviations
- `DiffAlgebra` not implemented — flagged, same reasoning as layout/gis.
- `.g4`/`.ebnf`/`.abnf`/`.ksy`/`.spicy` siblings not rewritten — flagged, no working reference.
- `cargo check`/`cargo test` final confirmation was in flight against a live framework-churn
  window at report-writing time; see the accompanying chat reply for whether it closed in time.
