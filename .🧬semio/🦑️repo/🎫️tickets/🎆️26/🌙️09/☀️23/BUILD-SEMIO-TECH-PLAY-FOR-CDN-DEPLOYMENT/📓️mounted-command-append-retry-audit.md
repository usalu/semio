# Mounted Command Append Retry Audit

Root read current MountedOwner/PlainGroupOwner/ReceiptSet/CommandEntry production paths after the earlier readiness fault. A stale command append is validated before output/command handoff. That refusal returns the owner to phase8 with original preborn receipt/command ownership still present.

The shared commit is synchronous under the same mutable app/graph borrow and retained keyed publication claim; its fixed grant is one item/4096, exactly the Group commit guard. Current ready/projection/entry stages prepare the future view before common commit. There is no await between final append validation, preborn row/output handoff, common decision and command append. Thus the suspected record-already-taken stale retry is not an observed reachable transition in this production path. No source repair or grant changes were made from this suspicion.

The actual runtime issue remains the mounted publisher90-second stall from native39182; Editor owns the bounded fixture-only phase witness and coherent retry. Current final acceptance must still prove all-member atomic flip, genuine operation/outbox receipt, retained ACK barrier, and controlled original closure. Primitive/source audits are not substitutes for that runtime.
