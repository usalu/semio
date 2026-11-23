# Lane status after the handoff

Read at 2026-11-23 ~01:40 CET from `T/🗑️generated` and `ps`. Ports 6272 and 6070 were not listening. Nothing from the handoff's four lanes was finished and green.

## Native suites

| Log | State |
|---|---|
| `claude-suites/rust-five-3.log` | Failed earlier, exit 101. `semio-plugin-window-kits` called `scene_surface` with 3 arguments; `semio-plugin-window-transient` already took 5 (`name`, `description`). |
| `claude-suites/rust-five-4.log` | Running. `cargo test` on `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/Cargo.toml` for the five xlsx packages (pid 70955). Past source admission, compiling framework crates. No `error[E` in this log. The kits call sites in the tree now pass `surface_name(...)`. |
| `claude-suites/assembly-2.log` | Same 3-vs-5 `scene_surface` failure, exit 101. |
| `claude-suites/assembly-3.log` | Running. `cargo test` of `every_bundled_artifact_definition_assembles_without_violations` (pid 67639). Compiling, no compile error in this log. |
| `claude-suites/claims-2.log` | Running. Graph slice inventory, past 1400 owner checks and 31 minutes. Owner checks are increasing. The live tree keeps adding observations, so this pass is slow. Left running. |

## Base mutation feature run

`claude-oracles2/feature-subject2.log` finished with exit 1. It executed 0 cases: `--case mutate-xlsx-ecma-376` is not the directory name. The authored case is `🔀️mutate-xlsx-ecma-376`. Rerun is waiting on cargo capacity (the suite above already holds the managed cargo).

## Standalone pane 6272

`claude-pane/cargo6.log` failed, exit 101, on a stale `semio_plugin::scene_surface` that still took 3 arguments and did not export `surface_name`. Current `plugin/🦀️.rs` reexports the 5-argument function and `surface_name`. `claude-pane/cargo7.log` is the retry (pid 62591) and has been blocked on the wasm artifact-directory lock the whole time. That lock is held by the full-S cargo, which the pane script deliberately shares (`CARGO_TARGET_DIR` = `T/🗑️generated/claude-pane/native/target`). Left blocked. Publish, activate, vite and `standalone-pane-journey.ts` have not started. Port 6272 is closed.

## Full S host 6070

`claude-full-s/build-space-stdio.log` is an older failed build. `claude-full-s/space-cargo.log` is the live `claude-fulls-cargo.sh` (pid 62180), compiling `semio-hub-space-component` for wasm32-wasip2. No `could not compile` in that log. `vite-s.log` is the previous serve and is stale. Port 6070 is closed.

## Done in this pass

The openpyxl differential for an unreferenced shared-string entry is in the tree and the Python replay is green. See the Follow-up in `claude-xlsx-oracles.md`.

## Still open

1. rust-five-4, assembly-3 and claims-2 have to finish, and any new compile error has to be fixed at the root.
2. Rerun the feature subject with `--case 🔀️mutate-xlsx-ecma-376` and record the counts.
3. When the wasm lock frees, let cargo7 finish, then publish, activate, serve 6272 and run `standalone-pane-journey.ts`.
4. When space wasm finishes, publish, activate, serve 6070 and run `claude-fulls-journey.ts` for en and de.
5. Cargo-run `the_pool_reader_matches_openpyxl_on_an_unreferenced_entry` (it rides with rust-five-4 if that crate has not been fingerprinted yet).
6. Delete `T/🗑️generated` and close the ticket only after the five remaining items above are green.

## Later read, ~02:18 CET

rust-five-4, assembly-3 and pane cargo7 are still compiling, with no `error[E`. Pane rustc is on the stdio artifact crates. No hub wasm has been linked yet. Space cargo2 is still blocked on that wasm lock. Claims-2 is still in graph inventory. Exhaustive parity is alive in `claude-oracles2/feature-parity2.log` and is selecting the generated host workspace; its oracle cargo process has exited and the log has no compile error and no case counts yet. Ports 6272 and 6070 are still closed. The pane and full S waiters are still attached.
