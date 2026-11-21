# FEM2D editor_app_2 fixture audit

Read-only source review; no Cargo invocation. The target includes nine command test modules and the window-config ownership suite, independently of the 163-error `editor_app` build. The earlier classification's shared context work applies here, but did not cover these modules.

`patch-section/🧪️tests/🔬️unit/🦀️.rs` and `patch-support/🧪️tests/🔬️unit/🦀️.rs` use `super::*` as though mounted in their command owners; in this integration binary the parent is `app-2`, which supplies no `handle`, `ConfigView`, or `protocol` imports. Support also uses `crate::FemDof` although the integration root does not define it. Section's replay calls `central_apply::apply_fem2d_mutation` with a persistent snapshot; that API accepts `Fem2dSnapshotValue`, so use cold reference rows and current mutation laws. These are source-level compile candidates, not observed compiler diagnostics.

Region, section and support helpers parse `Fem2dSnapshot` and drop it raw, including temporary `&demo()` calls; `set-active-example` likewise creates empty and decoded owned snapshots without retirement. They need the existing retiring guard or explicit controlled retirement. A failed assertion before retirement also needs guarded cleanup. Verify other fixture-owned snapshots as these suites are brought up.

The browser-relevant coverage at risk includes inspector edits of region/section/support, changing examples, deletion, camera movement, playback and same-kind window isolation. Existing `editor_boot` covers one real example-load and node-edit route, while session, configuration and results-owner suites provide partial underlying evidence. These fixture defects do not establish a production browser regression; execute current app and assertions before drawing that conclusion.

## Fixture repairs

The four integration files under `🎮️commands/{🩹️patch-section,🩹️patch-support,🩹️patch-region,📚️set-active-example}/🧪️tests/🔬️unit/🦀️.rs` now import their own public typed handlers and borrow the mounted snapshot for inspection. Direct lowering uses an admitted snapshot wrapped in `Fem2dSnapshotRetiring`. The three patch replay laws use cold reference rows for application and inversion, preserving the assertions about independent fields, replay order, and untouched siblings. Their mounted assertions operate on the editor's bundled starting document.

The example tests retire their input and decoded Pack snapshots through the same guard. The unknown-example law starts on the bundled demo and compares the emitted document against an independently admitted empty snapshot. The no-document-mutation, load effect, action-kind, and example-option checks remain.

No runtime sources, Cargo manifests, or other app_2 law files changed. Native type checking and execution await the shared build lane.

## Current compiler boundary

The 14 diagnostics from the current `editor_app_2` build involved retained view inputs, guarded snapshot borrows, one sealed material ID, the window pack interface, and the async query depth. The integration root now allows depth 256 for its generated test futures. The command repairs cover result animation ticks, example selection, section replay, and region deletion through selection.

The window law still checks the exact Pack and SPR after reopening, same-kind sibling isolation, typed camera and result settings, and preservation across example load. It reads bytes through the retained pack source, restores each addressed pack through the bounded testkit, and closes both mounted apps even when an observation returns an error. The source check compares the command's canonical load-case identity, and example replacement checks effects from both invocation and typed settlement.

The scoped whitespace and Rust syntax checks passed across all six edited test files. Native compile and execution of the repaired target remain pending.
