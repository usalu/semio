# F3 — dxf (r12) — Report

Ticket: `26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION`. Plan:
`~/.claude/plans/the-current-schemas-are-scalable-journal.md`. Design recipe:
`🧬️schema-design.md`. Prior waves: `w0-recon-report.md`, `s1-spine-report.md`,
`s2-spine-report.md`.

## Summary

Rewrote `stdio.dxf` (r12) end to end: the old flat `tags: Vec<DxfTag>` passthrough model became
a complete typed DXF R12 ASCII document (`$VAR`-keyed header, name-keyed LAYER/STYLE/LTYPE
tables + raw-retained other table kinds, index-keyed blocks with nested entities, index-keyed
top-level entities typed as Line/Circle/Arc/Polyline/Text/Solid/Insert with an `Other` raw-
retention fallback), a handcrafted sparse `DxfDiff` (name-keyed triples for header vars and each
table kind, index-keyed triples for blocks and entities, `Replace`-on-kind-change for entities),
`impl DiffAlgebra<DxfSnapshot> for DxfDiff`, and an 19-variant `DxfMutation` enum with handcrafted
`diff()`/`inverse()` per variant. All 6 law suites present and green; all snapshot/diff/mutations
facet mirrors (TS/GraphQL/JSON Schema/proto) and grammar leaves (.g4/.ebnf/.grammar.semio for
text; .ksy/.spicy/.abnf/.protocol.semio for binary) handcrafted with real, non-placeholder
content.

## Snapshot design

- `DxfValue` — typed union over DXF group-code value kinds: `Str`/`Int`/`Double`/`Point`
  (`Point([f64;3])` combines a 10/20/30-style code triplet, or a 10/20 2D pair with z=0, into one
  value — the ticket's own listed kinds explicitly include "point-component"). Deliberate,
  documented divergence from a literal single-group-code-per-header-var reading: without `Point`,
  multi-component header vars like `$INSBASE`/`$EXTMIN`/`$EXTMAX` would need three same-named
  entries, breaking the name-keyed collection's uniqueness invariant. `group_code` is retained as
  the primary code (e.g. `10`) for round-trip fidelity; a small `extra_group_codes` fallback field
  on `DxfHeaderVar` losslessly retains any further codes beyond a plain scalar or point (rare in
  practice, never fabricated, never silently drops data).
- `header_vars: Vec<DxfHeaderVar{name, group_code, value, extra_group_codes}>` — name-keyed
  (`$VAR`), complete `HEADER` section coverage.
- `tables: DxfTables{layers, styles, linetypes}` — the three table kinds the ticket asked for,
  each name-keyed with its own `unknown_group_codes` raw retention. `other_tables:
  Vec<DxfOtherTable{name, tags}>` — a documented, honest ADDITION beyond the ticket's literal
  table-kind list: real R12 files also have VPORT/VIEW/UCS/APPID/DIMSTYLE/BLOCK_RECORD tables;
  without this the codec would silently drop real on-disk data, violating the recipe's core
  honesty rule. Raw-retained verbatim via the demoted `DxfTag` struct.
- `blocks: Vec<DxfBlock{name, base_point, entities, unknown_group_codes}>` — index-keyed per the
  ticket's explicit instruction; each block's `entities` reuses the exact same `DxfEntity`/
  `DxfEntitiesDiff` machinery as the top-level collection (genuine code reuse, not a copy).
- `entities: Vec<DxfEntity>` — `Line`/`Circle`/`Arc`/`Polyline`/`Text`/`Solid`/`Insert`/`Other`.
  `Polyline` models the REAL R12 `POLYLINE`/`VERTEX`.../`SEQEND` multi-record group (not the
  R14+-only `LWPOLYLINE` entity the old pre-overhaul code modeled by name only — a deliberate,
  documented spec-accuracy correction; CLAUDE.md: greenfield, no legacy support, fix
  inconsistencies). Every typed entity carries `unknown_group_codes`; `Other{kind, group_codes}`
  covers every unmodeled kind (3DFACE, POINT, DIMENSION, …) — proven in
  `parses_every_section_and_entity_kind`.
- Codec: `parse_dxf_document`/`print_dxf_document` fully replace the old
  `tokenize_dxf`/`write_dxf_tags` pair (the tokenizer itself survives internally as the
  tag-stream-producing first pass). Documented NORMAL FORM: `decode(encode(snap)) == snap`
  (fixed point immediately — proven for a fixture spanning every section/entity kind), and
  `encode(decode(text))` is a text-level fixed point from the SECOND generation onward
  (`codec_retention_is_a_fixed_point_from_generation_two`) — incidental source formatting isn't
  preserved, every group code's semantic content is.

## Diff design

Two small intra-file generic cores (mirroring `stdio.obj`'s proven `ObjIndexElem`/
`generic_apply`/`generic_between`/`generic_absorb_pair` pattern) do the position/name algebra
ONCE: `DxfIndexElem` (index-keyed: `DxfEntity` — reused for both `entities` and each block's
nested `entities` — and `DxfBlock`), `DxfNamedElem` (name-keyed: `DxfHeaderVar`, `DxfLayer`,
`DxfStyle`, `DxfLinetype`; no rename tracking needed, matching `stdio.obj`'s groups/objects
shape). Every PUBLIC diff type stays a fully concrete, per-artifact named type.

`DxfEntityDiff` is the enum-collection-element case the plan calls out: `Replace{entity}` when
the entity KIND changes at an index, or one of `Line`/`Circle`/`Arc`/`Polyline`/`Text`/`Solid`/
`Insert`/`Other` (each a sparse per-field patch struct) when it doesn't. `diff_absorb` for
entities handles the extra case this shape introduces beyond `stdio.obj`'s precedent: a
`Replace` absorbed with a same-kind field diff patches INTO the carried replacement payload (the
canonical "patch into added/replaced payload" case, generalized from index/name payloads to a
kind-tagged enum payload) — proven in `absorb_law`'s "Add+Replace(kind-change)" case.

Zero `snapshot: Option<DxfSnapshot>` full-replace slot anywhere (grep-confirmed — the only hit is
a doc-comment describing the OLD template being replaced). `diff_set_snapshot(base, next)` is the
sparse `DxfDiff::between(base, next)`, same as every other rewritten artifact.

## Mutations (19 variants)

`SetSnapshot`, `SetHeaderVar`/`RemoveHeaderVar`, `InsertLayer`/`RemoveLayer`/`SetLayer`,
`InsertStyle`/`RemoveStyle`/`SetStyle`, `InsertLinetype`/`RemoveLinetype`/`SetLinetype`,
`InsertEntity`/`RemoveEntity`/`SetEntity`, `InsertBlock`/`RemoveBlock`/`SetBlock` (`SetBlock`
added beyond the ticket's literal "InsertBlock/RemoveBlock" list for full-field-coverage
symmetry with the other collections — cheap given the shared `DxfIndexElem` machinery, and
needed for `field_sweep`'s block-modify coverage). Every `diff()` handcrafted; every `inverse()`
handcrafted, name/index-aware, reading pre-state from `base`.

## Bugs found and fixed during verification (real, not scratch-crate-only)

1. **`parse_tables_section` unknown-table body-start**: originally skipped ALL leading
   non-zero-code tags (meant for known kinds' `70` count field) before capturing `other_tables`
   raw retention — this silently dropped an unknown table's ENTIRE content when it had no
   leading informational field before its first `0/<ENTRY>` marker, and produced asymmetric
   round-trips for hand-built fixtures. Fixed by splitting known-kind body-start computation
   (skip-to-first-entry-marker, needed for `split_table_entries`) from unknown-kind capture
   (raw, unsliced, from right after `2/<name>` to `ENDTAB`) — caught by
   `codec_retention_law`/`snapshot_parse_dsl_print_dsl_round_trips`.
2. **Duplicated vertex `8`/layer tag on print**: `print_entity`'s `Polyline` arm hardcoded
   `push_tag(out, 8, layer)` per vertex IN ADDITION TO re-emitting the vertex's own captured `8`
   tag (already retained in `unknown_group_codes` by `build_vertex`, since `DxfVertex` has no
   dedicated `layer` field) — every re-encode doubled that tag. Fixed by removing the fabricated
   hardcoded emit; caught by `codec_retention_is_a_fixed_point_from_generation_two`.
3. **`InsertLayer`/`InsertStyle`/`InsertLinetype` `inverse()` read the wrong snapshot**: looked up
   the name to remove via `base.tables.layers.get(*index)` — `base` is PRE-insertion state, so
   that index (if occupied at all) held whatever WAS there before, never the item this mutation
   is about to insert. Fixed to read the name directly off the mutation's own payload
   (`layer.name`/`style.name`/`linetype.name`). Caught by `inverse_law` for `InsertLayer`
   (would have also broken `InsertStyle`/`InsertLinetype` identically — fixed all three).
4. Own test bug: `parses_every_section_and_entity_kind` asserted `entities.len() == 9` against a
   fixture that (correctly) parses to 8 real entities (Line/Circle/Arc/Text/Solid/Insert/
   Polyline/Other — the POLYLINE/VERTEX/VERTEX/SEQEND group collapses into ONE `Polyline`
   entity, not several) — an off-by-one in my own hand count, fixed to 8.

All four were caught by the REAL crate's tests, not a scratch crate (I iterated directly against
the mounted files given the artifact's moderate size; the field_sweep/codec_retention/absorb law
suites are exactly the mechanism that caught them, matching the ticket's warning that scratch-
crate-only verification is insufficient).

## Concurrent-wave churn observed (not fixed, not mine)

`gif` (87a/89a) is a sibling F3 agent's artifact, actively being rewritten during this session.
`cargo check` showed 12-13 gif-only compile errors (E0308/E0560/E0609/E0063/E0433 in
`gif/…/⚙️engine`, `gif/…/🧬️migrations`, `gif/…/🪆️subsets/✳️any/🧬️schema`) that cleared on their
own mid-session (confirmed via polling `cargo check`/`cargo test` every ~60s); the full crate
test run subsequently showed **5 remaining failures, all in `artifacts::gif::…`** (unrelated
runtime bugs in gif's own lzw/frame-encode logic, not compile errors) — none touch any dxf/png/md
file path. Classified via own-module filter per the ticket's guidance; not investigated further,
not fixed (out of scope, gif's own agent's territory).

## Verification

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::dxf"` → **13 passed, 0 failed**.
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **847 passed, 5 failed** — all 5
  failures in `artifacts::gif::…` (see above); zero in dxf/png/md.
- Grep gates: `snapshot: Option<` → 0 struct-field hits (1 doc-comment mention describing the old
  template). `impl DiffAlgebra` → present (`impl DiffAlgebra<DxfSnapshot> for DxfDiff`). `field_sweep`
  → present (`field_sweep_every_mutable_field_changes`). Apply-and-capture pattern → none found.
- `bun ./📜️script.ts policy` → ran to completion (large pre-existing repo-wide backlog, unrelated
  to this ticket, matching S2's documented baseline); zero occurrences of any of the 4 new S-8
  rule kinds (`facet-mirror-drift`/`grammar-honesty`/`diff-algebra`/`field-sweep`) mentioning
  `dxf` in the printed breach listing.

## Facets and grammar leaves

All handcrafted, real content (no placeholder markers), for `📸️snapshot`, `🔺️diff`, `🧬️mutations`,
and the top-level `🧬️schema` (`DxfArtifact`) facets:
- `🟦️component.ts`, `🔗️component.graphql`, `🔣️component.json` (JSON Schema),
  `🛰️component.proto` — real interfaces/types matching the Rust shapes field-for-field
  (camelCase), discriminated unions on `mutation`/entity-kind/diff-kind tags.
- `📝️text/{🅰️.g4, 🔤️.ebnf, 📖️.grammar.semio}` — snapshot's describe the real DXF R12 ASCII
  group-code/value tag stream and section structure; diff's/mutations' describe the real JSON
  wire shape (`protocol::OpText` IS `serde_json`, so these name the real sparse fields rather
  than restating RFC 8259, matching `stdio.obj`'s precedent).
- `💾️binary/{🥋️.ksy, 🌶️.spicy, 🔠️.abnf, 📡️.protocol.semio}` — the shared `.semio` binary
  envelope (magic/token-len/token/payload) wrapping UTF-8 DXF text (snapshot) or UTF-8 JSON
  (diff/mutations) — same shape as every other rewritten stdio artifact's binary facet.

## Deviations / scope cuts (all explicitly permitted by the brief)

1. **DxfValue::Point vs literal single-group-code header vars** — see Snapshot design above;
   documented, necessary for correctness (name-keyed uniqueness + no data loss).
2. **`other_tables` raw retention added beyond the literal ticket table-kind list** — honesty
   requirement (nothing real on disk silently dropped); VPORT/VIEW/UCS/APPID/DIMSTYLE/
   BLOCK_RECORD are NOT typed-modeled, only raw-retained.
3. **`DxfStyle`/`DxfLinetype` field scope**: minimal but real — `name`/`flags`/`font_name` (style)
   and `name`/`flags`/`description` (linetype), everything else per-entry falls into
   `unknown_group_codes`. The ticket explicitly allowed "if time allows" scope for these two
   table kinds; both are fully modeled with complete diff/mutation coverage, just not every
   spec field (e.g. STYLE's oblique angle/text-gen-flags/last-height/bigfont, LTYPE's dash
   pattern) — none of that is fabricated or silently lost, it's raw-retained.
4. **Entity kind coverage**: Line/Circle/Arc/Polyline/Text/Solid/Insert fully typed per the
   ticket's own suggested minimum ("it's fine to only fully model Line/Circle/Arc/Polyline/Text/
   Insert if time is tight" — Solid included too); every other kind (3DFACE, POINT, DIMENSION,
   SHAPE, ATTRIB, …) via `Other{kind, group_codes}` raw retention, proven lossless.
5. **Polyline modeled via the real POLYLINE/VERTEX/SEQEND record group, not LWPOLYLINE** — a
   correction of the pre-overhaul code's spec inaccuracy (LWPOLYLINE doesn't exist in R12);
   greenfield/no-legacy-support license used per CLAUDE.md.
6. **`schema/🦀️component.rs` (`DxfArtifact`, the top-level "artifact" facet)** — updated in
   lockstep with `DxfSnapshot`'s field rename since it mirrors the same persisted fields
   one-for-one and its old shape (`tags: Vec<DxfTag>`) would not otherwise compile; this file
   sits directly alongside the three explicitly-named mounted files as a "sibling facet leaf."
7. **`🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs`** (the pre-existing optional triad-dir
   scaffold) — its `diff(snapshot)` helper's signature was widened to `diff(base, snapshot)` to
   match `diff_set_snapshot`'s new two-argument sparse-patch signature (no full-replace slot to
   short-circuit into); this triad leaf is not required per S2's finding but exists on disk and
   needed a one-line signature fix to keep compiling — same shape as the io serializer/
   deserializer fix already flagged as "settled/harmless" in the brief.
8. **Two accidental mis-typed directories** (`🖊️dxf/🏅️标准/...`, the Chinese-character variant of
   "🏅️standards" — an input-rendering glitch, reproduced twice) were created and immediately
   removed (`rm -rf`) before any content besides throwaway test writes landed in them; confirmed
   via `find`/`git status` that no trace remains under `🖊️dxf/`. Two similarly-named EMPTY
   (zero real files) directories were found pre-existing under `📝️md/🏅️标准/` and
   `📰xml/🏅️标准/` — NOT created by this session (outside `🖊️dxf/`, never touched), left alone
   per the ownership boundary.

## glue_followup

- No new top-level directories were created; `glue.rs` was never touched.
- **Non-glue.rs shared-file note for the wave's closer**: `📜️script.ts`'s S-8 allowlists
  (`POLICY_FACET_MIRROR_DRIFT`, `POLICY_GRAMMAR_HONESTY`, `POLICY_DIFF_ALGEBRA`, the field-sweep-
  presence allowlist) still carry dxf's seeded "still drifted/placeholder/missing" entries from
  S2 — this wave's work now satisfies all four checks for dxf (verified: real facet mirrors, real
  grammar leaves, `impl DiffAlgebra` present, `field_sweep` test present) but per this ticket's
  explicit boundary I did not touch `📜️script.ts` myself. The closer should shrink dxf's entries
  out of all four allowlists (S2's report: a stale low-priority breach will point at exactly
  which entries once the seed generator script `s2-artifacts/gen_s8_seeds.ts` is re-run or the
  keys are located by dxf's `policyNormalizeRelPath` short-form).

## Facets updated

`snapshot`, `diff`, `mutations`, `schema` (top-level `DxfArtifact` facet) — all four, Rust +
TS + GraphQL + JSON Schema + proto + all 7 grammar leaves each (text ×3, binary ×4) for
snapshot/diff/mutations (schema/top-level facet has no grammar leaves of its own, consistent with
every other rewritten artifact).
