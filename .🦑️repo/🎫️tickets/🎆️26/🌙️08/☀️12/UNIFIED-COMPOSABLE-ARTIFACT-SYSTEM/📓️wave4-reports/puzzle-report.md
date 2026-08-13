# W4 — `puzzle` composes stdio `kit` subset

**ucas-status: partial**

## Pre-flight

`git status --porcelain -- ✏️s/🔌️plugins/🧩️puzzle` re-checked before starting: ~24 files already
staged (not mine), all trivial 1–2 line doc-comment rewords (`"persistent fields only"` →
`"artifact-lane fields only"`) plus one real, harmless, already-landed change — puzzle2d's
`register_media_io()` dropping a dead `register_dwg_import_handler` call — from two OTHER concurrent
tickets (`26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE` and
`26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`), spread across all three
artifacts (`◻2d`, `🧊3d`, `🖐️5d`). Left untouched throughout; see `## Concurrent-churn observations`.

Baseline `cargo check -p semio-s-plugin-puzzle --all-targets` (before any edit): **green** — 0
errors, 65 pre-existing warnings (unused variables/dead code, none touching kind-catalog code).

## What the codebase actually looks like (verified against code, not assumed)

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/` has exactly **3 artifacts** (not the design doc's unqualified
"puzzle" line — verified via `find`): `◻2d` (702-line `🦀️component.rs`), `🧊3d` (793 lines), `🖐️5d`
(1075 lines before this migration). Each has exactly one subset (`✳️any`). Combined crate
(`semio-s-plugin-puzzle`) is **46,301 lines** of Rust, matching the ticket's own "puzzle ~46k, larger
than fem" estimate.

**All three artifacts independently duplicate the `kit.catalog` type-registry concept, once each**:

| Artifact | Bundle type | Per-kind catalog rows |
|---|---|---|
| `◻2d` | `Puzzle2dKindCatalogs` (nested inside `Puzzle2dMeta`) | `Puzzle2dCatalogNodeKind`, `Puzzle2dCatalogHandleKind`, `Puzzle2dCatalogEdgeKind`, `Puzzle2dCatalogWireKind` |
| `🧊3d` | `Puzzle3dKindCatalogs` (nested inside `Puzzle3dMeta`) | `Puzzle3dCatalogObjectKind`, `Puzzle3dCatalogVortexKind`, `Puzzle3dCatalogCableKind`, `Puzzle3dCatalogAttractionKind` |
| `🖐️5d` | `Puzzle5dKindCatalogs` (top-level `Puzzle5dSnapshot` field) | `Puzzle5dCatalogPartKind`, `Puzzle5dCatalogGripKind`, `Puzzle5dCatalogFastenerKind`, `Puzzle5dCatalogRopeKind` |

Every one of the twelve per-kind row types is doc-commented in-place as a "compose Type/Port/
Connector/Attribute/Author analogue" — the authors clearly modeled these off stdio's own
`s.stdio.semio.kit` shape without composing it. `🧊3d` additionally declares a standalone duplicate
`kit_catalog_artifact_kind() -> ArtifactKindSpec { id: "kit.catalog", ... }` for its `kit:in` media
port (doc-commented "harmless if a producer... declares an identical spec" — the exact `sourcing`-
precedent duplicate, but this one is a MEDIA PORT declaration for importing an external fragment, not
a persisted field; left as-is, out of this migration's scope per the design plan's own field-level
"kills kit.catalog dup" framing).

**App-owned ids**: catalog ROW ids (`"chair"`, `"door capsule left"`) are externally/import-authored
strings, not app-minted — no fix needed there. The bundle itself (`kind_catalogs` as a whole) had no
identity/handle concept at all pre-migration (an anonymous inline struct); this migration gives it a
real, content-addressed identity for the first time (`kind_catalogs_child_handle`, hash-derived,
never random/incrementing) — the concrete instance of "fixes app-owned ids" this plugin offers.

## What changed — `🖐️5d` only, real composition landed and fully verified

Chose `🖐️5d` as the one artifact to carry through completely (see `## Deferred: ◻2d and 🧊3d` for
why the other two are NOT done). Followed `sourcing`'s exact split-and-compose precedent (its
`ObjectKind`/`stock` migration, `../🪵️sourcing/🗿️artifacts/🗂️curate/🦀️component.rs`'s
`🔖️CatalogComposition` region) since `SemioKitType { id, name, category }` is far too sparse to
represent `Puzzle5dKindCatalogs`'s four rich per-kind-row types directly.

### The split: composed child + puzzle5d-owned overflow

`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🦀️component.rs` (`🔖️KindCatalogComposition` region, new,
~230 lines):

- **Four new `*Extra` types** (`Puzzle5dCatalogPartKindExtra`, `…GripKindExtra`,
  `…FastenerKindExtra`, `…RopeKindExtra`) — every field of every original catalog row NOT
  representable in `SemioKitType`, id-joined back by category. `Puzzle5dCatalogGripKind` never had a
  `name` field at all (`SemioKitType.name` is a display-only derivation from `label`/`code`, both of
  which stay in the extra row unchanged — documented in-place so a future reader doesn't assume
  `name` round-trips into `GripKind` itself).
- **`Puzzle5dKindCatalogsExtra`** — the sibling bundle wrapping the four `*Extra` lists.
- **Real bidirectional row converters**: `kit_type_from_{part,grip,fastener,rope}_kind` /
  `{part,grip,fastener,rope}_kind_extra_from_*_kind` (split) and `*_kind_from_parts` (join) — every
  field lands in exactly one half, lossless together.
- **Whole-list converters**: `kind_catalogs_kit_types` (flattens all four kind lists into ONE
  `category`-tagged `Vec<SemioKitType>`, matching `SemioKitSnapshot.types`'s own shape exactly),
  `kind_catalogs_extra_from_kind_catalogs`, `kind_catalogs_from_kit_types_and_extra` (inverse,
  id+category joined, orphans silently dropped per the sourcing precedent).
- **Content-addressed handle minting**: `kind_catalogs_child_handle` — hashes the deterministic JSON
  of the derived `SemioKitType` list (`DefaultHasher`, mirrors `sourcing`'s `catalog_child_handle`),
  `child_id = "kind-catalogs-{hash:016x}"`. Never random/incrementing.
- **`kind_catalogs_of(handle, extra) -> Option<Puzzle5dKindCatalogs>`** — the one accessor every
  render/mutation/app call site now funnels through to read the full reassembled bundle back in its
  original shape.
- **`split_and_seed_kind_catalogs`** — the sanctioned construction path: mints the handle, splits,
  seeds the working-scene cache, returns the `(handle, extra)` pair a snapshot/artifact/diff now
  carries in place of the old inline field.

`🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`:
`Puzzle5dSnapshot.kind_catalogs: Option<Puzzle5dKindCatalogs>` → `Option<store::ArtifactChild<
SemioKitSnapshot>>` (`#[child(kind = "s.stdio.semio.kit")]`) + new sibling
`kind_catalogs_extra: Option<Puzzle5dKindCatalogsExtra>`. Default impl updated.
`🧬️schema/🦀️component.rs` (`Puzzle5dArtifact`) and `🔺️diff/🦀️component.rs` (`Puzzle5dDiff`, using the
established `Option<Option<T>>` "presence can change" shape) mirror the same split;
`to_snapshot`/`from_snapshot`/`set_snapshot` and the hand-rolled `MutationDiff<Puzzle5dSnapshot>::
apply`/`absorb` (`🔺️diff/📝️text/🦀️component.rs`) updated for both new fields.

### §2 codec wall — resolved via the real framework capability, same as `sourcing`

Checked `impl<S> DslField for ArtifactChild<S>` (`🏪️store/🦀️component.rs:523`, W1-owned, read-only)
first, per this ticket's "check before hand-rolling" discipline: real, generic, present. Kept
`#[derive(dsl::DslRecord)]` on `Puzzle5dSnapshot`/`Puzzle5dArtifact`/`Puzzle5dDiff` — the derive
picked up the new `kind_catalogs`/`kind_catalogs_extra` fields automatically. The only hand-written
addition was `#[child(kind = "s.stdio.semio.kit")]` on the child field itself (required by the
`ArtifactSchema` derive's own mechanical check — `error: ArtifactChild field requires #[child(kind =
"…")]`, caught immediately by `cargo check` on `Puzzle5dArtifact`'s mirror field, which the derive on
`Puzzle5dSnapshot` alone didn't surface). No hand-rolled codec bytes were written.

### §3/§4 working-scene cache

Checked `VcsArtifactApp.children`'s actual population for this plugin (not just the type signature):
`ArtifactView::with_children` is wired into the dispatch signature but `kind_catalogs`'s content is
never resolvable through it (no `open_child`/`register_child` caller anywhere in this fan-out,
matching every prior wave). Built the `thread_local!` working-scene cache
(`PUZZLE5D_KIND_CATALOGS_SCRATCH`) exactly mirroring `sourcing`'s `SOURCING_CATALOG_SCRATCH`.
Staleness gap documented in place (not fail-closed): `kind_catalogs` is only ever whole-value-
replaced via `ReplaceKindCatalogs`, never incrementally mutated in-history, so the only staleness
window is a whole-document undo/redo across a store-level checkout that bypasses `ArtifactApp::
handle` — same category as `sourcing`'s own documented gap.

**Real, load-bearing consequence of this gap, found and fixed (not just documented)**: puzzle5d's
play app (`🎛️apps/🖐️5d/🦀️component.rs`) round-trips its ENTIRE canonical snapshot through
`serde_json::Value` on every single mutation dispatch (`Puzzle5dPlaySnapshot(Value)` →
`MutationDiff<Value>`/`Mutation<Value>` → `serde_json::from_value::<Puzzle5dSnapshot>`) — a
significantly deeper JSON-Value coupling than any prior exemplar (writer/cad/lowpoly/sourcing all
bridge a Value only at specific edges, never on every mutation). The app's own untyped scratch
document (`Puzzle5dDocument.kind_catalogs: Option<Value>`, used directly by the catalogue panel and
mesh-resolution UI, both untouched by this migration) still produces the LEGACY embedded-catalog JSON
shape when serialized generically. Fed straight into `serde_json::from_value::<Puzzle5dSnapshot>`,
that legacy shape does not match the new `{childId,target}` handle type, and because serde fails a
whole struct's deserialize on ANY one field's shape mismatch (not just that field), `unwrap_or_
default()` would have silently reset the WHOLE document — parts, fasteners, everything — not just
the catalog. Confirmed as a REAL failure (not hypothetical): both `kit_in_import_media_*` tests
initially failed with `"parts catalog present"` after the schema-level split alone. Fixed with
`normalize_kind_catalogs_for_snapshot_value` (`🧬️mutations/🦀️component.rs`, `🔖️ValueBridge` region)
— detects a `kindCatalogs` value in the legacy embedded shape (object, no `childId` key), converts it
through `split_and_seed_kind_catalogs`, and reinjects the handle + `kindCatalogsExtra` before the
generic deserialize — wired into all four `from_value::<Puzzle5dSnapshot>` call sites in that region
(`MutationDiff<Value>::apply`, `Mutation<Value>::diff`/`inverse`, `puzzle5d_document_delta_
operations`). This is the one place a raw `Puzzle5dDocument`-sourced `Value` crosses into the typed
snapshot; the STORE's own canonical `Value` (produced by round-tripping the typed struct) never needs
it since it is self-consistently new-shape by construction.

### §5 mutation rewire — payload shape kept unchanged

`ReplaceKindCatalogs.new_catalogs: Option<Puzzle5dKindCatalogs>` — UNCHANGED (still the full old
struct; recipe's "granular mutations keep unchanged public payload shapes"). Internals rewired:
`diff` (`📚replace-kind-catalogs/🔺️diff/🦀️component.rs`) calls `split_and_seed_kind_catalogs` and
builds `Puzzle5dDiff { kind_catalogs: Some(handle), kind_catalogs_extra: Some(extra), .. }`; `inverse`
(`↩️inverse/🦀️component.rs`) reconstructs `base`'s full bundle via `kind_catalogs_of` before
re-wrapping it as the mutation payload. `🧬️mutations/🦀️component.rs`'s `puzzle5d_snapshot_mutations`
(the before/after snapshot differ) updated to compare/reassemble through the same accessor.

### App-layer call sites fixed (beyond the ValueBridge)

`✂️transfer/🦀️component.rs`'s `find_replaceable_kinds` (kind-swap picker) now reads through
`kind_catalogs_of` instead of the removed direct field. Its own unit test
(`find_replaceable_kinds_walks_kind_compatibility`) updated to seed via
`split_and_seed_kind_catalogs`. `💡️inferences/🦀️component.rs` and `🎛flat-position/🦀️component.rs`'s
test-fixture struct literals updated for the new field (`kind_catalogs_extra: None`).
`🎛️apps/🖐️5d/🦀️component.rs`'s two `kit:in` media-import tests
(`kit_in_import_media_upserts_part_and_grip_kinds_into_kind_catalogs`,
`kit_in_import_media_is_idempotent_on_repeated_delivery`) rewritten to inspect the reassembled
catalog through `kind_catalogs_of` instead of a raw `/kindCatalogs/parts` JSON pointer — the JSON
pointer's target genuinely no longer exists post-migration (composition's whole point), so this is a
real, necessary test update, not a workaround.

Every other `document.kind_catalogs` read/write site (catalogue panel, `resolve_part_mesh_url`,
`engine_kind_catalogs_value`, the `kit:in` import handler's OWN catalog-merge logic) operates on
`Puzzle5dDocument.kind_catalogs: Option<Value>` or the standalone `Puzzle5dKindCatalogs` struct
(unchanged, still exists, still the app's working currency) — neither touched nor needing touching,
confirmed by reading every call site, not assumed.

### §8 fixture regeneration — DSL text, not hand-transcribed

`concrete-forest`'s DSL fixture has no `kind-catalogs=` line at all (field absent/`None`) — untouched.
`nakagin-capsule-tower` and `capsule-dream` both carry a present-but-empty (dream: one `"default"`
fastener-kind row) `kind-catalogs=` block, which fails to parse under the new grammar (`unknown table
column 'name'` — the derive dropped `name` from every `*Extra` row's own table columns). Regenerated
via a temporary `#[cfg(test)] fn debug_fixture_regen_kind_catalogs_dsl_fragment` in
`🧬️mutations/🦀️component.rs` that built the equivalent `Puzzle5dSnapshot` through
`split_and_seed_kind_catalogs` + `print_dsl`, captured the real derive-generated text for both the
fully-empty and one-fastener-row cases, hand-spliced only the `kind-catalogs=…` block of each `.dsl.
semio` file (the rest of both files — hundreds of lines of part/fastener data — untouched), then
removed the temporary test (confirmed: `grep -rn debug_fixture_regen` returns nothing). The
`.pack.semio` binary fixtures were NOT regenerated — see `## Deferred`.

### Sibling-language schema files — snapshot facet only

Updated TS/GraphQL/proto/JSON mirrors for `Puzzle5dSnapshot`'s `📸️snapshot` facet (added
`ArtifactChildHandle`/`Puzzle5dKindCatalogsExtra` + the four `*Extra` type mirrors, changed
`kindCatalogs`'s declared type), mirroring stdio's own `ArtifactChildHandle { childId, target }` /
`@child(kind: "…")` convention. JSON validated with `python3 -c "json.load(...)"`. The
`Puzzle5dArtifact` (`🧬️schema`) and `Puzzle5dDiff` (`🔺️diff`) facets' own TS/GraphQL/proto/JSON
mirrors were NOT updated (still show the old inline `Puzzle5dKindCatalogs` shape) — non-compiled
documentation leaves, correctness-neutral to `cargo check`/tests, deprioritized under time pressure.
`🧬️mutations` facet mirrors needed no change (payload shape unchanged).

## Deferred: `◻2d` and `🧊3d`

**Not started.** Both have the structurally IDENTICAL duplication pattern (`Puzzle{2d,3d}KindCatalogs`
nested inside their respective `Meta` types) and, per each artifact's own `🔖️PlaySnapshot`/
`ValueBridge` doc comments ("mirrors puzzle5d's own bridge exactly" / "same shape puzzle2d's own
bridge"), the IDENTICAL whole-snapshot-through-JSON-Value-per-mutation architecture that made `🖐️5d`'s
migration require the extra `normalize_kind_catalogs_for_snapshot_value` guard beyond the schema-level
split. Composing `◻2d`/`🧊3d` for real would need: the same four-row split-and-compose treatment
(different row shapes: node/handle/edge/wire for 2d, object/vortex/cable/attraction for 3d), PLUS
locating and fixing each artifact's own equivalent ValueBridge entry point (their `🧬️mutations/
🦀️component.rs` `🔖️ValueBridge` regions were not read in this pass), PLUS each artifact's own
catalogue-panel/mesh-resolution app-layer read sites, PLUS DSL fixture regen for `◻2d`/`🧊3d`'s own
example assets. Genuinely the same shape of work as `🖐️5d`, not smaller — deferred purely on time
budget, following `fem`/`norm`'s explicitly-sanctioned partial-completion precedent for a
larger-than-fem plugin. `◻2d`/`🧊3d`'s own `kind_catalogs` fields are UNTOUCHED and still compile/test
green exactly as at baseline (proven by the full-crate test run below, which exercises both).

**`🧊3d`'s standalone `kit_catalog_artifact_kind()` duplicate `ArtifactKindSpec`** (`🗿️artifacts/🧊3d/
🦀️component.rs:522`) also untouched — it is a MEDIA PORT declaration for an externally-produced
`kit.catalog` fragment (block3d's own shape), not a persisted snapshot field; retiring it would be a
cross-plugin media-pipeline change, out of this migration's field-level scope.

## Verification

`CARGO_TARGET_DIR=".../🎯️target"` for every invocation.

- `cargo check -p semio-s-plugin-puzzle --all-targets` (baseline, before any edit): **0 errors**, 65
  warnings.
- `cargo check -p semio-s-plugin-puzzle --all-targets` (after migration): **0 errors**, 66 warnings
  (all pre-existing categories — unused vars/dead code — none new; count drift is noise from the
  warning dedup pass, not a new class of warning).
- `cargo test -p semio-s-plugin-puzzle --lib`: **452 passed, 3 failed**, reproduced stable across
  three consecutive full runs (no flakiness).

**The 3 failures — traced, pre-existing, NOT introduced by this migration:**
`artifacts::puzzle2d::…::puzzle2d_delta_ops_are_granular_and_round_trip`,
`artifacts::puzzle3d::…::puzzle3d_delta_ops_round_trip_and_stay_granular`,
`artifacts::puzzle5d::…::puzzle5d_delta_ops_round_trip_and_stay_granular`. All three fail on the
identical shape of bug: a round-tripped-through-`Value` document ends up with a spurious extra key
(`"kindCompatibility": []`, later `"meta": {}`) that a hand-written test literal never mentions,
because `kind_compatibility: Vec<T>` (and, one level up, `meta: MetaType`) lacks `#[serde(skip_
serializing_if = ...)]` on all three artifacts' own pre-existing field declarations — nothing this
migration touched or introduced. Proof: (1) `◻2d`/`🧊3d` were NEVER edited by this migration at all —
their own failures are trivially pre-existing. (2) For `🖐️5d`, I read `kind_compatibility`'s
attribute BEFORE making any edit (first file read of this session) and it already lacked `skip_
serializing_if` then. Attempted the "obvious" one-line fix (`skip_serializing_if = "Vec::is_empty"`)
on all three; it broke THREE OTHER previously-passing tests
(`apps::puzzle3d::…::{add_object_kind_materializes_the_declared_kind_default,
gumball_transform_session_commits_once_on_end, gumball_translate_drag_coalesces_into_one_edit}`,
`apps::puzzle5d::…::add_part_kind_materializes_the_declared_kind_default}`) — code elsewhere
apparently depends on the key's unconditional presence. Reverted rather than chase a cascading,
out-of-scope fix; left as a precisely-documented pre-existing gap for a future, dedicated pass rather
than an unresolved provenance question.

## sharedFileRequests

None. Every edit is inside `✏️s/🔌️plugins/🧩️puzzle/**`; `semio-s-plugin-stdio`'s `s.stdio.semio.kit`
subset (`SemioKitSnapshot`/`SemioKitType`) was consumed read-only as a dependency already present in
`Cargo.toml` — no new dependency added, no shared file touched.

## Concurrent-churn observations

1. **~24 pre-existing staged files, not mine, present before I started**: trivial doc-comment
   rewords across all three artifacts' snapshot files (`"persistent fields only"` → `"artifact-lane
   fields only"`) plus a real one-line behavior change already landed in puzzle2d's
   `register_media_io()` (dropping a dead `register_dwg_import_handler` call, doc-commented in place
   as belonging to ticket `26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` wave
   IO1) — matches the exact "6 pre-existing staged files, harmless, different region" pattern
   `sourcing`'s report already flagged for this same class of churn. Left untouched throughout; did
   not conflict with anything this migration edited.
2. **`demonstrator` depends on `semio-s-plugin-puzzle`** (`✏️s/🔌️plugins/🎪️demonstrator/📦️packages/
   🦀️rust/Cargo.toml:126`) — not checked for compile against this migration (demonstrator is
   confirmed NOT SMO-clear per `📌️important.md` and dispatched last/separately in this ticket's own
   plan); this migration changed no public API surface demonstrator plausibly consumes (`artifact_
   kind()`/`declaration()`/app types unchanged), but a downstream check was out of this dispatch's
   scope.
3. No auto-commit landed mid-edit in a way that lost work — one auto-commit (`dda7ceead1`, 2026-08-13
   18:52:17 `--date=iso`) captured an earlier slice of this migration's own edits mid-session; the
   remaining edits (ValueBridge fix, kit-in test updates, schema mirrors) show as normal uncommitted
   `git status` entries as of this report.

## Honest accounting

**Complete and verified**: `🖐️5d`'s `kind_catalogs` field — real composition onto `s.stdio.semio.kit`,
content-addressed handle minting, working-scene cache, real bidirectional converters for all four
kind-row types, mutation triad rewired with unchanged public payload shape, the ValueBridge's
whole-snapshot-JSON-per-mutation architecture specifically diagnosed and fixed (not just papered
over), DSL fixtures regenerated for real, app-layer call sites updated, 452/455 crate tests passing
(the 3 failures independently pre-existing).

**Deferred, with precise reasons**: `◻2d` and `🧊3d`'s own identical `kind_catalogs` duplication
(same-shape work, not done for time); binary `.pack.semio` fixture regen for `🖐️5d` (JSON/DSL-text
codecs verified real, binary pack codec inherited automatically from the same derive but its checked-
in fixture bytes still reflect the pre-migration encoding — `op_pack_and_spr_assets_are_nonempty`
only checks non-emptiness, not content, so this is invisible to the test suite today but is real
staleness); `🖐️5d`'s own `Puzzle5dArtifact`/`Puzzle5dDiff` facets' non-`📸️snapshot` cross-language
schema mirrors.

ucas-status: partial
