# Base Native Borrow Validation Versus Paid Copy Cancellation

Read-only actual control/base source; no execution.

Base binary native text20 invokes NativeDecodeControl.borrow_text then copy_text. Borrow validates immutable UTF8 in at most64KiB plus scalar-boundary extension, reports total=raw length/completed progress, owns no new text. copy_text subsequently charges complete text length, reports initial0, reserves exact String, copies boundary-safe spans and reports actual copied bytes. Thus canceling the first schema-sized interior progress cancels uncharged borrowed validation and cannot establish paid owner backing. Current production refusal/admission settlement is consistent with this path.

Exact binary paidcopy witness can observe one completed==schema_bytes validation frontier, then cancel a subsequent matching schema total with interior completed>=65536<total. This keeps actual primitive decode and existing admitted>=schema_bytes assertion; it changes observational stage authority, not production behavior. A distinct borrowed-validation cancellation law may assert no paid schema copy occurred. Do not make all borrowed validation artificially charge output storage.

Text hex_text differs: hex first allocate_vec(value.len/2) under actual finite control, begins that decoded-byte stage, writes one byte per pair with control.step, then validates borrowed UTF8 and transfers same Vec into String. Its first hex schema-sized interior is already paid. Preserve encoding-specific witness rather than counting an invented universal callback ordinal. Native progress exposes owned_bytes internally but current Base adapter forwards completed/total only; no compatibility API is required to select the actual two-stage binary observation.

Allocation stage always returns native.owned_bytes alongside normal success/refusal/cancellation. Exact paid copy cancellation should retain the same full credit settlement and real typed Canceled refusal. Physical exact allocator/retirement evidence remains separate from the progress assertion.
