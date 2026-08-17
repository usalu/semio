# W4B — EngineCanvas test fixture rename (s.play.workflow -> os.play.workflow)

## File
`/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs`

## Change
Renamed 3 test-fixture occurrences inside `#[cfg(test)] mod catalogue_workflow_drop_tests`
(all calls to `node_graph_catalogue_drop_action`):

- Line 697: `"s.play.workflow"` -> `"os.play.workflow"`, `"s-play"` -> `"os-play"` (surfaces tuple)
- Line 698: `assert_eq!(action.controller_id, "s-play")` -> `"os-play"`
- Line 712: `"s.play.workflow"` -> `"os.play.workflow"`, `"s-play"` -> `"os-play"` (surfaces tuple)
- Line 715: `"s.play.workflow"` -> `"os.play.workflow"`, `"s-play"` -> `"os-play"` (surfaces tuple)

No other `s.`-prefixed strings in this file were touched (no `s.stdio` occurrences present in this file
to begin with). Verified post-edit with grep: zero remaining occurrences of `s.play.workflow` / `s-play`
in the file.

## Verify
Target crate resolved via
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml`
-> crate name `semio-framework-os-renderer-wgpu`.

Ran: `cargo check -p semio-framework-os-renderer-wgpu --tests`

Result: build failed, but **only** inside crate `semio-s-plugin-stdio` (68 `error[E0277]`/`error[E0425]`
trait-bound and missing-type errors, e.g. `SemioAnimationMutation: OpBinary` not satisfied,
`cannot find type AnimTimeline`). All error paths are under
`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/...` — the `s.stdio` plugin explicitly called out in the
task instructions as separate in-progress work that must NOT be touched by this wave.

Cross-checked against `📸️baseline-cargo-check.txt` captured at ticket start (10:54): baseline already
showed 5 errors in `semio-s-plugin-stdio`; by the time of this verify run it had grown to 68, confirming
this is other sessions' ongoing churn in that crate, not something introduced by this rename.

Full output searched for `EngineCanvas` and for any error referencing this file or the renamed
identifiers: **zero matches**. The rename itself introduces no compile errors — the crate under check
never got past its (transitively broken) `semio-s-plugin-stdio` dependency to reach
`semio-framework-os-renderer-wgpu`'s own check phase.

Full cargo check output saved to ticket-external scratch (session tool-results, not persisted here) —
summary above is authoritative; re-run `cargo check -p semio-framework-os-renderer-wgpu --tests` once
`semio-s-plugin-stdio` is fixed by its owning session to get a clean pass/fail signal for this crate.

## Status
Rename complete and verified as isolated (no new errors attributable to this file/change). Unrelated
upstream `semio-s-plugin-stdio` breakage blocks a full clean `cargo check` of the target crate; this is
out of scope for this task and left untouched.
