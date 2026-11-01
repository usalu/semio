STATUS: wasm32-wasip2 check GREEN for all 11 (below). Native tests: IN PROGRESS (driver `C/1101-tools/capext/test.sh t2`, logs `captest-<b>-t2.txt`); host confirmation UNVERIFIED (act15 still building).

Proof logs in `C/🗑️generated/1101/`, each `.rc` = 0:
- process-metal, process-robotic, process-wood, cad-aec-building, cad-aec-energy, cad-aec-structure, cad-spatial-shape, sourcing-slabs, sourcing-windows: `extfix-<b>-r3.{txt,rc}`
- process-concrete: `extfix-process-concrete-r4.rc` (r3 was red until fix 1)
- sourcing-beams: `extfix-sourcing-beams-r4.rc` (r3 was red until fix 2)

Fixes (all in shared crates; the extension crates themselves needed none):
1. `semio_snapshot_retirement!` in stdio semio-base (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/📦️packages/🦀️rust/../../🦀️.rs`): `retire` returns `RetainedWorkOutcome<..>` per `SnapshotRetirementFactory::retire` (brep + drawing snapshot crates).
2. stdio png editor `close_step` closures (`📷️png/.../✏️edit/🎮️commands/🎨️paint-native-region` and `🩹️patch-pixel-region`) ported to `RetainedWorkOutcome{result,work}` (pattern of the wav edit-audio port; `admit_shared_retirement` result unwrapped via `.result`/`.work`).

Observed, not mine: tiff `🎨️paint-region` close_step has the same old `Ok(..)` form (not in my dep chain).
Script: `C/1101-tools/capext/run-retry.sh <tag> <batches>`.

## Native tests / hosts (in progress)
- t1: died in `semio-framework-os-store-replay` (replay-owned-state-operations, file edited by replay lane minutes earlier); not mine, retried as t2.
- t2 process-concrete: `cargo:test stopped: timeout 3599348ms` after 723 crates compiled (capacity queue + cold native build of the stdio closure), infrastructure not a test fault (captest-process-concrete-t2.txt). Following batches reuse that build.
- Hosts: `C/🗑️generated/1101/act15.log` still in `[bazel:build]` (no served app), so `scopeContributionsJson` / `process.machines` / `cad.computer` / `sourcing.module` not yet observable. Registry dist `plugins.json` under demonstrator is dated Oct 12 (stale, not evidence).
