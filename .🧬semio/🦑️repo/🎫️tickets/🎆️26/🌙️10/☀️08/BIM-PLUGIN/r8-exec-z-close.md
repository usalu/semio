# r8 exec `z-close` — window-config close stall: cleanup, root cause, fix

`FW` = `🧰️framework/🛍️products/💻️os/🔨️modules`, `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.

## 1. Cleanup (u-editor's temporary debug code)
* `FW/🏪️store/🦀️.rs`: removed all 6 u-editor `[DEBUG]` blocks (`take_next_owner`, cursor funding, envelope child step, cursor tail, `close_owned_store_step` tail, the `let r = …; r` wrapper restored to the plain tail call). The pre-existing `deferred reprojection` `[DEBUG]` (line ~23035, already in 674) stays. The non-debug store changes since 674 (displaced queue pops a terminal owner by its frame size, `next_close_byte_demand(&store)`) were kept.
* `FW/🔌️plugin/🪟️window/🎚️config/🦀️.rs`: both `[DEBUG]` lines removed; file was byte-identical to 674 before the fix below.
* `S/✏️editor/🧪️tests/🔬️unit/🦀️.rs`: the four `zz_debug_close*` tests removed. `grep` of the BIM plugin finds no `[DEBUG]`; the only other `zz_debug_*` is `zz_debug_timing` in `S/🧬️schema/💡️inferences/🕸️model-graph/🧪️tests/🔬️unit/🦀️.rs` (z-graph's, not touched).
* Untouched on purpose: the env-gated `SEMIO_DEBUG_CLOSE_PHASE` `[DEBUG]` diagnostics in `FW/🔌️plugin/🦀️.rs` (already in 674).

## 2. Root cause (framework bug, not BIM)
The stall is not BIM-specific: the framework's own toy test `tool_run_window_settings_reads_follow_the_starting_window_only` (publishes a window config, then closes) failed with the same `registered fixture did not reach its exact terminal-empty witness`. Temporary `[DEBUG]` prints (removed again) showed the partition's last line `Pending { 0, 0 } bytes=4096 demand=23040`.

* Every `ArtifactStore` carries resident backings that are released as ONE physical allocation each (the 1 024-slot snapshot-read registry alone is 32 768 B, the displaced queue / history pages 8–23 KB). `close_owned_store_step` refuses (`Pending {0,0}`) until the grant covers the whole allocation; `ArtifactStore::next_close_byte_demand` publishes that demand.
* `close_registered_fixture_app` (and the runtime) grant `max(app.next_close_byte_demand(), 4096)`. The app's `next_close_byte_demand` priced stages 0, 1, 2, 7 (document, config, draft, interaction store) but answered `1` for stage 5 (window-config registry): its partitions' stores were never asked. A partition store therefore got the ordinary 4 096 B page for ever.
* BIM was only the first to see it: every window instance that is rendered/addressed creates a partition; "close without a window" and "render a body without a window instance" never create one, matching the bisect of u-viewer/u-editor. Nothing is held beyond a lease by BIM (the thread-local render memo holds document clones, not window-config state).

## 3. Fix (minimal, framework)
* `FW/🔌️plugin/🪟️window/🎚️config/🦀️.rs`: `ErasedWindowConfigStoreOwner::next_close_byte_demand` (first partition store's demand, else the opened actor retirement's) and `WindowConfigOwnerRegistry::next_close_byte_demand` (parked loads pay their own grant, then the first owner), mirroring exactly which partition/owner `close_step` serves next.
* `FW/🔌️plugin/🦀️.rs`: `next_close_byte_demand` stage `5 => self.window_config_store.next_close_byte_demand()`.
* New law `a_published_window_config_closes_through_its_own_published_physical_demand` in `FW/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs` (publish a window config on the toy app, close through the exact ladder; failed before, passes now).
* Window-transient store (stage 6) is a `TransientStore` with whole-grant disposer and `AwaitingInput`, not affected.

## 4. BIM-side changes needed to compile/test (all small)
* `S/✏️editor/🫧️transient/🦀️.rs`: `transient_root!` now takes `diff:` and `fields:`.
* `render/🪟️window-config` `bim_window_config!`: the whole-record arm (needed the removed `store::impl_whole_record_config!`) is gone; only the sparse-diff form remains; `assert_window_config_laws` always checks `between`. Editor kit `window_config!` (`type Config, Diff, Mutation, Owner;`) now emits the sparse form with `fields:` from its field table; the three editor window configs name their `…ConfigDiff`. (z-baseline adapted the three editor config tests to it.)
* `📦️packages/🦀️rust/Cargo.toml`: dev-dependency `semio-framework-os-kernel` feature `mutation-testing` (derive tests use `mutation_fixture_ops`; needed since the per-artifact workspaces).
* Viewer test `a_window_addressed_viewer_closes_to_its_terminal_empty_witness` un-ignored; the `close_or_leak` workaround removed (persistence test now closes through `close_registered_fixture_app`).

## 5. Verification (all through `🚦️gate.sh z-close`)
| Command | Result |
|---|---|
| `cargo test --manifest-path Cargo.toml -p semio-framework-plugin --lib -- tool_run_window_settings_reads_follow` (before fix) | FAILED (30 s close deadline) |
| same filter family `tool_run_`, after fix | window tests pass; 35 passed / 7 failed (see 6) |
| `-p semio-framework-plugin --lib -- a_published_window_config member_run_abort` | new law ok |
| BIM `--lib -- editor:: viewer:: render::` | 282 passed, 0 failed, 0 ignored |
| BIM `--lib` (whole crate) | **3688 passed, 0 failed, 0 ignored** (was 3677/9/1) |

## 6. Open / not verified
* Drawing's window-config close law could NOT be run: the drawing graph does not compile at the moment (`semio-s-artifact-stdio-semio` diff: `NamedAdded`, `at` field, in-flight DIFF-ONLY migration, not mine). The framework toy law above covers the same mechanism.
* 7 framework `tool_run_tests` still fail and are unrelated to window configs (no partition involved; `SEMIO_DEBUG_CLOSE_PHASE` shows the member run stuck at stage 0, the document store): `a_retained_config_over_one_envelope_page_closes_after_a_render` (app config store, `Pending {0,0}` 2.6 M turns), 5x `member::*` (`child-root-retirement-saturated`), `tool_run_large_finalize_reclaims_each_folded_root_before_the_next_op`. I did not run them on an unmodified tree; my change only alters stage 5 when partitions exist, so I do not expect it to be the cause. Likely the same missing-demand class at other stages (owner/another agent should check stage 0/1 demand for member stores).
* Gate slot `target-bim-3` held a stale/corrupt cache (spurious `serde::Serialize` E0277 in `semio-framework` on `cargo test` while `cargo check` passed); a slot-4 build was clean. If it reappears in slot 3: `cargo clean -p serde -p serde_derive -p semio-framework-actor -p semio-framework` via the gate.
* Scratch `T/🗑️generated/z-close/` deleted.
