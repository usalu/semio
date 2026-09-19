# 📋️ Fleet Brief v2 (shared by every agent of the play fleet, 2026-09-19 late session)

Repository: /Users/ueli/Documents/semio (macOS, 10 cores / 32 GB, bun + nx + Rust nightly). Read `AGENTS.md` first and
follow it, with these overriding constraints. Ticket folder (abbreviated `$T` below):
`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP`

## Hard rules
- NEVER run modifying git commands (commit, stash, checkout, reset, restore, clean) and NEVER use git worktrees.
  Other developers and other agents edit the tree concurrently, even the same files — ignore unrelated changes,
  never revert them, re-read a file right before editing it, keep working in conjunction with them.
- Do NOT open/close/reopen tickets or goals. Do NOT delete or sweep any `🗑️generated` folder. Do NOT spawn
  sub-agents. Run every build/test in the FOREGROUND (background children die with your turn). macOS has no
  `timeout`. `cd` explicitly in every Bash call.
- Scratch files/logs ONLY under `$T/🗑️generated/<your-topic>/` (use `.txt`, not `.log`).
- Do NOT start, stop or restart dev servers (ports 6033, 6040, 6056, 6063, 6076, 6080 are live and owned by others).
- Cargo: `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 RUST_MIN_STACK=33554432`. The fleet shares one machine and one
  cargo build dir (fine-grain locking): a cargo at 0 % CPU that has a `rustc` child, or that waits on a lock, is
  working — be patient, never kill cargo/rustc processes you did not start. Run `df -g /System/Volumes/Data`
  before big builds; below 20 GB free prune only stale incremental session dirs older than 2 h
  (`find <⚡️cache>/cargo/build/*/incremental <⚡️cache>/cargo/build/*/*/incremental -mindepth 2 -maxdepth 2 -type d -mmin +120`).
- A watchdog kills native test binaries running longer than 10 minutes; a hanging test is a defect to fix.
- `cfg(target_arch = "wasm32")` code never compiles natively: when you change production (non-test) plugin code,
  also run `cargo check -p <crate> --target wasm32-wasip2 [--features component-app-assembly]`.

## Test-debt goal (fix agents)
Every native test of your crates passes:
`cargo test -p <crate> --lib --tests --no-fail-fast [--features component-app-assembly when the crate declares it] -- --test-threads=4`.
Baseline (20:45–22:19 today, partly stale — an earlier fleet already fixed some crates; ALWAYS re-run first):
`$T/🗑️generated/baseline/<crate>.txt`, summary `$T/🗑️generated/baseline/summary.tsv`, crate lists per plugin
`$T/🗑️generated/plugin-crates.json`.

### Rules for fixing
- Fix ROOT causes. Never delete, `#[ignore]`, weaken or loosen a test, never special-case test inputs.
- Committed fixtures are contracts. When fixture and code disagree, decide which side is wrong from sibling
  crates that pass, the schema, enum attributes, doc comments, `git log`/`git show` (read-only). Regenerate a
  fixture only when the code is right, with the crate's own generator/printer, never by hand-matching.
- A test exercising a removed/renamed API is updated to the current contract, preserving what it proved.
- Mutation enums whose fixtures are `{"mutation": "setSnapshot", ...}` carry
  `#[value(tag = "mutation", rename_all = "camelCase")]`.
- Framework code under `🧰️framework` may change when the defect is there; keep it general, run that framework
  crate's tests and `cargo check` a couple of dependents.

### Known stale-test buckets (check in this order before suspecting the plugin)
1. `Number(0.0)` vs `Number(0)` → fixture float canonical form; `manifestId: Null` → hand-written `ToValue` must skip `None`.
2. `edit history insertion requires its exact mutation retirement factory` / `artifact store reached Drop without
   its exact terminal-empty shallow-shell witness` → bare `ArtifactStore::new` in tests. Use an owners-installing
   guard (`new_<x>_store` → `Owned<X>Store` with Deref + Drop close loop; reference: trinity jack `⚙️operations/🦀️.rs`,
   block3d `♻️retirement`), close every store before drop.
3. `interactive-job.live-instance` + stale projections → mounted harness: `bind_instance_id(1)`,
   `settle_registered_typed_operation` after every `dispatch_typed`, `settle_framework_reserved_admission` after
   `handle_action("interactionSelect")`, `settle_history_verb` for undo/redo, close on Drop unless panicking.
   `result.mutations` is EMPTY on a mounted app — assert receipt lanes / history length.
4. `BuiltChildren requires retained page transport` → `artifact_app_laws::project_and_retire_fixture_tree`;
   out-of-doc lanes → `decode_fixture_scene_with_lanes::<Scene>`.
5. Tree rows carry no `"selected":true` → assert `"interactionDomain":"<domain>"`.
6. "… did not finish" spins → the loop never drains `take_typed_operation_completion()`.
7. `RetainedJobPayload` closes need `JOB_PAYLOAD_PAGE_BYTES` (16 KiB) grants.
8. `derived child dialect … not declared by this app's member roster` → `new_app_with_registry_and_members::<_, SemioMembers>`.
9. Oracle catalog lives in `🔮️oracles/🔣️.json`; structural-correspondence check must match `"::mutation::"`.
10. Stack overflow → `RUST_MIN_STACK=33554432`.
11. `ephemeral transfer inline ownership exceeds its fixed metadata bound` → box the big inline field (256 B cap).
12. Registryless `testkit::new_app` is unusable → `new_app_with_registry` + instance id.

### Cross-cutting buckets with ONE owner (everyone else: do not fix these, fix the rest, re-run at the end)
- **XCUT-TOOLPROOF** — `tool factory proof rejected tool '…' … generated_migrated=false`
  (`interactive-job.catalog-authority`, panics in `🧰️framework/…/🔌️plugin/…/🦀️.rs`; 94 baseline failures across
  cad, demonstrator, flow, forms, note, process3d, playbook, sequence, shooting, space, vcs, plugin-flow,
  plugin-energy, plugin-norm, plugin-space). Status note: `$T/🗑️generated/xcut-toolproof/STATUS.md`.
- **XCUT-DICT** — `final Dictionary ownership must be explicitly retired or owned by a cold boundary`
  (panics in `🧠️neural/⚙️engine`; 179 failures in flow-extension-*, imperative-*, playbook-procedural,
  imperative-procedure, sequence). Status note: `$T/🗑️generated/xcut-dict/STATUS.md`.
If one of these blocks your crate, read the owner's STATUS.md (it says when the fix landed and what per-crate
follow-up, if any, each crate needs), apply that per-crate follow-up yourself, and re-run.

## Report (final message)
Concisely: per crate, first-run pass/fail → final pass/fail (from runs you actually made, with the log path),
root causes grouped, files changed (absolute paths), anything left failing with the precise reason and the next
step. Do not claim anything you did not run. Also write the same report to `$T/🗑️generated/<your-topic>/REPORT.md`.
