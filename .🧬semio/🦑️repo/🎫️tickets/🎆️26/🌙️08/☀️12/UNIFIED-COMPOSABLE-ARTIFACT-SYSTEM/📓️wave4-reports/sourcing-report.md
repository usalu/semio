# W4 — `sourcing` composes stdio `kit` subset

**ucas-status: complete**

## Pre-flight

`git status --porcelain -- ✏️s/🔌️plugins/🪵️sourcing` re-checked before starting: 6 files staged
(not mine), all trivial doc-comment rewords (`"persistent fields only"` → `"artifact-lane fields
only"` etc.) plus a mechanical `type Transient = semio_framework_plugin::NoTransient;` /
`type TransientMutation = …NoTransientMutation;` addition to `SourcingCurateApp`'s `ArtifactApp`
impl — a repo-wide framework trait-surface addition landing elsewhere in this ticket, unrelated to
this migration. Left untouched; see `## Concurrent-churn observations`.

Baseline `cargo check -p semio-s-plugin-sourcing --all-targets` (before any edit): **red**, but the
4 errors (`cannot find module or crate graph_core`) all originated in
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/🔗️causal-internals/🦀️component.rs`
— DKM's live `math`→`geometry`/`graph` crate-extraction rename, confirmed by grep (zero errors
referenced `sourcing`). Not attributable to this migration; cleared on its own after the migration
edits landed (see Verification).

## What sourcing duplicated

`🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`'s
`CurateSnapshot.stock: Vec<ObjectKind>` was a hand-rolled, app-owned type/catalog registry —
`ObjectKind { id, name, module_id, typology_path, availability, geometry }` — duplicating the
same "type registry" concept `s.stdio.semio.kit`'s `SemioKitType { id, name, category }` exists to
own once. `🎛️apps/🗂️curate/🦀️component.rs`'s `catalog:out` port and
`💡️inferences/🦀️component.rs`'s `sourcing_catalog_fragment` both independently declared a
`kit.catalog` `ArtifactKindSpec`, doc-commented in-place as "harmless duplicate — `s/plugin/block`'s
`3d` app declares the SAME shape independently" — exactly the `kit.catalog` dup the design plan
names. `ObjectKind.id` was an arbitrary app-minted string (`"beam-glulam-gl24h"`), not
content-addressed.

`ObjectKind` carries fields (`module_id`, `typology_path`, `availability`, `geometry`) that
`SemioKitType` cannot represent — this is NOT a lossless 1:1 field rename, so a straight "replace
the field with a child handle" (the lowpoly/cad/writer pattern) would have silently dropped data.

## What changed

### The split: composed child + sourcing-owned overflow

`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🦀️component.rs` (`🔖️CatalogComposition` region, new):

- **New type** `ObjectKindExtra { id, typology_path, availability, geometry }` — the half of
  `ObjectKind` NOT representable in `SemioKitType`. `#[dsl(defines = "object")]` moved here from
  `ObjectKind` so `CuratedItem.object_id`'s `#[dsl(refs = "object")]` referential-integrity check
  still validates against the right namespace.
- **Real bidirectional converters**: `kit_type_from_object_kind`/`object_kind_extra_from_object_kind`
  (split) and `object_kind_from_parts` (join) — every `ObjectKind` field lands in exactly one half,
  lossless together. `catalog_snapshot_from_stock`/`stock_extra_from_stock`/
  `stock_from_catalog_and_extra` lift these to whole-list operations.
- **Content-addressed handle minting**: `catalog_child_handle(stock: &[ObjectKind]) ->
  store::ArtifactChild<SemioKitSnapshot>` — hashes the deterministic JSON of the derived
  `SemioKitType` list (`DefaultHasher`, mirrors lowpoly's `mesh_child_handle`), `child_id =
  "catalog-{hash:016x}"`. Never a random/incrementing id — this is the "fixes app-owned ids" half of
  the design plan's directive.
- **`stock_of(document: &CurateSnapshot) -> Vec<ObjectKind>`** — the one accessor every
  render/export/inference/command call site funnels through, reassembling the full catalogue from
  the working-scene cache + `stock_extra`.
- **`curate_snapshot_from_stock`/`seed_catalog_scratch`** — the sanctioned construction paths; every
  fixture/test/command that used to write `CurateSnapshot { stock, .. }` directly now goes through
  one of these.

`🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`:
`CurateSnapshot.stock: Vec<ObjectKind>` → `catalog: store::ArtifactChild<SemioKitSnapshot>`
(`#[child(kind = "s.stdio.semio.kit")]`) + `stock_extra: Vec<ObjectKindExtra>`. Hand-written
`impl Default` (no blanket `Default` on `ArtifactChild<S>`), mints the same empty-stock handle
`catalog_child_handle(&[])` would.

`🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`: `CurateArtifact`
mirrors the same split (`to_snapshot`/`from_snapshot`/`set_snapshot` updated); `filtered_stock` now
returns owned `Vec<ObjectKind>` (was `Vec<&ObjectKind>`, borrowing directly off the now-gone
`stock` field) via `stock_of`; `curation_decision_for_delta`/`_for_set` read `availability` straight
off `stock_extra` (no need to resolve the composed child just to clamp a count);
`default_document`/`empty_document` seed the working-scene cache via `seed_catalog_scratch` before
parsing the persisted DSL (a composed child is a handle only, never inline content, in the parent's
own text — the fixture's embedded `catalog` handle is content-addressed from the SAME
`demo_stock()`/`&[]` this seeds from, so they resolve to the same `child_id` by construction). New
`demo_stock()` helper replaces what used to be 5 independently-duplicated
`sourcing_modules().iter().flat_map(...)` call sites across test modules.

`🧬️schema/💡️inferences/🗃entries/🦀️component.rs`: `compute_curate_entries`'s `stock_count` now
reads `stock_extra.len()` directly (1:1 with the composed catalog's `types`, no need to resolve the
child just to count it).

### §2 codec wall — resolved via a real framework capability, not hand-rolling

The recipe says `ArtifactChild<S>` has no `DslField` impl and to drop `dsl::DslRecord` + hand-roll
the whole codec. Checked first against `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
(W1-owned, read-only) per this ticket's own "check before assuming the workaround is needed"
discipline: `impl<S> crate::os_dsl::DslField for ArtifactChild<S>` (`:523`) now exists — generic
over any `S`, no bound, `Shape::Record` of `(child_id, target)` — a real capability, not present when
lowpoly/cad/writer/stdio's own `kit` subset were migrated (all four hand-roll). Kept `dsl::DslRecord`
on `CurateSnapshot`, added the `catalog`/`stock_extra` fields with `#[child(kind = "…")]` /
`#[dsl(defines = "object")]` alongside the existing `#[state(artifact)]`, and the existing
hand-rolled `impl store::ArtifactDsl`/`impl store::ArtifactPack` (which delegate to derive-emitted
`__dsl_spec()`/`__dsl_to_record()`/`__dsl_from_record()`) needed ZERO changes — the derive picked up
the new fields automatically. No hand-rolled codec bytes were written for this snapshot; every field
round-trips through both codecs because the SAME derive-generated machinery that already covered
`curated`/the old `stock` now covers `catalog`/`stock_extra` too (verified by the round-trip test
suite, not assumed — see Verification). Same pattern applied to `CurateDiff`'s `catalog`/`stock_extra`
fields (still `Serialize`/`Deserialize`-only, no `dsl` codec on the diff type, matching its pre-
migration shape).

### §3/§4 working-scene cache

Checked `VcsArtifactApp.children`'s actual population for this plugin (not just the type signature)
per the recipe's explicit instruction — `ArtifactView::with_children` is wired into
`ArtifactApp::render`/`handle`'s call signature but `catalog`'s content is never resolvable through
it (no `open_child`/`register_child` caller anywhere in this fan-out, matching every prior wave's
finding). Built the `thread_local!` working-scene cache
(`SOURCING_CATALOG_SCRATCH: RefCell<HashMap<String, SemioKitSnapshot>>`) in
`🗿️artifacts/🗂️curate/🦀️component.rs`, next to the plugin's existing module-level statics
(`CONTRIBUTED_SOURCING_MODULES`). Populated at construction time (`curate_snapshot_from_stock`,
`seed_catalog_scratch`); read through the single `stock_of` accessor. Staleness gap documented
(not fail-closed — this is a read-only catalogue display path, not a destructive edit path: `catalog`
is never incrementally mutated in-history, only whole-document-replaced, matching `stock`'s
pre-migration bulk-population rule, so there is no undo/redo-of-an-edit scenario to go stale across
the way lowpoly's mesh-editing session has).

### §6 — SetSnapshot already banned pre-migration, nothing to do

`stock` was never mutated through `SourcingMutation`'s three variants
(`CreateCuratedItem`/`DeleteCuratedItem`/`ChangeCuratedItemCount`) even before this migration — the
mutations file's own doc comment already states `stock` is "a bulk-populated reference catalogue …
whole-catalogue population goes through `store::ArtifactStore::reset`". `CurateDiff.stock` (now
`catalog`/`stock_extra`) is likewise never populated by any real mutation triad, same as before. No
`whole_document_operation` override existed to remove; `reset_document_effect` already the sole
whole-document-replace path. `apps/curate/🎮️commands/📄️artifact/🦀️component.rs`'s
`stock_from_catalogue::handle` (the one command that bulk-merges catalogue kinds) rewired to build
its stock via `stock_of(doc.snapshot)` + push, then `curate_snapshot_from_stock` to remint the
handle and reseed the cache — same "whole-document replace via `reset_document_effect`" shape as
before, just working over the reassembled `Vec<ObjectKind>` instead of a directly-embedded field.

### §8 fixture regeneration

`🖼️assets/🗣️example.dsl.semio` (the demo-stock fixture) and the hand-literal `EMPTY_CURATION_TEXT`
constant (`📸️snapshot/📝️text/🦀️component.rs`) were both in the obsolete pre-migration format
(`stock=[...]`). Regenerated via the temporary-debug-test technique: ran the plugin's existing
`#[ignore] export_demo_stock_fixture_text` test (`cargo test … export_demo_stock_fixture_text --
--ignored --nocapture`) for the demo fixture, and added + ran + removed a temporary
`debug_fixture_regen_empty_curation_text` test for the empty-curation constant (confirmed removed:
`grep -rn debug_fixture_regen` returns nothing). Both captured outputs written verbatim as the new
fixture content — never hand-transcribed.

### Sibling-language schema files

Updated the TS/GraphQL/Proto/JSON facet mirrors for the three changed types (`CurateSnapshot`,
`CurateArtifact`, `CurateDiff`) to replace `stock`/`CurateStockDelta`/`CurateObjectKindPatchEntry`
with `catalog`/`stockExtra`/`ObjectKindExtra`/`CurateStockExtraDelta`/
`CurateObjectKindExtraPatchEntry`, mirroring stdio's own `s.stdio.semio.kit` snapshot's established
`ArtifactChildHandle { childId, target }` / `@child(kind: "…")` cross-language convention exactly
(`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️kit/🧬️schema/📸️snapshot/{🟦️,🔗️,🛰️,🔣️}component.*`,
read-only reference). Mechanical type-shape updates only (no codec logic in these languages either
before or after — they are documentation/schema-descriptor leaves, `include_str!`'d, not compiled).
All three JSON files re-validated with `python3 -c "json.load(...)"`.

## No mutation-triad rework needed

`stock`/`catalog`/`stock_extra` never had a `🧬️mutations/<slug>/` triad (confirmed: the mutations
family doc comment explicitly excludes `stock` from the closed vocabulary, `curated`'s three verbs
`create`/`delete`/`change` are unaffected). No new/restructured facet authored, so SMO's mechanical
gates (triad↔dispatch 1:1, unique emoji, real leaves, non-stub TS) don't apply here — nothing to
check against `../SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md`.

## Verification

`CARGO_TARGET_DIR=".../🎯️target"` for every invocation below.

- `cargo check -p semio-s-plugin-sourcing --all-targets`: **0 errors** (26 pre-existing warnings,
  none introduced by this migration except one trivial `unused variable: item` at
  `🧬️schema/🦀️component.rs:344` which predates this migration — same shadow pattern existed before
  my edit touched that function's first two lines only — fixed outright, `Some(item)` →
  `Some(_item)`, per this ticket's "cheaper to just fix than chase" guidance).
- `cargo nextest run -p semio-s-plugin-sourcing --no-fail-fast`: **78 tests run, 78 passed, 1
  skipped** (the `#[ignore]` manual-fixture-export test — expected). Reproduced stable across two
  consecutive full runs.
- `cargo check`/`cargo nextest run -p semio-s-plugin-sourcing-beams -p
  semio-s-plugin-sourcing-windows -p semio-s-plugin-sourcing-slabs --all-targets` (the three sibling
  extension crates that depend on `sourcing_curate::artifacts::curate::ObjectKind`/`TypologyNode`/
  `SourcingModule` for their `sourcing.module` contributions): first attempt hit the SAME transient
  upstream `entropy_internals` churn (below); retried in the foreground, **0 errors**, then **3
  tests run, 3 passed** — `ObjectKind`'s own shape is unchanged (only stopped being a direct
  `CurateSnapshot` field), so these crates needed no source changes at all.

**Post-verification churn escalation, noted honestly**: a further re-run (done as an extra
confidence check after the report above was drafted) hit a WORSE state of the same DKM math-
dissolution churn — `semio-framework-math` itself now fails (`unresolved import crate::algebra` in
`🌫️fuzzy/🦀️component.rs`), and a subsequent `cargo check -p semio-s-plugin-sourcing --all-targets`
showed 30 stdio errors (up from the original baseline's 4). Confirmed via `grep -- "-->" | grep
🪵️sourcing` on that run's full output: **zero** error locations under
`✏️s/🔌️plugins/🪵️sourcing/**` — every single one is inside `stdio/**` or
`🧰️framework/🔨️modules/🧮️math/**`, both outside this fan-out's boundary and both mid-refactor by
another live session right now. Per this ticket's transient-failure protocol, not chased further
with retry-sleep loops; the 78/78-passed, twice-reproduced clean run recorded above (captured before
this escalation) stands as the last point at which the dependency tree was stable enough to fully
verify sourcing's own migration, and nothing in the diffs between that point and now touches
`sourcing/**`.

## sharedFileRequests

None. Every edit is inside `✏️s/🔌️plugins/🪵️sourcing/**`; stdio was read-only reference throughout
(`s.stdio.semio.kit`'s `SemioKitSnapshot`/`SemioKitType` consumed as a dependency, `semio-s-plugin-stdio`
was already a `Cargo.toml` dependency of this crate — no new dependency added).

## Concurrent-churn observations

1. **6 pre-existing staged files** (not mine, present before I started): trivial doc-comment
   rewords + a `Transient`/`TransientMutation` associated-type addition to `SourcingCurateApp`'s
   `ArtifactApp` impl (`🎛️apps/🗂️curate/🦀️component.rs`, 2-line addition). Left untouched — did not
   conflict with anything this migration touched (different region of the same `impl` block).
2. **Baseline red, upstream, cleared on its own**: 4 `cannot find crate graph_core` errors in
   `stdio`'s `✳️table/🧬️schema/🔗️causal-internals/🦀️component.rs` at the very first baseline check —
   DKM's live `math`→`geometry`/`graph` crate-extraction rename (matches `📌️important.md`'s explicit
   warning). Confirmed via grep: every error path was under `stdio/**`, zero referenced `sourcing`.
   Gone by the time the migration's own edits were checked.
3. **Extension-crate check hit transient churn too**: first `cargo check -p
   semio-s-plugin-sourcing-{beams,windows,slabs}` attempt produced 98 errors, ALL inside
   `🧰️framework/**` (`entropy_internals` unresolved-import churn in `🔨️modules/🧊️3d`/`📐️brep` and
   `dsl`/`spr`/`store`), zero inside `✏️s/🔌️plugins/🪵️sourcing/**` (confirmed via grep on the `-->`
   lines). Retried in the foreground once (no background waits), cleared, 0 errors.

ucas-status: complete
