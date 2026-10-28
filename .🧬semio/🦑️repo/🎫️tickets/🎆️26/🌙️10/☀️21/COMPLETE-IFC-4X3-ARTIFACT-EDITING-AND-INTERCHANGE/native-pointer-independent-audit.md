# Independent Native Pointer Frontier Audit

## Result

Read-only source audit found one P2 copy-frontier defect, reported it, and verified the worker's source correction. No outstanding additional confirmed P1/P2 at this checkpoint. Native compilation, cancellation/receipt execution and final registered-host behavior remain unverified. This is not approval of complete native details or retained semantic admission.

Reviewed details/pointer.rs, pointer_tests.rs, neutral pointer.json, host-pointer-plan.md, mounted work.rs admission/close references, and native value_edit.rs Move destination protection. Coordinated directly with the owning worker; no source changes were made by this reviewer. No authentic compiler bypass or native execution occurred.

## P2 Found And Corrected During Review

Before correction, pointer.rs demands() Phase::Scan quoted one Range, but its 64-byte scan loop could write many distinct ranges before yielding (up to 64 separator-only segments). That exceeded the copied-byte frontier even if only distinct retained destination range storage is counted. The denial tests used one long key, so they did not expose the separator-heavy case.

Reported to root and worker. The worker changed the slash branch to increment offset and break immediately after publishing one Range. Reviewed current source at approximately lines 184-195: it now stores at most one Range per turn; end-of-input also stores at most one. The worker is adding a separator-heavy exact/short-loan native regression, with execution explicitly false. The correction is source-inspected, not runtime-proven. Borrowed pointer/key reads are comparisons rather than materialized payload copies; Index/Key retain fixed scalar/ordinal frontiers.

## Pointer Semantics

- Pointer retains raw source byte ranges and inline ordinals, without segment string allocations. Scan validates the complete RFC6901 syntax before decoded() is used. It recognizes only ~0 and ~1, with no double decoding, and permits empty segments. UTF-8 keys compare byte-for-byte; Rust str already guarantees valid source encoding.
- Existing mode requires resolution through actual canonical-tree children. Object keys are compared as keys, while array indices require ASCII digits, canonical zero spelling, checked usize arithmetic and admitted bounds. A dash is accepted only for final array insertion. Object insertion admits a new final member while requiring existing ancestors. Rename requires an object member; final array members refuse.
- Segment capacity is 128; a 129th segment refuses. Traversal reconstructs through at most the retained ordinal depth per turn; scan/key/index/Move-prefix loops use at most 64 source bytes per turn.
- Move deliberately uses Syntax for destination, Existing for source, nonempty source/destination, and decoded segment ancestor comparison. Identical source/destination is permitted, strict descendants refuse. Destination parent resolution is deferred to reduction, matching the existing structural admission contract. The neutral /missing/child accepted case is therefore not an RFC6902 success assertion. Inspected value_edit.rs rejects collection-owner mismatch and out-of-bounds Move before rebuilding; cross-owner/whole-entity destination reduction remains a broader host test obligation.
- Omitted empty complex insertion explicitly admits only /entities/N/complex/0 or dash when the selected entity has no complex parts. Virtual reference fields are structural editing addresses; actual declaration, reference target and payload checks remain downstream obligations. The semantic /ifc- hook is bounded to the existing small editor-address interface and is not used to allocate/parse ordinary native paths.

## Custody And Control

Pointer binds root object address plus original path pointer/length. PathAdmission additionally binds the original event and snapshot addresses and verifies them before returning cached results. Cancellation and close state checks precede work. These identity checks rely on the enclosing original host preserving immutable contents; they are not content hashes or proof against in-place caller mutation.

PathAdmission quotes its scalar overhead together with the child pointer frontier, consumes overhead, and lets the child consume its item/fuel/copy record. Completed calls return without re-debiting. begin_close prevents further admission; close_step checks the actual grant before clearing retained pins. The inline arrays own no heap payload. Denied/cancelled ownership and actual receipt behavior still require authentic native execution.

Authored tests cover escaped UTF-8 and empty keys, canonical indices, append/bounds/overflow, virtual references and empty complex parts, original identity rebinding, semantic-hook invocation, denied grants/fuel, cancellation prefixes and close underfunding. Reading these tests and worker-reported fast-json-patch results does not execute native Pointer or prove native receipts. No native output marker was fabricated.

## Native Axis Regression Spot Check

Re-read the new native-gltf-export.rs paired Axis/RefDirection admission and original_ifc_axis_cases_admit_only_normative_three_dimensional_frames test. Flagged its initial InvalidEncoding expectation; root corrected it to InvalidValue matching invalid(), removed conventional serde_json::Value from controlled retirement, and added the actual shared model IfcEntity import. Current inspected test uses that exact entity model and existing IfcValue types. No further obvious import/API mismatch was established by this spot check. Rustfmt parsing remains distinct from type checking and execution.

## Remaining Verification

Execute authentic native compile/tests once the enforcing ingress is available, including the separator-heavy frontier regression and real host-event failure cases. Validate final reducer rejection for deferred invalid destinations and virtual target semantics, actual no-heap/receipt/cancellation/close behavior, and registered original revision/history publication. Existing eager action/reduction and semantic-machine gaps remain open, with semantic template admission refused.

## Separator-Heavy Regression Follow-Up

Re-read the mounted original_pointer_separator_scan_quotes_one_actual_range_and_denies_independent_short_byte_loan test after the source correction. It independently derives the neutral frontier from two Range words, three scalar words and one boolean/phase byte, checks it against the production quote, denies one byte less with unchanged state/receipt/heap, grants the exact frontier, and checks at most one published range per turn for both 128 and 129 separator paths. The worker made Phase repr(u8), making the byte-size contract explicit. Tests remain authored/unexecuted native evidence; they close the missing regression specification, not runtime proof. The single-range Scan source correction remains inspected and no new defect was confirmed in this regression.
