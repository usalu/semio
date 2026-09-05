# Terra P8yz-g Flow Retained VCS Remediated Fresh Independent Static Re-audit

Date: 2026-08-26  
Scope: exact `🌊️RetainedVcs` production region, Flow VCS fixtures, package glue, and read-only caller/ABI census. No production edit, Git mutation, Cargo, Nx, Wasm, or browser command was run while A4 source work is active.

## Verdict — GREEN for the requested source/static slice

The earlier local RED blockers are remediated in live source. `FlowRetainedVcs` now owns undo, redo, retired actions, and retired surfaces through `FlowFixedOwners<T, FLOW_VCS_MAX_HISTORY>`, whose backing is `[Option<T>; N]`; the retained region contains no growable `Vec`, capacity hint, clone, string-copy/materialization helper, generic Store/DSL/pack/serde route, or FlowHost route. The 13 typed features, fixed caps, source preflight-before-take, positional inverses, slot preflight before publication, lifecycle controls, fixed retirement, 19-entry fixture fingerprint, and the expanded hostile laws are all present and independently mutation-checked.

This is **not** native/Wasm/browser/runtime acceptance. A4's Wasm dispatcher does not consume `FlowRetainedVcs` yet; that adoption remains explicitly pending and receives no credit here.

## Evidence executed

```sh
rustfmt --edition 2021 --check '.../🌊️flow/🌿️vcs/🦀️component.rs'
rg -l --glob '*.rs' 'reject_whole_buffer_artifact_envelope_ingress' | sort
rg -n 'pub mod vcs|pub use vcs' '.../🌊️flow/📦️packages/🦀️rust/📦️glue.rs'
rg -n 'FlowRetainedVcs|FlowVcs' '.../🌊️flow/🌉️wasm/🦀️component.rs' '.../🌊️flow/🌉️bridge/🦀️component.rs'
bun -e '<isolated region, fixture, owner, oracle, and hostile mutation predicates>'
```

| Gate | Result |
|---|---|
| `rustfmt --check` | GREEN — exit 0; format/syntax shape only. |
| Exact retained slice | GREEN — fixed owners, all required lifecycle/publication/positional-inverse anchors present; forbidden spellings zero. |
| Fixed owner backing | GREEN — `slots: [Option<T>; N]`, `length`, `remaining`, push-before-full, LIFO pop/take, and last-owner cursor are live; all four VCS history/retirement fields instantiate it at max 256. |
| Fixture data | GREEN — 13 features, 20 laws, 8 cancel-transfer boundaries, 19 fingerprint entries, and 62 owner categories. |
| Test-only oracle shape | GREEN — `FlowSemanticOracle` returns owned `FlowOracleResult`; `SerdeJsonFlowOracle` and serde types are inside `#[cfg(test)]`; no public/runtime oracle claim. |
| Raw census | GREEN current observation — exactly 8 files: shared guard + DAG, Shooting, FEM2D, FEM3D, CAD, Puzzle5D, Puzzle3D. Flow absent. |
| Package reachability | GREEN — Flow package glue unconditionally declares/reexports `vcs`. |
| A4 adoption | PENDING — no Flow VCS consumer found in Flow Wasm/bridge source; not claimed. |

## Fixed owners and publication

`FlowFixedOwners` maintains a physical `[Option<T>; N]` backing plus logical length. `push` rejects before indexing when full; `remaining` is `N - length`; `pop` decrements then takes exactly the last slot. `undo`, `redo`, `retired_actions`, and `retired_surfaces` all use this same fixed owner with `N = 256`; no capacity hint can grow it.

Publication validates session/base/parent authority and action validity, then preflights every action-specific owner consequence before applying a document mutation: undo/redo capacity, displaced-redo retirement capacity, and surface-retirement capacity. It retains the generated inverse and moves displaced redo values one-at-a-time. Fixed close drains surface backing, actions, history, and document incrementally; `terminal_is_empty` requires all slots, credits, histories and retirements empty.

Positional inverses avoid copying identity: insert-widget/synapse returns `Remove*At { index }`; inverse removal restores the moved item at its original index. A first layout insertion returns `RemoveLayoutAtWidget { index }`, resolving the key from the retained widget slot when undoing; a replacement moves the prior layout directly. Static source has all three exact inverse forms.

## Preflight, lifecycle, and ownership

All ten payload-bearing begin methods calculate/validate their typed source census and call `preflight` before `source.take()`; patch methods first validate identifier agreement. The source preflight checks closed state, depth 12, zero/max/+1 item/byte limits, operation/page/output/event/control credits and identifier exhaustion. The 13 exposed small features are widget and synapse add/remove/move/patch, layout, document replacement, undo, redo and checkpoint.

The live source contains retained progress/checkpoint/preview/page-ready, leased page take/resume/retry/ACK, cancellation/fault/panic-fault, stale/ABA/lost-handle rediscovery, exact handback, surface retirement and idempotent terminal empty. The fixture records malformed, panic fault, idempotent close, and all eight cancellation-transfer boundaries (admitted, progress, checkpoint, publication, take, resume, retry, ACK). `FlowVcsResourceFingerprint` statically tracks the seven credit dimensions plus active/leased/history/retirement/revision/generation/digest/document/closing entries represented by the 19-field ledger.

## Independent hostile mutation probes

The baseline passed the exact retained-slice predicate. The following 11 in-memory mutations each made it fail:

- replace the fixed `[Option<T>; N]` backing;
- break remaining capacity, push, or pop accounting;
- remove action-specific history-capacity preflight;
- replace each widget, synapse, or layout positional inverse;
- remove the resource fingerprint, panic-fault feature, or one preflight-before-take call.

All **47** fixture mutations failed their predicate as required: deletion of every one of 20 laws, each of eight transfer boundaries, and each of 19 fingerprint entries. This is static mutation evidence only; the Rust laws were not executed because Cargo is embargoed.

## Historical census and deferred scope

The historical `9 → 8` explanation is documentary only: P8yz-a/P8yz-g reports describe Flow's raw caller removal before earlier auditing; this re-audit freshly observed only the eight live paths above and does not invent a fresh nine-path observation.

No Cargo test/typecheck, native or Wasm ledger comparison, runtime oracle equality, timing/watchdog test, browser trace, A4 dispatch integration, memory behavior, or accessibility/device test ran. The Flow VCS source/static slice is green; final end-to-end acceptance remains pending those serialized gates.

No production source was modified by this audit.
