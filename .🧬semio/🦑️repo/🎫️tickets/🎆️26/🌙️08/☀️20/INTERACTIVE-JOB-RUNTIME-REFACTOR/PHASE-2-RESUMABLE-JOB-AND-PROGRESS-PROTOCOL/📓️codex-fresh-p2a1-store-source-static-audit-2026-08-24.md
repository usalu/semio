# Codex Fresh P2a1 Store Source/Static Audit

Date: 2026-08-24

Verdict: **GREEN for the scoped P2a1 Store rejection-transfer source/static contract.**
This does not accept Phase 2 or any deferred runtime gate.

## Source Findings

- Invalid `reject` eligibility returns the original `Self` before any move. Valid rejection wraps
  the source in `ManuallyDrop`, transfers the exact record, field lease, registry Arc, ticket, and
  diagnostic, marks `Transferred`, and explicitly runs only the shallow source Drop
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:6743-6755`).
- The consumed source Drop recognizes `Transferred`, verifies that no record/field owner remains,
  and does not reclaim the still-live ticket (`:6935-6947`). Both populated source and rejection
  owners use `ManuallyDrop`; an early ordinary Drop therefore fail-closes through the assertion
  rather than recursively deep-dropping record pages or returning a ticket twice (`:6600-6608`,
  `:6950-7015`).
- Rejection close preserves identities at a zero grant, closes the field owner, calls
  `return_now` exactly once, waits for ticket reclamation, and then advances the record one close
  step (`:6964-7008`). The public law proves record-page pointer, lease ticket, registry identity,
  diagnostic, zero-grant identity preservation, double-return refusal, and generation reuse
  (`:18492-18530`).
- Success/cancel/fault use the same owned release path: field close/return, ticket reclamation, and
  record page close precede terminal publication (`:6650-6740`, `:6867-6927`).
- The only public `ArtifactEnvelopeDecodeAuthority::reject` callers are its scoped Store laws;
  production construction uses `try_new`, preserving the Result admission boundary. No caller
  bypasses the public Result-based rejection transfer.

## Verifier Fidelity

`toolJobArtifactEnvelopeRejectionTransferExact` requires the source `ManuallyDrop` transfer,
transferred Drop branch, delayed ticket reclamation, one-step rejected close, and public identity,
zero-grant, double-return, success, cancel, and generation laws (`📜️script.ts:1475-1541`). Its 13
no-op-guarded mutations directly remove each of those obligations (`:1544-1574`). No concrete
P2a1 false-green counterexample remained in this read-through.

## Executed Static Gates

```text
bun ./📜️script.ts verify interactivity tool-jobs --p2a1-only --self-test
[verify interactivity tool-jobs p2a1] live-source clean; hostile-mutations=13.

bun ./📜️script.ts verify interactivity tool-jobs --self-test
[verify interactivity tool-jobs] self-tests=365 clean.
```

`rustfmt --edition 2021 --check` passed for the Store component. Scoped
`git diff --check HEAD -- Store-component 📜️script.ts` emitted no whitespace errors.

P2d materials were preserved and not modified. Cargo, Nx, Wasm, browser/native runtime, network,
and broad builds were not run.
