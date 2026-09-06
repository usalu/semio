# Browser Actor Child Containment

## Current Result

Registered `os-hub:browser-actor-child-worker-containment-check` passed in real
Chromium, session54763: strict focused TypeScript=1, AJV=1, base laws14,
hostile child-wire laws13, owner fault laws9, actual transferred buffers2,
forced tight-loop terminations2, final reservations actors0/bytes0.
The actor modules are deliberately small synthetic containment probes, not GIS.

The schema/fixture and three first-party runtime files live under
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child`.
The permanent gate lives in the existing Hub `📜️script.ts`; its Nx target
and launch seed/generated entry are registered. The launch entry provides
the ticket artifact directory. Vite has no app/Hub/relay/Cargo dependency.
Its cache is ticket-owned. The latest source disables Vite watching so peer
tsconfig/build activity cannot trigger unrelated fixture cache invalidation.

## Ownership And Containment

The parent reserves one of two exclusive child slots and intentional byte
credit before constructing a fixed first-party module Worker. A private
MessagePort handshake binds a random nonce and activation generation. Boot,
load and invocation deadlines are parent-owned; one invocation is allowed.

A shape-valid `load` call synchronously transfers the caller buffer into a
private source owner before its first hash await. Cancellation immediately
settles the retained promise and wipes that private source. After parent
verification, the exact buffer is transferred to the child and rehashed
before Blob import. The child revokes its Blob URL and wipes its source after
import; forced termination is the fallback for non-cooperative module code.

Input/output validation accepts a bounded first-party structured-clone value
subset, denies shared/resizable/sliced/aliased owners, charges framing, and
rejects oversized input before transfer. A result is held until its exact
sequence's transfer observation arrives. Every parent terminal path aborts,
closes ports, terminates the Worker, settles retained work and releases credit.

The child host bridge denies effects. No Hub origin, credential, receipt,
lease, broker, path selector or parent callback is sent to the child.
Intentional buffer accounting is not a browser engine heap limit or security
sandbox. The child can run only within its normal browser origin capabilities;
no stronger confinement is claimed.

## Executed Progression

- 92052: expected preimplementation RED, owner module absent (404).
- 47580: initial real Chromium GREEN14.
- 26038: schema RED from AJV strictRequired on conditional wire-row fields;
  schema branches now declare those fields explicitly.
- 62449: Chromium GREEN14 + wire13 after consuming-at-call ownership change.
- 15857: Chromium GREEN14 + wire13 + faults9.
- 54763: same36 plus strict focused production TypeScript GREEN.

Wire rows are emitted by the actual child using hostile probe modules: wrong
nonce/generation, duplicate ready/loaded/result/transferred, extra field,
old/future sequence, non-finite/aliased/oversized result and detached-count
mismatch. Owner fault rows run in Chromium with restored native seams:
constructor failure, malformed init/boot deadline, load/invoke post failure,
abort while parent hashing, synchronous source consumption, malformed child
load/invoke, and valid-JavaScript byte substitution after parent hashing.

Registry check-generated15859 was RED only for unrelated generated
`🔌️plugins.json` and `🧩️plugins.ts`; launch bytes were fresh. This is not a
full registry pass. Scoped diff-check passed.

## Remaining Admission Boundary

This is not mounted to Backbone. The reservation function deliberately does
not constitute catalog authority: it takes an admission record and private
bytes from its caller. Before production use, the retained document lease
must be the sole owner deriving actor identity, policy, exact scope,
runtime-key exclusivity and activation generation. Child retirement must
precede lease wiping on revocation, reconnect and document close.

No actor body fetch/route was added and no real GIS activation, host effect,
Map commit, undo or MCP end-to-end result is claimed. The original goal stays
active. Terra's child audit and P4 MCP audit remain required follow-up inputs.

## Adjacent Native Failure Found By Execution

Identity86940/EKeoNp passed replication13 and three kernel laws, then failed
the full-field lease law: `manifest-component.byteLength` was self-admitted.
The retained socket authority had projected byte lengths from the candidate.
The fix retains the successfully admitted local lease and compares every
candidate field against that exact record before projecting plan identity.
An authority without an admitted lease cannot claim a full-field match.
Rerun59496/TUOA0T is GREEN17: replication13, kernel4. Replication executable
SHA-256 `cab29a8d500747f3d98d6c306e6ac373de6739609adefeaaafd5b5e3c6547a3e`;
kernel `38d30d5a9295effd24f83233994d255b42e225f877c43062ff715e2dfac813d1`.

After that pass, the source gained a public validating `from_plan` constructor
for external `HubSocketGrantSource` implementations, while `admitted_lease`
remains sealed. Production constructs before consuming the receipt and
rechecks expiry after exchange. The native target now includes four additional
existing real client admission/cancellation/mismatch laws (21 total). This
constructor refactor is source16557 GREEN, rustfmt/diff clean, but its expanded
native gate is queued with WG and must not inherit the earlier17-law result.
