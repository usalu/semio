# Windows Cleanup Metadata Current Inventory

Read-only Source inventory, no runtime/store effects. Current fixed/🟦️.ts13 syncFixedCleanupMetadataV1 still embeds Get-Volume -FilePath for a directory. This is separate from the corrected ready/maintenance syncNativeFixedImageMetadataV1 path.

The sole direct call is fixed/🟦️.ts74 captureCleanup→withFixedNativeImageCleanupRead→NativeStorageCleanupAccess.syncMetadata. NativeStoragePort interface exposes optional captureCleanup; persistent Mac owner implements its own retained-cleanup access. Current non-test framework census finds captureCleanup only in those two implementations and the interface, with no current production consumer invocation. Thus this read establishes an exposed retained-cleanup path gap, not that public preparation already executes the unsupported directory query.

When used, the cleanup operation must retain exact accepted cleanup enrollment/loan/physical backing authority and drain through its own cleanup command owner; do not route through ready borrower admission or create a self-close cycle. Replace directory query with actual retained exactroot mount mapping plus bounded physical root→target noalias/devino chain pre/post, as agreed for ready metadata. Numeric zero Flush, bounded receipt and mapping checks remain. Current Windows physical behavior remains unqualified.
