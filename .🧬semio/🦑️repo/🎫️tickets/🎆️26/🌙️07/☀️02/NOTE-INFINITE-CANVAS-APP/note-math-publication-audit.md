# Note Add Math publication

The first browser acceptance sequence added two text blocks, undid the second add and clicked Add Math. The add reported `interactive-job.publication-stalled` at 4,096 scheduler units. A later document refresh showed one text block and the newly added math block. The standalone Math action succeeded in a fresh browser. The exact-selector reproduction also stalled after adding Text and Table and undoing Table.

A browser run with guest diagnostics reproduced the stall after the same exact selectors. Math operation 512 held the completion, publication and pending artifact owner, with no result page. It remained at publication checkpoint 6 for 4,096 idle observations and then returned the stall fault. The working Text and Table operations also paused at checkpoint 6 and then continued. This localizes the false stall to the generic publication path. The Note command and Note one-item preparation remained unchanged.

The stalled witness previously counted only changing Store checkpoints during this arm. It now also counts nonzero paid progress from framing or retiring an acknowledged result and from the maintenance ladder retiring an acknowledged Store owner. Denied or zero-progress turns still leave the witness unchanged. The neutral evidence covers paid item or release progress, empty receipts, awaiting ACK, and final retirement.

The independent SQL oracle and Tree-sitter validation passed. The Rust law and the browser run with a guest containing the correction remain to be verified.

The exact witness was `stage=1 flags=0b01011100 progress=6 child=None group=None`. The set bits identify completion, publication, pending artifact publication and worker terminal. The clear result-page bit identifies a period before delivery or after acknowledgement, when paid closure can leave the operation in Publishing. The retained detail does not distinguish every substage of that period. No default grant, stall ceiling or command semantics changed.

- [Mounted publication witness](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs)
- [Paid Store retirement](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs)
- [Paid publication pump](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🏃️mounted-run-and-pump/🦀️.rs)
- [Neutral progress cases](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/progress.json)
- [Mounted Rust law](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🦀️.rs)
- [SQL and Rust syntax oracle](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🟦️.ts)

The follow-through traced the recorded Publishing state to the advanceable cursor. A unit waiting for a host acknowledgment normally enters AwaitingAck and leaves that cursor. The recorded witness contains ownership and checkpoint information. It does not identify the receipt substage or the active grant, so it supplies no independent evidence of a remaining acknowledgment or grant blocker.
