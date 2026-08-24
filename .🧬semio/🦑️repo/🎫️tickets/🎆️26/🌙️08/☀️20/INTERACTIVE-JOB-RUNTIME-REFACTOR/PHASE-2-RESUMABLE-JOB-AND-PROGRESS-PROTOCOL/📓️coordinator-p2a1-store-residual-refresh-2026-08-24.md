# Coordinator P2a1 Store Residual Refresh

Date: 2026-08-24  
Verdict: **RED — the former missing-close census is repaired, but the public rejection transfer is not valid.**

## Updated Census

The live Store `ArtifactEnvelopeDecodeAuthority` now implements mandatory `begin_close`, bounded
`close_step`, and `terminal_is_empty`, so the earlier report's literal sole missing-close residual is
stale. The production plugin and Animate callers both construct this authority through `try_new`.
This is a read-only refresh; no source or broad verifier was changed or run while other Rust packets
were active.

## Blocking Rejection-transfer Counterexample

`ArtifactEnvelopeDecodeAuthority::reject(self, diagnostic)` moves the exact record and field lease
into `ArtifactEnvelopeDecodeRejected`, sets the source authority to `Fault` and
`field_returned = true`, and then returns. The consumed source authority necessarily runs its `Drop`.
That Drop requires `field_registry.ticket_reclaimed(field_ticket)`, but the ticket is intentionally
still live because its lease was just transferred into the returned rejection authority. Therefore
the source Drop assertion fails on every successful rejection transfer.

This makes the advertised public exact rejection authority unusable and can panic during ownership
handoff. The current Store laws do not call this public seam, and the universal verifier does not
mutate the source-Drop/ticket relationship, so the existing self-tests can false-green it.

## Required Closure

- Make the source authority's ownership transfer explicit without running its populated terminal
  assertion. A shallow transfer guard/ManuallyDrop state must preserve the exact record, lease,
  registry ticket, and diagnostic in the returned rejection authority.
- Ordinary premature Drop must retain or fail closed without returning a live lease while then
  recursively dropping the record graph or panicking during another unwind.
- Add a hostile law that calls the public `reject`, checks exact record/page and lease/ticket identity,
  proves zero-grant preservation, incrementally closes the returned authority, and reaches one true
  terminal-empty/reclaimed ticket.
- Add faithful mutations for source Drop after transfer, premature ticket reclamation, raw record
  drop, and missing exact-owner identity.

After this closure, rerun the complete P2a1 production constructor/direct-drive/missing-close census
and the isolated P2a1 verifier before independent acceptance.
