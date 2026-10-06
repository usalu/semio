# Operation Pages Held Core Sink And Borrowed Reader Audit

Read-only held core-streamed-encoder.rs, borrowed-byte-reader.rs and script core branch. No provider execution or mount credit.

record_body_into borrows the caller's external output, retains controlled symbol scratch in the existing Encoder, measures before emission and applies the same caller maximum and file ceiling. The script keeps pure owned encoding unchanged by adding external=None to its Output constructors. External write_bytes receives the same NativeEncodeControl; if writing refuses after an accepted prefix, the borrowed caller output remains owned outside the failed function. Output length equality only applies on success, so a refusal does not fabricate a complete byte count.

The upfront measured logical length versus remaining allowance is conservative; actual paged backing and symbol allocations still consume the control independently. It is not proof that logical length accounts for metadata capacity. Caller output may already contain bytes, so the owning operation API must explicitly define append semantics or require empty output; this function currently appends without a pre-existing length contract. Control progress/terminal owner tests must cover that actual policy.

ByteSpan stores only borrowed Slice or the concrete OwnedOperationBytes lifetime with private checked offset/length. from_source cannot issue an arbitrary owned source or materialize a whole contiguous Vec. read_span preserves that borrow; fixed numeric reads copy bounded stack arrays. read_bytes refuses paged input before advancing its cursor. Core consumers requiring contiguous slices must be coherently joined to spans; do not treat the new reader alone as full arbitrary operation decode support.

Remaining gates are actual Core type/Native demand, coherent first-party required OpBinary emission/reader caller joins, and exact output prefix retirement on cancellation/refusal. No source-only allocator or transport credit is inferred.
