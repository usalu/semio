# 📏️ Wave W0-G — wgpu measures home

Lane: **W0-G wgpu measures home** (contract §0.7, §3.6 rows 3–4, §5 W0-G, §6 item 7h).

## 1. Claims verified before changing anything

| Claim | Verified |
|---|---|
| Native bridge `window_measures` returns an empty map | Yes — `wasm_program_exchange::window_measures` was `Ok(HashMap::new())` with a doc saying v12 has no measure record. |
| Web wgpu Shell stores `window_measures` and never reads it | Yes — only the field declaration, the initializer and the assignment in `refresh_ui` referenced it. The JS backend (`window_measures_js`) called a `windowMeasures` handle function that `🐚️plugin-bridge/🟦️.ts` never implements, so it was empty on the web too. |
| `render_window_measure_*` block has no caller | Yes — zero callers repo-wide (the block plus the `MEASURE_PROGRESS_*` consts it alone used). |
| Deeper finding | A v12 wire home already existed guest-side: `plugin_render_section` publishes `window_measures(view_state)` as canonical JSON on the reserved `framework.section.measures` retained surface (paged-text carrier), exactly like the catalogue. Nothing on wgpu ever read it. The Shell's `shell.measures.fold/unfold/focus/resize.*` handlers also existed with no emitter and no painted overlay. |

## 2. What changed

### Contract (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs`) — the retained record home
- `//#region 📜️PagedText` (~814-856): `UiDocumentLease::read_paged_text()` — depth-first reassembly of a paged-text carrier through one borrowed `try_read` (no per-page clone), index-by-id, cycle and dangling-child refusal, contended-lock retry. It replaces `interpreter::read_paged_text_document` (deleted, see §6).
- `//#region 🧾️RightSizedPublication` (~858-960): `UiDocumentLease::try_publish(surface, identity, nodes)` — the stepped assembly with a resident permit priced by the record count (the cold `UiDocumentBuilder` reserves the 8 MiB surface ceiling, and the aggregate holds four). It retires its assembly on refusal and retries contended arena steps. The browser bridge's own copy of this loop was folded into it.
- Tests: `📃️document/🧪️tests/📜️paged-text/🦀️.rs` over the fixture `🧬️contract/🧫️fixtures/📜️paged-text.json` (serde_json oracle).

### ProgramBridge (`…/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`)
- Deleted the native `window_measures` stub, `window_measures_js`, and the `ProgramBridgeEntry::window_measures` dispatcher.
- New `ProgramBridgeEntry::window_measures_section` (~580): publishes `framework.section.measures` through `render_with_document`, the same path on both backends.
- New `window_measures_from_section` (~650).
- The JS `render_with_document_js` now calls `UiDocumentLease::try_publish`. Deleted `assemble_browser_document`, `retire_browser_assembly` and `BROWSER_DOCUMENT_ASSEMBLY_BYTES`. `resident_refusal` became `retained_publication_refusal` (~898, not wasm-gated, shared with the Shell).
- Test: `🌉️ProgramBridge/🧪️tests/📏️wgpu-window-measures-section/🦀️.rs`. It builds the section with the REAL producer `semio_framework_plugin::app::paged_text_carrier`, and serde_json is the oracle.

### Shell (`…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`)
- Removed the field `window_measures`. Added `window_measures_documents: HashMap<String, UiDocumentLease>` and `window_measures_minted: HashMap<String, (u64, u64)>` (~2638-2642).
- `//#region 📏️WindowMeasures` (~2846-3140):
  - `window_measures_surface_id` / `window_measures_owner`: the surface key is `{windowId}/framework.section.measures` (React `windowMeasureDomId` rule).
  - `window_measures_overlay_records`: partitions with `partition_window_measures` (general measures only), then projects:
    - Number → `Input{kind:number, commit:"blur"}` bound on Commit
    - Slider and Select → bound on Change
    - Toggle → `Toggle` bound on Change, with React's `pressed:!pressed` in the authored args
    - Group → `Section` with an optional header slider and its children
    - label → `Field`
    - loading/waiting → `Activity`
  - `refresh_window_measures`: reads the section once per refresh, then retires and republishes each live window's overlay. Faults are recorded as surface faults.
  - `publish_window_measures`: the engine ingress-generation rule, keyed on a content hash, so an unchanged overlay pays no ingress.
  - `paint_window_measures_step`: backdrop, the same stepped `render_ui_document_step` window bodies use, a border, and hit registration above the body.
  - `window_measures_rect`: right-aligned, at `measures_width` or the theme default, inset.
- `refresh_ui` (~4223) calls `refresh_window_measures`. `render_main_window_step` gains phase `8` (overlay) after each body.
- Input routing:
  - `dispatch_action` (~5441) rewrites a measures-surface `windowId` to its owning window.
  - `route_retained_pointer_press` (~7293) activates the owning window and clears the sibling surface's content focus.
  - Keyboard content-focus routing (~8797) prefers the focused Measures overlay of the active window.
- `push_chrome_group_border` is now imported unconditionally (it was a `cfg(test)` import).
- Test: `🐚️Shell/🧪️tests/📏️wgpu-window-measures/🦀️.rs`:
  - The projection fixture is compared as contract wire JSON.
  - The overlay is painted through the real stepped paint, and hit targets are asserted.
  - Keyboard: Tab, Backspace×2, "40", Enter → `setFillCount {value:40, windowId:"puzzle3d-main"}`.
  - Pointer: press the toggle → `setGridVisible {pressed:false, value:false, windowId}`.
- Language-agnostic fixture: `🧑‍🎨engine/🧫️fixtures/📏️window-measures/🔣️.json`. The expected outline was produced by an independent Python spec of the projection rules.

### Widgets (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs`)
- Deleted `//#region 🔖️WindowMeasureBorrowedControls` and the `MEASURE_PROGRESS_*` consts (−129 lines). No compat.

## 3. Public API as landed
```rust
// ui_contract
pub enum UiPagedTextError { Lease(UiDocumentLeaseError), MissingNode(UiNodeId), Cyclic }
pub const UI_DOCUMENT_PUBLISH_STEP_BYTES: usize;
pub enum UiDocumentPublishError { Empty, Permit { fault: UiResidentFault, items: usize, bytes: usize }, Assembly(UiDocumentAssemblyError), Budget }
impl UiDocumentLease {
    pub fn read_paged_text(&self) -> Result<String, UiPagedTextError>;
    pub fn try_publish(surface: SurfaceId, identity: UiDocumentAssemblyIdentity, nodes: Vec<UiNodeRecord>) -> Result<Self, UiDocumentPublishError>;
}
// renderer crate::program_bridge
impl ProgramBridgeEntry { pub async fn window_measures_section(&self, instance_id: u32, view_state: &ViewModel) -> Result<UiDocumentLease, String>; }
pub fn window_measures_from_section(document: &UiDocumentLease) -> Result<HashMap<String, Vec<WindowMeasure>>, String>;
pub(crate) fn retained_publication_refusal(body_key: &str, error: ui_contract::UiDocumentPublishError) -> String;
// renderer crate::shell
pub(crate) fn window_measures_surface_id(window_id: &str) -> String;
pub(crate) fn window_measures_owner(surface_id: &str) -> Option<&str>;
pub(crate) fn window_measures_overlay_records(window_id: &str, measures: &[WindowMeasure], active_utility_id: Option<&str>) -> Result<Option<Vec<ui_contract::UiNodeRecord>>, String>;
```

## 4. Tests run (foreground; outputs in `T/🗑️generated/W0-G/`)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-ui-contract --lib paged_text` | **3 passed, 0 failed** (`contract-paged-text-test.txt`) |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- window_measures app_catalogue` (3 consecutive runs) | **10 passed, 0 failed** each: 2 bridge, 3 shell measures, catalogue reader tests |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- shell:: dock:: program_bridge:: interpreter:: --test-threads=1` | 287 passed, **10 failed** — see §7 (all outside the measures code). `dock::tests::utility_options_partition_gates_tagged_group_by_active_utility` passes. |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib --test-threads=1` | 411 passed, 7 failed — see §7 (process-arena, permit, timing and slot-table budget laws; none are layout-geometry laws). |
| `cargo check -p semio-framework-ui --features wgpu-engine` | exit 0, no warning in touched files |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` | **NOT reached**: `semio-framework-plugin` fails upstream with 7× `(dyn InteractiveJob) cannot be sent between threads` (W0-D's in-flight `🔌️plugin/🦀️.rs`). Retried for ~45 min. My wasm-relevant change is the `render_with_document_js` → `try_publish` rewrite, which is unverified on wasm32. |
| Final native rerun after the last edits: `cargo test -p semio-framework-os-renderer-wgpu --lib -- window_measures app_catalogue dock::` | 53 passed, 1 failed (`dock_stack_content_fills_full_bounds_through_one_silhouette_clip`, see §7). All 5 measures tests, 5 catalogue tests and every other dock test pass (`renderer-measures-test.txt`). |

Type-check proof: the final renderer lib-test build reached warnings for the crate, and none are in the touched files.

## 5. Commands to register in launch.json
- `cargo test -p semio-framework-os-renderer-wgpu --lib -- window_measures` (wgpu measures home).
- `cargo test -p semio-framework-ui-contract --lib paged_text`.
- `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` (if not already registered).

## 6. Deviations and foreign edits

**Deviations**
- The "retained contract record" is the existing reserved `framework.section.measures` paged-text surface, read through the new contract API. No new wire type was minted, because the guest already publishes there and a second encoding would collide.
- The Measures overlay is unfolded by default on wgpu. React folds it by default, but no wgpu fold/resize chrome is painted yet; the existing `shell.measures.fold/unfold` handlers and `measures_width` are honoured.
- Number is projected as a typed number `Input` (Enter commits `value`), not a +/− stepper. The retained router has no keyboard commit for `NumberStepper`, and keyboard operability was required.

**Foreign edits**
1. `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs` (+5): new `LayoutNodeKind::Control { height: theme.control_height }` for Input/Select/Toggle/Slider/NumberStepper/Button/Ring/IconSelect. Controls previously had intrinsic height 0, so every control inside a `Section` was laid out 0 px tall and never hit-registered. That was a general engine gap which made grouped measures invisible and inoperable.
2. `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`: deleted `read_paged_text_document` (moved to contract).
3. `🗣️Interpreter/🧪️tests/🛍️wgpu-app-catalogue/🦀️.rs`: calls `document.read_paged_text()`.
4. `🧬️contract/🧪️tests/🛍️catalogue-carrier-map/🦀️.rs`: docstring reference only.
5. The Shell catalogue call site now uses `read_paged_text`.

## 7. Open items
- Failing renderer tests, none touching measures code:
  - `dock_stack_content_fills_full_bounds_through_one_silhouette_clip` (scissor count 2≠3)
  - `window_silhouette_border_emits_notched_outline_segments`
  - `panel_anchor_model` ×2 (prefs store)
  - `directory_home_bootstrap_…`
  - `shell_document_retirement` ×2 (fixed-ceiling `UiDocumentBuilder` ArenaFull, also fails in isolation)
  - `render_plan_validator` svg/raster ×3 (process raster credits)

  I could not produce a pre-change baseline without modifying git state. The coordinator should compare them against a peer-free run.
- Failing ui crate tests: `ui_surface_slot_table…` (element bytes +32, consistent with W0-C's `UiNode::Progress`), prepared permits, arena ArenaFull, and one 8 ms timing law.
- The wasm32 check of the renderer must be re-run once `semio-framework-plugin` compiles for wasm32.
- Not painted yet on wgpu:
  - Utility Options rail (utility-tagged groups): the partition's `utility_options` bucket.
  - Measures fold/resize chrome.
  - `ready` extent bar.
- Retained router gaps (`⚡️events`, not this lane):
  - No Space/Enter keyboard commit for Toggle, Slider or Select; the Number measure is keyboard-operable today.
  - No chrome-level keyboard entry into a window's content (Tab cycles windows).
- `window_engagements` is still an empty stub on both backends. Its surface `framework.section.engagements` could reuse `read_paged_text` the same way.
- §6 item 7h browser evidence (count measure visible on the wgpu shell) is not captured. It needs a live deploy after the peer breakage clears.
