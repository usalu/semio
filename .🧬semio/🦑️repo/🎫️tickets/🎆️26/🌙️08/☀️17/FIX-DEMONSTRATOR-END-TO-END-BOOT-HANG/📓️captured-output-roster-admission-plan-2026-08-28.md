# Captured Output Roster Admission Plan

## Bounded Next Slice

After the source evidence join, replace the existing synchronous `instance.reserveReturn(maximumResponses)` with a phased `instance.reserveReturn(maximumResponses,grant)` result `{step,source}`. This is the existing captured-return entry point, not a second transport or compatibility overload. The final `source` is returned only after the original roster and facade are constructed under an installed same-ledger domain record. All authored callers must be updated together. This document is a proposed source plan, not an implemented or executed admission gate.

The instance is already captured and owned by its original activation. Its existing private `ShardInstanceOwner` holds five additional inline construction fields: returnCell, returnRecord, returnPhase, returnFault and returnCapacity. The actual shared ledger comes only from the original Shard client. No caller ledger argument, new pool, current actor-name lookup, public record capability or generic registration callback is introduced.

The pending admission is prepared for the exact original instance object. The parent stores the actual prepared cell before claim. Capacity is retained before the first resource admission; retries with another capacity refuse without changing the original request. Wrapper-lost prepare/claim/record results recover from that cell and exact private record type, not from a returned temporary facade.

## Strong Parent Before Construction

Install the original instance object as the neutral record's exact shell before allocating CapturedReturn, OwnedShardReturn or OwnedActorTurnOutputs. The record covers this explicit return-domain contribution; it does not certify or charge every pre-existing instance/activation object. Installing that already-retained parent first prevents ledger close from refunding an apparently empty record while a later constructor has placed real children under the parent.

Then construct the CapturedReturn state and put it into the original activation.returned slot before either child facade finalizes. Its output slot starts null. The actual output roster constructor must install its original shell in that state before any fallible finalizer. The return facade similarly installs in state.facade before finalization, preserving its existing exact capture law. Missing constructor returns are recovered through these exact private slots. The first original arbitrary fault stays in the parent/cell; a different fault remains caller-owned. No refund or replacement is inferred from failure.

Replace the current per-return submit closure with an original-client pointer in the same CapturedReturn field position. Dispatch uses a module-private typed bridge to the original client's private implementation. This removes that per-return closure from the proposed inventory; it grants no public arbitrary submit callback. This change is not needed for evidence detachment and must not overlap that packet's captured hashes.

## Proposed Inventory and Grants

Use the actor packet's existing logical64-byte-record plus16-byte-field model, not the UI source's16+8 model and not UI's208-byte controller contribution. The conservative proposed fixed contribution is:

| Role | Fields | Logical Bytes | Slots / Owners |
| --- | ---: | ---: | ---: |
| Original instance's five-field return construction role | 5 | 144 | 1 / 1 |
| CapturedReturn state, submit replaced by client | 16 | 320 | 1 / 1 |
| Actual seven-field output roster | 7 | 176 | 1 / 1 |
| Actual return facade | 1 | 80 | 1 / 1 |
| Proposed domain envelope | 29 | 720 | 4 / 4 |

Neutral record264/3/3 and mandatory admission cell296/6/6 bring this proposed retained total to1280/13/13. This is not a physical heap measurement or all-source allowance. The actor neutral tests must validate actual AST fields and independently calculate these prices before implementation. No output slot, pending Promise/controller, raw4161 backing, framing/projection, fault payload, page, field or UI destination is hidden in720.

Proposed separately granted progression: bootstrap296; capture original cell64; claim64; claim observation64; reserve record264; capture original record64; install original instance64; installation observation64; state construction320; roster construction176; facade construction80; final observation64. Each consumed phase returns pending, never reuses its grant for the next child. Published repeat may return the exact source with zero work; changed capacity, foreign parent or faulted construction rejects without a replacement allocation. Actual work costs and boundaries must be source-bound by tests rather than assumed from this plan.

## Retirement and Dispatch Boundary

This first record cannot be refunded merely because the output roster is empty, cancelEmpty returned true, dispose posted, a route changed or the source facade is inaccessible. It covers live parent construction state, source state and the roster until their genuine original captured-return/lifecycle terminal join. Until that later proof exists the record stays installed and charged. No whole-Shard or arbitrary-fault cleanup is claimed.

Source reservation alone does not grant live dispatch. Actual per-request output-slot metadata, receiver external backing and parser/projection admission must precede postMessage, using the canonical one-response credit and binary response grammar. The current whole-result decoder is not an interim mounting path. The next packet must introduce or verify this guard before exposing the new reservation in production; controlled tests may exercise only the reservation and original constructor/refusal frontiers.

## Tests and Open Review

Declare neutral grant/census/phase vectors and actual-source tests for: every short grant; foreign/changed identity; one original pending cell under competing composition work; prepare/claim/reserve/install wrapper-after-actual-call faults; before/after state, roster and facade finalizers; no public neutral capability; ledger close while record installed; repeated admission; preserved original owner after operation revocation; zero worker posts from every construction test. Preserve the existing returned-envelope and private-field tests without claiming their old whole-result transport is the new live inbox.

The prerequisite pre-Open instance/activation allocation remains unadmitted in current source and is explicitly outside this subset. The plan needs a coordinated concrete callable release and exact schema before changes; it does not authorize an all-constructor or global memory-bound claim.

## Constructor Refinement Before Schema

The output roster is currently a separate module whose constructor freezes itself before a caller can install it. Avoid a generic installation callback or a cyclic structural parent protocol: make that constructor initialize only its fixed private fields, assign the returned shell directly to the already-installed CapturedReturn slot, then freeze that exact shell in the same176-byte construction phase. There is no fallible finalizer inside the constructor before parent installation. Existing low-level roster tests exercise the same class, not a compatibility implementation. CapturedReturn itself is placed in activation.returned before its fixed-shape Object.seal finalizer; sealing permits its declared mutable fields to advance. The return facade retains its existing constructor installation-before-freeze law. Actual before/after finalizer tests must prove each parent slot remains exact.

A fresh authored-caller census found reserveReturn only in actor tests and the UI-owned nativeInstanceFixture at UiDocumentStore613. There is no production content mount to update in this packet. The UI fixture owner must receive the phased callable/fixture before the signature cutover. Existing controlled transport tests remain separate from canonical receiver admission; no new live caller is added. The dispatch bridge change is direct original-client authority, not a public callback. The raw receiver/credit guard is still required before eventual production mounting.
