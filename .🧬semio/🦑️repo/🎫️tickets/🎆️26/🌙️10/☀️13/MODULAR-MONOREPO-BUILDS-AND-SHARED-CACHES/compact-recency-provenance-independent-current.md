# Compact Recency Provenance Independent Audit

Source-only review, 2026-10-15. No tests, image reads, probes, runtime or production edits.

No actionable authority or guard defect established in the inspected diagnostic change.

`pruning/🟦️.ts:114` obtains optional provenance from the same single bigint lstat supplying that measurement, rather than recapturing timestamps later. Its conversion at line 9 splits nanoseconds into whole seconds plus fractional milliseconds, preserving fractional timestamp information rather than truncating bigint atimeMs. Size is converted to Number as in the former number-stat measurement. The aggregate remains max(file atimeMs,mtimeMs), and byte counting, directory/symlink exclusion and zero starting maximum remain unchanged (`:127-139`). This Source review does not independently certify cross-platform runtime equivalence of Node's number Stats construction.

Maximum-source selection is deterministic on equal maxima by path/field (`:10-11`), retains only one source per measurement, and does not replace the aggregate recency. Build and incremental scanners retain bounded optional copies. Each CacheWalk caps retained provenance at 1 MiB; each serialized source is capped at 2048 bytes and relative paths are capped by character and UTF-8 byte checks (`:116`, recency policy). Repeated retention of a session source can exhaust the diagnostic pool earlier, but merely drops optional provenance; it grants no deletion authority.

`budget/⚙️execution/🟦️.ts:50` still refuses missing units, changed bytes, changed aggregate recency, or lockHeld before deletion. It calls the new formatter only after deciding to refuse. `⏱️recency/🟦️.ts:17-19` includes bounded original/current diagnostic facts, falls back to no source if the mismatch image exceeds 8192 bytes, and preserves existing changed booleans. Neither recencySource nor its field/device/inode values influence planning, lock custody, the refusal predicate, or the selected deletion row.

Production budget scans opt into this measurement mode while using the same scan/plan/recensus pipeline. The previously unexplained Actor timestamp decrease remains unexplained until an actual next mismatch retains original and current witnesses. No past aggregate is retroactively adopted as file identity, and no safe-refusal relaxation is introduced here.

Runtime/Ajv/SQLite results reported by the implementation owner were not rerun or independently claimed in this audit.
