# W11 — crate closure and lane integration (schema-id namespace · declaration tree · calibration · window payloads · production dispatch)

Scope owned: plugin root `🦀️.rs`, artifact root `🗿️artifacts/📸️remodeling/🦀️.rs`, `…/✳️any/✏️editor/**`
(except `🚪️io/**`), `👁️viewer/**`, `📚️examples/**`, `🧬️schema/**` schema-id strings, the crate
`Cargo.toml`, targeted edits in `📦️packages/🦀️rust/🦀️.rs`, plus the one `production_dispatch` field in
W2b's `🧪️tests/📸️mutate-remodeling-1/🦀️.rs`.

## 0. Build status — BLOCKED by live framework churn, not by remodel

| run | log | result |
|---|---|---|
| coordinator central check (pid 96422) | `🗑️generated/central-check-2.txt` | died at 22:35 after 3 h 57 m: `error: failed to write … full.rmeta: No space left on device` while compiling `semio-s-plugin-stdio`. Transient — peers' target dirs filled the volume; 42-48 GB free afterwards. It never reached `semio-s-plugin-remodel`. |
| W11 check 1 (pid 33641) | `🗑️generated/w11-check-1.txt` | `error: could not compile semio-framework-plugin-host (lib) due to 5 previous errors; 10 warnings emitted` |
| W11 check 2 | `🗑️generated/w11-check-2.txt` | same five errors |

The five errors are **not remodel's and not in any file this ticket owns** — a peer session is
mid-flight adding `cold_pair_ingress` to `semio_framework::kernel::TurnResult`
(`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs:1912`) and has updated `⚛️reactor/🔄️turn/🦀️.rs:818` but not
`🖥️host`:

```
🖥️host/🧵️shard/🦀️.rs:1910:30   E0063 missing field `cold_pair_ingress` in initializer of kernel::TurnResult
🖥️host/⏳️runtime/🦀️.rs:283:8   E0063 same
🖥️host/⏳️runtime/🦀️.rs:380:95  E0061 this method takes 5 arguments but 4 were supplied
🖥️host/🦀️.rs:2105:100          E0061 same
🖥️host/🦀️.rs:2136:12           E0063 same
```

`semio-framework-plugin-host` is upstream of `semio-s-plugin-remodel` in the dependency graph, so the
check aborts before the remodel crate is ever compiled. The same central check DID compile
`semio-framework-plugin-host` cleanly at 21:16, so this landed inside the last ~80 minutes. Nothing in
this packet can unblock it; fixing framework host files is outside this lane's grant and would collide
with the peer's in-progress edit. **Every Rust change below is therefore syntax-verified but NOT
compiler-verified** — `rustfmt --edition 2021 --emit stdout` parses all 12 changed/created files with
zero errors, and that is the whole of the mechanical proof available. `cargo check --tests` and
`cargo test -p semio-s-plugin-remodel --lib` could not be reached at all.

Non-Rust verification that DID run against the current tree:
- `🐍️mount-check.py`: 493 `#[path]` attributes, **0 dangling** (includes the two new mounts).
- `🐍️fixture-audit.py`: 406 fixture URIs, **0 missing**.
- `🐍️schema-validate.py`: 503 committed documents, **0 violations**; 134 payloads valid. The two example
  DSL documents still report the framework-side `child_id` snake-case print and the absent
  `durable-artifacts` block (W8's finding, unchanged and not this lane's).

## 1. Schema-id namespace unified on `s.remodel.…` (task 2)

Applied repo-wide inside the plugin; **no consumer outside `✏️s/🔌️plugins/📸️remodel/` referenced the old
ids** (verified by grep; the only other hits are prose in older ticket folders).

| substitution | sites |
|---|---|
| `s.remodeling.remodeling` → `s.remodel.remodeling` | 20 Rust (`#[artifact_schema(id=)]` ×8, descriptor literals, `InferenceFieldSpec` ids, `FIELD_ID`, the two `definition()` rows) |
| `…/schema/s/remodeling/remodeling/` → `…/schema/s/remodel/remodeling/` | 9 JSON-schema `$id` |
| `…/schema/app/remodeling/remodeling/` → `…/schema/app/remodel/remodeling/` | 2 JSON-schema `$id` (config, presence) |
| `package semio.app.remodeling.remodeling;` → `semio.app.remodel.remodeling;` | 2 proto |
| docstring `` `remodeling.remodeling.…` `` → `` `remodel.remodeling.…` `` | 7 TS twins |

29 files. Peer shape confirmed against `🧱️block` (`s.block.block2d`, `s.block.2d.config`,
`app/block/2d/config.json`, `semio.app.block.2d.config`). The ticket's own generator
`🐍️schema-author.py` was carrying the stale `semio.s.remodeling.remodeling` proto package and the stale
`$id` prefixes — updated so a regeneration reproduces the tree rather than reverting it.

**Deliberately NOT renamed** (a different namespace, and W10 owns the grammars): the DSL wire envelope
`semio remodeling.remodeling.dsl v1` and the `artifact-mark = "remodeling.remodeling"` in the four text
grammars / two binary protocols / `📸️snapshot/🦀️.rs:72`, plus the matching TS
`REMODELING_DSL_ENVELOPE` in W6b's `🚪️io` tree. These are internally consistent with each other and
with every committed example and fixture; renaming them is a wire-format change that must go with a
fixture regeneration. Peers spell this `block.block2d`, so `remodel.remodeling` is the eventual target —
**follow-up for W10 + W2c, together**.

## 2. Plugin root onto the declaration tree (task 3)

Two new files, both copied from `🗒️note`/`🔱️trinity`'s shape:

- `🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🦀️.rs` — `standard() -> StandardDeclaration<crate::RemodelApps>`,
  `StandardId("1")`, `MediaDeclaration { mimes: ["application/vnd.semio.remodeling+json"], extensions: ["remodeling"] }`.
  The extension is the real carried-over value (the old `definition()` codec row's
  `codec-extension: "16:remodeling.scene:remodeling"`); the mime is a documented synthesis, the same
  deviation note/trinity carry.
- `🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs` — `subset() -> SubsetDeclaration<crate::RemodelApps>`
  with `dialect: REMODELING_DIALECT`, `SchemaDeclaration{descriptor, inferences, inference_services: vec![]}`,
  **`io: io::io()`** (W6b's `pub fn io()` had landed on disk by 21:08 — verified by reading the function,
  its five `LanguageSpec`s and its 16 `IoEntry` rows), both surfaces via
  `editor_surface`/`viewer_surface`, and `examples: examples()`.

Artifact root: `declaration()` **and** `pilot_languages()` deleted outright (atomic cutover, no dual
channel) and replaced by
`artifact() -> ArtifactDeclaration<crate::RemodelApps> { kind: ArtifactKindId::parse("s.remodel.remodeling"), localization: &[], standards: vec![standard()] }`.
`definition()` is kept unread, exactly as `🗒️note` keeps its own (debt D1: the en/de names live only on
its capability rows). `.composers(native_composer_entries())` is gone — `SubsetDeclaration` has no
composers field; `io()`'s typed `IoEntry` slice is the replacement channel, the same removal note/trinity
made. **W6b should be told `native_composer_entries()` now has no caller.**

Plugin root: `.artifact(declaration()?)` + `.editor::<…>(…)` + `.viewer::<…>(…)` → one
`.declare_artifact(crate::artifacts::remodeling::artifact())`. `.editor_mutation_roster()` /
`.viewer_mutation_roster()` / `.activation()` / `.execution()` / `.requests()` unchanged.

Wiring file: two `#[path]` mounts added (`standards::v1::standard_root`, `standards::v1::subsets::any::subset_root`),
each re-exported with `pub use …::*`; mount-check reports 0 dangling.

**This is what closes W4 §4's SDK gap.** `project_artifact_declarations`
(`🔌️plugin/🦀️.rs:28218`) copies `subset.examples` onto the EDITOR surface's `App`, which is
`manifest.apps[].examples` — the field `PluginBuilder::.editor::<E>(AppDefinition)` had no way to fill.
ShellHost's picker (`🏛️ShellHost/🟦️.tsx:6418`) reads that array.

### Examples registered (task 3, second half)
`✏️editor/📚️examples/🦀️.rs` gained the third row — `synthetic-orbit`, exactly W7a's proposed literal —
so `REMODELING_EXAMPLES` is now `demo`, `demo-session`, `synthetic-orbit`, and a new
`example_source_slice() -> &'static [ExampleSource]` (a `OnceLock` over `example_sources()`) supplies the
`&'static` borrow `SubsetDeclaration.examples` needs. W7a's leaf was de-asynced (`pub async fn label`/
`source` → `pub fn`), which is the `E0277 LocalizedLabel: From<impl Future>` at `🛰️synthetic-orbit/🦀️.rs:56`.

`🎮️commands/🎬️set-active-example` now imports the selected example's media: a new
`example_media_operations(example_id, current)` emits one `create-asset` per `synthetic_orbit::FRAMES`
row (decoding each committed PNG through the plugin's own `decode_still_image` for the true width/height,
base64 through `base64_codec`), skipping asset ids the document already carries so re-selecting is not a
stream of rejected duplicate creates. The other two examples declare no media and answer with an empty
set. Without this the synthetic-orbit DSL's frame table points at ten asset ids nothing ever mints.

## 3. Calibration → engine (task 4)

`✏️editor/⚙️engine/🦀️.rs`:
- New `assumed_focal_ratio(&CalibrationState) -> Option<f64>` = `max(fx,fy) / (2 · max(cx,cy))`.
  The engine's own `default_intrinsics(w, h, ratio)` (`🏭️reconstruction/🦀️.rs:280-283`) is
  `fx = fy = ratio · max(w,h)` around a CENTRED principal point (`cx,cy = w/2,h/2`), so the sensor's long
  side is recoverable as `2 · max(cx,cy)` under that same assumption — which is why no frame-dimension
  plumbing is needed (`MediaStream`/`FrameRef` carry none). On synthetic-orbit: `272 / (2·160) = 0.85`,
  the fixture's true ratio, against the engine's default `1.0`. Degenerate rows return `None` and leave
  the engine's default rather than fabricating a number.
- `build_engine_params(params)` → `build_engine_params(params, calibration)`; call site
  `🎮️commands/🏗️run-reconstruction/🦀️.rs:653` passes `&scene.calibration`.
- Distortion is NOT fed: `EngineParams` has no distortion slot at all and `default_intrinsics` hard-codes
  `Distortion::None`. Documented in the fn's own docstring rather than silently dropped.

`📚️examples/🛰️synthetic-orbit/🧪️tests/🦀️.rs`: `UNCALIBRATED_GAUGE_SLACK` **12.0 → 1.0**, docstring rewritten
to say why. The app-level pose assertions are now the strict 2° / 3 % engine-level bounds.
⚠️ **Unmeasured**: W7a stated (did not measure) those bounds, and no run has ever executed. The first
green `cargo test` must confirm — if the app-level run now misses at 1.0 the honest next step is to
investigate the residual, not to reinstate a slack multiplier.

## 4. W5's three runtime defects (task 5)

1. **`remodeling-frames` emitted a shape the canvas host does not read.**
   `🎭️modes/📷️capture/🪟️windows/🖼️frames/🦀️.rs` rewritten against
   `JsonLayersCanvasSession` (`🧰️framework/…/📐️Canvas2dHost/🟦️.tsx:385-545`): `kind` not `type`
   (the host branches on `layer.kind === "image" | "polyline" | "circle" | "line"`), and **every** layer
   now carries an explicit `x/y/width/height` — `layerBounds()` returns `null` without all four and the
   record falls through to the "print the label in a corner" branch, which is exactly the "draws nothing
   recognizable" W5 saw. The frame is laid out centred on the canvas origin (`x = -w/2, y = -h/2`, matching
   the host's own camera transform) and each GCP observation became a `kind: "circle"`,
   `role: "handle"` marker box of half-side 6 px sharing that origin, so document pixel coordinates land
   on the pixels they annotate. The old `{type:"points", points:[{x,y,label}]}` aggregate is gone — the
   host's `points` field is `[[x,y],…]` and belongs to `polyline`, never to markers. Emission moved from
   `serde_json::json!` to `pack::json_object`/`json_array`/`json_to_string`.
2. **`report_table_json` emitted no `sortable`.** `🎭️modes/🔍️analyze/🪟️windows/📊️report/🦀️.rs` rewritten
   with `column(id, label)` / `row(cells)` / `optional_number(…)` helpers; every column of all six datasets
   now carries `sortable: true` (all remodeling cells are scalars, so all six sort), which is the flag
   `TableHost` (`📊️Table/🟦️.tsx:20,154`) reads. Also moved off `serde_json`.
3. **No document could render a mesh.** Root cause was not the resolver: `resolve_bounded_remodeling_mesh`
   already answers `REMODELING_BOX_MESH_CHILD_ID` and `REMODELING_EMPTY_MESH_CHILD_ID` inline with no
   `durable_artifacts` at all, and `default_remodeling_scene()` already uses
   `placeholder_remodeling_mesh_handle()`. The failure was in the committed EXAMPLE text: both
   `📚️examples/🎬️demo` and `📚️examples/🛰️synthetic-orbit` declared `source=placeholder` beside a
   *content-addressed* handle `child_id=remodeling-mesh-901ccade3f60f8f1`, which needs a
   `durable_artifacts` entry no DSL carries — so the boot document (now `boot_snapshot()`, which parses
   `demo`) resolved nothing. Both DSL documents now name the placeholder constant
   (`child_id=remodeling-mesh-constant-box target="remodeling-mesh-constant:box!s.stdio.semio@v1/mesh"`),
   which resolves inline and is what `source=placeholder` was always claiming. `schema-validate.py` still
   reports 0 violations after the edit.

W9's `MeshData: Serialize` items at `🎭️modes/🧊️model/🪟️windows/🧊️model/🦀️.rs:58` and
`👁️viewer/…/🧊️model/🦀️.rs:69,88` were **already closed by W4** — both windows build the wire object
field by field through a local `mesh_data_json`. They still use `serde_json::json!` on `Vec<f32>`, which
compiles; converting them to `pack` is cosmetic and was left alone.

## 5. `production_dispatch` (task 6)

`🧪️tests/📸️mutate-remodeling-1/🦀️.rs` (W2b's file, this one field only): all three subject handlers now
end in `.dispatched(…)`. Two new consts in the `subject` module — `PRODUCTION_BRIDGE_VERSION: u32 = 1`
(the `bridgeVersion` all 35 `productionDispatch` rows in `🔮️oracle/🔣️.json` declare) and
`ROUND_TRIP_BRIDGE_OPERATION = "round-trip-remodeling-dsl"`. `mutate` and `inverse` report
`vector.kind`, which is exactly the manifest's `productionDispatch.operation` for that row;
`identity-round-trip` has no manifest mutation row, so it names the codec bridge it actually reached
rather than borrowing a mutation id. Gate satisfied: `vectorReplayBreaches`
(`🧪️test/📦️packages/🟦️typescript/🟦️.ts:5509-5518`) refuses **every** subject result without
`productionDispatch.invoked`, including `identity-round-trip`; the operation string is not
cross-checked against the manifest, only `productionDispatch.variant` is (a runtime-inventory
comparison, not an outcome-record one).

## 6. Task 1 — compile closure, item by item

| item from the brief | state |
|---|---|
| `semio-framework-job` dep | already in `📦️packages/🦀️rust/Cargo.toml` (W4 added it, with `semio-framework-ui-contract`); no change needed |
| `Label: From<Label>` ×44 in `📌️panels/**` | already fixed by W4 (`ui_label()` helper + `String`/`&str` rows) |
| `MeshData: Serialize` ×3 in the two 3D windows | already fixed by W4 (field-by-field `mesh_data_json`) |
| `📚️examples/🛰️synthetic-orbit/🦀️.rs:56` `LocalizedLabel: From<impl Future>` | **fixed** — de-asynced `label()`/`source()` |
| `📚️examples/🎬️demo/🧪️tests` (3 × E0433) | already fixed by W7a (`#[cfg(test)]` on the mount) |
| artifact-root `#[cfg(test)]` serde_json on `RemodelingSnapshot` | **fixed** — both round-trip tests moved to `pack::to_json_string` / `pack::from_json_str`; `mint_and_stash_mesh`'s digest source moved to `pack::to_json_string(&mesh)` (`MeshData` has a first-party `pack::value::ToValue` at `🏗️mesh-engine/🦀️.rs:89`, and its serde derives are `#[cfg_attr(test, …)]` in a crate compiled WITHOUT our test cfg — a real `--tests` error) |
| ditto on `PackedF32`/`PackedU8`/`ReconstructionStage` (:1728-1777) | **left on serde deliberately** — all three still derive `Serialize`/`Deserialize` (no `ArtifactChild` in their closure), and the tests assert the serde wire form itself |
| `populated_scene_fixture()` | was `async fn` called without `.await` — de-asynced |

## 7. What is still open, by owner

| item | owner |
|---|---|
| `TurnResult::cold_pair_ingress` — 5 errors in `🔌️plugin/🖥️host` blocking the whole dependency graph | live framework peer (NOT this ticket) |
| Re-run `cargo check --lib`, then `--tests`, then `cargo test -p semio-s-plugin-remodel --lib` once that clears; every Rust change here is syntax-verified only | next lane |
| Confirm `UNCALIBRATED_GAUGE_SLACK = 1.0` actually holds on a real run | next lane |
| DSL wire envelope / grammar artifact-mark `remodeling.remodeling` → `remodel.remodeling` (+ every committed example and fixture) | W10 + W2c, one pass |
| `native_composer_entries()` (`🚪️io/🦀️.rs:436`) now has no caller — delete it or say why it stays | W6b |
| `durable-artifacts` absent and `child_id` printed snake_case in example DSL | framework `store::ArtifactChild` `DslRecord` inconsistency (W8's finding) |
| Regenerate owner-root `🔣️.json` / `🛂️.descriptor.semio` via `describe` — the declaration tree, not `definition()`, is now the registration channel, and the committed descriptor still carries the old `3d.remodel` / `s.remodel.remodel@1/*#…` spellings | after the crate builds |

## 8. Files changed

Created: `🏅️standards/🔖️1/🦀️.rs`, `🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs`.
Edited: plugin root `🦀️.rs`; artifact root `🗿️artifacts/📸️remodeling/🦀️.rs`; wiring `📦️packages/🦀️rust/🦀️.rs`;
`✏️editor/📚️examples/🦀️.rs`; `✏️editor/⚙️engine/🦀️.rs`; `✏️editor/🎮️commands/{🎬️set-active-example,🏗️run-reconstruction}/🦀️.rs`;
`✏️editor/🎭️modes/📷️capture/🪟️windows/🖼️frames/🦀️.rs`; `✏️editor/🎭️modes/🔍️analyze/🪟️windows/📊️report/🦀️.rs`;
`📚️examples/🛰️synthetic-orbit/{🦀️.rs,🧪️tests/🦀️.rs,🖼️assets/🗣️.dsl.semio}`; `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`;
`🧪️tests/📸️mutate-remodeling-1/🦀️.rs`; plus the 29 namespace-rename files listed in §1 and the ticket's
`🐍️schema-author.py`.
