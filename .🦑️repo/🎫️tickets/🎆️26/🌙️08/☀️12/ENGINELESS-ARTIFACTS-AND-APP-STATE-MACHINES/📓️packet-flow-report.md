# Packet report — `✏️s/🔌️plugins/🌊️flow` artifact-tree `⚙️engine` elimination

Target: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (413 LOC, single file). Directory **deleted**.

## Region → destination map (as executed)

| Region (old) | Symbol(s) | Destination | Rationale |
|---|---|---|---|
| `🔖️Constants` | `FLOW_DEFAULT_PROXIMITY_DISTANCE`, `FLOW_DEFAULT_GRID_FACTOR` | `🧬️schema/🦀️component.rs` (new `🔖️Constants` region) | **Deviation from naive app-guess**: these look like view-state defaults, but the schema's own `FlowArtifact::from_snapshot` (in the very same file) already consumed them directly. An artifact must never depend on an app, so they could not move to `apps/flow` — moved to schema instead. |
| `🔖️Constants` | `FLOW_WIDGET_DRAG_MIME` | `🎛️apps/🌊️flow/📌️panels/🛍️catalogue/🦀️component.rs` | Sole consumer; pure UI drag-payload MIME key, co-located with the two functions that use it. |
| `🔖️Constants` | `FLOW_EVAL_TICK_ACTION` | `🎛️apps/🌊️flow/🎮️commands/🧮️eval/🦀️component.rs` | All 3 real call sites of `eval_tick_effect()` (which alone reads this const) live in this one file. |
| `🔖️Register` | `register()`, `register_artifact_schema()`, `register_pilot_languages()` | `🎛️apps/🌊️flow/🦀️component.rs` (new `🔌️Registration` region) | Rule 6 — `register()` calls `register_document_codec_for_app::<FlowPlayApp>`, app-constitutional; matches the validated block2d exemplar, which moved its entire Register region (not just the app-codec line) to the app's top file. |
| `🔖️Host` | `seed_host_catalogue`, `apply_canvas_options`, `host_from_snapshot`, `host_operations` | `🎛️apps/🌊️flow/🦀️component.rs` (new `🔖️Host` region) | All take/produce `FlowHost`/`FlowConfig` (app types) and are consumed across 5+ command/window/panel files — cross-cutting app behavior. |
| `🔖️Host` | `snapshot_operations` | `🧬️schema/🧬️mutations/🦀️component.rs` (existing `🌉️FrameworkBridge` region) | **Deviation**: pure over two `FlowSnapshot`s, zero `FlowConfig`/`FlowHost` dependency — not app behavior. It directly reuses this file's own `from_framework_mutation`, so it belongs beside it rather than under `apps/flow` even though its only call site today is a command file. |
| `🔖️Host` | `eval_tick_effect()` | `🎛️apps/🌊️flow/🎮️commands/🧮️eval/🦀️component.rs` | Single-consumer file (3 call sites), kept beside its const. |
| `🔖️Selection` | `sync_host_selection`, `sync_host_selection_domains`, `focus_selection_camera` | `🎛️apps/🌊️flow/🦀️component.rs` (new `🔖️Selection` region) | Operate on `FlowHost`/`FlowConfig`; `sync_host_selection` delegates into `sync_host_selection_domains` so they stay together; co-located with `host_operations` et al. |
| `🔖️Widgets` | `split_endpoint`, `fixture_to_workflow` | `🎛️apps/🌊️flow/🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️component.rs` (new `🔖️Workflow` region) | Sole consumer (main window render); `fixture_to_workflow` takes a `DagFixture` (host runtime type) and returns `ui_wgpu` UI records — pure view projection, not schema. |
| `🔖️Widgets` | `widget_id`, `widget_kind_label`, `widget_tree_label` | `🧬️schema/🦀️component.rs` (new `🔖️Widgets` region) | Pure functions over `Widget`, the schema's own document type (`FlowArtifact.widgets: Vec<Widget>`). Rule 3. Bonus: this let us delete a *duplicate* private `widget_id` in `🧬️schema/💡️inferences/🧭topology/🦀️component.rs` that re-derived the same match arms with a docstring explaining it couldn't depend on the (still-nonexistent) canonical one — it now imports `crate::artifacts::flow::schema::widget_id`. |
| `🔖️Widgets` | `flow_widget_descriptor`, `flow_widget_drag_json` | `🎛️apps/🌊️flow/📌️panels/🛍️catalogue/🦀️component.rs` (new `🔖️WidgetDescriptors` region) | Sole consumer; build UI drag/catalogue JSON, not derived from a `Widget` value. |
| `🔹ArtifactEngine` | `struct FlowEngine` | **Deleted outright** | Zero construction sites confirmed (grep found only its own `struct`/`impl`) — matches the packet's repo-wide finding exactly. |
| `🚪️DerivedIoRegistry` | `io_registry` module (whole) | `🚪️io/🦀️component.rs` (new `🚪️DerivedIoRegistry` region) | Rule 5, verbatim move. |
| Tests | all 5 `#[test]` fns | split to their new homes (see below) | Rule 7. |

## Call sites updated

23 non-test call sites/imports across **18 files** (plus the `⚙️engine` module mount + a dead "external callers" shim in `glue.rs`):

- `✏️s/🔌️plugins/🌊️flow/🦀️component.rs` (plugin root `.setup(...)`)
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️component.rs` (the "thin `io_registry` wrapper" the packet warned about — `standards::v1::engine::io_registry` → `standards::v1::subsets::any::io::io_registry`)
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️component.rs` (dedup, not just call-site rename)
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📦️glue.rs` — removed `pub mod engine;` mount (was the only `#[path]` pointing at the deleted directory) **and** the pre-migration shim `pub mod engine { pub use super::standards::v1::engine::*; }`. Verified nothing outside this plugin crate imports `semio-s-plugin-flow`, so the shim was genuinely dead (per CLAUDE.md's no-legacy-support rule).
- 14 files under `🎛️apps/🌊️flow/**` (commands/panels/modes/config) — import path rewrites, listed in full in `grep` output below.

Verification grep (must be 0, confirmed):
```
grep -rn "flow::engine\|flow::standards::v1::engine\|::engine::" ✏️s/🔌️plugins/🌊️flow --include="*.rs" | grep -v "brep::engine"
```
→ 0 matches. (The one excluded hit, `🧩️extensions/📐️brep/🦀️component.rs`, is `semio_framework_3d::brep::engine` — an unrelated 3D boundary-rep kernel, not this artifact's engine.)

## Structural verification

- `grep -rn "flow::engine\|flow::standards::v1::engine" ✏️s/🔌️plugins/🌊️flow` → **0**
- `⚙️engine` directory under the artifact tree → **gone** (`find ... -name "⚙️engine"` returns only the pre-existing, already-empty `🎛️apps/🌊️flow/⚙️engine/` — untouched leftover debris from other work, out of this packet's scope, not the artifact-tree engine this packet targeted)
- `FlowEngine` struct → **0** occurrences anywhere in the plugin
- Every destination file confirmed to contain its moved code (see grep dump captured during the session)
- Assertion count: **5 tests / 9 `assert!`+`assert_eq!` before → 5 tests, same 5 names, after**, each now in exactly one file:
  - `split_endpoint_defaults_port_to_out` → `🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️component.rs`
  - `widget_id_and_kind_label_agree_across_variants` → `🧬️schema/🦀️component.rs`
  - `flow_widget_drag_json_wraps_descriptor_under_drag_mime` → `📌️panels/🛍️catalogue/🦀️component.rs`
  - `flow_eval_session_neural_cache_is_per_instance_not_process_wide` → `🎮️commands/🧮️eval/🦀️component.rs`
  - `host_from_snapshot_deletes_edge_selected_by_synapse_domain` → `🎛️apps/🌊️flow/🦀️component.rs`

## Compiler verification — re-run after `semio-s-plugin-stdio` turned green

`semio-s-plugin-stdio` itself now compiles clean (0 errors, confirmed independently). Re-ran both mandated commands after that:

```
RUSTC_WRAPPER="" cargo check -p semio-s-plugin-flow --all-targets
RUSTC_WRAPPER="" cargo test -p semio-s-plugin-flow
```

Both still fail with the **exact same 4 errors** as before stdio went green (byte-identical error text, same 4 files, same line numbers — `scratch-flow-cargo-check-2.txt` / `scratch-flow-cargo-test-1.txt`). Root cause is now precisely identifiable (not inferred): `semio-s-plugin-stdio`'s `MdSnapshot` and `JsonSnapshot` (defined in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/…/🧬️schema/📸️snapshot/🦀️component.rs:95` and `.../🔣️json/…/📸️snapshot/🦀️component.rs:439`) now shape:
- `MdSnapshot { schema: String, blocks: Vec<MdBlock> }` — no `body` field any more (was a flat `String`, is now a structured block AST).
- `JsonSnapshot { schema: String, value: JsonValue }` — `JsonValue` is stdio's own enum (`✏️s/🔌️plugins/🗄️stdio/…/🔣️json/…/📸️snapshot/🦀️component.rs:27`), not `serde_json::Value`.

Flow's own bridge files (**last touched 2026-08-10**, two days before this session, confirmed via `git log`, and `git diff --stat` shows **zero** changes to them from this packet) still write/read the pre-refactor shape (`MdSnapshot { body: ... }`, plain `serde_json::Value`):

```
error[E0560]: struct `MdSnapshot` has no field named `body`
 --> .../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️component.rs:8:62
  = note: available fields are: `blocks`

error[E0609]: no field `body` on type `&MdSnapshot`
 --> .../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️component.rs:9:59
  = note: available fields are: `schema`, `blocks`

error[E0308]: mismatched types
    --> .../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:9:28
     |     expected `Value`, found `JsonValue`

error[E0308]: mismatched types
 --> .../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs:9:66
  |   expected `JsonValue`, found `Value`

error: could not compile `semio-s-plugin-flow` (lib) due to 4 previous errors; 9 warnings emitted
```

**This is repo-wide, not flow-specific**: `grep`-ing for the same stale-bridge pattern (`MdSnapshot { schema: ..., body: ...` / plain `serde_json::Value` assigned to a stdio `JsonSnapshot.value`) turns up the identical shape in **35+ other plugins'** `🚪️io/📥️import`/`📤️export` leaves (block, puzzle, procedural, writer, dag, trinity, fem, draw, and more) — every plugin that bridges to `stdio.md`/`stdio.json` is in the same state. This is fallout from `semio-s-plugin-stdio`'s snapshot-shape overhaul (the mutation-vocabulary refactor) landing repo-wide, not something this packet's engine-deletion touched or introduced. Converting flow's md/json bridge to the new structured `MdBlock` AST and stdio's `JsonValue` enum is real, non-trivial feature work (not a rename) spanning dozens of plugins — out of this packet's scope (`✏️s/🔌️plugins/🌊️flow`'s artifact-tree `⚙️engine` only) and out of `CLAUDE.md`'s "no scratch scope-creep" spirit to patch piecemeal in just one plugin. Flagged as a follow-up task (see ticket coordinator notes) rather than fixed inline here.

**Conclusion: refactor complete and structurally verified. Compiler check still fails, but on 4 pre-existing errors in 2-day-old files this packet never touched, now provably caused by `semio-s-plugin-stdio`'s (unrelated, already-landed) snapshot-shape change rather than by stdio failing to build.** Every file this packet actually edited compiles clean — all 4 errors are outside that set.

## Files touched

Modified:
- `✏️s/🔌️plugins/🌊️flow/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️标准/…` (see below, exact path) `🧬️schema/💡️inferences/🧭topology/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎚️config/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎮️commands/🔄️layout/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎮️commands/🕸️node-graph/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎮️commands/🧩️widget/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎮️commands/🔗️synapse/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎮️commands/🗂️selection/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎮️commands/🧮️eval/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/📌️panels/📄️artifact/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/📌️panels/🔍️inspection/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/📌️panels/🛍️catalogue/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎭️modes/✏️edit/🪟️windows/🗣️compiled/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎭️modes/✏️edit/🪟️windows/🌊️main/🎚️options/📏️proximity/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/🎭️modes/🧬️generate/🎮️commands/🧬️generation/🦀️component.rs`

Deleted:
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (and the now-empty `⚙️engine/` directory)

## Out of scope, flagged not fixed

- `✏️s/🔌️plugins/🌊️flow/🎛️apps/🌊️flow/⚙️engine/` — an **already-empty** directory, unrelated leftover (not the artifact-tree engine this packet targeted, contains no files, `git` doesn't track it). Left untouched.
- One file under this plugin, `🧬️schema/📸️snapshot/📝️text/🦀️component.rs`, was already staged (`M`, first column) before this session started — not touched by this packet, flagged here only because `git status` surfaces it.
