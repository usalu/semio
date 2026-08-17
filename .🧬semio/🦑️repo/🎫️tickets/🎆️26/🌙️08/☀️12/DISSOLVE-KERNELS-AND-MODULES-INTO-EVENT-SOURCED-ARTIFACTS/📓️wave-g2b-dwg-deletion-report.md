# Wave G2b — finishing G2's DWG relocation: host repoint + registrant flip; deletion still blocked

## Status: Jobs 1–2 **DONE and verified**. Job 3 (module deletion) **`blocked` — real live callers remain, none in my boundary.**

## 1. Mount-resolution evidence — independently re-derived, not taken on faith

Resolved every `#[path = "…"]` in the repo to a realpath and compared against the two candidate files, with a small script (not a text grep):

```
target1 (bare os component.rs): 🧰️framework/🛍️products/💻️os/🦀️component.rs
target2 (host component.rs):    🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs

Only mount found: ('🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📦️glue.rs', '../../🦀️component.rs', → target2)
```

**Confirmed independently**: `💻️os/🦀️component.rs` (bare) has zero `#[path]` mounts anywhere in the repo — genuinely dead code, left untouched per the mission's explicit instruction. `💻️os/🖥️host/🦀️component.rs` is mounted exactly once, unconditionally (not behind any `#[cfg(feature)]`), by `🖥️host/📦️packages/🦀️rust/📦️glue.rs:27-29`, into crate `semio-framework-os` (`Cargo.toml:2 name = "semio-framework-os"`, `package.metadata.semio.id = "os-host"`).

**Dependency-closure check, done by reading every `Cargo.toml` in stdio's real chain, not by inference**: `semio-s-plugin-stdio` depends only on `semio-framework-mesh-engine`, `semio-framework-os-kernel`, `semio-framework-plugin`, `semio-framework-schema`. Read each of those four's own `Cargo.toml` (`semio-framework-plugin` → `semio-framework`, `semio-framework-os-kernel`, `semio-framework-schema`, `ui_wgpu`; `semio-framework` → `ui_wgpu`, `semio-framework-hash`, `semio-framework-mesh-engine`, `semio-framework-os-kernel`; `ui_wgpu` → `ui_styling`, `semio-framework-geometry`, `semio-framework-os-kernel`) — **`semio-framework-os` (the `🖥️host` crate) appears nowhere in that closure.** Cross-checked the reverse direction too: `grep -rl "🖥️host/📦️packages/🦀️rust\""` over every `Cargo.toml` shows only plugins that already depend on stdio *and* on `semio-framework-os` as siblings (raster, process, demonstrator, animate, space, procedural, gis, shooting, layout, puzzle) — none of them sit between stdio and `semio-framework-os` in a way that would cycle. **The edge `semio-framework-os → semio-s-plugin-stdio` is legal.** Real compilation (below) is the definitive proof, not just the closure reasoning.

## 2. Job 1 — host repointed

`🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml`: added
```
semio-s-plugin-stdio = { path = "../../../../../../✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust", package = "semio-s-plugin-stdio" }
```
(caught my own off-by-one — first wrote 7 `../` instead of 6, verified the realpath with `os.path.normpath` before proceeding, corrected it).

**Immediately after the manifest edit**, as instructed:
```
$ RUSTC_WRAPPER="" cargo metadata --no-deps --format-version 1 >/dev/null && echo WORKSPACE_OK
WORKSPACE_OK
```

`🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`, inside `pub mod media_export_raster`:
- `use semio_framework::{DwgColor, DwgDrawing, DwgEntity, DwgGeometry};` → `use semio_s_plugin_stdio::artifacts::dwg::{DwgColor, DwgDrawing, DwgEntity, DwgGeometry};`
- All 8 fully-qualified call sites (`semio_framework::dwg_to_bytes` ×2, `dwg_from_bytes` ×4, `dwg_drawing_to_mesh` ×1, `mesh_to_dwg_drawing` ×1) repointed to `semio_s_plugin_stdio::artifacts::dwg::*` — these are called via full path, not the local `use`, so needed their own replacement.
- Two more `semio_framework::dwg_from_bytes` calls found in `workflow`'s own `mod tests` (lines 3603, 3613 — a different module, feature-gated behind `os-host-full`, not reachable from the local import) — repointed too, for the same reason: leaving them would keep the module a live caller.

`grep -n "semio_framework::[Dd]wg" 🖥️host/🦀️component.rs` → **zero hits** after the edit.

## 3. Job 2 — registrant census, corrected past what the brief named

`grep -rn "register_dwg_import_handler"` found **four real registrants**, not the one (`cad`) the brief anticipated:

| Registrant fn | Registered from | Param type before |
|---|---|---|
| `cad_document_from_dwg` (+ its own callee `cad_working_scene_from_dwg`) | `🎪️demonstrator/🎪️panes/📐️koordinator/🦀️component.rs:29` | `&semio_framework::DwgDrawing` |
| `gis2d_document_json_from_dwg` | `🎪️demonstrator/🎪️panes/🗺️verfolgen/🦀️component.rs:20` | `&DwgDrawing` (local import from `semio_framework_plugin`) |
| `puzzle2d_document_json_from_dwg` | `🧩️puzzle/🎛️apps/◻2d/🦀️component.rs:1387` (self-registers) | `&semio_framework::DwgDrawing` |
| inline closure `\|_drawing\| Ok(json!({...}))` | `🪐️space/🎛️apps/🪐️space/🎮️commands/🖼️media/🦀️component.rs:126` | ignores the param — signature-agnostic, needs no change |

All three typed registrants + the host's own signature had to flip **together** — confirmed by checking each crate's `Cargo.toml` already lists `semio-s-plugin-stdio` as a dependency (`cad`, `gis`, `puzzle` all do), so no manifest edits were needed for them.

**A fifth file was pulled in by the same mechanics, not by `register_dwg_import_handler` itself**: `animate`'s `animate_present_document_json_from_dwg(drawing: &semio_framework::DwgDrawing)` passes `drawing` straight into `semio_framework_os::dwg_drawing_to_svg(drawing)` — a function whose signature I changed in Job 1. Leaving `animate` on the old type would have been a straight type-mismatch the moment Job 1 landed, registrant or not. Fixed in the same change.

Edits made (all mechanical type-path swaps `semio_framework::Dwg* → semio_s_plugin_stdio::artifacts::dwg::Dwg*`, plus fixing stale doc comments that called the type "legacy"/"frozen" now that it has a real home):

- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — `cad_working_scene_from_dwg`, `cad_document_from_dwg` signatures + body literal.
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🦀️component.rs` — test-only `DwgDrawing`/`DwgEntity`/`DwgColor`/`DwgGeometry` constructions.
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs` — top import switched from `semio_framework_plugin::{DwgDrawing, DwgGeometry}` to `semio_s_plugin_stdio::artifacts::dwg::{DwgDrawing, DwgGeometry}`; the 3 fully-qualified `semio_framework_os::DwgEntity`/`DwgColor` test-construction sites moved to a test-scoped `use semio_s_plugin_stdio::artifacts::dwg::{DwgColor, DwgEntity};` (kept out of the top-level import specifically to avoid an unused-import warning on the non-test `--lib` build — caught this via a warning surfaced while checking `semio-s-plugin-demonstrator`, fixed, re-verified 171/0 unchanged).
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🦀️component.rs` — `puzzle2d_document_json_from_dwg` signature + one test construction.
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs` — `animate_present_document_json_from_dwg` signature + doc comment + 2 test constructions.

`🎪️demonstrator`'s own two files needed **zero source edits** — they only pass function *values* (`cad_document_from_dwg`, `gis2d_document_json_from_dwg`) into `register_dwg_import_handler`, so once the callee signatures matched the new host signature, type inference closed the loop with no text change on the demonstrator side.

### Stray-file incident (caught and removed immediately)

While staging a `touch` for the animate file I fat-fingered a path and created an empty `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️标准` (wrong CJK glyphs instead of `🏅️standards`) — the exact failure mode a prior wave's report warned about. Caught on the next `ls`, removed with `rm -f` before anything was written into it. Re-verified with `ls … | grep -i 标准` → no match.

## 4. Verification — every command, with real output

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-framework-os --all-targets
    Finished `dev` profile [unoptimized] target(s) in 39.02s      # 0 errors, only pre-existing warnings
```
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-framework-os --lib
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
The 0/0 is real and expected, not a hidden failure: the DWG round-trip tests (`svg_to_dwg_round_trip_produces_a_polyline`, `mesh_dwg_registrar_round_trips_a_box`) live inside `pub mod workflow`, which is entirely behind `#[cfg(feature = "os-host-full")]`, off by default. Ran that feature explicitly too:
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-framework-os --lib --features os-host-full
error: could not compile `semio-framework-os` (lib test) due to 107 previous errors
```
**All 107 are pre-existing and unrelated to DWG** — `LocalizedLabel: From<&str>` not satisfied, `store::test_support::assert_op_line_round_trip`/`assert_dsl_round_trip`/`assert_operation_round_trip`/etc. genuinely don't exist anywhere in the kernel crate (confirmed: `grep -rn "fn assert_op_line_round_trip"` over the whole `os-kernel` package → zero hits), `PluginManifest` missing `artifact_kinds`, `ArtifactPack::encode_pack` returning `Vec<u8>` with no `.expect()`. None of the 107 errors' `-->` lines fall inside the DWG region I edited — checked explicitly, line-by-line, against every line I touched (2618, 2652, 2656, 2734, 2742, 2787, 2789, 2821/2822, 2831/2832, 3603, 3613): **zero matches**. Since rustc reports every error it finds in a compilation unit (not just the first), and my lines produced none, this is real evidence the DWG edits type-check cleanly even under this broken feature — I did not silently skip verifying them.

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-cad --lib
test result: ok. 139 passed; 0 failed; 1 ignored     # matches the ticket's stated cad baseline exactly
    (incl. cad_document_from_dwg_creates_one_object_per_layer_with_geometry ... ok,
           cad_document_from_empty_dwg_mints_no_shape_model_child ... ok)

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-gis --lib
test result: ok. 171 passed; 0 failed; 0 ignored
    (incl. dwg_import_collects_point_and_line_vertices ... ok,
           dwg_import_lowers_a_closed_polyline_through_a_draw_node_and_carries_the_close_segment ... ok,
           dwg_import_falls_back_to_default_document_when_empty ... ok)

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-puzzle --lib
test result: FAILED. 452 passed; 3 failed
    (incl. apps::puzzle2d::component::tests::dwg_import_returns_empty_board_with_no_camera_field ... ok)
failures:
    artifacts::puzzle2d::standards::v1::subsets::any::schema::mutations::component::tests::puzzle2d_delta_ops_are_granular_and_round_trip
    artifacts::puzzle3d::standards::v1::subsets::any::schema::mutations::component::tests::puzzle3d_delta_ops_round_trip_and_stay_granular
    artifacts::puzzle5d::standards::v1::subsets::any::schema::mutations::component::tests::puzzle5d_delta_ops_round_trip_and_stay_granular
```
**All 3 failures are pre-existing and DWG-unrelated** — a `"camera"` field mismatch in mutation round-trip tests (`◻2d`/`3d`/`5d` `…mutations::component::tests`), a different file/subsystem from anything I touched. Confirmed pre-existing by attribution, not assumption: `stat -f '%Sm' …/◻2d/…/🧬️mutations/🦀️component.rs` → `Aug 13 00:44:33 2026`; `git log --date=iso -1` on that path → `2026-08-13 01:03:02`, both well before this session started. The one DWG-specific test in this crate passes.

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-animate --lib
test result: ok. 225 passed; 0 failed; 0 ignored
    (incl. from_dwg_never_errors_on_empty_drawing ... ok, from_dwg_builds_single_slide_deck_from_entity ... ok)
```

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-s-plugin-demonstrator --all-targets
error: could not compile `semio-s-plugin-procedural` (lib) due to 93 previous errors
```
**Not mine, not DWG.** The errors are `E0252` "the name `change_schema`/`clear_widget_layout`/`connect_synapse`/… is defined multiple times" and `E0432` unresolved imports (`create_generation::create_generation` etc.) inside `🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — this is the flow/widget-mutation vocabulary SMO's ticket is actively decomposing (`create-widget`/`connect-synapse`/etc., per `📌️important.md`'s SMO verb-ruling table), mid-edit. Attribution: `stat` → `Aug 13 00:15:09`; `git log --date=iso -1` → `2026-08-13 00:29:42`, both before this session. `demonstrator` itself only *passes function values* into `register_dwg_import_handler`/`register_mesh_dwg_export_handler` (confirmed by direct grep — no `Dwg*` type usage of its own in either pane file), and those functions (`cad_document_from_dwg`, `gis2d_document_json_from_dwg`) already independently compiled and passed their own crates' tests above, so I'm confident demonstrator's DWG wiring is sound even though I could not get a green `cargo check` for the whole crate today.

**Re-confirmed the four crates I did not touch remain exactly at their stated baselines** (dependency direction — plugins/products depend on framework, never the reverse — means my new `semio-framework-os → semio-s-plugin-stdio` edge cannot reach them):
```
$ cargo test -p semio-framework --lib             →  127 passed; 0 failed   (matches baseline)
$ cargo test -p semio-framework-3d --lib          →  413 passed; 0 failed   (matches baseline)
$ cargo test -p semio-framework-mesh-engine --lib →  20 passed; 0 failed    (matches baseline)
$ cargo test -p semio-s-plugin-stdio --lib        →  2430 passed; 5 failed  (identical 5-failure set to G2's baseline: binary/dwg-ac1018/dxf/ifc/zip inference_default_law / fixture_honesty_law)
```

## 5. Job 3 — zero-live-callers proof, and it's **not zero**

Final repo-wide grep for every old-path spelling (`semio_framework::Dwg*`/`dwg_*`, `semio_framework_os::Dwg*`/`dwg_*`, `semio_framework_plugin::Dwg*`/`dwg_*`), after all of the above edits landed:

**Real, load-bearing callers — none in my boundary, all previously identified by G2 or flagged as such by this mission:**

| File | Owner | Hits | Nature |
|---|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🦀️component.rs` | G1a | 2 | `dwg_encode_mesh_json` still calls `semio_framework::mesh_to_dwg_drawing`/`dwg_to_bytes` — G1a's own report (`📓️wave-g1a-osflow-report.md`) explains this is kept because a TS file dynamically imports the compiled wasm and calls it; not G1a's to remove until that TS side moves first |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️component.rs` | G1a | 18 | `export_dwg_sync`/`import_dwg_sync` and their whole call chain, kept for the same reason — a live plugin (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs`) calls through `export_dwg_json`/`import_dwg_json` |
| `🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs:196` | W3a | 1 | `export_dwg`/`import_dwg` call `semio_framework::mesh_to_dwg_drawing`/`dwg_drawing_to_mesh` directly — verified still present as instructed, **not edited**, reporting rather than touching W3a's file |

Both `flow` files' mtimes (`Aug 13 14:31:08`) and `mesh-io`'s (`Aug 12 20:22:52`) predate this session's edits, confirming these are current, not stale artefacts of an old grep.

**Minor residual glob-path references — plugin-owned, outside Job 2's specific registrant scope, left untouched deliberately:**

| File | Nature |
|---|---|
| `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🖼️media/🦀️component.rs:122-123` | test-only `semio_framework_os::DwgDrawing::default()` / `dwg_to_bytes(&drawing)` — reaches the old framework type through host's still-untouched crate-root `pub use semio_framework::*;` glob, not through anything Job 1/2 changed |
| `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:396,602,611` | `shooting_document_json_from_dwg(_drawing: &semio_framework_plugin::DwgDrawing)` — not registered anywhere (confirmed by the `register_dwg_import_handler` census), left on the old path by G2 "out of caution", still there |

These didn't need fixing for Job 1/2's own correctness (neither is a `register_dwg_import_handler` registrant, neither is called by anything I changed), and fixing them wouldn't unlock Job 3 by itself — the `flow`/`mesh-io` blockers above are the binding constraint. Flagging them now so a future wave doesn't have to re-discover them.

**Conclusion: at least 21 real call sites in 3 files outside my boundary still reference the framework module. Per the mission's own instruction ("If even one live caller remains, do NOT delete — report it"), `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs`, its `pub mod mesh;` mount, and its `pub use mesh::{…};` re-export block in `🧰️framework/📦️packages/🦀️rust/📦️glue.rs:21-22,59-62` are all left completely untouched.** Same bounded-duplication state G2 left it in — the codec now exists verbatim in both the old framework location and the new stdio location — narrowed by exactly the plugin-layer consumers this wave closed off (cad/gis/puzzle2d/animate/host), widened by nothing.

## 6. `sharedFileRequests` — unchanged from G2, still open

1. **G1a** — `flow/🌉️wasm`'s `dwg_encode_mesh_json` and `flow/🖍️drawing`'s `export_dwg_sync`/`import_dwg_sync` still need their downstream (TS wasm import, `flow/🧩️extensions/🖍️draw` plugin) repointed before these framework-side functions can be removed.
2. **W3a** — `📐️brep/📦️mesh-io`'s `export_dwg`/`import_dwg` still call `semio_framework::{mesh_to_dwg_drawing, dwg_drawing_to_mesh}` directly.
3. **Whoever owns `💻️os/🦀️component.rs` + this wave's own `🖥️host/🦀️component.rs`** (now moot for `🖥️host` — it's repointed) — no longer a blocker, since `🖥️host` is the only live one and it's now fixed. `💻️os/🦀️component.rs` remains genuinely dead and was not touched, per instruction.
4. **New, minor**: `🪐️space`'s media command test code and `🎥️shooting`'s schema file still reach the old framework `Dwg*` types via the `semio_framework_os`/`semio_framework_plugin` glob re-exports — neither blocks anything today, but both will need a one-line repoint (mirroring this wave's cad/gis/puzzle2d pattern) before the framework module can be deleted.

## 7. Files touched this wave

Edited:
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml` (+`semio-s-plugin-stdio` dependency)
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` (`media_export_raster` import + 8 call sites in the module + 2 call sites in `workflow`'s test module)
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🦀️component.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`

Not edited (Job 3 blocked, see §5): `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` and its mount/re-export in `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`.

Not edited (out of my boundary, see §5): `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🦀️component.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖍️drawing/🦀️component.rs`, `🧰️framework/🔨️modules/🧊️3d/📐️brep/📦️mesh-io/🦀️component.rs`, `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/🎮️commands/🖼️media/🦀️component.rs`, `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`, `🧰️framework/🛍️products/💻️os/🦀️component.rs` (dead, confirmed).

Created then immediately deleted (typo, caught before use — see §3's stray-file incident): `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️标准` (wrong CJK glyphs instead of `🏅️standards`; empty file, never written into, removed with `rm -f`).

## 8. Honest remainders

- `semio-framework-os --lib --features os-host-full` has 107 pre-existing, DWG-unrelated compile errors (broken `store::test_support` helpers, `LocalizedLabel::From<&str>`, `PluginManifest` field, `ArtifactPack::encode_pack` return type) — not caused by this wave, not fixed by this wave, blocks running the DWG round-trip tests that live inside `workflow`'s `mod tests` under that feature. DWG-region lines type-check cleanly under this feature (checked explicitly, zero errors touch any line I edited); only the *test-execution* of `svg_to_dwg_round_trip_produces_a_polyline`/`mesh_dwg_registrar_round_trips_a_box` is blocked, not their compilation.
- `semio-s-plugin-demonstrator` could not get a green `cargo check` — blocked entirely by `semio-s-plugin-procedural`'s 93 pre-existing, DWG-unrelated errors (flow/widget mutation vocabulary mid-decomposition, SMO's territory). Demonstrator's own DWG wiring needed zero edits and is indirectly verified via cad's and gis's own green, passing test suites.
- `semio-s-plugin-puzzle` has 3 pre-existing, DWG-unrelated failures (a `"camera"` field round-trip mismatch in mutation tests) — attributed, not touched.
- Job 3 (module deletion) remains blocked, same conclusion as G2, now narrowed to exactly 3 files / 21 hits, all outside this wave's boundary (G1a ×2 files, W3a ×1 file), plus 2 minor unblocking residuals (space, shooting) that don't gate anything today but will need a one-line fix each before deletion is possible.
