# P2-FG2 Fix: jpg 5-role `LanguageSpec` + `register_schema_spec` Registration

## Scope

Sole file touched: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/⚙️engine/🦀️component.rs`.
Added two new functions (`register_pilot_languages`, `register_schema_specs`) and wired both
into the existing `register()` (additive — `composer::register`/
`register_artifact_schema_descriptor`/`register_document_codec`/baseline composer register calls
are untouched).

## What was added

### 1. 5-role `LanguageSpec` registration — `register_pilot_languages()`

Real, non-fabricated registration for all 5 roles, following tiff's/las's own exemplar shape
(this exact wave's own siblings) exactly:

- `stdio.jpg` (Document) — grammar+protocol from `schema::snapshot::text`/`schema::snapshot::binary`
- `stdio.jpg.op` (Ops) — grammar+protocol from `schema::mutations::text`/`schema::mutations::binary`
- `stdio.jpg.diff` (Diff) — grammar from `schema::diff::text`, `protocol: None` (matches every
  sibling — the 5-role scheme has no dedicated "diff binary" role even though
  `🔺️diff/💾️binary/📡️component.protocol.semio` is a real file, exercised directly by
  `conformance_laws::protocol_walk_law`, not through a 6th `LanguageRole`)
- `stdio.jpg.pack` (Pack) — protocol from `schema::snapshot::binary`
- `stdio.jpg.spr` (Spr) — protocol from `schema::mutations::binary`

All use `dsl::passthrough_hooks(id)`. All grammar/protocol constants (`COMPONENT_GRAMMAR_SEMIO`/
`COMPONENT_GRAMMAR_PATH`/`COMPONENT_PROTOCOL_SEMIO`/`COMPONENT_PROTOCOL_PATH`) already existed on
disk under `🪆️subsets/✳️any/🧬️schema/{📸️snapshot,🧬️mutations,🔺️diff}/{📝️text,💾️binary}/🦀️component.rs`
— confirmed present before wiring, not invented.

### 2. `register_schema_specs()` — deliberately empty, with real verification

Before writing this as empty (matching las's own resolution), I did NOT assume — I ran the real
experiment the ticket's note asked for:

1. Temporarily added `#[derive(dsl::DslRecord)]` to `JpgSnapshot` and every nested value type it
   transitively contains (`JfifThumbnail`, `JpgFrameComponent`, `JpgFrameHeader`,
   `JpgQuantTable`, `JpgHuffmanTable`, `JpgSegment`) plus `dsl::DslScalar` on `JfifDensityUnits`/
   `JpgHuffmanClass` (the two unit-only enums it references).
2. Ran a real `cargo check -p semio-s-plugin-stdio --lib`.
3. Result: every field compiled cleanly EXCEPT `jfif_version: (u8, u8)`:
   ```
   error[E0277]: the trait bound `(u8, u8): DslField` is not satisfied
      --> .../📸️snapshot/🦀️component.rs:213:23
       |
   213 |     pub jfif_version: (u8, u8),
       |                       ^^^^^^^^ the trait `DslField` is not implemented for `(u8, u8)`
   ```
   This is the EXACT SAME bare-tuple gap las's own `register_schema_specs` doc comment already
   documents (`LasPointDiff::rgb`'s `(f64, f64, f64)` / `LasMutation::SetScaleAndOffset`'s
   `(f64, f64, f64)`) — confirmed by grep: there is no blanket `impl<...> DslField for (T, T)`
   anywhere in `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs` (only `bool`/`f32`/
   `f64`/`String`/`Wire`/ints via macro/`Vec<T>`/`BTreeMap<String, T>`/`[T; N]`/`DslValue`).
4. Reverted the derive probe (`git checkout` on the snapshot file) since fixing it would mean
   reshaping `jfif_version`'s wire type (touching engine/mutations/diff codecs and the `.proto`/
   `.semio` protocol files well beyond this ticket's registration scope) — out of scope for a
   registration-only fix, filed as a `mechanism_gaps` entry instead, per the recipe's documented
   rule (same resolution json/csv/zip/png/tiff/las already apply to their own genuine gaps).

`JpgDiff` and `JpgMutation` were independently ALREADY confirmed hand-rolled before this session
(pre-existing `F6 CONFIRMED HAND-ROLL` doc comments in `🔺️diff/🦀️component.rs`/
`🧬️mutations/🦀️component.rs`):
- `JpgDiff`: `frame: Option<JpgFrameChange>` — `JpgFrameChange` is a genuine data-carrying enum
  (`Modify(JpgFrameFieldsDiff)` / `Replace{frame}`), no `DslField` impl for it (only
  `DslRecord`-derived structs and `DslScalar`-derived UNIT-only enums implement `DslField`) —
  PLUS `re_encode_quality`/`jfif_thumbnail`/`restart_interval`'s tri-state `Option<Option<T>>`
  fields (no `impl<T: DslField> DslField for Option<T>` anywhere in `dsl`, so `Option<Option<T>>`
  is doubly blocked).
- `JpgMutation`: `#[derive(dsl::DslOps)]` confirmed to fail (pre-existing comment).

So `register_schema_specs()` is correctly empty — no fabricated spec, `stdio.jpg`/
`stdio.jpg#diff` filed as this wave's own `mechanism_gaps` entries.

## Deviations from a literal reading of the assignment note

- The note framed jpg as potentially closer to gif89a's shape ("GifSnapshot DOES carry a genuine
  derived RecordSpec constructor" — 1-of-2 registered). Real verification showed jpg is instead
  shaped like las (0-of-2 registered, same bare-tuple blocker) — `JpgSnapshot` itself, not just
  `JpgDiff`, is blocked. This was determined by actually running the derive + `cargo check`, not
  assumed.

## Verification

```
cargo check -p semio-s-plugin-stdio --lib          → 0 errors (363 pre-existing warnings, none new)
cargo test -p semio-s-plugin-stdio --lib "artifacts::jpg"
  → running 40 tests
  → test result: ok. 40 passed; 0 failed; 0 ignored; 0 measured; 1734 filtered out
```

Notably passing: `engine::tests::conformance_laws::protocol_walk_law`,
`grammar_conformance_law`, `ops_grammar_conformance_law`, `diff_grammar_conformance_law` —
these exercise the newly-registered `LanguageSpec`s directly.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/⚙️engine/🦀️component.rs` (only
  file changed by this session — `+262` lines, two new `pub fn`s, `register()` extended
  additively)
- This report.

(Other jpg-tree diffs visible in `git status`/`git diff --stat` — `🔺️diff/🦀️component.rs`,
`🧬️mutations/🦀️component.rs`, `.semio` grammar/protocol files, `📚️examples/` fixtures — are
pre-existing concurrent work from other sessions on this live shared tree, present before this
session started and NOT touched here.)
