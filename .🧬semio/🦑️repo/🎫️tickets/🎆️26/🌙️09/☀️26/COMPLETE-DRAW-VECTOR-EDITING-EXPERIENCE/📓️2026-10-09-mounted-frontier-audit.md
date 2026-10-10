# Mounted Original-Owner Frontier Audit

Date: 2026-10-09. Read-only source audit inside the existing OPEN Draw ticket. Source files were being edited concurrently; line numbers below describe the final reads during this audit. No source edits, Git mutations, ticket changes, native compile, or runtime tests were performed.

## Findings

### P1 — Plugin close cannot satisfy its own terminal witness

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, `close_retirement_demands` at 34324, `close_step` at 34368, and terminal witness at 34392.

The lifecycle implementation only selects direct window ingress, granted mounted owners, tool runs, time travel, overlays, and document/config/draft/interaction store retirement. The retained `close_owned_stage` field is initialized to zero (26643), required to reach eight by `close_terminal_is_empty`, and has no remaining assignments in the file. Therefore even an otherwise empty app cannot reach the terminal witness. The demand method eventually returns UnsupportedOwner instead of admitting a stage transition. The same close implementation lacks an operation-turn route for Worker/Publishing/AwaitingAck entries and lacks the final mounted operation/lease removal route after the four-axis payload owners drain. Existing `retire_typed_operation_unit` performs that latter work through publication continuation, but close does not call it.

Required repair: restore actual grant-controlled lifecycle transitions for every remaining retained owner, including cancellation and worker handback, mounted final removal, window/presence/transient owners, and close-stage advancement. Do not weaken the terminal witness. Validate an empty-app close and a close while a mounted operation remains in each stage, with receipts and final owner emptiness.

### P1 — The new retirement frontier bypasses rejected-publication outcome custody

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs`, lines 17–18; root `🦀️.rs`, `retirement_turn` at 21975 and closing publication return at 32907.

A Closing pending publication is now drained through `granted_retirement_step`, which calls `close_step` directly and later takes the terminal owner without capturing or conveying its rejection. The publication pump merely returns while that owner is Closing. The existing `retirement_turn` helper deliberately remembers whether the store rejected the publication, propagates rejection after terminal retirement, and gives only WindowTransient the explicitly documented lenient outcome. It has no remaining call sites. Consequently a store-generated rejection that transitions into Closing can lose its final operation fault when maintenance removes the pending owner, allowing the remaining ladder to proceed to a success terminal. Upstream failures that already returned an error are a separate case; this finding concerns the terminal rejection outcome owned by the pending store cursor.

Required repair: retain an explicit rejection outcome before draining its physical fault payload and carry that outcome to the mounted publication state after actual owner/frame release. Preserve WindowTransient's intentional policy. Add a rejected artifact/config/window-config publication law that runs maintenance between publication turns, checks exactly one fault terminal, and asserts no success terminal or unintended commit.

## Source evidence that is sound within inspected scope

- Mounted completion and output originals enter inline ControlledRetirement carriers, whose `new` method creates no retirement scaffold. Their subsequent demand methods expose work/capacity/release/depth; the unchanged original grant is forwarded to their step methods.
- Draw declares its own completion issuer at editor root lines 1693–1694. Its completion cursor covers document/window-config mutations, transaction fields, supported effects, interaction writes, window transient, download/fault, and physically retained empty containers.
- Completion aliases use explicit Arc return; last-cell return extracts the original payload and forwards it to the app issuer. Unsupported payload authority remains retained instead of being shallowly dropped.
- Pending WindowConfig/WindowTransient publication Box frame release is a separate frontier turn after owner terminal emptiness, priced by `size_of_val` of the concrete erased owner. It is not folded into the prior body release receipt.
- Mounted terminal emptiness includes completion, publication, and output retirement carriers. An unacknowledged result page prevents frontier admission.

These are source observations only. They do not establish native compilation, heap receipt correctness, cancellation runtime behavior, bounded completion, or actual terminal closure. The parent reported a separate native Kernel import failure being repaired; this audit did not execute mounted native laws or repeat native compilation. Concurrent fixes must be re-read and validated before relying on either finding as current.
