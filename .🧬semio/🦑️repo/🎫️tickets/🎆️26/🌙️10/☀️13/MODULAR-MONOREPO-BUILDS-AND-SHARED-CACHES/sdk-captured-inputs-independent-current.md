# SDK Captured Inputs Independent Audit

Source-only review, 2026-10-15. No tests, probes, runtime, image reads, or Source edits.

No concrete actionable defect established in the inspected captured-input change.

`sdk/🎟️capacity/📥️physical/🟦️.ts:23-25` validates each permitted input path, uniqueness, count, Uint8Array type and cumulative byte ceiling before copying that input. Private files and their byte count reside in a WeakMap. The public frozen facade returns fresh Buffer copies through get, entries, values, iteration and forEach; keys expose only immutable strings. Mutating returned bytes or the original input Map does not modify private files. It is not a mutable Map instance exposing set/delete/clear.

Line 47 requires that exact privately issued facade and passes its private byte count to the derivative formula together with privately authenticated projection facts. A copied facade, forged public numeric inputBytes, or independently called pure formula cannot admit a physical writer. `formula/🟦️.ts:12` requires the derivative schema and strips the input field before invoking common material headroom validation. The frozen quote's kind selects the formula inside its private issuer, rather than trusting a public claimed kind.

Physical lines 40-41 admit writeInputs only under the exact live publication operation, exact store/control, genuine live provision EX and exact quote receiver. The initial stage must be empty, canonical, under store.root/nx and named .stage-*. The admitted stage device/inode and path are pinned and rechecked. Writes use exclusive creation and readonly input mode. One synchronous consumption flag prevents a second input write operation. Captured private files are the actual bytes written.

Accepted writeInputs promises enter operation.pending before returning. Physical admission drains those promises after its callback, including callback failure, before its registry callback can settle; issuer retirement waits publicationWork. The draining stage checker intentionally does not require operation.live after acceptance, while it still requires quote/store liveness and actual registry custody. Detached new operations are rejected at method entry after operation.live retires.

The common 64 publication-entry allowance still exceeds the inspected extra namespace: at most 32 input files, one patches directory, stage directory, source index and its temporary, captured snapshot, compiler program, final authorizer temporary/final rename, binary/tooling receipts and receipt temporaries. Copied SDK tree entries remain separately charged. This is a Source accounting assessment, not a filesystem-allocation or runtime success claim.

Runtime and callback-failure drain regression outcomes remain unqualified in this independent review. Concurrent hostile stage replacement can cause refusal; actual before/end inode checks do not imply an OS transaction over all per-file writes.
