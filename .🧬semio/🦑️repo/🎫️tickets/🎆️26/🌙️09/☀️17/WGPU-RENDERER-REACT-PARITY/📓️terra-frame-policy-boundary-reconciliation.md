# Frame Policy Boundary Reconciliation

## Scope

This packet reconciles the recorded red frame-transaction policy run with the active validator and active source. It is read-only. No Cargo command or policy command was run while native work is active.

The recorded run is [`frame-transaction-policy-9.log`](🗑️generated/astra-runtime/frame-transaction-policy-9.log). Its source validator is [`📜️script.ts`](🔬️frame-transaction-policy/📜️script.ts), which invokes [`interactivityMountedFrameTransactionFailures`](../../../../../../📜️script.ts:11185).

## Conclusion

The red gate contains both invalid source scopes and live policy violations. Removing failing checks or accepting the current baseline would hide the live violations. Conversely, treating every emitted Shell, paint, and scene label as a product breach is unsound because several source slices have no valid end boundary and therefore cover nearly the remainder of their file.

The gate must stay red until both classes are resolved:

1. Repair the validator so a missing or reversed boundary is an explicit policy-scope failure, never a broad `slice(..., -1)` scan.
2. Repair the live bounded-work violations listed below, then bind the mutation tests to the current source.

## Invalid scope construction

The validator constructs source slices with `indexOf` and does not validate either index ([`📜️script.ts`](../../../../../../📜️script.ts:11221)). In JavaScript, `slice(start, -1)` returns from `start` to the final character, rather than an empty or failed range. `interactivityProductionSource` also removes `#[cfg(test)]` items before several of those slices are built ([`📜️script.ts`](../../../../../../📜️script.ts:12320)).

| Validator scope | Current marker result | Consequence |
| --- | --- | --- |
| `chromeGroupBoundary` ([`📜️script.ts`](../../../../../../📜️script.ts:11255)) | `struct WindowMeasuresRailOutcome` is absent from the live Shell source. | The retained-group scan includes nearly all following Shell code. Its `chrome_text`, `measure_text`, and `while` findings are not scoped to the retained group. |
| `shellChildren` ([`📜️script.ts`](../../../../../../📜️script.ts:11239)) | The old `fn render_navbar(` marker is absent; current production uses `fn render_navbar_step` at [Shell `🦀️.rs:24128`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:24128). | Navbar, tutorial, footer, and overlay checks inspect unrelated later regions. |
| `chromeTourBoundary` ([`📜️script.ts`](../../../../../../📜️script.ts:11257)) | Its `#[cfg(test)] fn render_navbar` end marker is absent from the production-filtered source. | Tour findings include later preferences and legacy test-oriented text. |
| `chromeBoundary` ([`📜️script.ts`](../../../../../../📜️script.ts:11233)) | It ends at `fn body_rect`, before the live `render_chrome_step` at [Shell `🦀️.rs:22541`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22541) and its child calls. | Required chrome-child evidence is excluded, while its early-struct checks give a misleading partial view. |
| `interactiveSyncBoundary` ([`📜️script.ts`](../../../../../../📜️script.ts:11247)) | It ends at `find_child_by_key`, a `#[cfg(test)]` function in the paint source. | The production-filtered boundary runs to the end of the paint file. |
| `paintNodeBoundary` ([`📜️script.ts`](../../../../../../📜️script.ts:11246)) | It ends at `paint_tree`, which is `#[cfg(test)]`. | The production-filtered boundary runs to the end of the paint file. |
| `sceneNodeBoundary` ([`📜️script.ts`](../../../../../../📜️script.ts:11248)) | It ends at `collect_scene_slots_node`, also `#[cfg(test)]`. | The production-filtered boundary runs to the end of the scene-slot file. |

These defects account for the broad labels in the recorded log concerning Shell chrome children, chrome group, dialog/tour/navbar/overlay text, interactive synchronization, retained paint, and scene-node ownership. Those labels are **invalid evidence**, not findings that those individual production bodies violate the policy.

Two narrower labels are also stale textual expectations rather than demonstrated contract failures:

- The deferred-boundary requirement expects `runtime.interaction = Some(interaction)` ([`📜️script.ts`](../../../../../../📜️script.ts:11339)), while active code returns ownership through `runtime.return_interaction(interaction)` in the retained deferred path ([renderer `🦀️.rs:10243`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:10243)).
- The maintenance-authority requirement embeds fully qualified `std::sync::atomic::Ordering` spellings ([`📜️script.ts`](../../../../../../📜️script.ts:11342)); the live owner uses the imported `Ordering` name. The current label does not establish a missing generation/release mechanism.

The retained implementations themselves are present under their current names: `render_retained_chrome_group_item_step` ([Shell `🦀️.rs:16297`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:16297)), `sync_interactive_state_node_step`, `paint_node_step_with_driver` ([paint `🦀️.rs:803`](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:803)), and `ScenePaintCursor` ([scene slots `🦀️.rs:38`](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📨️scene_slots/🦀️.rs:38)). The validator needs to inspect their actual bodies directly.

## Stale mutation needles

The log identifies eight mutations that no longer alter their selected source. Each is a stale test binding, not proof that the corresponding production invariant is absent:

| Logged mutation | Why it no longer binds |
| --- | --- |
| `select-whole-materialization` | The live Select arm is now guarded as `UiNode::Select(select) if node.state.open`, not the older exact arm. |
| `maintenance-authority-release-erasure` | The source imports `Ordering`; it no longer spells the fully qualified orderings embedded in the mutation. |
| `bulk-deferred-close` | `FrameDeferredCursor::close_step` now delegates one owner to `self.actions.close_step()` rather than directly popping an action. |
| `whole-nontext-paint` | The retained tree arm changed shape while keeping `retained_tree_node_step`. |
| `whole-shell-glyph-callee` | Shell text now calls `paint_retained_glyph_step_flowed` through `chrome_text_step`. |
| `ui-bypasses-host-storage-read` and `ui-bypasses-host-storage-write` | The current calls pass a local `max_bytes`, rather than the older literal. The underlying size contract is separately broken below. |
| `missing-paint-output-credit` | The old fixed literal was replaced by the live retained-output helper and item-dependent call sites. |

The mutation harness has a second limit while the baseline is red. It treats a mutation as detected whenever the validator returns any failure ([`🟦️.ts`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-frame-transaction/🟦️.ts:116)), so unrelated existing baseline failures mask whether a successfully applied mutation has its own targeted failure. Once the baseline is repaired, each mutation should assert a specific expected policy label rather than merely a non-empty failure list.

## Live bounded-work violations

The following are not caused by invalid slices. Their active source directly contains the construct forbidden by the gate.

| Violation | Evidence | Policy consequence | Confidence |
| --- | --- | --- | --- |
| Decode work can run repeatedly inside one transaction call without charging an iteration to `StepContext`. | [`renderer `🦀️.rs:12960`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12960) loops on `pump_renderer_asset_decode_step()` after only the initial fuel charge. | This is a real violation of the one-retained-work-unit rule. A clock-based deadline is not an item-credit bound, and an unavailable or non-advancing clock can let the loop consume successive decode owners in a single transaction. | High |
| Transaction arithmetic saturates instead of failing closed. | The decode deadline saturates at [`renderer `🦀️.rs:12836`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12836); the brush cursor saturates at [`renderer `🦀️.rs:13037`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:13037). | Saturation hides exhaustion and is explicitly forbidden for this ownership cursor/deadline boundary. | High |
| Worker-owned interaction access can panic inside the frame transaction. | [`renderer `🦀️.rs:12986`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12986) and [`renderer `🦀️.rs:13079`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:13079) use `expect`. | The source breaches the fail-closed policy. The prior `interaction_available` check makes the normal path plausible, but it is not a proof that a panic is unreachable under every future state change. | High for policy breach; medium for current runtime reachability |
| The frame job advances multiple units in loop-driven turns; the browser branch additionally drives it on the caller. | `ActiveFrameBuild::step` loops at [`frame job `🦀️.rs:411`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:411). The wasm branch uses `try_step_on_caller` and a deadline loop at [`frame job `🦀️.rs:639`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:639). | These are current deliberate implementations, not string-slice artifacts. `StepContext`/deadline checks may limit typical work, but they do not satisfy this gate's one retained owner per submission and no caller-drive rule. The recorded two-versus-three take/resume count follows from this branch shape. | High |
| Native preference I/O calls a one-page service with an inadmissible 64 KiB ceiling. | Shell selects `OS_SHELL_CONFIG_MAX_BYTES` at [`Shell `🦀️.rs:26667`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:26667) and [`Shell `🦀️.rs:26675`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:26675). The service rejects every maximum above 16 KiB at [`services `🦀️.rs:1668`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs:1668) and [`services `🦀️.rs:1690`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs:1690). | This is an actual contract mismatch: native reads and writes of `semio.os.config` fail rather than being one admitted page. It also explains the live `OS_SHELL_CONFIG_STORAGE_KEY` boundary finding. | High |

The shell maintenance scheduler is not itself disproved by its stale label: it does select a single pending category in `advance_chrome_maintenance_step` ([Shell `🦀️.rs:22947`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22947)). The 64 KiB preference document path remains a real incompatibility with the declared 16 KiB host-page authority.

## Required repair and validation order

1. Preserve every current rule. Add a checked boundary helper to the validator: it must require that start and end markers both exist and that end follows start. An invalid scope emits one explicit `P5a policy boundary is stale` failure and skips dependent substring assertions. This prevents invented product diagnoses while remaining fail-closed.
2. Replace scopes that end at missing or test-only symbols with direct production function/impl body extraction. The current live anchors are `render_chrome_step`, `render_*_step`, `render_retained_chrome_group_item_step_with_trailing`, `sync_interactive_state_node_step`, `paint_node_step_with_driver`, and the `ScenePaintCursor` implementation. Function-body extraction should balance braces rather than infer a body from an unrelated later name.
3. Repair the live source before declaring the baseline green: remove the uncharged decode loop; use checked arithmetic with explicit terminal/refusal behavior; replace transaction `expect` calls with fault/terminal handling; make browser frame driving one submitted retained turn; and bring the native preference document under a real fixed-page or explicitly retained multi-page authority. Do not change the gate to permit these constructs.
4. Rebind the eight mutations to active, production-visible source text. For every mutation, assert the expected named failure appears, so unrelated failures cannot mask a non-detecting mutation.
5. Run the existing Bun/Nx ticket policy target after the changes. A green result requires an empty clean baseline and every mutation to add its assigned failure. It does not require Cargo.

No command was run for this audit; the cited red result is the pre-existing ticket log, and active sources may continue to change concurrently.

## Existing parser and causal mutation proof

The repository already has the needed comment- and literal-aware Rust block scanner: `toolJobRustBlock` in [`📜️script.ts`](../../../../../../📜️script.ts:845). Given the opening brace it returns the exact body and `undefined` when no matching brace exists. The policy-named `policyExtractFnBody` is also available at [`📜️script.ts`](../../../../../../📜️script.ts:16137), but it returns the trailing source for an unclosed block, so it is not sufficient on its own for a fail-closed boundary gate.

The frame validator should add a local checked wrapper around `toolJobRustBlock`, with these preconditions:

1. The production-normalized source contains exactly one declared function or impl header for the target.
2. The header has an opening brace after it.
3. `toolJobRustBlock` returns a matching close.

Any failed precondition must add one explicit stale-scope label and make the dependent check inapplicable for that run. It must never substitute a whole-file scan. This reuses the established parser; it does not introduce a new textual slicing convention.

For mutation proof, the tuple must include its expected label. First require that the clean baseline has no failures. Then, for each changed mutant, require that its own expected label appears in the mutant result. The current `failures.length === 0` predicate only says that *some* failure survived; it cannot prove causal detection when an unrelated failure is already present.
