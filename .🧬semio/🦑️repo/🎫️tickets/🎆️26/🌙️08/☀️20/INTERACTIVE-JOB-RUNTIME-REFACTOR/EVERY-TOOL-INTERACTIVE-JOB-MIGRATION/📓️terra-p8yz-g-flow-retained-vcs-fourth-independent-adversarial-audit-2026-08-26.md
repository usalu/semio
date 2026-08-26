# P8yz-g Flow Retained VCS Fourth Independent Adversarial Audit

Date: 2026-08-26  
Auditor: Terra — independent read-only source/static audit  
Verdict: **RED — the formerly missing fresh fault-at-rollback evidence is now represented correctly, but `FlowRetainedVcs` has no production consumer or mounted Flow route. It is only a public, unused Rust module plus `#[cfg(test)]` callers.**

## Scope And Constraints

Read in full: the applicable root, `✏️s`, and Flow `AGENTS.md` files; the current P8yz-g implementation report; every prior P8yz-g adversarial audit in this ticket (including the third); the live retained VCS region and test module; all three local language-neutral fixtures; package glue; the Flow bridge/Wasm candidates; and the raw-caller census.

No production, fixture, or test source was edited. No Cargo, Nx, Bun/Vitest, Wasm, browser, cache-writing command, or Git command was run. `rustfmt --edition 2021 --check` on the VCS component exited 0; this is parse/format evidence only, not a typecheck or test result.

The requested folder name `EVERY-TOOL-COMMAND-MUST-BECOME-A-BOUNDED-RESUMABLE-JOB` is not present in the master ticket. This report is therefore placed in the existing, containing P8yz-g folder `EVERY-TOOL-INTERACTIVE-JOB-MIGRATION` alongside its implementation report and the first three audits.

## Blocking Counterexample: Exported Is Not Mounted Or Reachable

`FlowRetainedVcs` is declared in `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs:1424`. The only non-test package connection is `pub mod vcs; pub use vcs::*;` in `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/📦️glue.rs:64-66`. A repository-wide raw-symbol census excluding ticket artifacts returns only the declaration file itself for `FlowRetainedVcs`; the only constructor/call sites are the private `flow_vcs_tests` module in that same file.

Flow's production bridge and Wasm implementation retain their separate `FlowRetainedFeature` / `FlowProgram` path; neither names nor constructs `FlowRetainedVcs`. Thus an actual Flow request cannot enter its preflight, one-unit cursor, cancellation, fault, ACK, or close code. Re-exporting an otherwise unused public type is compilation exposure, not a retained mounted route. This is also consistent with the current implementation report's explicit deferred A4 production adoption.

Consequently no source-static evidence establishes the requested production reachability or retained mounted route. This alone blocks GREEN, irrespective of the quality of the isolated worker and its test-only ledger.

## Current Evidence That Does Pass Static Inspection

| Requested area | Source-static finding |
| --- | --- |
| Fresh rollback controls | GREEN in test design. The control loop constructs `let mut session` inside each control iteration (`component.rs:4136-4139`) before invoking exactly that row's `cancel` or `fault` (`4164-4168`). This remedies the third audit's false duplicate-fault evidence. |
| Five cursors | GREEN in fixture shape: `rollbackSteps` are exactly `1`, `2`, `3`, `6`, `7` in `📒️lifecycle.json:1748,1851,1954,2057,2160`; the test advances precisely that many close steps and rechecks the target (`4172-4178`). |
| Stage, authority, document, page, history, fingerprints, surface owner | GREEN in test design. The boundary helper checks Cancelled/Faulted stage and both expected/operation authority (`3793-3802`), reconstructs and compares every 14-field `FlowSurfaceOwner` (`3774-3810`), then compares complete canonical document, explicit-null page, history, and all 16 `FlowVcsResourceFingerprint` fields (`4179-4185`; extraction at `3396-3548`). |
| Duplicate-control ordering | GREEN in test design. The initial control occurs before the rollback cursor; only after the exact cursor-state assertion does it repeat the same control and require `duplicateControl` with an unchanged resource fingerprint (`4186-4193`). |
| 13-operation third-party oracle | GREEN in source shape. The private `SerdeJsonFlowOracle` independently evaluates all 13 fixture operations, and the test separately drives the retained session through page ACK and incremental close before comparing extracted actual document/page/history/handback (`3903-3922`). No literal feature-result matrix remains. |
| UTF-8 contracts | GREEN in fixture/test shape. `acceptedMultibyte` is 4 scalars / 12 bytes; maximum is `é` x32,768 / 65,536 bytes; plus-one is `a` x65,537 / 65,537 bytes (`📒️lifecycle.json:2676-2810`). The live law reconstructs source text, checks character and byte lengths, checks source retention and full admission/final state (`3989-4031`). |
| Malformed, authority, and grant/fuel vectors | GREEN in fixture/test shape: 4 authority, 3 malformed, and 5 grant records are asserted in `component.rs:3958-3961` and driven through live typed methods at `4033-4127`. Controls and close reject grants that have no controls or fail `permits_work` (`1676-1701`, `1740-1743`). |
| Preflight/admission and one unit | GREEN in isolated source shape. Every payload `begin_*` gets and censes the source, calls `preflight`, then performs the first `take` (`1508-1573`); `preflight` refuses before `admit` mutates credits or source ownership. Action cursor phases and rollback close each perform a discrete unit (`1740-1810`, `1984-2108`). |
| Hostile fixture mutation | Partial/static only. The test recomputes stored fixture signatures and mutates every scalar of all eight signed fixture groups (`3926-3985`). It is not a compiled/executed claim under this audit's command embargo. Its source-string omission checks are weaker than a semantic code mutation test, but the production-reachability failure already decides RED. |

## Raw Caller Census

The current `reject_whole_buffer_artifact_envelope_ingress` census has eight files: Store guard, directed DAG, Shooting, FEM 2D, FEM 3D, CAD, Puzzle 5D, and Puzzle 3D. Flow is absent. This matches the current tree rather than the older report's historical attribution.

More importantly, the separate `FlowRetainedVcs` census above has **zero production call sites**. The Flow Wasm source instead defines a different retained feature implementation. Therefore neither census supplies the missing mount.

## Required Resolution

Add one actual production Flow bridge/Wasm/native dispatcher route that constructs and drives `FlowRetainedVcs` through typed admission, poll, page lease/ACK, cancellation/fault, and incremental close. Retain a source-static caller census proving that route. Then run the focused native/Wasmtime ledger gates when the embargo permits; this audit makes no runtime-pass claim.

