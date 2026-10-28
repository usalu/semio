# Native Scalar Control Retirement Independent Audit

Source-only review of declaration/view/staged-form/control.rs and adjacent payload foundation. No native execution/type-check proof: authentic cargo refuses before rustc.

## Confirmed P2, corrected in source

control.rs close previously forwarded child OriginalBuiltTreeRetirement Complete while retaining Some(Box<...>) in self.retiring. Its own terminal_is_empty was false. A direct caller stopping on Complete could drop the still-owned cursor and hit its paid-retirement assertion before releasing the funded box.

Worker acknowledged and corrected this during review. Current line76 returns Progress for every child step, including child Complete; the following turn releases the terminal box with declared release/copy receipt, and only an empty cursor returns Complete. staged-source-tests.rs close_control now loops on the public Complete result and asserts terminal_is_empty at that boundary, rather than concealing mismatch by prechecking emptiness. This is source closure only; authored native regression remains unexecuted.

## Other bounded checks

Scalar choice count refuses above32 before Select construction. Choice duplicate scan is bounded32; scalar UiText conversions refuse unavailable representation. Binding args move into resident owner only after reserved binding capacity exists, and push refusal restores args. Status moves payload into Extension owner. Partial node and retirement box remain retained across original close. RetireOwned scaffold exposes birth/release demand. Existing payload correction reports pending typed-retirement copied_bytes equal to preflight close demand when progressed; payload maps cap32 before arena birth.

The scalar cursor pins owner/address only and explicitly requires enclosing original render to own immutable completed spec metadata and post receipts. This is a caller obligation, not authentication of arbitrary mutable specs. Full body/producer/mounted route is absent; no browser or native guided-form completion claim follows. Exact physical capacity/copy sufficiency and child allocator behavior remain native verification gates.
