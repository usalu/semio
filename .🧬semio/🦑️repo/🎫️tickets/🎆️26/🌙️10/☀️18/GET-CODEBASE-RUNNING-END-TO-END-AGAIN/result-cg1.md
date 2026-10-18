# Result CG1 (executor prefix `cg1`): crate closures back under the source-boundary gate

Executor: Sonnet 5.5, foreground, no sub-agents, no cargo/nx/`bun repo`, no modifying git command. `C` = this ticket, `F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. Tools and backups of every touched shared file: `F/cg1-tools/` (`orig/`, `specs/`, `pos/`, `newfw.py`, `adddep.py`, `repoint_decode.py`, `cgc-census.ts`, `rows.py`, `tomlcheck.ts`). Static evidence only: nothing was compiled.

## 0. Verdict

**All three crates are at or below 0.90 (worst target, current tree, measured with the canonical reader).** Two of the three were already fixed by `RESULT-CRATE-HEADROOM` section 7 (HDR-2); the numbers in the task (1.084 and 0.988) are the pre-HDR-2 baseline that `audit-untangling-state.md` copied. Only `job` still needed a carve and got one.

| crate | task figure | measured now (lib / test) | binding | action |
|---|---|---|---|---|
| `semio-s-artifact-process-process3d-host` | 1.084 | 0.660 / **0.832** | test lines 3,328 of 4,000 | none: HDR-2 peeled `retained-cursors` and `publication-authority`; wiring re-verified (members, `[workspace.dependencies]`, `testkit` forwarding) |
| `semio-framework-3d-brep-boolean` | 0.988 | 0.834 / **0.878** | test lines 3,511 of 4,000 | none: HDR-2 moved the 452-line white-box test to `semio-framework-3d-brep-boolean-unit-tests` (0.117); wiring re-verified (framework members lines 319-320, workspace dependency, owner row) |
| `semio-framework-job` | 0.991 | 0.810 / **0.857** (was 0.853 / 0.991) | test bytes 224,773 of 262,144 | carved, section 1 |

Full census run on the two affected workspaces (`F/cg1-tools/cgc-census.ts`, same reader as `cgc-census.ts`; 576 crates, framework + process3d): **0 oversized lib, 0 new crates above 0.90**; the only two test-only rows over the gate are peer defects unrelated to this slice (`semio-framework-plugin-reactor-patches` and `semio-framework-ui-contract-conformance`: "Missing Rust source input", reader refuses before counting).

## 1. The `job` carve

`job` is one private knot (`StepContext`, payload ledger, caller-source family, failure family): almost every child names root privates (`hdr-tools/hubdeps.py` over the root: the children use `super::*` or private root items such as `StepContext` fields, the ledger and `OriginalRetainedLoanIdentity`), so no child can leave without widening the knot. The carve therefore takes the two pieces that are separable by design:

1. **`semio-framework-job-native-decode`** (new, upward crate): `⏱️context/🛬️decode` (`OriginalNativeDecodeSource`, `Policy`, `Issued`, `Outcome`), 12,489 bytes. It only used job's public API (`StepContext`, `OriginalCallerSourceLoan`, `OriginalTurnFailure`, `close_original_retained_step`, `InteractiveJobCloseStep`), so no widening was needed. Nothing inside job referenced it.
2. **`semio-framework-job-unit-tests`** (new companion crate, test-only): the two white-box law files `🧪️tests/🔬️retained-ownership/🦀️.rs` (4 tests) and `♻️retirement/📄️payload/🧪️tests/🦀️.rs` (4 tests), 23.7 KB. The files stay where they are (their TS siblings, fixtures and text readers keep working); the new lib root `🧪️tests/🔬️unit/🦀️.rs` mounts them by `#[path]` with the same module names (`retained_ownership_tests`, `payload_retirement_tests`) and re-declares the test heap witness and `TEST_RETAINED_POLICY`. Test names and counts are unchanged (8 before, 8 after).

Job-side changes (all minimal, no behaviour change):

* `🦀️.rs`: decode mount and `pub use` removed; `JobPayloadOperationLedger` became `#[doc(hidden)] pub struct` (fields and methods stay private); the `retained_ownership_tests` mount removed; a `cfg(any(test, feature = "testkit"))` `pub mod testkit` of seven thin accessors added (`payload_ledger`, `payload_ledger_terminal_is_empty`, `step_context_with_payload_ledger`, `staged_backing_identity`, `rejected_backing_identity`, `ledger_demands`, `close_ledger`), the same pattern HDR-2 used for the boolean companion.
* `♻️retirement/📄️payload/🦀️.rs`: `ledger_demands` widened to `pub(super)`; the `mod tests` mount removed. The root and producer strings that `♻️retirement/📄️payload/🧪️tests/🟦️.ts` asserts (`mod payload_retirement;`, the `pub use payload_retirement::{...}` line, `pub struct JobOutcomeSlot`, `self.ledger.is_none()`, ...) are untouched.
* Moved test files: `use super::*` replaced by explicit imports; `Arc::new(JobPayloadOperationLedger::new(..))` -> `payload_ledger(..)` (6 sites), `StepContext::with_payload_ledger(` -> `step_context_with_payload_ledger(` (10), `writer.staged/rejected...backing_identity()` -> accessors (6), `ledger.terminal_is_empty()` -> accessor (1), `crate::component::TEST_RETAINED_POLICY` -> `crate::TEST_RETAINED_POLICY` (10), the four `fixture_text!` paths re-based to the new manifest directory. Law bodies are otherwise byte-identical (`diff` against `F/cg1-tools/orig/`).

Measurement (`F/cg1-tools/cgc-census.ts`, default features):

| crate | lib files / bytes / lines (ratio) | test files / bytes / lines (ratio) |
|---|---|---|
| `semio-framework-job` | 22 / 212,353 / 2,267 (0.810) | 24 / 224,773 / 2,353 (**0.857**) |
| `semio-framework-job-native-decode` | 1 / 12,559 / 94 (0.048) | 1 / 12,559 / 94 (0.048) |
| `semio-framework-job-unit-tests` | 1 / 727 / 0 (0.003) | 3 / 24,265 / 223 (0.094) |

Considered and rejected: moving `OriginalJobFixtureCaller` (10.8 KB; 44 consumer files plus about 30 in-tree tests and worker's `testkit` module; would need a manifest edit in about 45 crates), `park` (10 KB; needs about nine new public setters on `StepContext` state), the caller-source/child family (copies 15 private `StepContext` fields), and the payload storage trio (its writer takes `&mut StepContext`).

## 2. Registrations

* `🧰️framework/Cargo.toml`: members `🔨️modules/🧵️job/⏱️context/🛬️decode/📦️packages/🦀️rust` and `🔨️modules/🧵️job/🧪️tests/🔬️unit/📦️packages/🦀️rust` (next to the job member); `[workspace.dependencies.semio-framework-job-native-decode]` (the companion has no dependents, so no dependency table).
* `📋️build-owners.json`: rows `@semio-tech/job-native-decode-rs` and `@semio-tech/job-unit-tests-rs`; both packages carry a `📋️project.json` (data only, `newfw.py`).
* `.vscode/launch.json`: six routes per crate (orders `900.96039/40`, `922.96039/40`, `903.96039/40` for decode; `...41/42` for the companion). The two existing gates that ran the moved tests through the job project (`test(job_payload_retirement_)` and `retained_ownership_tests`) now call `@semio-tech/job-unit-tests-rs:test`, in `launch.json` and in `🧩️launch.seed.jsonc`.
* Consumers of decode (no compat re-export; every path updated, import lists split where mixed): 8 source files and 8 manifests.

| consumer file (crate) | dependency added |
|---|---|
| `os/.../🏪️store/📬️command/🛬️receiving/🪜️cursor` (`semio-framework-os-store-command`) | `[dependencies]` |
| `plugin/⚛️reactor/🔄️turn/📥️receiving/📦️completed` (`semio-framework-plugin-reactor-turn`) | `[dependencies]` |
| `plugin/👷️job/🏛️reserved` (`semio-framework-plugin-job`) | `[dependencies]` |
| `plugin/🧵️instance/🏪️stores/.../📮️action-ingress` (`semio-framework-plugin-instance-stores`) | `[dependencies]` |
| `plugin/🧵️instance/📨️dispatch/.../⌨️typed-command` (`semio-framework-plugin-instance-dispatch`) | `[dependencies]` |
| `🎭️actor/🎟️retained-turn/💼️job/⏸️park/🧪️tests` (declared `[[test]]` of `semio-framework-actor`) | `[dev-dependencies]` |
| `trinity/🔌️jack/.../💾️binary/📸️snapshot/🫴️receiving` (`semio-s-artifact-trinity-jack-inferences`) | `[dependencies]` |
| `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🧪️tests` (`semio-s-dev-trinity-jack`) | `[dev-dependencies]` |

## 3. Verification (static)

* Typecheck/privacy differential, `F/cg1-tools/pos/pos.py` (copy of the HDR oracle, nightly-2026-07-20 `--emit=metadata`, baseline = git `7880cc36f78`), family `cg1-job.json` (job + decode + companion), cfgs `lib,test`: lib NEW 0 / GONE 0; test NEW 5, all five E0560 on the stubbed external `RetainedCloneGrant` literal in the payload law (flagged stub-layer noise by the tool; the identical literal is an ordinary pub-field construction in the job root); privacy 0, resolve 0, type 0, trait-scope 0, other 0 non-stub. The typeck canary (E0308, E0616, E0624) is reached on both sides. Sensitivity: a seeded `cx.fuel_remaining=0` in the decode crate was reported as E0616 across the crate boundary.
* Consumer family `cg1-job-consumers.json` (job, decode, the five framework consumer crates and `semio-framework-actor`; the two trinity consumers are in other workspaces and not in the family), cfgs `lib,test`: privacy 0, resolve 0, trait-scope 0, other 0 non-stub new rows in both cfgs. Two non-stub type rows (E0282 "type annotations needed", `plugin-reactor-turn` `entry-lifecycle-and-retirement` line 139 and `turn-finish` line 16) are `with_pending_patches(|pending| ...)` closures over the stubbed `reactor-patches` crate; the family needed a second stub-fill iteration on the current side only (36 GONE / 37 NEW in lib), the lines name neither job nor decode, so I classify them as stub noise, not a regression. The decode import/path rewrites in the reactor-turn, store-command, plugin-job, instance-stores and instance-dispatch files resolved.
* `rustfmt +nightly-2026-07-20 --config skip_children=true --check` on every edited or new `.rs`: no parse error (only format diffs, as before). All edited manifests, the owner registry and `🧰️framework/Cargo.toml` parse (Bun TOML, JSON).
* Text readers: `♻️retirement/📄️payload/🧪️tests/🟦️.ts` asserts were checked against the edited root and producer; no reader names the decode path, `retained_ownership_tests` as a mount, or the removed `mod tests`.

## 4. Crates to compile (`bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib --tests`, offline lock refresh first)

New: `semio-framework-job-native-decode`, `semio-framework-job-unit-tests`.
Carved: `semio-framework-job` (lib, tests, and `--features testkit`).
Consumers: `semio-framework-os-store-command`, `semio-framework-plugin-reactor-turn`, `semio-framework-plugin-job`, `semio-framework-plugin-instance-stores`, `semio-framework-plugin-instance-dispatch`, `semio-framework-actor` (`--tests`, target `original-owned-async-turn`), `semio-s-artifact-trinity-jack-inferences`, `semio-s-dev-trinity-jack` (`--tests`).
Unchanged but part of the gate set: `semio-s-artifact-process-process3d-host` (+ `-retained-cursors`, `-publication-authority`), `semio-framework-3d-brep-boolean` (+ `-unit-tests`).
Test runs: `@semio-tech/job-unit-tests-rs:test` (8 laws), `@semio-tech/framework-job-rs:test` (the remaining job lib unit test plus the integration targets), `semio-framework-actor` target `original-owned-async-turn`.

## 5. Open

* **Cargo.lock refresh.** The two new packages and the eight new dependency edges are in no lock. 179 of 180 tracked `Cargo.lock` files resolve at least one of the consumer crates, so the offline refresh of the compile window (checklist item 1) has to run before any `--locked` step; I did not hand-edit locks.
* Pre-existing, not caused by this slice: `🧪️tests/📏️close-demand/🟦️.ts` reads `../🔬️retained-ownership/🦀️.rs` of job and asserts `fn caller_worker_close_policy(` and a `HostileJob` impl; those laws live in worker's file since the worker carve, so that TS test already failed before this slice. The two test-only census rows named in section 0.
* `JobPayloadOperationLedger` is now a (hidden, field- and method-private) public type of the job root so the companion can name it; all constructors stay behind the `testkit` module.
* Headroom of `job` is 0.857 on bytes; a further cut would need the fixture caller or the park/caller-source family (rejected above).
