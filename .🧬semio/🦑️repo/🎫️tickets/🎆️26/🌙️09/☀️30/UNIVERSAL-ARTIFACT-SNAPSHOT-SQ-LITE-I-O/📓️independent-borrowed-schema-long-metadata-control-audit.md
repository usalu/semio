# Independent Borrowed Schema Long Metadata Control Audit

Read-only before/after and actual preflight/schema provider. No execution by auditor.

Enum canonical ordering remains ordinal then literal UTF8 label; statement ordering remains literal UTF8 keyword then original index. Only comparator/sort control changes. Exact field key, enum labels, statement words, quantity/angle unit symbols and ref/embed strings now compare through existing paged compare_text under the same NativeEncodeControl. Its byte comparison and length tie-break preserve Rust str ordering, including NUL/Unicode, while reporting at most65536 bytes per page. Fixed scalar field-id sorting remains bounded256 inline entries.

Graph edge discovery now passes actual same control through shape traversal and statement sort; every shape visits a control step. Record frontier64, variant/field frontier256 and declared depth64 bounds remain explicit. No metadata cloning or new heap frontier is introduced. HashSink still hashes exact original byte slices in the same order, but begins a byte stage and advances actual part length instead of one unit per chunk. Existing BLAKE3 hash recipe is unchanged. Controlled target lookup errors replace previous unwrap in statement comparison rather than erasing the failure.

No static canonical-order or source-control regression found in this narrow provider. Author reports genuine Native RED270856ce with original2passed/new2failed, Source4/4 541assertions and new AFTER pending. Those author reports are not this auditor's executions or independent receipt reread. Same actual context and exact long metadata cancellation need the pending owning Native selection before runtime credit.

Subsequent independent receipt readback of physical-record-metadata-native-after.log confirms actual Nextest abb4e088-c716-4347-a0cc-179c6f98aaa9,4/4 passed,121 skipped,2.545seconds. This closes the pending four-law owning selection, without claiming universal breadth or small-stack behavior. This auditor read the completed result rather than executing it.
