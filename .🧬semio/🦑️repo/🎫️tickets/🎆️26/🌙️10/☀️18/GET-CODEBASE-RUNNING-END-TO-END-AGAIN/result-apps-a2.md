# Result apps-a2 (wave 1022a): A-L plugins, diff/mutation port + trim-paths + lock refresh + check

## Done (09:15-09:33)
- Killed apps-a stale runs (all ours): retry.sh 57868 + cargo 57871/57872 (dag all-targets), flow-bim libs cargo 86505/86506 (since 05:58). runner.pid 4892 was already dead.
- Codemods from `apps-b-tools` run over the 16 A-L plugin dirs (candidate list `apps-a2-tools/cand.txt`, 969 files with Mutation* impls; generator `apps-a2-tools/lists.py`): dry run first (47 diff files, 502 kind files, 0 `!!` unportable), pre-edit backup `🗑️generated/apps-a2-x/pre-port.tar`, then `portdiff.py`, `fixselfdiff.py`, `portkind.py`, `fixselfdiff.py`. Second dry run = 0 / 0 (idempotent). No-ops for flow/dag/fem/energy kinds (already ported). Per plugin kind files ported: bim 207, block 105, layout 48, gis 28, draw 26, lowpoly 22, cad 21, architect 16, forms 15, animate 10, imperative 3, demonstrator 1.
- `cargo-features = [ "trim-paths" ]` prepended to 22 standalone workspace root Cargo.toml (16 bridges + fem engine node/material + 4 generator/probe crates `.../📦️packages/🦀️rust`) that lacked it.
- Runner: `apps-a2-tools/run.sh apps-a2-tools/ws-order.tsv 🗑️generated/apps-a2-x/ws-results.tsv` (detached; per workspace `update --workspace --offline` then `check --workspace --keep-going --message-format=short`, retries 10x on infra faults), logs `🗑️generated/apps-a2-x/<md5-8>-{update,check}.log`, pid in `🗑️generated/apps-a2-x/runner.pid`.

## Status table (workspace -> status)
(filled below as the runner reports)

## 09:35-10:10 Findings and live edits
- `bun repo cargo` flake 09:31-09:37: "Original capture invocation changed" (capture/🟦️.ts retain clause 8, isDeepStrictEqual of structuredClone(policy,capabilities); appears with the Bun 1.4.2 launcher after package.json `packageManager` 09:23); did not reproduce at 09:39 (my [DEBUG] instrumentation in capture/🟦️.ts was reverted byte-identical). Runner now retries on it.
- `cargo update --workspace --offline` takes the EXCLUSIVE `cargo:registry` lock and convoys behind every long shared holder (root `check --workspace --all-targets` 83147); each cargo call also waits in the memory queue (need 3584 MB, "5 running, N ahead", load 15-26). dag lock refreshed 09:39 (+engine-close, +member-group).
- dag artifact `check --workspace` reached real compilation (09:47) and failed only in FRAMEWORK crates (fixed live, UNVERIFIED until the next check): 
  - `plugin-ownership` `🫧️transient/🧵️publication/🦀️.rs`: `<M as Mutation<P>>::diff` -> `semio_framework_os_kernel::ColdMutation`, `P: Clone` on preparation factory/impl/`bounded_transient_preparation_factory`/`transient_store_disposer`.
  - `os-infinite-board-engine` `🦀️.rs`: `FromValue as _` import (guard_decoded/from_value_controlled are `FromValue` trait methods).
  - framework flow document: `🌿️vcs/🦀️.rs` import `ColdDiffAlgebra, ColdMutationDiff`; `🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs` `semio_framework_value::ordered::RetirementStep` -> `semio_framework_value_ordered::ordered::RetirementStep` (two sites).
- Missed by the apps-b codemods (all plugins): (1) `#[mutations(...)]` aggregates need `reference = <snapshot>, reference_diff = <diff>` (else no `ColdMutation` impl; only fem/energy/puzzle had it): `apps-a2-tools/addreference.py` added it to 27 aggregates in my plugins (idempotent). (2) call sites: `protocol::apply_diff(` no longer exists -> `apply_diff_cold(` (~800 sites), `<T as protocol::Mutation<S>>::diff|inverse` -> `ColdMutation` (~750), `ColdMutation`/`ColdDiffAlgebra`/`ColdMutationDiff` imports next to `Mutation`/`DiffAlgebra`/`MutationDiff` imports: `apps-a2-tools/portcalls.py` (idempotent; 669+8 files in my plugins; fem/energy io files were also still on the old API). apps-b (M-Z) needs both tools: `python3 C/apps-a2-tools/addreference.py @<list>` / `portcalls.py @<list>`.

## 10:00-11:30 More live edits (all UNVERIFIED by a green compile until stated)
- Plugins A-L: `apps-a2-tools/adddeps.py` (copy of apps-b's with my 16-plugin list) added `semio-framework-os-command`/`os-store-one-item`/`ui-locale`/`schema-state` path deps to the 34 crates that use `derive(Mutations)` (33/34/10/13 crates) and `semio-framework-value` to 5 crates using the ToValue/FromValue derives (derive expands to `::semio_framework_value`). Locks are stale again after that (runner re-updates on "cannot update the lock file").
- Framework crates (root workspace members, found failing in the dag closure by the 11:02 check):
  - `os-infinite-board-normal-scene` Cargo.toml +`semio-framework-value`; engine `Camera` is now `pub use semio_framework_canvas::camera::Camera` (canvas Camera derives `Copy, PartialEq`): two identical Camera types mismatched.
  - `os-infinite-board-dag-model` / `-dag-graph`: `graph::algorithms::would_create_cycle_ids` -> `semio_framework_graph_algorithms::…`, `ui_wgpu::wgpu::minimap::*` -> `semio_framework_ui_wgpu_light::minimap::*` (+ Cargo deps).
  - `artifact-flow-flow-copy`: `RetainedCloneSource::<FlowHostSnapshot,SnapshotRead<R>>::admit_borrowed`.
  - `os-flow-vcs` (broken by commit bd9e4e02bd0): three `pub(super) /// doc` + `fn` lines (syntax error; also `🖥️host/🔌️ports/🦀️.rs`) rewritten as doc-then-`pub(super) fn`; `Mutation::inverse/diff` -> `ColdMutation::…`; dead field `layout_update` + `LayoutUpdate` import removed.
- NOT fixed (peers): `plugin-window-config` (28 errors, a peer's `check -p plugin-window-config -p plugin-operation` is running), `os-flow-host-source` (`dyn RetirementCursor/OperatorJob/DomainOwner` not Sync under `derive(RetireOwned)`; flow-only).
- Runner `apps-a2-tools/run3.sh` (retries infra faults, re-runs `update` on a stale lock, stops only on real compiler/resolver errors); results `🗑️generated/apps-a2-x/ws-results*.tsv`. Each workspace costs 2 capacity-queue waits (10-20 min each at load 15-25) + update, so I am iterating on the dag artifact workspace first (shared framework closure), then the other plugins' artifact workspaces.

## 12:05 STOP (coordinator order: memory critical, graph re-inference pid 25619 is the critical path)
- No cargo run is active from me: the runner (run3.sh) and its queued, not-yet-admitted `check` of the dag artifact workspace were killed 12:06 (child first). No new cargo runs until the coordinator says the graph is sealed. `🗑️generated/apps-a2-x/stop` exists (run3.sh stops on it).
- To resume: `rm 🗑️generated/apps-a2-x/stop; python3 C/chain-tools/detach.py <log> bash C/apps-a2-tools/run3.sh C/apps-a2-tools/ws-dag.tsv 🗑️generated/apps-a2-x/ws-results-dag.tsv` (dag artifact first; `ws-order.tsv` lists all 64 workspaces, artifacts first). Expect update (stale lock after the new path deps) then check.
- Last compile evidence (11:54 dag artifact check): after my framework fixes only `plugin-window-config` (8 errors, peer's) and `os-infinite-board-engine` (E0117 from my Camera unification, reverted 12:04 to `From<Camera>` conversions in normal-scene) remained. Both fixes WRITTEN BUT UNVERIFIED.
- Edits needing no build are complete: codemods, addreference, portcalls, adddeps, trim-paths (22 manifests).
- 20:08 resumed after hold: runner run3.sh over ws-order.tsv (all 64), pid 9473, results .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/apps-a2-x/ws-results.tsv

## 22:10 WIND-DOWN: workspace -> status table
| workspace(s) | status |
|---|---|
| dag artifact `🕸️dag/🗿️artifacts/🕸️dag` | COMPILING (first run since the 13:14 hold): ~20:30 lock auto-refreshed, `check --workspace` blocked 20:30-21:55 by lock-auto's `ensureCargoLockCurrentV1` loader fault (uncaptured provider import, owner lock-auto) + capacity queue; 22:07 compiling framework crates with warnings only, no errors yet. Result row appears in `🗑️generated/apps-a2-x/ws-results.tsv` (`check_rc`, `errors`); log `bdef6c5a-check.log` |
| the other 63 A–L workspaces (flow, gis, animate, demonstrator, fem, bim, architect, lowpoly, forms, layout, cad, imperative, energy, draw, block, extensions, bridges, generators) | NOT CHECKED (queue time); codemods, reference/deps, trim-paths applied, unverified |
| TS `:test` targets of the 16 plugins | NOT RUN |

## HANDOVER (deliberate detached run, NOT killed)
Runner pid in `🗑️generated/apps-a2-x/runner.pid` (bash `apps-a2-tools/run3.sh`, list `apps-a2-tools/ws-order.tsv`, 64 workspaces, artifacts first). It retries infra faults, re-updates stale locks, writes one TSV row per workspace to `🗑️generated/apps-a2-x/ws-results.tsv` (`dir plugin md5-8 check_rc errors seconds`); per-workspace log `🗑️generated/apps-a2-x/<md5-8>-check.log`; `apps-a2-tools/errsum.sh <log>` lists the errors. Stop it with `touch 🗑️generated/apps-a2-x/stop` (finishes the current workspace). Framework-chain errors go to the plugin-chain2 owner.
