# Test Presence Policy Preflight

The coordinator independently compiled and executed nine isolated Rust fixtures against the live `mutation/test-presence` policy. The seven fixtures without runnable tests were all falsely accepted: empty test directory, unmounted test file, empty `cfg(test)` module, string/comment decoy, `cfg(not(test))` module, ignored-only law, and a nested test function inside an ordinary helper. Both genuine positive cases (inline and explicitly mounted laws) were accepted and each ran one compiler-discovered test.

The current predicate is directory existence OR a module whose `cfg` tokens contain `test`. It does not prove executable test presence, module reachability, or enabled configuration. Root policy and the shared Rust structural inspector need a bounded source-aware correction. Do not confuse structural test discovery with proof of domain-law completeness or actual successful production execution.

## Evidence

- Neutral vectors: `🧪️test-presence-preflight/🔣️vectors.json`.
- Executed command: `bun ./📜️script.ts nx exec --projects=workspace --skipNxCache -- bun <ticket>/🧪️test-presence-preflight/📜️script.ts`.
- Initial red transcript: `🧪️test-presence-preflight/🧪️red.log`; nine compiled/executed cases, seven policy mismatches, exit1.
- Every individual compiler source, compiler stderr/stdout and runtime transcript is retained under `🧪️test-presence-preflight/🧫️run-iZYJwq`.
- The refined harness creates an empty tests directory only in the empty-directory case; the other cases isolate their own mechanism. The rerun again compiled/executed all nine cases and found the same seven mismatches: `🧪️test-presence-preflight/🧪️isolated-red.log`, retained run `🧫️run-vzKwyC`. This confirms the defects independently of the shared empty-directory setup.

## Required Executor Boundary

Use parsed item attributes and source module resolution. Count only actual test functions in reachable leaf-owned source, excluding comments, literals, nested function decoys, known disabled test configuration and ignored-only coverage. Do not whitelist mutation names or treat arbitrary JSON fixtures as runnable tests. Cover both inline and direct mounted tests, optional child facets, missing targets, symlinks, path escapes and conditional attributes. Retain the compiler oracle's actual test execution, promote the neutral contract into registered tests, and preserve real production test execution as a separate gate.

No production policy or parser was edited during this preflight. This is a confirmed outstanding requirement, not an accepted feature.

## Parsed-Inspector Independent Review

After FND-TEST-PRESENCE-08 replaced the old directory/module predicate, the coordinator extended the neutral/compiler replay from9 to16 cases. All original9 now agree with actual compiler/runtime discovery, but all7 new cases disagree. Transcript: `🧪️test-presence-preflight/🧪️parsed-review-red.log`; complete fixtures/compiler/runtime output: `🧪️test-presence-preflight/🧫️run-CLrWlh`; exit1.

Five false positives remained: `#[ignore = "reason"]`, an enabled `cfg_attr` adding a false `cfg`, an enabled `cfg_attr` adding reason-bearing `ignore`, a scope-level inner false cfg, and an unmounted file mistaken for an inline module's mounted source. Two false negatives remained: the genuine inline-mounted law and an enabled `cfg_attr` carrying a path mount. Rust compiled every case; all positive fixtures actually ran one test and all negative fixtures ran zero tests.

The inspector must apply nested/inner conditional attributes and resolve actual Rust module directories, not flatten every explicit path against the physical source's parent. The root also requested exact source path spelling (no unproved NFC rewrite) and bounded subprocess execution. All corrections remain with the designated TypeScript writer. This packet is not accepted on its initial fourteen-vector green.

The executor subsequently passed the16-case replay and expanded its registered gate, but the coordinator's next two cases exposed ignored inline `#[path]` overrides. With `#[path="redirected"] mod tests { #[path="law.rs"] mod law; }`, rustc resolves `redirected/law.rs`; the inspector still chose `tests/law.rs`. The18-case replay had two mismatches: an unmounted decoy falsely accepted and a real mounted test missed. Evidence: `🧪️test-presence-preflight/🧪️inline-override-red.log`, run `🧫️run-Ow0iwa`, exit1. The writer retains the correction, including nested/conditional inline path bases. Earlier registered greens are not final acceptance.

## Final Bounded Acceptance

After the inline path-base correction, the coordinator observed the registered gate pass with1 test/148 assertions and the independent18-case compiler/runtime replay pass with zero mismatches. Both process sessions were subsequently polled to exit0. Some earlier raw logs/results were absent on the next filesystem inspection; their removal cause is not established, and historical references above must not be read as claims of current artifact availability.

The coordinator reran the registered gate with fresh retained artifacts on2026-08-27: `🧪️test-presence-root-recheck.log` records1 pass,148 assertions,289 filtered,0 failures, and exit0. Artifacts are under `🧪️test-presence-root-recheck-artifacts`. This accepts only enabled, reachable, non-ignored test discovery; it does not prove that a particular mutation's laws are adequate or that its production tests pass.

The fresh independent18-case replay also completed with zero mismatches and exit0. Transcript: `🧪️test-presence-preflight/🧪️root-recheck.log`; per-case compiler/runtime evidence and `🔣️results.json`: `🧪️test-presence-preflight/🧫️run-fUd9vs`. The ticket harness now bounds compiler/runtime processes at30 seconds and checks status, signal, and launch errors.
