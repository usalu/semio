# 📶️ Wave W0-C: UI contract `Progress`

Lane W0-C of `📋️tool-run-contract.md` §5. Status: **landed**. `Component::Progress` works end to end on
the contract, wgpu (reconcile, paint, accessibility projection, browser ARIA mirror) and React (Interpreter).
`WindowMeasure::Progress`, `MeasureProgressStep`, `MeasureProgressStepKind` and `measure_progress_cancel_label`
are removed. No compatibility layer was added.

Paths below are relative to `🧰️framework/` unless they start with `✏️s/` or `T/`.

## 1. What changed

### Contract (`🔨️modules/🖱️ui/🧬️contract/`, crate `semio-framework-ui-contract`)

- `🧩️component/🦀️.rs:382-411`
  - Adds `ProgressProps`, its `is_determinate()` method and the free function `progress_fraction`.
  - Adds the variant `Component::Progress` (`:539`) and its `credited_clone` arm.
- `♿️accessibility/🦀️.rs`
  - Role `progressbar` (`:83`); a progress bar is not focusable.
  - Adds `AccessibilityValue` and `accessibility_value` (`:189-208`).
  - `AccessibilityProjectionNode` gains `value_min`, `value_max`, `value_now`, `value_text` and `busy` (`:174-183`).
  - Determinate bars fill min/max/now/valuetext. Indeterminate bars set only `busy: true`.
- `🛡️limits/🦀️.rs`
  - Text bytes count `value_text` (`:152`).
  - `completed` and `total` must be finite, otherwise the node gets a `NonFiniteNumber` violation (`:169`).
- `🧬️schema/🦀️.rs`
  - `ProgressProps` TypeScript metadata (`:555-563`) and the updated `Component` union (`:263`).
  - The `typegen_export` count goes from 79 to 80.
- `🏗️builder/🦀️.rs:1319-1360`: new `ProgressBuilder`, `progress(completed, value_text)` and `.total(total)`.
- Field roster and exhaustive variant lists:
  - `🧾️typed/🦀️.rs`: `ProgressProps { 0 completed, 1 total, 2 value_text }`.
  - `🪞️copy/🦀️.rs`, `⚖️compare/🦀️.rs` and `♻️retirement/🌳️typed/🧱️component/🦀️.rs`: the new variant.
- TypeScript twins:
  - `♿️accessibility/🟦️.ts`: role, plus `uiAccessibilityValueV1` and `uiProgressFractionV1`; the projection now spreads the value.
  - `🧵️retained/📦️wire/🧾️typed/🟦️.ts`: a `progress` decode case.
  - `🧵️retained/🛡️validation/🔬️graph/🟦️.ts`: finiteness check.
  - `🧵️retained/🟦️.ts`: text accounting.
- Fixtures and schemas, hand-edited from 18 to 19 components:
  - `♻️retirement/🌳️typed/{🧩️components.json,🧫️fixtures/🔣️.json,🧬️schema/🔣️.json}`
  - `🪞️copy/{🧫️fixtures,🧬️schema}`
  - `⚖️compare/{🧫️fixtures,🧬️schema}`
  - `⚖️compare/📃️document/{🧫️fixtures,🧬️schema}`
  - `♻️retirement/🩹️patch/📨️pending/📦️whole/{🧫️fixtures,🧬️schema}`
  - `🧵️retained/📦️wire/{🧬️schema/🔣️.json,🧫️fixtures/🧾️typed/🔣️.json}`: one new component row and one new hostile row.
- New conformance case `🧫️fixtures/🧪️conformance/🧩️component/📶️progress/{📸️snapshot,🎯️expect}.json`. It is a group holding one determinate and one indeterminate bar.
  - The catalog now has 63 cases.
  - Counts were updated in `🧪️tests/🔬️conformance-unit/🦀️.rs`, `🧪️tests/🔬️conformance-corpus/🟦️.ts` and the Interpreter's `🧪️unknown-component-placeholder` test.
- **New language-neutral fixture `🧫️fixtures/📶️progress.json`.**
  - It holds 5 cases: determinate EN, determinate DE complete, overshoot, zero total, and indeterminate.
  - Each case pins the exact serde wire string, the fill `fraction`, and the accessibility role, focusable flag and value attributes.
- `🧫️fixtures/♿️accessibility-projection.json`
  - Adds 2 role rows (determinate and indeterminate).
  - Adds document nodes 7 (`#evaluation`, determinate) and 8 (`#preparation`, indeterminate), with the value attributes in their expected rows.
- Regenerated `🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` with `bun ./📜️script.ts generate` in `🧬️contract/📦️packages/🦀️rust`.

### UI runtime (`semio-framework-ui-runtime`)

- `🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs:1055`: the surface semantic census counts `value_text`.

### wgpu (`semio-framework-ui`)

- `🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`
  - Removes `MeasureProgressStepKind`, `MeasureProgressStep`, `WindowMeasure::Progress`, `measure_progress_cancel_label` and the `impl MeasureProgressStepKind`.
  - Adds `UiProgressNode` (`:2206`) and `UiNode::Progress` (`:3217`), with the presence and menu accessor arms.
- `🎯️targets/🧊️wgpu/🦀️.rs:242`: the removed names are gone from the re-export.
- `🔀️reconcile/🦀️.rs`
  - `progress_node` (`:686-690`) and the arm in `ui_node_from_record` (`:760`).
  - Test-only discriminant `20` and `explicit_id` arms.
- `🖌️paint/🦀️.rs`
  - Retained `paint_node_step` arm (`:957`) and test-only `paint_node_self` arm (`:1756`).
  - New `📶️Progress` region (`:1784-1817`): `PROGRESS_INDETERMINATE_SHARE`, `progress_bar_rects` and `paint_progress`.
  - It uses theme tokens only: a `separator` track and a `progress` fill. The track is `padding_standard` tall and vertically centred.
  - The fill width comes from `ui_contract::progress_fraction`. Indeterminate bars get a centred, **static** one-third sweep; see §5.

### React

- `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`
  - New `ProgressView` (`:1565`) renders a real `role="progressbar"`.
    - Determinate: `aria-valuemin`, `aria-valuemax`, `aria-valuenow` and `aria-valuetext`.
    - Indeterminate: `aria-busy`.
    - It also gets the record's `accessibilityAriaProps` (label and description).
  - The fill uses `uiProgressFractionV1`. The indeterminate sweep pulses only under `motion-safe:`.
  - The `case "progress"` arm is at `:1652`.
  - The busy shell never replaces a progress bar (`:1614`), because a progress bar already is the busy indicator.
- New test `…/🗣️Interpreter/🧪️tests/📶️progress/🟦️.tsx`, registered in the Interpreter's vitest block.

## 2. Public API as landed

```rust
// ui_contract
pub struct ProgressProps { pub completed: f64, pub total: Option<f64>, pub value_text: Label }
impl ProgressProps { pub fn is_determinate(&self) -> bool }
pub fn progress_fraction(completed: f64, total: Option<f64>) -> Option<f64>
pub enum Component { …, Progress(ProgressProps), … }            // wire: {"type":"progress","completed":12.0,"total":100.0,"valueText":"…"}; total omitted when None
pub struct AccessibilityValue { pub min: Option<f64>, pub max: Option<f64>, pub now: Option<f64>, pub text: Option<String>, pub busy: bool }
pub fn accessibility_value(component: &Component) -> AccessibilityValue
// AccessibilityProjectionNode += value_min, value_max, value_now: Option<f64>, value_text: Option<String>, busy: bool (camelCase, skipped when None/false)
pub struct ProgressBuilder; pub fn progress(completed: f64, value_text: Label) -> ProgressBuilder; impl ProgressBuilder { pub fn total(self, total: f64) -> Self }
// ui_wgpu (semio-framework-ui, feature wgpu)
pub struct UiProgressNode { pub id: String, pub completed: f64, pub total: Option<f64>, pub value_text: Label, pub presence: UiPresence, pub menu: Option<UiMenuRef> }
pub enum UiNode { …, Progress(UiProgressNode), … }
```

```ts
export type ProgressProps = { completed: number, total: number | null, valueText: Label };   // @semio-tech/framework
export function uiAccessibilityValueV1(component: Component): UiAccessibilityValueV1;         // 🧬️contract/♿️accessibility/🟦️.ts
export function uiProgressFractionV1(completed: number, total: number | null | undefined): number | null;
```

## 3. Tests run (foreground; logs in `T/🗑️generated/W0-C/`)

**Contract.** Command: `RUST_MIN_STACK=134217728 cargo nextest run -p semio-framework-ui-contract --all-features --no-fail-fast` (`contract-test-3.txt`).

- Result: 193 run, 169 passed, 24 failed.
- Every new or affected law passed:
  - `component::tests::progress_wire_shape_matches_the_serde_oracle_and_the_value_codec` (serde_json oracle and first-party value codec over `📶️progress.json`)
  - `progress_with_a_non_finite_number_is_rejected_by_validation`
  - `every_component_variant_round_trips`
  - `typed_wire_neutral_component_defaults_match_serde` (19 rows)
  - `accessibility::tests::every_component_implies_the_role_the_shared_fixture_declares` (19 components)
  - `every_published_record_projects_the_way_the_shared_fixture_declares`
  - `every_progress_case_announces_the_value_the_shared_fixture_declares`
  - `conformance::tests::*`, including the new `📶️progress` case and `every_component_variant_appears_in_the_corpus`
  - `typegen_export::exports_typescript_bindings` (passes after `generate`)
- **All 24 failures are the same pre-existing assertion.** It is `step.released_items <= 1 && step.released_bytes <= grant`, inside the shared retirement close loops.
  - It also fails in `action::binding_copy_tests::*`, which touch no `Component` at all.
  - Temporary `[DEBUG]` probes, reverted afterwards, showed:
    - `instance_lifetime_ui_typed_all_components_account_exact_payload_bytes` fails first on the `container` and `text` rows. The new `progress` row passes at grants 4096, 64 and 1, with bytes = 1 as the fixture says.
    - `retained_component_copy_all_variants_match_native_serde` copies the `progress` row with an exact serde match and closes it.
  - This looks like fallout from the retirement change in commit 78b716e661, which is outside this lane.

**wgpu.** Command: `RUST_MIN_STACK=134217728 cargo nextest run -p semio-framework-ui --features wgpu-engine --no-fail-fast` (`ui-test-1.txt`).

- Result: 418 run, 415 passed, 3 failed.
- New tests passed:
  - `wgpu::accessibility::tests::a_mounted_progress_bar_announces_its_value_or_busy_and_mounts_as_a_retained_progress_node`
  - `a_mounted_document_publishes_the_accessibility_tree_the_shared_fixture_declares`, which now covers nodes 7 and 8 and the value fields
  - `wgpu::paint::tests::a_progress_bar_fills_the_fraction_the_shared_fixture_declares`
  - `a_progress_bar_paints_its_track_and_fill_from_theme_tokens`
  - `wgpu::component::ui::ui_node_wire_format_tests::ui_progress_node_wire_format_round_trips`
  - The presence and menu exhaustiveness tests with the Progress rows
  - The layout wire-format and value round-trip suites after the Progress removal
- The 3 failures are not caused by this lane:
  - `ui_surface_slot_table_is_heap_first_and_fits_a_bounded_thread_stack`: `UiSurfaceSlot` 159928 bytes vs committed 159896.
    - A size probe (removed afterwards) showed `UiNode=3360`, `UiProgressNode=160`, `Component=3096`. The new types don't enlarge any stored enum.
    - `UiWindow` changed in peer commit ac22984a03 (20:45), after the fixture update.
  - `max_plus_one_stale_aba_…`: `ArenaFull` in a Separator-only hostile document builder.
  - `large_layout_and_shaping_job_keeps_every_observed_slice_below_eight_ms`: timing under load, 11 ms.

**UI runtime.** Command: `RUST_MIN_STACK=134217728 cargo nextest run -p semio-framework-ui-runtime --no-fail-fast` (`runtime-test.txt`).

- Result: 122 run, 120 passed, 2 failed. Both failures are the same retirement close-loop problem.
- A `[DEBUG]` probe (reverted) showed `runtime_tree_retirement_preserves_occupied_sources_and_closes_exact_payloads` failing on the first row, `container`.

**TypeScript twins.** Run with `bun -e` in `🧬️contract/📦️packages/🦀️rust`.

- `accessibilityProjectionSelfTests()`: 223 checks. They cover the progress rows, `📶️progress.json` value and fraction, and 19 components.
- `conformanceCorpusSelfTests()`: 63 cases.
- `fixedListStorageSelfTests()`: passes the 19-component `componentCount` check at line 49. It then fails at line 60 on a pre-existing `⚖️compare` `frame.pageCount` 434 vs 256 mismatch, which this lane did not touch.

**React Interpreter.** Command, in `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript`: `bun ./📜️script.ts test long "🗣️Interpreter"` (`interpreter-test-3.txt`).

- Result: **96/96 passed.**
- The 5 fixture cases check valuemin/max/now/valuetext/busy/tabindex and the fill width through jsdom `getByRole("progressbar", { name })`.
- One more test checks that a loading record is still a progressbar.
- The conformance corpus now loads 63 cases, including `🧩️component/progress`.
- `bun ./📜️script.ts test exhaustive "📃️UiDocumentStore/🟦️.tsx" -t "normalizes all native component variants"` passes 1/1: the 19-row typed wire with the Immer oracle (`typedwire-test.txt`).
- `bun ./📜️script.ts typecheck`: 791 errors, all pre-existing (`import.meta.dir`, db, …). None are in the new or changed lines (`typecheck-react.txt`).

**Compile gates.**

- `cargo check -p semio-framework-ui --target wasm32-wasip2 --features wgpu`: OK, and `semio-framework-ui` reached warnings (`ui-wasip2.txt`).
- `cargo check -p semio-framework-ui-contract -p semio-framework-ui-runtime --target wasm32-wasip2`: OK, and ui-runtime reached warnings (`contract-runtime-wasip2.txt`).
- `cargo check -p semio-framework-plugin --tests`: OK, lib test with 248 warnings (`plugin-tests-check.txt`).
- `cargo check -p semio-framework-os-renderer-wgpu` (`--tests`, and `--target wasm32-unknown-unknown`): **blocked** by the expected plugin breakage in `semio-s-artifact-puzzle-3d` (§6). So the foreign edit in its Interpreter wgpu target file is not compiler-verified yet.

TDD note: fixtures and tests were written before the implementation in each step. Red runs were not recorded separately; the whole wave was compiled once at the end.

## 4. Commands to register in launch.json

- `bun nx run @semio-tech/ui-contract-rs:generate` (already exists; must be re-run whenever `🧬️schema/🦀️.rs` changes)
- `bun nx run @semio-tech/ui-contract-rs:test`
- `cargo nextest run -p semio-framework-ui --features wgpu-engine` (the ui crate's own `📜️script.ts test` runs the `wgpu-engine` feature)
- `bun ./📜️script.ts test long "🗣️Interpreter"` in the renderer-react package (`@semio-tech/framework-renderer-react` target `test-long` with args `🗣️Interpreter`)

## 5. Deviations from the contract

1. **The wgpu indeterminate sweep is static, not animated.**
   - The wgpu CPU paint pass has no motion clock; only the GPU shader sees `time_seconds`. It also receives no reduced-motion signal.
   - A still, centred one-third band is correct under `prefers-reduced-motion: reduce` by construction.
   - React animates only under `motion-safe:`.
   - Animating wgpu needs a new shader instance kind plus a host reduced-motion flag in `Theme` (open item 3).
2. **`progress_fraction` is a free function, not a method.** Every renderer fills through one law: Rust wgpu calls it on `UiProgressNode` fields, and the TypeScript twin `uiProgressFractionV1` is tested on the same fixture.
3. The React busy shell never replaces a `progress` record.
4. `total: Some(0.0)` counts as determinate (`aria-valuemax=0`, empty fill), following §2.3 ("total absent = indeterminate").

## 6. Foreign edits (smallest possible)

- `🔨️modules/🛂️manifest/🟦️.ts` (W0-B): re-exports `ProgressProps` (2 lines).
- `🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`: regenerated only.
- `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` (W0-E): `UiNode::Progress` arms in `ui_node_kind_tag` (`"progress"`), `ui_node_declared_id` and the introspection visual fields (value text, `theme.progress` / `theme.separator`). Not compiler-verified yet; see §3.
- `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts` (unowned): the wgpu ARIA mirror copies `valueMin`/`valueMax`/`valueNow`/`valueText`/`busy` onto `aria-valuemin`/`max`/`now`/`valuetext`/`busy`, so the wgpu progressbar actually reaches assistive technology.
- `🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️world3d-host-unit/🦀️.rs`: dropped the `WindowMeasure::Progress { id, .. }` arm.
- `🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs`: census arm (exhaustive match).
- `🔨️modules/🖱️ui/🧬️contract/{🪞️copy,⚖️compare,🧾️typed,♻️retirement/…,🧵️retained/…}`, the fixtures and schemas in §1, and `📃️UiDocumentStore/🟦️.tsx`: new variant arms and 18→19 counts, all forced by the exhaustive variant.
- `🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs`: `UiNode::Progress` joins the existing "no immediate-mode widget" placeholder arm.

W0-G had already deleted the dead `render_window_measure_progress` block in `🪀️widgets/🦀️.rs`, so this lane did not touch it.

## 7. Open items / broken call sites for waves 1–2

**Broken plugin call sites.** These are expected; wave 1/2 replace them with `ToolRunDefinition` plus ticks and the framework panel's `Component::Progress`.

| Plugin | Call sites |
|---|---|
| **puzzle 3d fill** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` | `:12` (imports `MeasureProgressStep`, `MeasureProgressStepKind`), `:54-64` (`progress_steps`), `:79` (`WindowMeasure::Progress`) |
| **puzzle 3d brush** `…/🧊️3d/…/✏️edit/🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️.rs` | `:72` (`WindowMeasure::Progress`); doc reference at `:61` and a comment at `…/🪟️windows/🧊️main/🦀️.rs:755` |
| **puzzle 2d fill** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs` | `:10`, `:75-85`, `:100` |
| **puzzle 2d fill tests** `…/◻️2d/…/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` | `:60`, `:64`, `:71`, `:86`, `:90`, `:94` |
| **puzzle 5d fill** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/☑️options/🪣️fill/🦀️.rs` | `:17`, `:49-59`, `:75` |
| **puzzle 5d tests** `…/🖐️5d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | `:203-206`, `:920` |

Verified compiler errors so far: `semio-s-artifact-puzzle-3d` has E0432 at `fill:12`, E0599 at `fill:79` and E0599 at `brush:72`.

**Other open items.**

1. **W0-B** still carries `MeasureProgressStep(Kind)` and the `progress` measure arm in `🔨️modules/🧬️schema/📽️projection/🦀️.rs:870-884,1869` and in the generated manifest. The hand-mirrored legacy `UiNode` TypeScript union in the projection has no `progress` arm for the new `UiProgressNode`; W0-B should add it when regenerating `@semio-tech/framework-rs`.
2. The pre-existing retirement close-loop failures (24 in the contract crate, 2 in ui-runtime) mask the full 19-row roster suites. The progress row was checked separately via probes, as described in §3. Someone should triage commit 78b716e661 in its own lane.
3. An animated wgpu indeterminate sweep needs a shader instance kind plus a reduced-motion flag delivered from the host into `Theme` (deviation 1).
4. The committed boxed-fixed-slot budget `🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json:85` (159896) is stale against current `UiWindow` (159928). The growth came from a peer.
5. Re-run `cargo check -p semio-framework-os-renderer-wgpu --tests` once wave 1 fixes the puzzle plugins, to verify the Interpreter wgpu foreign edit.
