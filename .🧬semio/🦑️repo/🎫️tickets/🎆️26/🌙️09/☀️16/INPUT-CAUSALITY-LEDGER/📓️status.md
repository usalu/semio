# 🎯️ Input Causality Ledger — status

Ticket opened 2026-09-16 by session ⚪37cccb68 (Opus 5). Start commit: `🗑️generated/start-commit.txt`.
Origin: fem2d "swallowed click after marquee" report; investigation refuted the busy-drop hypothesis on the
direct-actor path and found the real gates (see `📓️design-input-causality-ledger.md` §0).

## Definition of done
1. Laws L1–L7 of the design hold, with pure law tests for the ledger, CAS registers, sample batching, refusal surfacing, causal dequeue and the browser-actor queue.
2. The 8 s `setActiveUtility` echo-off gate and the `pointerleave`→`pointerUp` forgery are deleted; no `performance.now()` remains in `onAction` semantics.
3. Interaction verbs emitted by a guest fold in-reactor (one round trip per pick/hover/marquee, no host re-dispatch).
4. Acceptance probe (design §6) green on the fem2d React lane and on one hub-mounted lane.
5. Crates green natively + wasm32-wasip2; existing laws green.

## Log
- 2026-09-16 investigation + design written (phases 1–4). No code changed yet.
