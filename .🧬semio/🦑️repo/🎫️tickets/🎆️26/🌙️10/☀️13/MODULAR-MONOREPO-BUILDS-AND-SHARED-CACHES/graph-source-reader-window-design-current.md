# Exact Source Reader Verification Windows — Proposed Separate Cut

This is a design assessment after the managed reader/cold Graph correction, not an implemented or qualified performance change. The current `observe` loop remains unchanged. Root observed that the real 64 MiB configuration aggregate law can exceed its existing per-case 25-second Source fence under fleet load; the failed logs remain Root-owned evidence. No fence, capacity, clock or Original is raised or renewed.

The actual reader retains one owned file descriptor opened only after finite extent admission. Its current `verify` checks every captured ancestor, the path and the descriptor before each 64 KiB chunk, plus EOF/delivery checks. Repeating full ancestry for every chunk contributes many system calls unrelated to which inode the descriptor reads.

A coherent next cut keeps full ancestry/path/FD checks before the first read, after EOF and before delivery, while each chunk checks the original opened descriptor stamp, bounded read length/bytes, current issued control and yield. Reads continue to use only the held descriptor and admitted offsets. A transient namespace swap cannot redirect that descriptor’s bytes; a lasting changed namespace still fails the full final verification. Descriptor stamp changes, shortened reads, growth beyond EOF and changed content must still refuse. A restored namespace must not become evidence for any path-derived Store authority. Real managed Store rechecks and accepted operation drain remain unchanged.

Before production changes, closed neutral window/race vectors must cover original versus substituted inode, temporary and permanent leaf swaps, temporary and permanent parent swaps, same-inode write/truncate/growth, empty input, cached facts, cancellation, exact Error/undefined, EOF interposition and descriptor-close primary preservation. An independent Node/SQLite or system projection must match the finite DATA laws. Actual ordinary descriptor race tests must demonstrate the current behavior, the proposed Source consumer RED and final refusal/delivery behavior. Positive managed Native/provider/store claims remain under HOLD.

Actual I/O work accounting must count the operations still performed, preserving every existing quota and admission bound. The change cannot erase prior charges, retry under a fresh phase, re-open a substituted path, return a partial digest or silently turn failed custody into absence/corruption. Full namespace and descriptor verification must finish before any successful delivery. This rationale does not itself prove the race implementation.

Observed reader SHA256 `69f227276eb00730a52d288bd14bf263593d0c27462b0cf1a09208b31b6d3e7d`, 14759 bytes. Defining policy SHA256 `3bb515be6be4314e43fd8026c9f8ccdcf97a33b2153a62970d00475203d7f54e`, 2627 bytes; the policy is unchanged in this managed-reader slice.

```json
{
  "version": 1,
  "maximumChunkBytes": 65536,
  "maximumPathUnits": 256,
  "maximumPathBytes": 1024,
  "maximumAncestors": 64,
  "maximumFilesPerClosure": 65536,
  "maximumUniqueSourceFiles": 262144,
  "maximumInventories": 16384,
  "maximumSourceFileBytes": 536870912,
  "maximumSourceBytes": 8589934592,
  "maximumClosureBytes": 134217728,
  "maximumTomlBytes": 33554432,
  "maximumMetadataBytes": 268435456,
  "maximumMetadataNodes": 262144,
  "maximumMetadataDepth": 64,
  "maximumReadOperations": 1048576,
  "maximumBatchFailures": 3
}
```
