# Original Export Audit

Read-only audit of the actual native export builder, retained Drawing ownership, queue hook and core original-work boundary. Native compilation and execution remain UNRUN because the required repo binary is unavailable. Source reading is not compilation.

## Results

No confirmed API/type mismatch or ownership blocker was established in the inspected current sources.

- Queue transfer is idempotent: retained export receiver remembers output_received; the shared work hook can call it again after producer_output_chunks becomes None without replacing the queue. A replacement Some queue is refused.
- Actual snapshot authority is retained through snapshot_read.take_current and SnapshotRead::admit_retained_clone_source; the parent pointer is compared with the actual work input before child lookup. Child lookup, qualification stamp and custody stay in DrawingOwners.
- ExportOwners declares child drawings before parent source and original snapshot custody. The outer close path fully closes the core and checks drawing_consumers_terminal_is_empty before releasing its inner frame or retiring those sources.
- The retained Drawing wrapper header is included in its phase demand and consumed before each phase operation. Provider issuance and source routing use paid to post success and refusal receipts. Parent-source admission preserves a rejected original lease and posts refusal progress.
- The fixed 65 source addresses cover Document plus 64 pages. This is consistent with the existing MAX_LAYOUT_EXPORT_PAGES=64 contract: from_admitted starts in Validate; validate-plan-and-svg rejects excess pages before Plan, while required_drawing_source only returns a source during Plan with an active drawing. The literal 65 should derive from the shared constant to avoid future policy drift; this is a maintenance concern, not evidence of a new page limit.

## Verification Still Required

Compile the native target through repo, then exercise actual factory/queue transfer, boundary retained grants, cancellation during child preparation, and closure after a child read refusal. This audit does not certify runtime grant behavior, generated RetireOwned expansion ordering, or successful Rust type checking.
