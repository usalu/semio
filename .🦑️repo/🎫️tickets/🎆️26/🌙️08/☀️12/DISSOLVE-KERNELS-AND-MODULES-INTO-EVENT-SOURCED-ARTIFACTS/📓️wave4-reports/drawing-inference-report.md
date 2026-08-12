# Wave 4 report — ✳️drawing inference facet

Author: this session (inference facet for `✳️drawing`; the 17 mutation triads were already DONE and
verified by an earlier wave — untouched here).

Boundary: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🧬️schema/💡️inferences/**`
plus the `✳️drawing` inference mount block in stdio's `📦️glue.rs`. `🧬️mutations/`, `🔺️diff/`, and
`📸️snapshot/` were not touched (confirmed via `git status`/`git log` — zero commits from this session
against those paths).

## Starting state — most of the facet was already authored, not a blank slate

On arrival, `💡️inferences/` already existed on disk (family-root `🦀️component.rs`/`🟦️component.ts`/
`🔗️component.graphql`/`🔣️component.json`/`🛰️component.proto` + declaration-only `💾️binary`/`📝️text`
leaves + the `🎛flattened-scene/` slug dir with `🦀️component.rs`/`🟦️component.ts`), and the
`📦️glue.rs` mount block for it was already present and correct (verified byte-for-byte against a
fresh `os.listdir()` of the real directory — see "Mount verification" below). `git log` shows this
landed in this ticket's own auto-commit history (`382ace1b27`), i.e. earlier work on this exact wave,
not another session's. This session's job was therefore: **verify the existing facet is honest and
conforming, strengthen its incrementality-law coverage to explicitly match the three shapes the
ticket names, and produce a real, executed, pasted-output verification** — not author it from zero.

## The flagship field — `flattenedScene`

`DrawFlattenedScene` (`💡️inferences/🎛flattened-scene/🦀️component.rs`) is a real
`store::InferredField<SemioDrawingSnapshot>`: one `InferenceStep` per scene-graph entity (a `Group`
or one of its descendants), keyed `"<layer>:<p0>.<p1>..."` (the same structural `NodePath` address
every mutation triad in this facet already uses in place of a stable node id — `DrawNode` carries
none). This is the direct schema-level replacement for the framework's own (this-ticket-deleted)
`◻2d/🗄️store/🦀️component.rs` `DrawingEngine::compute`/`DrawingStore::flatten_handle`/
`flatten_scene_sync`: world transform composed down through nested `Group`s (`compose_transform`,
standard TRS scene-graph rule — scale then rotate then translate, matching `SemioTransform`'s own
field order) plus, for `Path`/`Text` leaves, `style: Option<String>` resolved into the real
`DrawStyle` value (not merely its name).

**The dependency chain is honest, not a whole-snapshot cache**: a `Group`'s `InferenceStep.parents`
is its enclosing `Group` (none for a layer root); `dep_input` for a `Group` covers only its own local
`transform` fields; `dep_input` for `Path`/`Text` covers only its `style` reference AND the
referenced style's resolved fields (so a `change-stroke-color`/`replace-fill`/… on the style, which
never touches the entity's own node fields, still correctly invalidates it). `Path.segments` and
`Text.value`/`Text.at` are deliberately excluded from `dep_input` — `compute` never reads them
(`flattenedScene` only ever produces `world_transform`/`resolved_style`, never geometry), so including
them would be dishonest padding, not a real dependency.

**`bounds`/per-node bounding-box was considered and NOT authored.** Unlike brep's `validation-report`
(a whole-document check with a legitimate single-key root chain), a per-node bounding box over `Path`
geometry requires evaluating `PathSegment` curves (bezier/arc extents) — the same "no honest home at
this layer yet" reasoning brep's own report already applied to `tessellation`/`mass-properties`. No
such field is present in `SemioDrawingInference`; nothing was silently dropped, there was simply
nothing else with an honest per-entity chain to author (`canvas`/`styles` are already fully persisted
snapshot fields, not derived).

## Incrementality laws — authored AND executed, matching the puzzle3d pilot's three shapes

The facet already had two incrementality tests when this session started
(`changing_the_referenced_style_invalidates_only_entities_referencing_it`,
`changing_the_root_transform_recomputes_the_whole_subtree`). This session strengthened and extended
them to explicitly match all three shapes the ticket names, mirroring
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🎛flat-position/🦀️component.rs`'s
own test pattern:

1. **`changing_a_leaf_own_style_does_not_recompute_ancestors_or_siblings`** (renamed/strengthened from
   the pre-existing style test) — changes a LEAF's own field (`style`, the only field of `Path`/`Text`
   this inference reads). Added a `hits` assertion (`after.hits - before.hits == 3`) alongside the
   existing `misses` assertion: a miss-count-only check can't distinguish "only the leaf missed" from
   "the leaf missed and nothing else was ever looked up" — the hits delta proves the root and the
   unrelated nested-Group/nested-Text subtree were actually consulted and found warm.
2. **`changing_the_root_transform_recomputes_the_whole_subtree`** (pre-existing; added a
   `hits delta == 0` assertion) — an ancestor (the layer root) change must miss for every entity in
   the plan, since every descendant transitively folds the root's `DepHash` into its own chain.
3. **`an_unrelated_sibling_edit_leaves_the_other_siblings_chain_warm`** (new) — a dedicated two-Path,
   two-independent-style fixture (`two_independent_styled_siblings`, siblings under a shared root,
   neither an ancestor of the other) proves editing sibling 0's referenced style leaves sibling 1's
   flattened value **byte-identical** (`assert_eq!(values[...], sibling_before)`) and its cache entry
   warm (`hits delta == 2`: root + sibling 1).

All three, plus the pre-existing `disabled_cache_matches_pure_recompute` (cache-transparency law),
`style_reference_resolves_to_the_real_value`/`world_transform_composes_down_through_nested_groups`
(functional correctness), and `quaternion_rotation_of_identity_is_a_no_op` (math unit test), were
**actually run**, not merely written — see below.

## Mount verification — paths generated from a real directory listing, not hand-typed

Per the ticket's unicode-normalization warning, the mount block's four `#[path]` strings were
diffed against a fresh `os.listdir()`/`os.path.exists()` of the real filesystem rather than trusted
by inspection:
```
top level: ['🎛flattened-scene', '💾️binary', '📝️text', '🔗️component.graphql', '🔣️component.json',
            '🛰️component.proto', '🟦️component.ts', '🦀️component.rs']
flattened-scene: ['🟦️component.ts', '🦀️component.rs']
```
All four `#[path]` targets in `📦️glue.rs`'s `pub mod inferences { … pub mod flattened_scene { … } }`
block (lines ~6417–6431) resolved `True` via `os.path.exists()` against this listing. The block was
already present and correct on arrival — no edit was needed or made to it this session.

## Files touched

**Updated** (1 file, test-region only):
- `🧬️schema/💡️inferences/🎛flattened-scene/🦀️component.rs` — strengthened one existing
  incrementality test with a `hits` assertion, added `hits` assertion to another, added the
  `two_independent_styled_siblings` fixture + `an_unrelated_sibling_edit_leaves_the_other_siblings_chain_warm`
  test (region `🧪️IncrementalityLaw`, `#🧪️Tests`).

**Verified, not modified**: `💡️inferences/🦀️component.rs`, `💡️inferences/🟦️component.ts`,
`💡️inferences/🔗️component.graphql`, `💡️inferences/🔣️component.json`, `💡️inferences/🛰️component.proto`,
`💡️inferences/💾️binary/**`, `💡️inferences/📝️text/**`, `💡️inferences/🎛flattened-scene/🟦️component.ts`,
and the `📦️glue.rs` mount block — all present, non-stub, and correct on arrival; confirmed by full
read and the directory-listing diff above.

**Created**: this report.

## Verification commands run, with real output pasted

Forced a real recheck (touched the crate file first, per the "cached check re-emits no diagnostics"
warning):
```
touch "✏️s/…/✳️drawing/🧬️schema/💡️inferences/🎛flattened-scene/🦀️component.rs"
RUSTC_WRAPPER="" CARGO_TARGET_DIR=".…/🎯️target" cargo check -p semio-s-plugin-stdio --all-targets
```

### Concurrent churn encountered — 9 of 10 check runs, ~10 minutes, zero attributable to this facet
Runs 1–9 of `cargo check -p semio-s-plugin-stdio --all-targets` were red, with the specific error set
changing almost every time — the same non-atomic-concurrent-write pattern brep's own wave report
documented for `✳️drawing` (there, from the other side). This time it was `✳️mesh`, another session's
subset (explicitly "SMO's/APA's, not ours" per `📌️important.md`'s hot-file table), actively landing
its own triad vocabulary + inference facet live:
- Run 1: `error[E0433]: cannot find inferences in schema` — a stray `✳️step`/ap214 site, the exact
  documented "known residual", unrelated to mesh or drawing.
- Run 2: `unresolved imports …mesh::schema::diff::diff_set_material_base_color…` +
  `NoMutation found for enum SemioMeshMutation` (8 errors, all `✳️mesh`).
- Runs 3–6 (spanning ~4 minutes, `git status`/`stat` confirmed `📦️glue.rs` was being actively rewritten
  each time): `couldn't read …✳️mesh/🧬️schema/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs: No
  such file or directory` — a dangling `#[path]` mount to a file mesh's session had deleted but not
  yet unmounted (`git status --porcelain` on `✳️mesh` independently confirmed: 6 `set-snapshot` files
  ` D` deleted-in-worktree, 15 new untracked triad dirs, `🧬️mutations/🦀️component.rs` modified — a
  session mid-flight, not gone).
- Runs 7–9: `cannot find type SemioMeshSnapshot` (4 errors, all `✳️mesh` inference files) — mesh's
  session had moved on to a different in-progress error.
- **Zero of these nine runs' errors named `✳️drawing`, `✳️brep`, or `✳️any`** — confirmed each time by
  `grep -c drawing` on the raw stderr (0 every time) and by inspecting every `-->` location.

Per protocol (retry at intervals, prove zero errors in own paths, do not fix another session's file):
polled `📦️glue.rs`'s mtime until it went quiet (~3 minutes stable) rather than editing anything, then
re-ran.

### Final clean run (10th attempt)
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=".…/🎯️target" cargo check -p semio-s-plugin-stdio --all-targets
```
```
warning: `semio-s-plugin-stdio` (lib) generated 694 warnings (run `cargo fix --lib -p semio-s-plugin-stdio` to apply 317 suggestions)
warning: `semio-s-plugin-stdio` (lib test) generated 786 warnings (608 duplicates) (run `cargo fix --lib -p semio-s-plugin-stdio --tests` to apply 171 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 1m 46s
```
Zero errors.

### `cargo test -p semio-s-plugin-stdio --lib` — run twice, identical result both times
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=".…/🎯️target" cargo test -p semio-s-plugin-stdio --lib
```
```
failures:
    artifacts::binary::standards::v_raw::subsets::any::schema::inferences::extent::component::tests::inference_default_law
    artifacts::dwg::standards::v_ac1018::engine::tests::conformance_laws::fixture_honesty_law
    artifacts::dxf::standards::v_r12::subsets::any::schema::inferences::bounds::component::tests::bounds_matches_hand_built_entity_extent
    artifacts::ifc::standards::v2x3::engine::tests::conformance_laws::fixture_honesty_law
    artifacts::zip::standards::v2_0::subsets::any::schema::inferences::entries::component::tests::inference_default_law

test result: FAILED. 2406 passed; 5 failed; 5 ignored; 0 measured; 0 filtered out; finished in 22–36s
```
Identical 5-item failure set both runs (deterministic). All 5 are in `binary`/`dwg`/`dxf`/`ifc`/`zip` —
none in `✳️drawing`, `✳️brep`, or `✳️any`; `dwg`/`ifc` `fixture_honesty_law` match brep's own baseline
exactly (pre-existing, IIF/APA territory); the other 3 (`binary::extent`, `dxf::bounds`, `zip::entries`)
are other IIF-authored inference facets, not this wave's.

### `✳️drawing`'s own inference tests — all 10 pass, re-run in isolation for a second confirmation
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=".…/🎯️target" cargo test -p semio-s-plugin-stdio --lib drawing::schema::inferences
```
```
running 10 tests
test artifacts::semio::standards::v1::subsets::drawing::schema::inferences::component::tests::inference_default_law ... ok
test artifacts::semio::standards::v1::subsets::drawing::schema::inferences::flattened_scene::component::tests::style_reference_resolves_to_the_real_value ... ok
test artifacts::semio::standards::v1::subsets::drawing::schema::inferences::flattened_scene::component::tests::world_transform_composes_down_through_nested_groups ... ok
test artifacts::semio::standards::v1::subsets::drawing::schema::inferences::flattened_scene::component::tests::quaternion_rotation_of_identity_is_a_no_op ... ok
test artifacts::semio::standards::v1::subsets::drawing::schema::inferences::component::tests::inference_determinism_law ... ok
test artifacts::semio::standards::v1::subsets::drawing::schema::inferences::flattened_scene::component::tests::disabled_cache_matches_pure_recompute ... ok
test artifacts::semio::standards::v1::subsets::drawing::schema::inferences::flattened_scene::component::tests::changing_the_root_transform_recomputes_the_whole_subtree ... ok
test artifacts::semio::standards::v1::subsets::drawing::schema::inferences::flattened_scene::component::tests::an_unrelated_sibling_edit_leaves_the_other_siblings_chain_warm ... ok
test artifacts::semio::standards::v1::subsets::drawing::schema::inferences::component::tests::inference_matches_direct_infer_field_call ... ok
test artifacts::semio::standards::v1::subsets::drawing::schema::inferences::flattened_scene::component::tests::changing_a_leaf_own_style_does_not_recompute_ancestors_or_siblings ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 2406 filtered out; finished in 0.00s
```

## Gates — checked mechanically (adapted for a single-field inference facet, not a 13-verb dispatch)
- Named-inference-dir ↔ struct-field count: **1 ↔ 1** (`🎛flattened-scene/` ↔ `flattened_scene`),
  confirmed by reading `SemioDrawingInference`'s one `#[state(inferred)]` field.
- Unique emoji: `🎛` — trivially unique (only one named inference this facet authors).
- Real leaf: `DrawFlattenedScene` has genuine `reads`/`plan`/`dep_input`/`compute` — `plan` walks the
  real scene graph (`walk()`, recursive over `DrawNode::Group.children`), `dep_input` reads only what
  `compute` reads (verified above), never apply-then-capture.
- Non-stub `🟦️component.ts` beside every `🦀️component.rs`: family root (14 lines, real
  `SemioDrawingInference`/`FlattenedNode` interfaces) + `🎛flattened-scene/` (10 lines, real
  `FlattenedNode` interface) — both present, both real.

## sharedFileRequests

1. **Inference-catalog registration remains unwired** — `SemioDrawingInference`'s descriptor
   (`semio_drawing_artifact_inference_descriptor()`) is authored and ready but no
   `register_artifact_inference(...)` call exists for it. Every other `s.stdio.semio.*` subset
   registers its own inference descriptor from its own `🚪️io/🦀️component.rs` (confirmed by grepping
   all 14: `animation`/`any`/`audio`/`cad`/`document`/`flow`/`graph`/`image`/`kit`/`model`/`object`/
   `presentation`/`table`/`value`/`text` each call `register_artifact_inference` next to their own
   `register_artifact_schema_descriptor`); `✳️drawing/🚪️io/🦀️component.rs:145` registers the SCHEMA
   descriptor but has no analogous inference call. **`✳️brep` has the identical gap** — verified by
   grep, brep's own `🚪️io/🦀️component.rs` also has zero `register_artifact_inference` calls — so this
   is not a drawing-specific miss, it's the same open item brep's report already flagged, just at
   the correct file (drawing's own `🚪️io/`, not a shared 14-subset aggregator — brep's report named
   `🏅️standards/🔖️v1/⚙️engine/🦀️component.rs` as shared, but no such file exists at that path; the
   real registration site is per-subset). Not fixed here: `🚪️io/**` is outside this wave's boundary
   (`🧬️schema/💡️inferences/**` + the glue mount only). One-line fix, mirroring e.g.
   `✳️text/🚪️io/🦀️component.rs`'s own `register_artifact_inference` call site, for whichever session
   next holds `✳️drawing/🚪️io/🦀️component.rs`.

## Concurrent-churn observations
See "Concurrent churn encountered" above — 9 of 10 `cargo check` runs over ~10 minutes were red from
`✳️mesh` (another session's subset, explicitly out of DKM's ownership per the hot-file table), with
the specific error changing 4 times as that session progressed through deleting `set-snapshot`,
adding new triads, and authoring its own inference facet. Zero of these errors ever named `✳️drawing`,
`✳️brep`, or `✳️any`, confirmed by grep on every run's raw output. No action taken against `✳️mesh` —
waited for `📦️glue.rs`'s mtime to go quiet, then re-ran; did not edit anything outside this wave's
boundary.

## Honest pass/fail
**Pass.** The facet was substantially authored before this session (verified honest: real `DepHash`
chain, honest field omission reasoning already present in the module doc comment, no fake
whole-snapshot hashing). This session's contribution: strengthened the incrementality-law suite to
explicitly cover and pass all three shapes the ticket names (leaf-own-change, ancestor cascade,
sibling warmth) with both miss- and hit-delta assertions plus a byte-identical-value assertion for
the sibling case; verified the mount block against a real directory listing rather than trusting it
by inspection; and produced real, twice-repeated, pasted `cargo check --all-targets` /
`cargo test --lib` output showing zero errors or failures attributable to `✳️drawing`, after
transparently working through ~10 minutes of unrelated `✳️mesh` concurrent-churn rather than fixing
or attributing it to this wave. One `sharedFileRequests` item remains open (inference-catalog
registration in `✳️drawing/🚪️io/🦀️component.rs`, outside this wave's boundary, matching brep's
identical unresolved item).
