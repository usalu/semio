# 🛍️ Aggregator catalogue — verification and fix (2026-09-16)

Scope: `📓️app-aggregator.md` §8 items (1) catalogue empty for `concrete-forest`, (2) tutorial
`tracks.document` empty, (3) memory note "catalog rows use `representations[].url`, not `meshUrl`".
Crate `✏️s/🔌️plugins/🧩️puzzle`, artifact tree `🗿️artifacts/🧊️3d`.

## 1. Findings (verified against current code, not the 08-28 audit)

### (1) "Catalogue empty for `concrete-forest`" — **STALE, the audit was wrong about the fixture**

Two things moved since 08-28:

* The fixture PATH moved. The audit cites `📚️examples/🌲️concrete-forest/🖼️assets/🗣️forest.dsl.semio`;
  the file is now
  `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio`
  (`🧬️schema/📸️snapshot/📝️text/🦀️.rs:13` is the `include_str!`).
* The audit read `kind-catalogs=` (`🗣️.dsl.semio:4`) as "empty". It is not a value — it is the
  PRESENCE MARKER the `dsl` printer emits for the optional nested `Puzzle3dKindCatalogs` block
  (`🗿️artifacts/🧊️3d/🦀️.rs:650-671`), whose four `#[dsl(table)]` members are printed FLAT at the same
  indentation right after it. Those tables are the catalogue:
  * `objects` — `🗣️.dsl.semio:5-8`, 2 rows: `"Hexagonal Cut Concrete Forest Left"` (:6) and
    `"Hexagonal Cut Concrete Forest Right"` (:7), each with 11 vortex templates.
  * `vortices` — :9-16, 6 rows (`b-l`, `b-l-m`, `b-s`, `b-s-m`, `c-b`, `c-t`).
  * `cables` — :17-19, 1 row (`cable.link`). `attractions` — :20-22, 1 row.
  * `kind-compatibility` at :23-41 belongs to `Puzzle3dMeta`, not to the catalogue bundle.

So nothing had to be hand-authored: **the concrete-forest catalogue was already real**, and
`meta.objects`' `representations[].url` already point at routes the served mesh catalog declares —
`/mesh/🧊️hexagonal-cut-concrete-forest-left.glb` and `…-right.glb` are both present verbatim in
`🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json`. Nakagin
(`📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio`) has the same shape with ~180 rows;
it needed nothing either.

The catalogue is now pinned by test rather than by reading, because every pre-existing catalogue law
reads `nakagin_fixture()` while the app BOOTS on `concrete-forest` — that asymmetry is exactly why an
empty default catalogue could have gone unnoticed. See §3.

### (3) `meshUrl`-only readers — **one real offender, fixed**

Already correct before this pass:

* `Puzzle3dKindMeshIndex::of` — `✏️editor/🦀️.rs:769-771` reads `representations[].url` first, `meshUrl`
  as fallback (it carries the memory note verbatim as a `🐛️` comment).
* Catalogue panel row builder — `📌️panels/🛍️catalogue/🦀️.rs:81-86`, `meshUrl` then `representations`.
* The MIGRATED interactive job `Puzzle3dAddObjectKindWork` — `✏️editor/🦀️.rs:4487-4498` walks
  `kind.representations`. This is the live path (`action_interactive_job("addObjectKind",
  InteractiveJobClassification::Migrated)`, `✏️editor/🦀️.rs:8693`).

Still broken, and fixed here:

* `✏️editor/🎮️commands/🌱️add-object-kind/🦀️.rs:19` read `entry.get("meshUrl")` and nothing else. On the
  non-migrated route every catalogued kind of both shipped examples placed an object with
  `meshUrl: None` — nothing to render, and `mesh-unavailable` for its fill/brush candidates.

### (3b) NEW defect found in the same seam — catalog templates seat at `point`, not `position`

`Puzzle3dCatalogVortexTemplate` (`🗿️artifacts/🧊️3d/🦀️.rs:481-518`) names the seat `point`; a PLACED
`Puzzle3dVortex` names it `position`. Two readers asked for `position` on a TEMPLATE:

* `puzzle3d_vortices_from_kind_template` — `✏️editor/🦀️.rs:968` → every seeded vortex of every
  catalogued kind collapsed onto `[0,0,0]`, so a freshly added object had a degenerate rim no brush or
  attraction could use.
* the catalogue panel's nested template rows — `📌️panels/🛍️catalogue/🦀️.rs:65` → every template's
  description rendered `[0,0,0]`.

Both fixed (`point` first, `position` kept as the fallback for the already-placed-shaped rows `kit:in`
imports carry).

### (2) Tutorial `tracks.document` — **cannot be recorded today; documented instead of invented**

Still `document: []`, now at `♻️mit-bestand/🧺️demonstrator/🪧️brand.ts:274` (the file is `🪧️brand.ts`,
not `🟦️brand.ts` as the audit says; `ENTWERFEN_MIT_BESTAND_TUTORIAL` starts at :126).

The audit's prescription — "run the tutorial recorder against a live Aggregator session" — **does not
work, because the recorder has no artifact-edit capture at all**:

* `TutorialRecorder` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:1094`)
  exposes exactly `recordEvent` (:1116), `recordUiDiff` (:1120), `recordSnapshot` (:1126),
  `sampleCamera` (:1131) and `addChapter` (:1141). There is no `recordArtifactEdit`/`recordDocumentEdit`.
* `build()` (:1147) hardcodes `document: []` at **:1155**. A recorder pass would therefore emit
  precisely the empty track that is already checked in.
* The only thing the recorder does capture about an action is the ANNOTATIONAL `TutorialEvent` at
  :6353 (`{ kind: "action", action, args }`) — which `brand.ts:266-269` already hand-authors for all
  four edits (`addObjectKind` @110s, `setVortexShow` @141s, `acceptSuggestion` @165s, `setFillCount`
  @181s). Annotational events pulse chrome; they do not mutate the document
  (`🛂️manifest/🟦️.ts:756-757`: the artifact track is "the SOLE source of document mutation during
  playback").

Playback, by contrast, is fully implemented: `applyTutorialSliceToShell` (:6972-7000) and the seek path
(:7063-7085) replay a `kind: "edit"` entry through
`plugin.applyMutations(instanceId, encodeMutationEnvelopesPack(kind.forwards | kind.backwards))`.

Exactly what is missing, for the coordinator to schedule as its own work package:

1. A capture hook on `TutorialRecorder` taking the same `MutationEnvelope[]` playback re-applies
   (`forwards`/`backwards`, plus optional `description`/`coalesceKey`), and a shell call site that
   observes a settled mutation — the recorder currently only sees the dispatch, never the resulting
   ops.
2. `build()` must then emit that array instead of the literal `[]` at :1155.
3. A decision on the TRACK KEY first: `🛂️manifest/🟦️.ts:770-778` already declares the member as
   `artifact`, while `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:6087` and `🪧️brand.ts:274` still
   say `document`. That rename is in flight (26/09/15/SNAPSHOT-FIXTURE-ASSET-TERMINOLOGY), which is a
   second, independent reason not to hand-author the array in this pass.
4. `kind: "load"` stays unreplayable regardless — no TS text→pack encoder exists; the gap is documented
   in place at ShellHost :6985-6991.

Hand-authoring the ops was NOT done, for the reason the code's own docstring gives
(`🪧️brand.ts:115-124`): invented `forwards`/`backwards` JSON is indistinguishable from a recording and
silently wrong.

## 2. Edits

| file | line | change |
| --- | --- | --- |
| `✏️s/…/✏️editor/🎮️commands/🌱️add-object-kind/🦀️.rs` | 11-28 (new `catalog_entry_mesh_url`), 36 | catalog row mesh identity reads `representations[].url` first, `meshUrl` as fallback |
| `✏️s/…/✏️editor/🦀️.rs` | 958-974 (`puzzle3d_vortices_from_kind_template`) | seed vortices from the template's `point`, falling back to `position` |
| `✏️s/…/✏️editor/📌️panels/🛍️catalogue/🦀️.rs` | 65 | template row description reads `point` then `position` |
| `✏️s/…/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` | +2 laws at end | concrete-forest catalogue non-empty / resolvable; renders one draggable row per kind |
| `✏️s/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | +3 laws after `add_object_kind_materializes_the_declared_kind_default` | add-from-catalogue carries the mesh url; templates seat at their points; both doors carry the catalogue |

No production file outside `🗿️artifacts/🧊️3d` was touched. The INSPECTION panel and its selection
plumbing were not touched (concurrent `InteractionView` threading).

## 3. New laws

* `the_default_concrete_forest_catalogue_declares_kinds_with_resolvable_mesh_urls` — the DEFAULT
  document's `objects` catalog is non-empty; every row names a `/mesh/` `representations[].url`; that
  url is what `Puzzle3dKindMeshIndex` resolves for an instance of the kind AND what `collect_mesh_urls`
  publishes on the world mesh lane.
* `the_default_concrete_forest_catalogue_renders_a_draggable_row_for_every_kind` — one rendered row per
  catalogued kind, each `draggable`, each drag payload carrying its own `objectKind` and a non-empty
  `meshUrl` (the key `World3dHost`'s catalogue-drop preview parses — the
  `PUZZLE-3D-CATALOGUE-DROP-LIVE-PREVIEW` path), and no row over 2 row actions.
* `adding_a_catalogued_concrete_forest_kind_places_an_object_carrying_its_mesh_url` — end-to-end
  through the real harness: `addObjectKind` on a catalogued kind adds exactly one object whose
  `meshUrl` equals that kind's representation url. Every pre-existing `addObjectKind` law fires
  `"Object"`, a kind no shipped catalog declares, which is how the `meshUrl`-only read survived.
* `catalogued_kind_templates_seed_vortices_at_their_catalog_points` — seats are not all at the origin,
  and each equals its template's `point`.
* `the_initial_snapshot_and_set_active_example_both_carry_the_concrete_forest_catalogue` — the boot
  snapshot and a round trip through `setActiveExample("")` → `setActiveExample("concrete-forest")`
  (which replaces catalogs through `replace_kind_catalogs`) both carry the same kind ids.

## 4. Commands

```
DEVELOPER_DIR=/Library/Developer/CommandLineTools RUSTFLAGS=-Awarnings CARGO_TERM_QUIET=true \
CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_BUILD_JOBS=4 \
cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib
```

⚠️ `--features component-app-assembly` is MANDATORY: without it the whole `✏️editor` tree is not
compiled and the lib target runs only 286 schema tests — a bare `cargo test -p
semio-s-artifact-puzzle-3d --lib` reports green while never touching the catalogue at all.

Outputs under `🗑️generated/`: `aggregator-catalogue-baseline.txt` (feature-less, 286),
`aggregator-catalogue-after-1.txt` (full suite), `aggregator-catalogue-targeted.txt`,
`aggregator-catalogue-after-2.txt` (final full suite).

## 5. Counts

TBD

## 6. What remains for the coordinator's browser pass on :6029

* Confirm the Catalogue panel's `Baukomponenten` section opens with the two concrete-forest rows
  (`Hexagonal Cut Concrete Forest Left` / `Right`) and that each row expands to 11 vortex templates
  whose descriptions are now real coordinates, not `[0,0,0]`.
* Drag one row onto the viewport: a translucent ghost must follow the cursor (the
  `PUZZLE-3D-CATALOGUE-DROP-LIVE-PREVIEW` path is React `World3dHost` `dragover` + wgpu
  `catalogue_drop_preview`; the guest half — payload with `meshUrl` — is now pinned by test but the
  host half is browser-only), and the dropped object must render a real mesh at the drop origin.
* Press a catalogue row (no drag): `addObjectKind` must add ONE object, select it, and the outliner
  must show it with a non-`[0,0,0]` rim.
* The tutorial still narrates / moves the camera / opens panels without materializing document edits —
  unchanged, and blocked on §1(2) above.

## 7. Pre-existing failures not caused by this pass

Present in the same suite before and after; none touch the catalogue:

* `standards::…::wire_format_guard::engine_command_rows_keep_their_pre_migration_wire_bytes` — also
  fails on the feature-less baseline (`aggregator-catalogue-baseline.txt`), i.e. before any edit here.
* `…::two_instances_converge_disjoint_object_edits_via_backbone`,
  `…::window_options_are_local_to_the_window_instance_not_shared_across_split_panes`,
  `…::the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind`,
  `…::every_context_menu_row_dispatches_a_declared_action`,
  `…::outliner_hide_reaches_the_world_instance_lane_and_flips_the_row_control`,
  `…::a_nakagin_lane_that_did_not_change_does_not_republish_on_a_partial_refresh`,
  `…::an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes`,
  `…::one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns` — peer churn
  (`InteractionView` threading / vcs fail-closed merge).
* `brush_suggestions_run_step_…`, `fill_run_job_step_…`, `penetration_of_flush_thousand_triangle_parts_…`
  — wall-clock interaction ceilings, failing under the loaded machine (37 ms vs 2 ms etc.).
