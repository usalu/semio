# Coordinator Shared Resident Pool Review

Root read the complete initial 141-line resident pool implementation, both neutral fixture files, the two authored tests and the ownership report on 2026-08-27. The pool's initial two-test GREEN is executor evidence, not yet a separate root run. Later roster and lifetime-witness integration remains in development.

## Accepted Primitive Scope

Explicit capacities cover logical resident payload bytes, page slots and live handles; no defaults or maxPatchBytes-derived lease. A short page still reserves its full 256-byte allocation before allocation. Aliases keep that page charged until the final handle scrubs the fixed page and separately releases metadata. Reads remain available to a live alias while parent admission is revoked. These are logical resource counters, not physical JavaScript heap-size or GC timing certificates.

## Required Strong Ownership Join

The initial implementation has upward parent references and child counters but no strong downward child roster; the pool's instance lookup is a WeakMap. If callers abandon or throw after receiving a payload or page, counters alone cannot recover the actual descendant for bounded close. Therefore this primitive is not sufficient authority for final OwnedUiInstanceRetirement and is not approved as a live composition mount.

The UI owner accepted the required follow-up: retain accepted payload builders and writer/reservation page owners in a bounded-step exact-instance roster before exposing them. Parent close may retire parent-owned writers after caller failure, but separately issued read aliases must remain usable until the real reader closes. Cancellation must stop further admission without losing roots. The final instance witness must wait for the actual roster and alias retirement, not only an externally reported count.

The separate instance maintenance regression must preserve blocked/rejected/over-grant results, thrown callback details and the original queue head. Child completion cannot spend an uncharged parent rotation in the same grant. The owner reports actual missing/incorrect-behavior REDs and targeted GREEN; root will verify the coherent combined release separately.

## Integration Boundary

The peer retains composition injection and canonical return field/fragment/release ownership; UI retains the shared resident pool, exact instance attachment and destination payload builder. Raw page input retirement, destination ownership and UI publication acknowledgements remain distinct. No public structural token or raw page length is accepted as ownership authority.

