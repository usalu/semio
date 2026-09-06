# W4 — remodel editor/viewer trait shape, dispatch classification, dialect, examples

## 1. Canonical dialect decision (relay this to W2b / W3)

**Canonical: `s.remodel.remodeling`** (two segments after `s.`, `s.<plugin-id>.<artifact-name>`).

Evidence — every peer plugin whose plugin id ≠ artifact name spells it this way, with `*_DIALECT`,
`#[artifact_schema(id=)]` and the `🚪️io` `DIALECT` agreeing per peer: `s.trinity.jack`,
`s.trinity.rewriting`, `s.puzzle.puzzle3d`, `s.block.block2d`, `s.procedural.generation2d`. Remodel's
own tree already uses `s.remodel.remodeling` in **five** places — `🗿️artifacts/📸️remodeling/🦀️.rs:79`
(`ArtifactIdentity::parse`), `:61` (`composer.native`), `🚪️io/🦀️.rs:286,428`, `🧬️schema/🦀️.rs:328` — and
disagrees only in the dialect const and the schema-descriptor namespace.

So `REMODELING_DIALECT.artifact_kind = "s.remodel.remodeling.remodeling"` (three segments) is a
typo/duplication, **and** the proofs' `"s.remodeling.remodeling@1/*#editor"` is the other wrong
spelling. Both become `s.remodel.remodeling`. `ArtifactKindSpec.id` stays `"3d.remodeling"` and
`component_kind` stays `"remodeling"` — that is the unrelated OS-level kind namespace (peers keep
theirs distinct too), and the plugin root's `ActivationEvent::OnArtifactKind` reads `artifact_kind().id`,
so it needs no change.

### Rust sites W4 changed
- `🗿️artifacts/📸️remodeling/🦀️.rs:180` `REMODELING_DIALECT.artifact_kind` → `"s.remodel.remodeling"` (+ its doc comment).
- `…/✏️editor/🦀️.rs` `bounded_first_step_tool_proofs!` `controller:` → `"s.remodel.remodeling@1/*#editor"`,
  and `owner_file:` corrected (it pointed at a non-existent `🔌️plugins/📸️remodeling/` path).
- `…/✏️editor/🧪️fixtures/🚧️retained-command-limits/🔣️.json` + `🧬️.schema.json` `controller` const.

### NON-Rust sites that MUST follow (not W4's files)
| file | field | current | target |
|---|---|---|---|
| `…/✳️any/✏️editor/🟦️.ts:6` | `REMODELING_EDITOR_DIALECT.artifactKind` | `s.remodeling.remodeling` | `s.remodel.remodeling` | (W3) |
| `…/✳️any/👁️viewer/🟦️.ts:5` | `REMODELING_VIEWER_DIALECT.artifactKind` | `s.remodeling.remodeling` | `s.remodel.remodeling` | (W3) |
| `…/🪆️subsets/🔣️.json:2,10` | `artifact`, `subsetPolicyRationale` | `s.remodeling.remodeling` | `s.remodel.remodeling` | (W2b) |
| `…/✳️any/🔮️oracle/🔣️.json:28,495,1194` | `format`, `artifact` (+ prose) | `s.remodeling.remodeling` | `s.remodel.remodeling` | (W2b) |
| `…/✳️any/🧪️tests/📸️mutate-remodeling-1/🥒️.feature:6`, `🐍️.py:1,3` | prose | — | — | (W2b, cosmetic) |

### Schema-descriptor namespace — deliberately NOT touched by W4
`#[artifact_schema(id = "s.remodeling.remodeling")]` (`🧬️schema/🦀️.rs:68`, `📸️snapshot/🦀️.rs:32`,
`✏️editor/🎚️config/🧬️schema/🦀️.rs:9,22,39,50,73…`), the descriptor literal `🧬️schema/🦀️.rs:228` and the two
`definition()` rows `🗿️artifacts/📸️remodeling/🦀️.rs:59,60` that quote it are internally consistent with
each other today, so renaming half would break more than it fixes. Peers keep schema id == dialect, so
a follow-up owner should rename this whole set to `s.remodel.remodeling` /
`s.remodel.remodeling.inference` in one pass with W2b's oracle JSON. The dialect/controller namespace
alone is what unbreaks `validate_tool_job_rows`.


---

## 2. Trait shape (task 1)

`ArtifactEditor`/`ArtifactViewer` are the AUTHORING traits and are fully sync; `ArtifactApp`/
`VcsArtifactApp` (RUNTIME) are NOT — `new_app`, `new_app_with_registry`, `dispatch_typed`
(`🔌️plugin/🦀️.rs:23323`, returns `impl Future`), `render` (`:11960`), `document_text`/`document_pack`/
`load_document_pack`/`ingest_operations_text`/`handle_action` are all async. That split is what the
two codemods encode.

- `🐍️w4-deasync.py` (ticket folder): **116 `async fn` → `fn` over 62 files**, test-attributed fns
  skipped (83). Covers `✏️editor/🦀️.rs` (19), `🎮️commands/**` (43 `handle` + helpers), `🎭️modes/**`,
  `📌️panels/**`, `📚️examples/**`, `🗣️terminology`, `👁️viewer/**`.
- `🐍️w4-await-testkit.py` (ticket folder): **95 `.await` re-added** at `testkit::{app,
  app_with_registry, dispatch, render}` call sites across 22 test modules (paren-matched, so nested
  payload literals are handled). `run-reconstruction`'s test module had 34 stale awaits stripped and 5
  genuine ones kept.
- `✏️editor/🦀️.rs:1509-1531` testkit restored to async (`new_app(..).await`, `dispatch_typed(..).await`,
  `render(..).await`); `render` now `format!("{:?}", …)` because `ui_runtime::ComponentTree`
  (`🎭️present.rs:85`) derives only `Debug` — neither `Serialize` nor `ToValue`.
- `command_from_action` (`✏️editor/🦀️.rs:274` bridge, `:1147` trait override): `Option<&serde_json::Value>`
  → `Option<&dsl::DslValue>`; the four accessors (`text`/`number`/`flag`/`vec3`) read `DslValue`
  directly (`as_str`/`as_u64`/`as_f64`/`as_bool`/`as_array`) rather than going through puzzle's
  `from_dsl_value` bridge, so serde_json leaves the dispatch path entirely.
- **UI contract flip** (not in the brief, but the crate cannot compile without it): the fleet moved
  from `ui_wgpu::UiNode` to `ui_contract::BuiltNode` / `ui_runtime::ComponentTree`; remodel was
  half-migrated. Ported to `🔱️trinity`/`🌊️flow`'s shape — editor `render` (`:1235`) and viewer `render`
  (`👁️viewer/🦀️.rs:64`) now `.map(built_to_component_tree)` with a `built_text_to_component_tree`
  fallback; the 7 panels moved from `ui_stack_vertical(vec![ui_text(..)])` to
  `PanelTreeBuilder::new(ns)?.section(..)?.build()` + `tree_item`/`tree_item_desc` (the media panel's
  `ui_import_drop_zone` becomes `PanelTreeBuilder::drop_action(..)`); the 4 window scenes moved from
  `build_{world_3d,canvas_2d,table}_scene` (→ `UiNode`) to
  `scene_surface(SURFACE_ID, ContractSurfaceKind::…, &scene)` (→ `UiAssemblyResult<BuiltNode>`);
  `Cargo.toml` gained `semio-framework-ui-contract` (the dep `🧱️block`/`🌊️flow` already carry).
- Command handlers: `app_commands!`'s generated `dispatch` (`🔌️plugin/🦀️.rs:10751`) calls
  `$module::handle(payload, doc, cfg)` with **no** `.await`, so all 42 `🎮️commands/*/🦀️.rs` `handle`
  fns are now sync — confirming W1 §5's reading over the brief's premise.

## 3. Classification + factory (task 2)

**The command count is 42, not 39** (`app_commands!` has 41 rows; `setActiveExample` makes 42) —
`📓️explore-editor-dispatch.md` §1 undercounted.

`validate_tool_job_rows` (`🔌️plugin/🦀️.rs:12245-12300`) demands **set equality**:
`expected = TOOL_JOB_IDS ∩ migrated` must equal the proof set, else `interactive-job.catalog-incomplete`.
So the retained id list, the `Migrated` list and the proof list are all 42 and are asserted equal by
the new law test. Two lane factories are mandatory (`:19563-19566`): an `Artifact` route is registered
with an unsupported publication contract without `build_artifact_store_one_item_preparation_factory`,
and likewise `Config`.

`✏️editor/🦀️.rs` `//#region 🧵️RetainedCommands` (rewritten) + new `//#region 📬️StorePreparation`:

| symbol | note |
|---|---|
| `REMODELING_RETAINED_TOOL_IDS` | all 42 ids, `app_commands!` order |
| `REMODELING_RETAINED_PAYLOAD_SCHEMA` | `"remodeling.scene.tool-command.v1"` (unchanged) |
| `REMODELING_RETAINED_RAW_BYTES` / `_WORK_ITEMS` | `65_536` / `4_096` (was `65_536`/`1`) |
| `REMODELING_PUBLICATION_CONTRACTS` | 42 rows via `artifact_route`/`config_route`/`host_route` const fns |
| `remodeling_retained_contract()` | `bounded_first_step(65_536, 4_096, 1, 262_144, 7_500)` — equal to the proof's `contract:`, which `validate_tool_job_rows` compares |
| `remodeling_retained_extent(..)` | rejects non-retained ids; bounds `streams + frames + assets + gcps + observations + cameras + 1` against `_WORK_ITEMS` |
| `RemodelingRetainedCommandJobFactory` | renamed from `RemodelingCommandJobFactory`; `keys()` now covers 42 tools |
| `RemodelingStorePreparationFactory` / `RemodelingStorePreparation` | document lane, mirroring `🖨️raster` row for row against remodeling's own collections |
| `RemodelingConfigStorePreparationFactory` / `RemodelingConfigStorePreparation` | config lane (six session verbs) |
| `remodeling_retained_edit(..)` | the single `protocol::Edit` envelope both lanes stage — written once, generic over the mutation type, instead of raster's verbatim duplication |

`bounded_first_step_tool_proofs!` switched to the `contract:` + `tools: [..]` array form,
`controller: "s.remodel.remodeling@1/*#editor"`, `factory_type: RemodelingRetainedCommandJobFactory`,
`owner_file:` corrected (it named a non-existent `🔌️plugins/📸️remodeling/` path).

Manifest (`create_remodeling_app`): added `remodeling_internal_action("setLocale", …, View)` (it was a
command row with no action declaration at all), the `setActiveExample` action + its `exampleId` select
(options generated from the registry), and **41** `.action_interactive_job(id, Migrated)` rows.
`setActiveUtility` is deliberately absent — `.utility(..)` injects it INSIDE `build_definition`, after
this chain runs, already `Migrated` via `ActionDefinition::resumable_framework_catalog`
(`🛂️manifest/🦀️.rs:1247`); declaring it here would silently match nothing.

The reconstruction continuation is intact: `runReconstruction`/`advanceReconstruction`/
`cancelReconstruction`/`retryStage`/`runStage` are all retained `Artifact` routes, and
`Effect::DispatchAction { action: ADVANCE_RECONSTRUCTION_ACTION_ID }` survives the retained path
(`Emit.effects` is carried through `ArtifactToolCompletionValue::Emit`).

### Classification table (42 rows — all `Migrated`)

| id | lane | id | lane | id | lane |
|---|---|---|---|---|---|
| runReconstruction | Artifact | setStreamSync | Artifact | setGeoParams | Artifact |
| retryStage | Artifact | editCalibration | Artifact | resetPlaceholderMesh | Artifact |
| runStage | Artifact | calibrateCameras | Artifact | clearSparse | Artifact |
| advanceReconstruction | Artifact | addGcp | Artifact | clearDense | Artifact |
| cancelReconstruction | Artifact | removeGcp | Artifact | clearMeshResult | Artifact |
| importFramePayload | Artifact | placeGcpObservation | Artifact | clearTracks | Artifact |
| importVideoFramePayload | Artifact | setIngestParams | Artifact | clearGeoProducts | Artifact |
| importVideoDone | Artifact | setFeatureParams | Artifact | clearResult | Artifact |
| importVideoBytesPayload | Artifact | setMatchParams | Artifact | setActiveExample | Artifact |
| addStream | Artifact | setSfmParams | Artifact | setCamera | Config |
| removeStream | Artifact | setDenseParams | Artifact | setLayerVisibility | Config |
| setFrameCursor | Config | setMeshParams | Artifact | setReportTable | Config |
| setActiveUtility | Config | setMotionParams | Artifact | setLocale | Config |
| importFrames | HostOnly | importVideo | HostOnly | exportQcReport | HostOnly |

Evidence: `grep -oh "Emit::…" 🎮️commands/*/🦀️.rs` — 33 handlers build `Emit::mutations`/`Emit::amend`/
`Emit { artifact_mutations, .. }`, 6 build `Emit::config`, 3 build `Emit::effect` only
(`RequestFileOpen`, `RequestMediaFrames`, `DownloadMediaExport`). No handler emits two store lanes, a
draft, a presence or a transient mutation, so every route declares exactly one lane (asserted).

## 4. Examples (task 4)

- New `✏️editor/📚️examples/🦀️.rs` — `RemodelingExample`, `REMODELING_EXAMPLES` (**the single
  append-only array**), `REMODELING_EXAMPLE_BOOT_ID = "demo"`, `example_text`, `example_label`,
  `example_sources()`, `boot_snapshot()`. Mounted as `editor::remodeling::examples`.
- New `✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs` — parses the named example's committed text and
  emits `replace_document_operations(current, next)`: delete every gcp/stream/camera, create the
  example's cameras/streams/gcps, overwrite all 8 param groups and the job. `results` and `assets` are
  deliberately not transplanted (engine-derived / media bound via `importFramePayload`). Unknown id or
  unparsable text = no-op, not a fault.
- `initial_snapshot()` boots on `boot_snapshot()` (parse the `demo` example, else
  `default_remodeling_scene()`) — block2d's `default_block2d_snapshot()` shape. The viewer keeps
  `default_remodeling_scene()` unchanged.
- **W7a**: `📚️examples/🛰️synthetic-orbit` is on disk and W7a has mounted it as
  `artifacts::remodeling::examples::synthetic_orbit` in the wiring file, but its `label()`/`source()`
  are still `pub async fn` (the central check reports
  `E0277 LocalizedLabel: From<impl Future<Output = LocalizedLabel>>` at `🛰️synthetic-orbit/🦀️.rs:56`).
  It is deliberately NOT in `REMODELING_EXAMPLES` — registering a leaf mid-flight in another lane would
  couple my compile to theirs. **To register it, append exactly one row** (nothing else in the crate
  changes — the manifest's `exampleId` options, the command bridge and the boot document all read the
  array):
  ```rust
  RemodelingExample {
      id: crate::artifacts::remodeling::examples::synthetic_orbit::ID,
      text: crate::artifacts::remodeling::examples::synthetic_orbit::PRIMARY_TEXT,
      icon: crate::artifacts::remodeling::examples::synthetic_orbit::ICON,
      label_en: "Synthetic Orbit",
      label_de: "Synthetischer Orbit",
  },
  ```
  (and de-async that leaf's own `label()`/`source()`).
- **ShellHost's picker is still blocked, and NOT by anything in this packet.** `manifest.apps[].examples`
  is fed only from `App { definition, examples }` (`register_app_factory`, `🔌️plugin/🦀️.rs:5691`) or
  `SubsetDeclaration.examples` (`:28218`). `PluginBuilder::.editor::<E>(def: AppDefinition)` takes a
  bare `AppDefinition` and `ArtifactDeclarationBuilder` has no `examples(..)` method at all — the same
  SDK gap raster/flow/trinity/process/norm all document in place. Closing it means moving remodel's
  plugin root onto the `SubsetDeclaration` tree (trinity's shape), which also carries W6's
  `IoDeclaration` — so it should be ONE combined follow-up, not two. `example_sources()` is already the
  exact `&[ExampleSource]` that migration needs.

## 5. Windows / panels (task 5)

Against the boot document (the `demo` example = the empty default scene):

| body | renders | non-empty? |
|---|---|---|
| `remodeling.play.main` (World3d) | placeholder mesh + instance; point layers empty | ✅ |
| `remodeling.play.report` (Table) | `frames` column set, 0 rows | ✅ (columns) |
| `remodeling.play.frames` (Canvas2d) | `layers_json == "[]"` — the frame cursor is unset and there are no frames | ⚠️ inherently empty until media is imported; a Canvas2d scene surface carries no text slot, so there is no honest placeholder to add. Fixed by W7a's `synthetic-orbit` example once it is registered and its frames imported. |
| `remodeling.play.pipeline` | job / status / utility rows | ✅ |
| `remodeling.play.media` | drop row + stream/asset summary | ✅ |
| `remodeling.play.results` | mesh/sparse/dense/trajectory/geo rows | ✅ |
| `remodeling.play.parameters` | 8 param-group rows | ✅ |
| `remodeling.play.calibration` | camera summary + gcp section | ✅ |
| `remodeling.play.tracks` | empty-state + documented-gap rows | ✅ |
| `remodeling.play.qc` | "no quality report yet" row | ✅ |

## 6. Tests (task 6)

- `✏️editor/🧪️fixtures/🚧️retained-command-limits/🔣️.json` + `🧬️.schema.json` regenerated: `controller`
  `s.remodel.remodeling@1/*#editor`, `factory` `RemodelingRetainedCommandJobFactory`, 42 routes (all
  `bounded`), 42 publication contracts with real lanes, `expected {routes:42, bounded:42, resumable:0}`.
  The schema now constrains the lane enum per row instead of hard-coding two `hostOnly` rows.
- `retained_command_catalog_matches_the_serde_json_oracle` / `retained_publication_oracle_rejects_hostile_tool_and_lane_fixtures`
  updated (host-only set is now the 3 shell routes); both hostile mutations are asserted to actually
  change the fixture text, so a stale hostile string cannot pass vacuously.
- New `retained_route_dispositions_are_exact_and_exhaustive`: retained ids == command-row ids, proof
  count == 42, publication-contract set == retained set with exactly one lane each, both store
  preparation factories present, and every retained id's built action carries `Migrated`.
  (`ArtifactBoundedFirstStepProof.tool_id` is private, so only the count is asserted — same as block2d.)
- `every_command()` gained the `SetActiveExample` row; the action-bridge test gained a
  `setActiveExample` case and its two existing cases now build `dsl::DslValue::from(&json!(..))`.
- New in-file tests: `example_ids_are_unique_and_resolvable`, `every_source_carries_its_committed_text`,
  `the_boot_document_parses_the_demo_example`, `every_registered_example_id_resolves_to_committed_text`,
  `an_unknown_example_id_is_a_no_op`.
- Two pre-existing manifest tests read `create_remodeling_app().definition` / `app.definition.*`, but
  the fn returns `AppDefinition` — fixed.

## 7. Compile / test status

**`cargo check` was NOT run by W4** — the host never re-entered the brief's gate. The central warming
check (pid 47704, `scratchpad/logs/central-check-1.txt`) finished at 05:24 with
`error: could not compile semio-s-plugin-remodel (lib) due to 128 previous errors; 39 warnings`, and
the host has been climbing ever since (05:31: load 59, swap **29.9 GB / 30.7 GB used** vs the brief's
`load < 40, swap < 16000M`). Starting a second full check into that would thrash the box and starve
the peers' builds. A background waiter on `swap < 16000M && load < 40` is armed; run
`CARGO_TARGET_DIR=<scratchpad>/target-remodel-w4 RUSTC_WRAPPER="" cargo check -p semio-s-plugin-remodel --lib -j 2 --message-format short`
after an rsync from `target-remodel` when it clears.

What WAS verified:
- **Syntax**: `rustfmt --edition 2021 --emit stdout` parses all 18 files W4 changed, clean (0 errors).
- **Semantics**: the central check's 128 errors were classified by file and every W4-owned one fixed.
  It compiled a snapshot of the tree that already contained most of W4's rewrite, so this is real
  compiler feedback on this packet, not a guess — but it predates `✏️editor/📚️examples/🦀️.rs`, the
  `🎬️set-active-example` command and the last round of fixes, which therefore remain **unverified by a
  compiler**.

### W4-owned errors found and fixed
| error | file:line | fix |
|---|---|---|
| E0433 `semio_framework_job` unlinked (×2) | `✏️editor/🦀️.rs:650,656` | added `semio-framework-job` to the crate's `Cargo.toml` (the dep `🖨️raster`/`🌍️gis` carry for the same signatures) |
| E0277 `Label: From<Label>` / `From<LabelText>` (×34) | all 7 `📌️panels/*` | two `Label` types coexist; `tree_item`/`section` take the semantic-contract one. Rows now pass `String`/`&str` directly, section labels go through a new `ui_label()` helper (`✏️editor/🦀️.rs:88`, `🧱️block`'s spelling) |
| E0308 `expected ActionDescriptor, found Result<(ActionId, …)>` + `expected UiValue, found Value` | `🧊️model/☑️options/👁️layers/🦀️.rs:20` | new `remodeling_window_action(action, Option<DslValue>) -> ActionDescriptor` (`✏️editor/🦀️.rs:82`); `WindowMeasure` is retained-vocabulary and never takes the fallible pair |
| E0277 `MeshData: Serialize` (×2) | `🧊️model/🪟️windows/🧊️model:58`, `👁️viewer/…/🧊️model:69` | `MeshData` derives `Serialize` only under `#[cfg_attr(test, …)]` (`🏗️mesh-engine/🦀️.rs:23`); both windows now build the wire object field by field via a local `mesh_data_json` |
| E0432 unresolved `semio_framework_ui_contract` (×4) | the 4 window files | the dep landed in `Cargo.toml` after that snapshot was taken |
| E0433 `semio_framework_async_macros` in the demo example's tests (×3) | wiring `📚️Examples` | already fixed by a peer — the mount now carries `#[cfg(test)]` |

### Errors in OTHER lanes (class + file, not touched — the crate cannot go green until they land)
| class | count | files | owner |
|---|---|---|---|
| E0277 `ArtifactChild<SemioImageSnapshot>: Serialize/Deserialize` | 7 | `🧬️schema/🦀️.rs:67,79`, `📸️snapshot/🦀️.rs:28,46`, `🔺️diff/🦀️.rs:12,26` | schema lane (post-W1) |
| E0283 type annotations needed in generated diffs | 16 | `🧬️mutations/{🧷create-asset,🧱replace-mesh-result,🏁commit-reconstruction}/🔺️diff/🦀️.rs` | schema lane |
| E0080 `Mutations descriptor variant must match its wrapped leaf` | 1 | `🧬️mutations/🦀️.rs:17` | schema lane (this one aborts const-eval and cascades) |
| E0599 `String::__dsl_spec` | 1 | `🗿️artifacts/📸️remodeling/🦀️.rs:224` | schema/dsl lane |
| E0053 `shape`/`to_value`/`from_value` "found future" | 9 | `🗿️artifacts/📸️remodeling/🦀️.rs:943-1522` (`PackedF32`/`PackedU8`/`RemodelingMesh` field impls) | schema lane — W1's codemod scope stopped at `🧬️schema/**` and never reached the artifact-root file's `FieldSpec` impls |
| E0046 missing `DESCRIPTORS`/`descriptor` | 2 | `✏️editor/🎚️config/🦀️.rs:232`, `✏️editor/👥️presence/🦀️.rs:98` | W1 (its two de-asynced files) |
| E0053 `reads`/`compose` "found future", E0433 `subsets::any` | 3 | `🚪️io/🦀️.rs:30,416,420` | W6 |
| E0277 `LocalizedLabel: From<impl Future<…>>` | 1 | `📚️examples/🛰️synthetic-orbit/🦀️.rs:56` | W7a |
| E0063 missing struct fields (`strl_extra`, `rc_frame_width`, `strh_extra`, `hdrl_extra`) | 3 | `✏️editor/⚙️engine/🎥️video/🦀️.rs:2952-2975` | engine lane — an AVI struct gained fields elsewhere |
| E0425/E0433 `BTreeSet` not in scope | 12 | `✏️editor/⚙️engine/🥽️mesh/🦀️.rs` (9 sites) | engine lane — a lost `use std::collections::BTreeSet;` |
| E0432 `build_engine_params` missing from `engine` | 1 | `🎮️commands/🏗️run-reconstruction/🦀️.rs:11` | engine lane |

`cargo test -p semio-s-plugin-remodel --lib editor` was likewise not run; it cannot pass before the
schema lane's E0080/E0277 wave lands.

## 8. Risks

1. **Unverified by a compiler**: the example registry, `🎬️set-active-example`, the 42-row proof block,
   the two store preparations and the last fix round. Highest-risk single item is the
   `bounded_first_step_tool_proofs!` `contract:`/`execution_contract()` equality — `validate_tool_job_rows`
   compares them field-for-field and a mismatch is a runtime `interactive-job.catalog-authority`, not a
   compile error. Both are literally `bounded_first_step(65_536, 4_096, 1, 262_144, 7_500)`.
2. **`setActiveUtility` is a live set-equality tripwire.** It is a generated command id AND a
   framework-injected `Migrated` action, so it MUST be in the proof list and MUST NOT be in the manifest's
   explicit classification list. Getting either wrong is `interactive-job.catalog-incomplete` at app
   construction. Encoded in the law test's action lookup, which reads the BUILT definition (post-injection).
3. **Retained-path effects.** `runReconstruction`'s tick continuation rides on `Emit.effects`
   (`Effect::DispatchAction`). The retained completion path checks store lanes, not effects
   (`🔌️plugin/🦀️.rs:22941-22948`), so it should pass through — but this is argued from the framework's
   admission code, not observed in a browser.
4. **`ComponentTree` has no `Serialize`/`ToValue`**, so the testkit's body assertions now match against
   `Debug` output. If the framework later gives it a JSON projection, those substring assertions should
   move onto it.
5. **The UI-contract flip is fleet-wide, and `🖨️raster` is still on the old side** (its editor `render`
   returns `UiAssemblyResult<ComponentTree>` while its window/panel arms return `UiNode`, and its testkit
   is fully sync against async runtime APIs). Remodel is now on the `🔱️trinity`/`🌊️flow` side. Nobody
   should "fix" remodel back toward raster.
6. **Schema-id namespace still split** (§1's last block): `s.remodeling.remodeling` in
   `#[artifact_schema(id=)]` + the descriptor + two `definition()` rows, vs `s.remodel.remodeling`
   everywhere else. Deliberately deferred; it needs one pass together with W2b's oracle JSON.
7. **ShellHost's example picker stays empty** until the plugin root moves onto the `SubsetDeclaration`
   tree (§4) — an SDK gap, not a remodel bug, and it should be combined with W6's `IoDeclaration` move.
