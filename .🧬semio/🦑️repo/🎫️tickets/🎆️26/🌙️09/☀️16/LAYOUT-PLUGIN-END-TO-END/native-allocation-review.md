# Native Construction Allocation Review

Read-only bounded audit of Rust paths declared by rust-working-files.json, mutations-working-files.json (createdUpdated), owned-emission-working-files.json and typing-original-working-files.json. No compiler/tests/code edits. Current source timing includes Rust fixes already removing direct constructor charges.

## Defining Contract

Framework job/construction admit_original_construction preflights owned_bytes + demand.capacity_bytes, invokes the callback, then unconditionally calls control.record_constructed_allocation(progress.retained_capacity_bytes), including error receipts. Therefore callbacks returning native capacity receipts must not have already charged that same capacity on that same control. Native payload operations may use their own receiving interface; they cannot be treated as unaccounted Box/String births merely because they return progress.

## Confirmed Same-task Offender

Plugin job/emission/host source lines95–99: phase4 wraps parser.step_source(text,1,control,grant,true) in admit_original_construction and returns parser.normal_step_progress. The actual JsonGrammarCursor step_source (pack/json/parser-and-grammar-cursor lines108–125) invokes advance on this supplied NativeDecodeControl. Its json_candidate_slot lines53–54 charges bytes before reserve_one and reports the allocated bytes in receipt.retained_capacity_bytes. Its string allocation branch line164 likewise charges scan.bytes and reports scan.output.capacity. The outer construction helper records this same receipt capacity again. This is an indirect duplicate, beyond the direct constructor charge fixes. Source_lifetime and root were notified immediately. Preserve genuine parser admission/control accounting; use a receiving contract suitable for already-accounted native progress rather than deleting parser charge blindly.

## Inspected Non-offenders

Current OriginalPressSettlement birth lines25–28 creates the retained Box/vector/actor/base and returns its capacity receipt without native.charge. ToolTagCapture receive line13 likewise returns its Box receipt without an extra charge. OriginalActionMetaCopy birth line13 and source admission line17 use construction receipts without direct native charge; its cursor advance line18 instead calls native.receive_construction_step directly and charges in that separate receiving callback, so it is not wrapped by admit_original_construction at that site. Typing commit/dispatch constructors and emission captured-source constructors read native identity but do not additionally charge their returned construction capacity. Host birth line50 currently has no direct charge. These are source observations, not runtime allocation-law passes.

Window config store-owners line142 calls OriginalRetirementControl.charge during a controlled preview retirement path; it is not the NativeDecodeControl construction callback under investigation. No duplicate-construction finding is asserted there. No unrelated existing callback defect was confirmed during this bounded review.
