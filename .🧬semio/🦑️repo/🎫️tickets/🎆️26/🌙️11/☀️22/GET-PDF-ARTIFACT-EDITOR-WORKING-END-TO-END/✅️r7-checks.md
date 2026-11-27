# R7 gate checks

Logs: `🗑️generated/r7-checks/`.

## 1. Repo MCP TypeScript gates for `@semio-tech/stdio-pdf`
- `mcp__repo__show_project` and `mcp__repo__run @semio-tech/stdio-pdf:check` (language=en): FAIL (infrastructure, not PDF code).
  - show_project: "this machine holds no sealed generation of the workspace" (graph policy current).
  - check: `Build owner @semio-tech/stdio-pdf cannot be inferred ... invalid build.closure.provider-generated-import {"port":"artifact-package","targets":"@semio-tech/ui-rs:generate"}` (reference b4375b6a, service log /var/folders/.../semio-repo/501/host-c7208dac666a.log). Reproduces on unchanged inputs. Likely tied to the concurrent manifest/closure restructuring; recorded, not fixed. build/test targets were not attempted (same owner inference).
- The TS package `📦️packages/🟦️typescript` has no `*.test.*` files (vitest: "No test files found").

## 2. Shell TS law
- `sh run-vitest-one.sh .../🛠️ShellHelpers/🧪️tests/🪟️tree-windows/🟦️.tsx <log>`: PASS, 1 file, 17/17 tests (log vitest-tree-windows.log).

## 3. Rust (via `native-cargo.sh`, serialized, 3 jobs, `cargo test -p <pkg>` full)
| package | result |
|---|---|
| semio-s-artifact-stdio-pdf-page-editor-tests | PASS (exit 0) |
| ...-page-editor-authoring-tests | PASS |
| ...-editor-kit | PASS |
| ...-editor-document | PASS |
| ...-image-route | PASS |
| ...-save-cause | PASS |
| ...-native-save (--lib) | FAIL: 12 passed, 3 failed |

native-save failures (reproduced in `rust-native-save-rerun.log`, 120 s):
- `a_second_incremental_save_stacks_on_the_first`: `assertion failed: two.starts_with(&one) && two.len() > one.len()` (tests/original-save/🦀️.rs:561)
- `incremental_over_an_xref_stream_original_appends_a_stream_section`: `assertion failed: saved.starts_with(&original)` (:506)
- `cancelling_an_incremental_save_midway_publishes_nothing`: `the Save plan settles or is cancelled: Some(false)` (:149), the plan returned ReferenceRequired, i.e. fell back to the compact route.

Diagnosis: all three drive a second incremental over an input that already has a resource/xref history, and the plan falls back to compact. The native-save working tree has uncommitted in-progress edits by another agent (`🧷️increment.rs` ExistingResources/ResourceClaims binding, `🧱️program.rs` Compact cause plumbing, `📤️producer.rs`, `🧾️job.rs`, test file, Cargo.toml). The failing paths are exactly the ones those edits touch, so this is that agent's unfinished work, not test drift; no fix was made to avoid clobbering it. First-save incremental, rotation, 1600-page, large-save cancel tests all pass.

## Fixes made
None.

## Remaining issues
1. Repo graph inference failure for `@semio-tech/stdio-pdf` (provider-generated-import on `@semio-tech/ui-rs:generate`); check/build/test targets unverified through the repo route.
2. 3 native-save incremental tests failing against the current in-progress increment/program edits; owner must finish and rerun `cargo test -p semio-s-artifact-stdio-pdf-native-save --lib -- a_second_incremental incremental_over_an_xref cancelling_an_incremental`.
