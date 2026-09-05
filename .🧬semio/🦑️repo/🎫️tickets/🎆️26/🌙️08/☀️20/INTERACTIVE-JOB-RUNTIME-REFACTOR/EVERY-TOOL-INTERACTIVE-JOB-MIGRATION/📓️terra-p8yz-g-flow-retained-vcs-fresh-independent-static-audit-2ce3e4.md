# Terra P8yz-g Flow Retained VCS Fresh Independent Static Audit

Date: 2026-08-26  
Scope: live Flow Rust VCS source and its three local language-neutral fixtures. Static/read-only inspection, `rustfmt --check`, and Bun probes only. No production edit, Git mutation, Cargo, Nx, Wasm, or browser command was run while A4 source work is active.

## Verdict — RED

The Flow retained VCS worker has useful typed, small-feature structure, but it fails the packet's explicit fixed-owner/no-`Vec`/no-clone rule in its **live production retained region**. `FlowRetainedVcs` owns four growable vectors at lines 1202–1205 (`undo`, `redo`, `retired_actions`, `retired_surfaces`) and initializes them with `Vec::with_capacity` at lines 1219–1222. Capacity is an allocation hint rather than a bound: a `Vec` can grow after it. The same retained region also contains `.clone()` calls. This is not a small reactive feature and cannot be accepted as fixed-capacity retained ownership.

The language-neutral lifecycle fixture also omits explicit laws for malformed input, panic fault, idempotent close, and cancellation around transfer. Rust tests nominally cover part of this area but were correctly not executed under the active-source embargo, so they do not repair missing fixture coverage or provide runtime evidence.

## Inputs read

- root `AGENTS.md` and master plan;
- P8yz retained canonical-pack prerequisite;
- accepted P8yz-a report;
- P8yz-g implementation report;
- live `🌊️flow/🌿️vcs/🦀️component.rs`, package glue, Flow bridge/Wasm sources, and all three Flow VCS fixture JSON files.

## Exact commands and results

```sh
rg -l --glob '*.rs' 'reject_whole_buffer_artifact_envelope_ingress' | sort
rustfmt --edition 2021 --check '.../🌊️flow/🌿️vcs/🦀️component.rs'
rg -n 'pub mod vcs|pub use vcs|🌿️vcs/🦀️component' '.../🌊️flow' -g '*.rs'
nl -ba '.../🌊️flow/🌿️vcs/🦀️component.rs' | sed -n '1188,1228p;1238,1328p;1504,1524p;1600,1672p;2090,2122p'
bun -e '<JSON feature/owner/oracle and hostile in-memory source probe>'
```

- `rustfmt --check`: **PASS**. It is syntax/format evidence only.
- package reachability: `📦️packages/🦀️rust/📦️glue.rs:64–66` unconditionally declares and reexports `vcs`.
- independent Bun fixture probe: **PASS** for 13 declared features, 16 declared laws, the ownership categories totaling 62 (excluding the fixture's 10 forbidden-route labels), caps 256/12/8, and private test-only oracle declaration.
- independent hostile source probe: correctly rejected in-memory deletion of preflight, publication authority validation, and terminal-credit condition; **baseline is RED** because retained source contains `Vec<` and `.clone()`.
- independent fixture-gap probe: missing `malformed`, `panicFault`, `idempotentClose`, and `cancelAroundTransfers`; resume/retry/ACK controls are declared.

## Raw caller census: current 8, historical attribution only

Fresh command output has exactly eight files: the shared Store guard plus framework directed DAG, Shooting, FEM 2D, FEM 3D, CAD, Puzzle 5D, and Puzzle 3D. **Flow is absent**.

P8yz-a recorded a historical count of ten and the P8yz-g implementation report explains an expected historical nine-to-eight reduction as Flow's old raw caller having been deleted before its work resumed. This audit did not observe a live nine-path tree, so it records only the evidence-supported historical attribution: reports document `9 → 8`; fresh observation is **8 = guard + 7 peers**, not a freshly proved nine.

## What the live retained route does prove statically

- The public surface contains the 13 requested small features: widget add/remove/move/patch, synapse add/remove/move/patch, set layout, replace document, undo, redo, checkpoint (lines 1248–1326).
- The ten payload features calculate a typed census then call `preflight` before `source.take()`; preflight checks closed state, depth 12, item/byte caps, available slots/credits, and identifier exhaustion.
- The declared caps are four operations/pages/outputs/controls, 256 items/history, 65,536 bytes, 12 events/depth, and an eight-millisecond deadline. The operation slots themselves are a fixed array.
- Poll exposes progress, checkpoint, preview, page-ready, and terminal; page lease/resume/retry/exact ACK, lost-handle rediscovery, cancel/fault/panic fault, incremental operation close, retired close, and terminal-empty predicates are present.
- Publication validates operation session generation, base revision, and parent revision immediately before direct typed action application (lines 1600–1654), updates the document atomically after validation, and preserves the prior document on pre-publication fault/cancel/stale authority.
- `flow_vcs_validate_action` and `flow_vcs_apply_action` implement direct typed inverses for widget/synapse/layout/document actions and undo/redo/checkpoint rather than using the old Store route.
- The oracle is private to `#[cfg(test)] flow_vcs_tests`; `FlowSemanticOracle` returns owned `FlowOracleResult`, and `SerdeJsonFlowOracle` is not a production API. This is source shape only: no runtime-oracle equality claim is made.

## Failing details

### 1. Growable retained owners (hard blocker)

`Vec::with_capacity(FLOW_VCS_MAX_HISTORY)` at construction does not prevent later `push`. The live mutation loop at lines 1642–1648 repeatedly pushes undo/retired actions; surface retirement also pushes into the growable vector. Some local length checks exist, but the representation remains a banned whole/growable ownership route and the checks are not a replacement for fixed backing. Replace all four with fixed, preallocated bounded owner slots and explicit generation/retirement cursors before re-audit.

### 2. Clone in the retained production region (hard blocker)

The independently isolated `🌊️RetainedVcs` source contains `.clone()` (for example typed ID copies used by action construction). The packet explicitly denies clone or renamed equivalents in the retained route. A successful redesign must move ownership rather than duplicate it, or narrow the acceptance contract with coordinator approval; this audit cannot silently exempt `String` clones.

### 3. Fixture completeness (hard blocker)

`📒️lifecycle.json` lists `zero`, `maximum`, `maximumPlusOne`, repeated rejected→valid, stale/ABA/lost handle, fault/cancel, interrupted close, and terminal empty. It does **not** list malformed input, panic-fault, idempotent terminal-empty close, or cancellation around every transfer. The contract requires those hostile ledger rows, not merely similarly named implementation methods.

### 4. A4 Wasm adoption remains pending

Package glue reaches the Rust worker, but a read-only search of Flow `🌉️wasm` and `🌉️bridge` found no `FlowRetainedVcs`/`FlowVcs` consumer. The implementation report correctly reserved A4's Wasm ABI/dispatcher work. This audit made no A4-file edit and cannot prove other agents' file history without Git state; it explicitly leaves final Wasm production-consumer adoption pending.

## Contract checks that remain deferred

No Cargo test, runtime oracle comparison, malformed parse path, native/Wasm ledger parity, actual 8 ms watchdog, browser flow, memory measurement, or A4 dispatcher trace was run. Therefore the following are unproven even apart from the RED source blockers: zero/max/+1 execution, 256→257→valid runtime behavior, wrong/stale/ABA/lost-handle behavior, cancellation at each transfer, panic/interrupted/idempotent close, resource handback, real UI reachability, and English/German accessibility/device behavior.

## Required remediation boundary

1. Replace the four `Vec` retained owners with fixed-capacity owner storage and resumable cursors; prohibit retained cloning.
2. Add language-neutral hostile fixture rows for malformed, panic, idempotent close, and each cancellation-transfer boundary, then source-mutate those claims.
3. Let A4 consume only the 13 retained features; do not restore a raw/whole-buffer compatibility bridge.
4. After source work quiesces, run focused native tests, Flow package tests, Wasm target checks, A4 dispatch reachability, and the serialized timing/browser matrix.

No production source was modified by this audit.
