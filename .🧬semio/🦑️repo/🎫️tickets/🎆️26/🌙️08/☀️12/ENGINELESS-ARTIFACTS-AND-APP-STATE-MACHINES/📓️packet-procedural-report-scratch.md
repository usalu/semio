# Packet `🌀️procedural` — engine dissolution report

Slice: the `🌀️procedural` plugin ONLY. Both artifact-tree `⚙️engine` directories dissolved and deleted.

## What changed — the two engine directories

| engine dir (deleted) | LOC | region → destination |
|---|---:|---|
| `🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` | 357 | `🔖️Io` (`procedural2d_io`) → `🎛️apps/◻2d/🦀️component.rs` new region `🔖️ArtifactIo`. `🔖️DocumentHelpers` (`host_from_fixture`, `host_from_fixture_with_session`, `host_operations`, `split_endpoint`, `fixture_to_workflow`, `collect_drawing_handles_from_eval`, `affine_transform_array`, `path_segments_from_node`, `scene_layers_from_drawing_handle`, `evaluate_generation_preview`, `generation_preview_layers`, `default_snapshot`, `empty_procedural2d_snapshot`) → `🧬️schema/🦀️component.rs` new region `🔖️DocumentHelpers`. `refresh_generation_preview` (takes `&mut Procedural2dConfig`, an app type) → `🎛️apps/◻2d/🎮️commands/🧬️generation/🦀️component.rs` new region `🔖️PreviewHelper`, made private (its only callers are in that file). `🔖️ArtifactEngine` (`Procedural2dEngine`) → **DELETED**. `🚪️DerivedIoRegistry` (`io_registry`) → `🚪️io/🦀️component.rs` new region `🚪️IoRegistry`. `🧪️Tests` → split to follow subjects. |
| `🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` | 975 | `🔖️Constants` (8 `PROCEDURAL_EXAMPLE_*`) + snapshot/fixture-pure part of `🔖️DocumentHelpers` (`default_snapshot`, `empty_procedural3d_snapshot`, `is_procedural3d_example_id`, `example_snapshot`, `example_document_json`, `generation_fixture_for`, `host_from_fixture`, `host_from_fixture_with_session`, `commit_fixture`, `split_endpoint`, `fixture_to_workflow`, `widget_id_from_instance_id`, `evaluate_generation_preview`) + all of `🔖️GumballTransforms` → `🧬️schema/🦀️component.rs` new regions `🔖️DocumentHelpers` / `🔖️GumballTransforms`. Config-referencing preview pipeline (`preview_tolerance`, `preview_camera_json`, `preview_selection_json`, `merge_status_json`, `preview_scene_status_json`, `is_brep_geometry_handle`, `collect_geometry_handles_from_eval`, `geometry_handles_for_widget`, `mesh_has_preview_geometry`, `apply_show_mode_mesh`, `preview_status_json`, `mesh_data_for_preview_handle`, `pending_preview_tessellate_handles`, `preview_tessellate_effects`, `preview_payload_from_eval`, `preview_payload_from_eval_with_session`) → `🎛️apps/🧊️3d/🦀️component.rs` new region `🔖️PreviewPipeline`. Mesh/DWG bridge (`merge_preview_meshes`, `export_mesh_from_document`, `procedural3d_mesh_from_document`, `procedural3d_document_from_mesh`) + `🔖️Register` (`register_dwg_mesh_bridge`) → same file, new region `🔖️MeshBridge`. `🔖️ExtensionContributions` (`ProgramContributionEntry`, `FlowExtensionTopicPayload`, `flow_extension_manifest_json`, `sync_flow_extension_contributions`) + `ensure_linked_flow_extensions` → same file, new region `🔖️ExtensionContributions`. `procedural3d_io` → same file, new region `🔖️ArtifactIo`. `🧪️TestSupport` (`test_support::lock`) → same file, new region `🧪️TestSupport`. `🔖️ArtifactEngine` (`Procedural3dEngine`) → **DELETED**. `🚪️DerivedIoRegistry` (`io_registry`) → `🚪️io/🦀️component.rs` new region `🚪️IoRegistry`. `🧪️Tests` (8 tests) → `🎛️apps/🧊️3d/🦀️component.rs`'s existing `mod tests`, new subregion `🔖️EngineComputeTests`. |

### The `*Engine` structs — deleted, confirmed dead

`grep -rn "Procedural2dEngine\|Procedural3dEngine" ✏️s/🔌️plugins/🌀️procedural --include="*.rs"` → **0 hits** after deletion, and 0 construction sites existed before (only the `pub struct` + its own `impl … ::new`). Neither was constructed anywhere. Deleted outright per the ticket ruling; **no exception found**.

### procedural3d numerical-code decision

`procedural3d`'s extra ~600 LOC over `procedural2d` is **not** a D6 pure geometric algorithm — it is the *preview render pipeline* (tessellation dispatch, show-mode mesh filtering, mesh/instance payload assembly) and the *flow-extension registration* wiring. Every function in that cluster either takes `&Procedural3dConfig` directly or is reachable only from one that does (`export_mesh_from_document` constructs a `Procedural3dConfig::default()` to run the same pipeline), and the extension-contribution cluster is stateful (`static Mutex<String>` / `static Once`). Both are app behaviour by the destination map's rule 4/D5, so **no module-level `✏️s/🔨️modules/🌀️procedural/⚙️engine/` was created** — the `🎛️apps/🧊️3d/` destination fits without stretching. The genuinely snapshot-pure part (fixture normalization, gumball graph splicing, DSL example parsing) went to `🧬️schema` as rule 3 prescribes.

## Unqualified paths qualified (before → after)

Every relocated body's unqualified references were rewritten to fully-qualified paths. The load-bearing ones:

| site | before | after |
|---|---|---|
| `🗿️artifacts/🌀️procedural2d/🦀️component.rs:127` `declaration()` `.composers(...)` | `…::standards::v1::engine::io_registry::entries()` | `crate::artifacts::procedural2d::standards::v1::subsets::any::io::io_registry::entries()` |
| `🗿️artifacts/🌀️procedural2d/🦀️component.rs` `io_registry` wrapper `use … as v1` | `crate::artifacts::procedural2d::standards::v1::engine::io_registry as v1` | `crate::artifacts::procedural2d::standards::v1::subsets::any::io::io_registry as v1` |
| `🗿️artifacts/🧊️procedural3d/🦀️component.rs:126` `declaration()` `.composers(...)` | `…::standards::v1::engine::io_registry::entries()` | `crate::artifacts::procedural3d::standards::v1::subsets::any::io::io_registry::entries()` |
| `🗿️artifacts/🧊️procedural3d/🦀️component.rs` `io_registry` wrapper `use … as v1` | `crate::artifacts::procedural3d::standards::v1::engine::io_registry as v1` | `crate::artifacts::procedural3d::standards::v1::subsets::any::io::io_registry as v1` |
| `🚪️io/🦀️component.rs` (both artifacts) relocated `io_registry` body: `crate::artifacts::procedural{2,3}d::io::…::deserialize_bytes` / `…serialize_bytes` | already fully qualified in source | kept fully qualified verbatim (no bare-name rebind introduced) |
| `🧬️schema/🦀️component.rs` (2d) `host_operations` body | `crate::artifacts::procedural2d::op::procedural2d_fixture_operations` | unchanged, already qualified |
| `🧬️schema/🦀️component.rs` (2d) `default_snapshot` body | `PROCEDURAL2D_EXAMPLE_TEXT` (bare, via `use`) | `crate::artifacts::procedural2d::dsl::PROCEDURAL2D_EXAMPLE_TEXT` |
| `🎛️apps/🧊️3d/🦀️component.rs` `PreviewPipeline` bodies | `widget_id(widget)` (bare, via `use`) | `crate::artifacts::procedural3d::widget_id(widget)` (5 sites) |
| `🎛️apps/🧊️3d/🦀️component.rs` `mesh_data_for_preview_handle` | `tessellate_geometry(...)` (bare, via crate-root re-export `use`) | `flow::tessellate_geometry(...)` |
| `🎛️apps/🧊️3d/🦀️component.rs` `procedural3d_document_from_mesh` / `export_mesh_from_document` | `default_snapshot()` / `host_from_fixture(...)` (bare) | `crate::artifacts::procedural3d::schema::default_snapshot()` / `…::schema::host_from_fixture(...)` |
| `🎛️apps/🧊️3d/🦀️component.rs` `EngineComputeTests` | `default_snapshot()`, `example_snapshot()`, `PROCEDURAL_EXAMPLE_*`, `PROCEDURAL3D_EXAMPLE_RECTANGLE_WIRE_TEXT` (bare) | all rewritten to `crate::artifacts::procedural3d::schema::…` / `crate::artifacts::procedural3d::dsl::…` |
| `🎛️apps/◻2d/🎮️commands/🧬️generation/🦀️component.rs` `refresh_generation_preview` body | `selected_generation(...)`, `evaluate_generation_preview(...)` (bare) | `flow::playbook::selected_generation(...)`, `crate::artifacts::procedural2d::schema::evaluate_generation_preview(...)` — the `selected_generation` qualification matters because this file separately imports `flow::playbook::select_generation` (different fn, adjacent name) |

### The `io_registry` shadow trap — kept safe

Both artifact roots still define their own `pub mod io_registry` wrapper returning `&'static [&'static ComposerEntry]`. The relocated real registry now lives at `…::subsets::any::io::io_registry` and returns `&'static [ComposerEntry]`. **No bare `io_registry::entries()` exists anywhere in the plugin** — verified:

```
$ grep -rn "io_registry::entries()" ✏️s/🔌️plugins/🌀️procedural --include="*.rs"
```
returns only fully-qualified call sites (4 total: 2 `declaration()` `.composers(...)`, 2 wrapper `v1::entries()` where `v1` is an explicitly-aliased `use` of the real path). The shadow-list invariant ("both procedural artifacts shadow-present, already-qualified") is preserved.

## `📦️glue.rs` mount changes

`✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs` (1,093 → 1,083 lines). Four removals, two per artifact:

1. `procedural2d`: removed the `#[path = "…/✳️any/⚙️engine/🦀️component.rs"] pub mod engine;` mount (was line 37-38).
2. `procedural2d`: removed the flattened re-export shim `pub mod engine { pub use super::standards::v1::engine::*; }` (was lines 423-425).
3. `procedural3d`: removed the `#[path = "…/✳️any/⚙️engine/🦀️component.rs"] pub mod engine;` mount (was line 460-461).
4. `procedural3d`: removed the flattened re-export shim `pub mod engine { pub use super::standards::v1::engine::*; }` (was lines 840-842).

No other `engine` mount or re-export existed in the file (checked with `grep -n "pub mod engine\|::engine::"` over the whole file before editing — exactly those 6 lines).

## Assertion / test counts — before vs after

Counted mechanically with `grep -o '<token>' | wc -l` on the exact tokens `assert!(`, `assert_eq!(`, `assert_ne!(`, `#[test]`. Baselines taken from `git show HEAD:<path>`.

### Sources (deleted engine files) — baseline at HEAD

| file | `assert!` | `assert_eq!` | `assert_ne!` | `#[test]` |
|---|---:|---:|---:|---:|
| `procedural2d/…/⚙️engine/🦀️component.rs` | 2 | 2 | 0 | 2 |
| `procedural3d/…/⚙️engine/🦀️component.rs` | 20 | 8 | 4 | 8 |
| **total to relocate** | **22** | **10** | **4** | **10** |

### Destinations — before (HEAD) → after

| file | `assert!` | `assert_eq!` | `assert_ne!` | `#[test]` |
|---|---|---|---|---|
| `🗿️artifacts/🌀️procedural2d/…/🧬️schema/🦀️component.rs` | 0 → 1 (+1) | 0 → 0 | 0 → 0 | 0 → 1 (+1) |
| `🎛️apps/◻2d/🦀️component.rs` | 10 → 11 (+1) | 13 → 15 (+2) | 0 → 0 | 14 → 15 (+1) |
| `🗿️artifacts/🧊️procedural3d/…/🧬️schema/🦀️component.rs` | 0 → 0 | 0 → 0 | 0 → 0 | 0 → 0 |
| `🎛️apps/🧊️3d/🦀️component.rs` | 13 → 33 (+20) | 4 → 12 (+8) | 0 → 4 (+4) | 12 → 20 (+8) |
| **destination totals gained** | **+22** | **+10** | **+4** | **+10** |

**Arithmetic balances exactly: gained == relocated, for all four token classes. Zero assertions lost.**

Per-test placement:
- 2d `default_snapshot_parses_the_bundled_example` (1 `assert!`) → `🧬️schema/🦀️component.rs` (follows `default_snapshot`).
- 2d `procedural2d_io_declares_the_params_and_drawing_ports` (1 `assert!`, 2 `assert_eq!`) → `🎛️apps/◻2d/🦀️component.rs` `🔖️PortTests` (follows `procedural2d_io`).
- 3d all 8 tests (20 `assert!`, 8 `assert_eq!`, 4 `assert_ne!`) → `🎛️apps/🧊️3d/🦀️component.rs` `🔖️EngineComputeTests` (they exercise `PreviewPipeline`/`MeshBridge`/`ArtifactIo`/`ExtensionContributions`, all now app-side). Their two private test helpers (`test_serial`, `preview_payload_from_evaluated_fixture`) travelled with them.

## Structural verification

```
$ find ✏️s/🔌️plugins/🌀️procedural -path "*🗿️artifacts*" -name "⚙️engine" -type d
(no output — 0 directories)

$ grep -rn "::engine::\|standards::v1::engine" ✏️s/🔌️plugins/🌀️procedural
(no output — 0 matches)

$ grep -n "⚙️engine\|::engine::" ✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs
(no output — 0 lines)

$ grep -rn "Procedural2dEngine\|Procedural3dEngine" ✏️s/🔌️plugins/🌀️procedural --include="*.rs"
(no output — 0 matches)
```

The two remaining `⚙️engine` directories under this plugin are the **app-side** stubs `🎛️apps/◻2d/⚙️engine` and `🎛️apps/🧊️3d/⚙️engine`, which are required by `appComponentDirs` and are the intended future home for behaviour — they stay, and are out of scope for the artifact-tree burn-down.

## Compiler verification

Verbatim command (both `RUSTC_WRAPPER=""` and `--all-targets` present, per rule 6), run three times:

```
cd /Users/ueli/Documents/semio
RUSTC_WRAPPER="" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES/🎯️target" cargo check -p semio-s-plugin-procedural --all-targets
```

### Run 1 — blocked upstream by stdio

```
For more information about this error, try `rustc --explain E0425`.
warning: `semio-s-plugin-stdio` (lib) generated 602 warnings
error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error; 602 warnings emitted
```

The crate the compiler *named* was `semio-s-plugin-stdio`, not ours — rustc never reached our crate. That is the hard-rule-7 upstream RED.

### Run 2 — stdio cleared; our crate compiled, 94 + 104 errors

```
warning: `semio-s-plugin-procedural` (lib) generated 78 warnings
error: could not compile `semio-s-plugin-procedural` (lib) due to 94 previous errors; 78 warnings emitted
warning: `semio-s-plugin-procedural` (lib test) generated 82 warnings (73 duplicates)
error: could not compile `semio-s-plugin-procedural` (lib test) due to 104 previous errors; 89 warnings emitted
```

Full output: `scratch-procedural-cargo-check-1.txt`.

**One error was genuinely mine and is fixed.** `E0599: no associated function or constant named parse_dsl found for struct Procedural2dSnapshot`, at the relocated `default_snapshot` in `🌀️procedural2d/…/🧬️schema/🦀️component.rs:371`. The `⚙️engine` file had `use store::ArtifactDsl;` at its top; I carried the function over but not the trait import. Fixed by adding `use store::ArtifactDsl;` to that file's import block. This is exactly the "every unqualified path in a moved body is a hazard" rule biting — here a *trait in scope*, not a path.

**The remaining errors are another session's in-flight mutation-vocabulary split, established by git rather than by file-path guessing:**

```
$ git log --oneline -5 -- ".../🌀️procedural2d/…/🧬️schema/🧬️mutations/"
62152fabcc 🐙️ueli🎆️26🌙️06☀️04🚩️499
31209e7afe 🐙️ueli🎆️26🌙️06☀️04🚩️498
20252aa16d 🐙️ueli🎆️26🌙️06☀️04🚩️496   <- this session's start commit
...
$ git diff --stat 20252aa16d..HEAD -- ".../🧬️mutations/"
19 files changed, 370 insertions(+), 298 deletions(-)
```

Commits **497–499 landed after this session started at 496**, creating 18 new mutation-slug files (`➕create-generation`, `➖delete-generation`, `🏷️rename-generation`, `🔁replace-widget`, `🔄replace-synapse`, `🔢change-generation-value`) and stripping 300 lines out of `🧬️mutations/🦀️component.rs`. Every remaining error class is a symptom of that half-landed split:

- `E0252` duplicate names (`change_schema`, `create_widget`, …) — old flat definitions plus new slug modules both in scope;
- `E0432` unresolved `create_generation::create_generation`, `crate::artifacts::procedural2d::widget_index`, `super::set_widget` — helpers deleted from the flat file, not yet re-exported;
- `E0433`/`E0423`/`E0425` missing modules `update_widget`/`delete_widget_position`/`update_camera` and "expected function, found module";
- `E0599` no variant `SetWidget` / `Generation` on `Procedural2dMutation`/`Procedural3dMutation` — the enum's variant vocabulary is being rewritten.

**53 of the 94 error locations are inside `🧬️mutations/` slug directories this packet never opened.** This packet's only touch anywhere in `🧬️mutations/` was a single `use` line per file (`…::engine::empty_procedural{2,3}d_snapshot` → `…::schema::…`) plus one doc-comment path — verified still present after the concurrent commits.

### Run 3 — post-fix re-verification (`scratch-procedural-cargo-check-2.txt`)

```
error: could not compile `semio-s-plugin-procedural` (lib) due to 93 previous errors; 78 warnings emitted
error: could not compile `semio-s-plugin-procedural` (lib test) due to 103 previous errors; 89 warnings emitted
```

**94 → 93 and 104 → 103: exactly one error removed from each target, which is precisely the `parse_dsl` fix landing.** The only remaining `parse_dsl` occurrences in the whole log are three lines of an unrelated *warning* (`unnecessary qualification`) inside `🗄️stdio`'s json snapshot — not an error, not ours.

**Error locations, run 3, by file — this is the load-bearing evidence:**

| location | errors |
|---|---:|
| `🧬️mutations/**` (both artifacts, flat file + slug dirs + `💾️binary`) | **89** |
| `🎛️apps/◻2d/🦀️component.rs`, `🎛️apps/🧊️3d/🦀️component.rs`, and the `🎮️commands/{🧬️generation,🎨️example}` files | 10 |
| `📸️snapshot/📝️text/🦀️component.rs` | 1 |
| **`✳️any/🧬️schema/🦀️component.rs` (2d) — the relocated `🔖️DocumentHelpers`** | **0** |
| **`✳️any/🧬️schema/🦀️component.rs` (3d) — the relocated `🔖️DocumentHelpers` + `🔖️GumballTransforms`** | **0** |
| **`✳️any/🚪️io/🦀️component.rs` (both) — the relocated `🚪️IoRegistry`** | **0** |

**Every file this packet relocated code *into* at the artifact layer now compiles with zero errors.** The 2d schema file went from 2 errors (my `parse_dsl` bug) to 0.

The 10 app-file errors are all the *same* error — `E0599: no variant named SetWidget found for enum Procedural{2,3}dMutation` — and are provably not this packet's:

```
$ sed -n '299p' 🎛️apps/◻2d/🦀️component.rs
    operations.push(Procedural2dMutation::SetWidget { index, widget: … });

$ git show 20252aa16d:"…/🎛️apps/◻2d/🦀️component.rs" | grep -c "Procedural2dMutation::SetWidget"
1                      # present, identical, at this session's START commit

$ grep -rn "SetWidget" "…/🧬️schema/🧬️mutations/🦀️component.rs"
                       # (no output — the VARIANT no longer exists)
```

The call sites are byte-identical to their state at commit 496 and were never touched by this packet; the *variant they name* was deleted out from under them by the 497–499 vocabulary split. Same for the `Generation` variant errors in the `🎮️commands/🧬️generation` files.

**Status: relocation COMPLETE, structurally verified, and — for every file this packet wrote into at the artifact layer — compiling clean. The crate as a whole is RED, but not from this packet:** 89 of 93 errors are inside `🧬️mutations/` directories this packet never opened, and the remaining 4+ are pre-existing call sites orphaned by the concurrent split. This packet's own bug count found and fixed: **1** (`use store::ArtifactDsl`). A whole-crate green is not obtainable until the 497–499 mutation-vocabulary split finishes, since it shares the compilation unit — recommend re-verifying in one pass then, per the manifest's standing practice.

## Concurrent-churn observations

- `ps` showed ~10 concurrent `cargo check -p semio-s-plugin-*` invocations from peer sessions sharing this ticket's `🎯️target` directory (architect, block, fem, gis, layout, lowpoly, process, stdio, writer). Build-lock contention made run 1 take >2 min before producing output — the expected "Blocking waiting for file lock" behaviour of hard rule 5, not a failure.
- `semio-s-plugin-stdio` went from RED (run 1) to compiling (run 2) inside this session's window, confirming the manifest's "a verification is a timestamp, not a property" warning in both directions.
- A mutation-vocabulary split (commits 497–499) is **live in `🌀️procedural` right now**, in the same crate as this packet. It was not present when this packet started. Not touched, not "fixed" — reported per protocol.

## Files touched

Updated:
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🌀️procedural/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🌉️wasm/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🎮️commands/🧬️generation/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🎮️commands/🕸️graph/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🎮️commands/🧩️widget/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🎮️commands/🧮️eval/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🌉️wasm/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎚️config/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/📌️panels/📄️artifact/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/📌️panels/🛍️catalogue/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/📌️panels/🔍️inspection/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎮️commands/🎨️example/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎮️commands/🧮️eval/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎮️commands/🧬️generation/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎮️commands/🕸️graph/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎮️commands/🧭️gumball/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎮️commands/🗣️locale/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎮️commands/🗂️selection/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎮️commands/🌞️sun/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎮️commands/👁️view/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎮️commands/🧩️widget/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️component.rs`

Removed (whole directories, no `Cargo.toml` inside — verified with `find <dir> -name Cargo.toml` before deletion, both empty):
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/`

Created (this folder only): `📓️packet-procedural-report-scratch.md`, `scratch-procedural-cargo-check-1.txt`.

## sharedFileRequests

None. Every file touched is inside `✏️s/🔌️plugins/🌀️procedural`. No edit to `🗄️stdio`, repo-root `📜️script.ts`, `🔣️taxonomy.json`, or any `AGENTS.md`.
