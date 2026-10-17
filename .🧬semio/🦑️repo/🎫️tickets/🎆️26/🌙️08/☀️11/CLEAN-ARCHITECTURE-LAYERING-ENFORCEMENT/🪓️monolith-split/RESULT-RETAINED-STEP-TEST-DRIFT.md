# 🧷 Result RETAINED-STEP-TEST-DRIFT (slice `rsd`, wave 7, 2026-10-17)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `ENG` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`. Logs: `F/🗑️generated/rsd-*`. Tools: `F/rsd-tools/` (`rsd.py` arity scanner, `rsd_apply.py` rewriter, `rsd_pos.py` pre-edit overlay runner for the POS oracle, `rsd_rowdiff.py` row-multiset diff, `rsd-overlay.json` = 34 edited files -> pre-edit snapshot, `rsdp.json` plugin family, `rsdi.json`/`rsdn.json` job/icon-host and native-probe families).

## 0. Verdict

**DONE.** Every test and fixture call site of the retained step API is on the current signatures: 1,509 `StepContext::new` / `StepBudget::new` / `StepBudget::from_duration` call sites in the whole tree, 1,483 on the current arity, 26 off, and all 26 are production renderer lib code, none in a test file (section 6, hand-off). No shims, no aliases, no compatibility layer. The native-socket-probe E0599 is an oracle stub artifact, not a source defect (section 4). A real `bun repo cargo` check was not possible (compile not admitted, mutex held by the coordinator probe); the exact command is in section 7.

## 1. Found state (wave 7, 07:28)

* Last RESULT text was from the 00:10 restart; the session since then (00:10 to 07:05) ran four more oracle rounds and edited 34 files, none of it recorded. Reconstructed from the tool folder, `git diff` against the pre-edit snapshots (`rsd-overlay.json` values, still on disk under the older scratchpad `6c384ddf-...`), and the logs.
* No half-applied batch: `rsd.py` over the live tree finds no site with a missing or extra argument in any edited file; all 34 files parse with `rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout` (0 errors).
* Oracle runs that finished before the restart: renderer family baseline `wb1` (104 crates, pre-edit overlay, 06:17); plugin family baseline `pb1` (07:10) and post-edit `pa1` (07:18); job/icon-host families `g1`/`g2`, native-probe family `n1`/`n2`. Renderer post-edit `wa1` was killed (SIGTERM, 06:34), so it was re-run as `wa2` in wave 7 (section 5).
* One snapshot is stale by design: the cad `🧪️tests/🔬️unit/🦀️.rs` shrank by 2,427 lines because a peer carved it into children after my snapshot (the E0220 row churn in the `pb1`/`pa1` diff is that carve, not my edit).

## 2. What changed (34 files, 2,858 diff lines incl. the peer cad carve)

Current API (`ENG`-independent, `🧰️framework/🔨️modules/🧵️job/🦀️.rs`): `StepBudget::new(fuel, deadline_us, retained: RetainedCloneGrant)`, `StepBudget::from_duration(fuel, start_us, duration_us, retained)`, `StepContext::new(operation, generation, budget, cancel, now_us, &mut preview_sequence, &mut RetainedCloneProgress)`, `InteractiveJob::close_step(&mut self, cx) -> Result<InteractiveJobCloseStep, OriginalTurnFailure>`.

* Plugin tests (17 files, `✏️s/🔌️plugins/**` and `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/**`): writer, mathematical equation, gismap, process3d (command-surface, retained-laws), lowpoly, layout export, cad, draw (simplify, export, mutation-arena-pool, edit/arena/candidate/duplicate laws, kit, retained-mutation-authority root, import-image admission), puzzle 2d board-host, generation3d incremental-eval and example-geometry harness. Each `StepBudget::new(a, b)` gained the file's retained grant (a `const` next to its first use, or the grant already in scope) and each `StepContext::new` gained a local `RetainedCloneProgress::default()` as the 7th argument.
* Renderer (`ENG`, 16 files): `🎯️targets/🧊️wgpu/🗺️engine-registry/🧰️fixtures` (`ENGINE_CLOSE_GRANT`, `with_engine_close_context` ported, new `close_engine_canvas_packet_step` helper because `EngineCanvasPacket::close_step` is now `Result`), `🗺️engine-registry/🦀️.rs` (re-export), `🧷️engine-attach-tests` (import), EngineCanvas `wgpu-standalone` and `wgpu-node-graph` (call the helper), `🧪️tests/{wgpu-renderer-async-boundary, wgpu-surface-lane-unit, wgpu-os-host-standalone}`, Shell tests `wgpu-display-conflicts-marketplace` and `wgpu-time-travel`, Interpreter tests `wgpu-introspection` and `wgpu-ui-command-wiring`, IconRenderHost `gpu-export` (rewritten onto `OriginalJobFixtureCaller` + `ui_prepared::testing::{close_job_step, drain_abandoned_preparations, drive_preparation_until_terminal}`; the old `drive_step`/`StepOutcome` path no longer exists).
* Docstrings with a unique emoji on every new item; no comments inside definitions; no `[DEBUG]` lines.

## 3. Static evidence

* Arity scan, whole tree (`git ls-files --cached --others`, 32,766 `.rs` files, `rsd.py`, log `🗑️generated/rsd-scan7.txt`): 1,509 sites, 1,483 ok, 26 BAD, **0 BAD in test paths**. Other constructors: `StepContextOwner::new` 25/25 on 3 args, `OriginalJobFixtureCaller::admit` 144/144 on 6, `StepContext::with_payload_authority` 75/75 on 8.
* Trait-method scan: 78 test sites of the form `close_step(<grant>)`, all on `StepContextOwner::close_step(grant) -> InteractiveJobCloseStep` (not the `Result` trait method), so their `assert_eq!(..., InteractiveJobCloseStep::..)` is valid; no site compares or reads `.progress()` on the `Result` of the `cx` form.
* rustfmt parse of the 34 edited files: 0 errors.
* Oracle limit, stated plainly: in every family the `semio_framework_job`/`value` crates are stubs (only `rsdi` holds the real job crate, and the `value` crate is still a stub there), so arity errors against them land in the stub layer (`E0061/stub=10,669` in `pa1`) and no POS oracle can show the fix, only the absence of regressions. The arity scan above is the proof of the fix; the real compile (section 7) is the final proof.
* Plugin family `pb1` (pre-edit overlay) vs `pa1` (live), 22 crates + `composition-laws`, lib and test: base 617/632 rows, post 617/632 rows; after dropping the peer's cad/gismap editor-carve renames (E0220, one E0433) the row diff is empty in both configs (`rsd_rowdiff.py`).

## 4. native-socket-probe E0599

The rows (`open_scope_now` on `Arc<()>` at `🔬️native-socket-probe/🦀️.rs:242` and `📮️mailbox-impl:24`; `decode_native_snapshot`, `encode_native_snapshot`, `preflight_sqlite_snapshot_encoding`, `SQLITE_SCHEMA`, `DESCRIPTORS` in `🪶️sqlite/🦀️.rs`) appear only when `store`/`TokioHostRuntime`/derive output is stubbed (`Arc<()>` is the stub type; the `RetireOwned`/mutation derives are stripped). Source check: `ArtifactNativeSnapshot` is implemented for `NativeSocketProbeSnapshot` in `🪶️sqlite/🦀️.rs:21`, imported from `store::os_store` (re-exported at `🏪️store/🦀️.rs:299`), `ArtifactSqliteSnapshot` is imported `as _`, `ArtifactSqliteSnapshotHost` is imported, and `open_scope_now` exists on `TokioHostRuntime` (`🛎️services/🚂️host-runtime/🦀️.rs:346`). The `n2` family (the crate alone) reports zero non-stub rows. In `ENGINE-CANVAS-PACKET-DRIFT` the row was "new" only because a peer edit moved the file path in the row key. No source change; confirm with the real check.

## 5. Oracle wa2 (renderer family, post-edit, finished 08:13)

Family `rii-tools/rii.json` (104 crates), lib and test, log `🗑️generated/rsd-wa2-run.log`, report `rsd-rii-wa2-report.txt`; baseline `wb1` ran on the pre-edit overlay (`rsd_pos.py` + `rsd-overlay.json`). `rsd_rowdiff.py rsd-wb1-run.log rsd-wa2-run.log`:

* lib: base 366 rows, post 366 rows, gone 0, new 0.
* test: base 429 rows, post 429 rows, gone 2, new 2.
  * GONE `E0600 [os_renderer_wgpu_icon_host] 🧪️tests/📤️gpu-export/🦀️.rs x2: cannot apply unary operator '!' to type 'Result<bool, IconExportFault>'`: the stale `!job.advance()` / `while !job.advance()` fixture, fixed by this slice's rewrite onto the caller-driven turn.
  * NEW `E0599 [os_renderer_wgpu_native_probe] .../🪶️sqlite/🦀️.rs x2 SQLITE_SCHEMA not found for NativeSocketProbeSnapshot`: a report-attribution flap, not a new error. The raw non-stub error text is identical in both runs (`rsd-rii-wb1-cur-test-it1-nonstub.txt` and `rsd-rii-wa2-cur-test-it1-nonstub.txt` each hold the same 4 `E0599 SQLITE_SCHEMA` diagnostics at `🪶️sqlite/🦀️.rs:50,53` and `🧪️tests:79,80`), and the lib section carries the identical row in both. Cause: the trait `ArtifactSqliteSnapshot` (with its `const SQLITE_SCHEMA`, `🚪️io/🪶️sqlite-snapshot/📸️snapshot/🦀️.rs:4`) lives in a crate the union stubs. Same class as section 4.
* New errors attributable to this slice: 0.

## 6. Foreign errors and hand-offs

26 production call sites in renderer lib code are still on the pre-`0c3e78944af` signatures. They need a retained grant that is a policy decision of the renderer owner, not a mechanical port, so they were not touched. Files and lines (`ENG` prefix):

| File | Lines | Shape |
| --- | --- | --- |
| `🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs` | 278, 282 | `from_duration` 3 args, `StepContext::new` 6 args; a `grant` is already in scope (line 277) and is passed to `close_component_world_step` |
| `🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs` | 357-360, 389-392, 417-420, 482-485, 625-628 | five contexts; `from_duration` 3 args, `StepBudget::new(0, 0)` fallback, `StepContext::new` 6 args; they drive `JobReplayLog::begin_capture`, which pages payload bytes through `cx.retained_grant()` (`🎭️actor/💼️job/📨️publication/🦀️.rs:1607`), so a zero grant would stall, a size is needed |
| `🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🗃️kernel-retained-state/🦀️.rs` | 690, 693 | one context, same replay-log close |
| `🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs` | 178, 181 | `close_engine_surface_step`; the close demand of an `EngineCanvasPacket` scales with its scene, so the grant has to come from the host (`os-host` callers at lines 344 and 464 would pass it) |
| `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🎲️board2d/🦀️.rs` | 491, 494 | `settle_board_pointer_authority_into` |
| `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/🖼️chrome-frame-and-main-window/🦀️.rs` | 25, 29 | `terrain_grant` is defined 9 lines later (line 38) and can be hoisted |

Recipe for the owner: add the grant as 3rd `StepBudget::new` / 4th `from_duration` argument, add `&mut RetainedCloneProgress::default()` as the 7th `StepContext::new` argument, thread the host grant through `close_engine_surface_step`. No other fleet agent had these files (last edit 10-16 22:12).

Other foreign observations: `layout_editor` privacy rows (E0624/E0616 on `close_step`, `from_admitted`, `LayoutPublicationSlot`) are present in both `pb1` and `pa1` (carve fallout, not this slice); `puzzle_2d` recorder E0603 rows likewise.

## 7. Commands

Run in wave 7:
* `python3 rsd-tools/rsd.py <repo> 🗑️generated/rsd-scan7.txt` -> `1509 26`.
* `rsd_rowdiff.py rsd-pb1-run.log rsd-pa1-run.log` -> empty after the carve noise.
* `rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout --quiet <34 files>` -> 0 parse errors.
* `pos.py run ../rii-tools/rii.json --cfgs lib,test --label wa2 --no-base` -> finished 08:13, row diff against `wb1` in section 5 (0 new errors, 1 stale E0600 gone).
* `python3 rsd_rowdiff.py rsd-wb1-run.log rsd-wa2-run.log` -> as section 5.
* Compile admission check: `📓️FLEET.md` has no `COMPILE ADMITTED` line (07:36); mutex `🗑️generated/cargo-mutex.lock` is held by the coordinator probe 2. No `bun repo cargo` call made by this slice.

Real check for the compile queue (one batched call under the mutex; plugin crates are members of the `✏️s` workspace (manifests under `✏️s/`), renderer crates of the framework workspace, so two calls):
`cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check $(python3 -c "import json;print(' '.join('-p '+c for c in json.load(open('.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rsd-tools/rsdp.json'))['crates']))") --tests --keep-going --message-format=short --manifest-path ✏️s/Cargo.toml`
and, for the renderer (representative test crates; the complete set is the 104 crates of `rii-tools/rii.json`), the same with `-p semio-framework-os-renderer-wgpu-engine-registry -p semio-framework-os-renderer-wgpu-engine-attach-tests -p semio-framework-os-renderer-wgpu-icon-host -p semio-framework-os-renderer-wgpu-frame-tests-1 -p semio-framework-os-renderer-wgpu-runtime-mailbox-tests -p semio-framework-os-renderer-wgpu-ui-engine-tests -p semio-framework-os-renderer-wgpu-shell-tests-docs-1 -p semio-framework-os-renderer-wgpu-shell-tests-windows-1 --tests --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml`. Expected before the section 6 sites are fixed: the renderer lib crates that own the 26 listed files fail with E0061 at exactly those sites and nowhere in a test file; if a test crate fails because of them, attribute it to section 6.
