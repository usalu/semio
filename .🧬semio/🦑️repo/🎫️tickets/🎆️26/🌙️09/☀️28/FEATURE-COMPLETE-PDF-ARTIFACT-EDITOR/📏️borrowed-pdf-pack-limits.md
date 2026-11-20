# Borrowed PDF Snapshot Pack Limits

The PDF 1.7 native producer shares all 32 root field IDs, keys, optional declarations and line layout with the reference Record codec. It borrows the typed snapshot directly. Retained COS streams and document ID octets are projected from their original buffers, including an empty byte string. The exporter validates the original caller, decoder, projection address and epoch on each advance. It writes final bytes to retained pages and hashes them as those pages receive the bytes.

The natural-file contract applies its maximum output length to the final wrapped Pack. The borrowed document cursor independently caps source paths at 63 components and uses 1 MiB uncompressed document frames. The production exporter leaves the optional aggregate raw Body limit unset. Its cursor does not carry all the reference decoder's `PackLimits`, including the symbol and decoded allocation ceilings. A compressed output cap alone cannot establish that large repetitive or highly structured snapshots reopen under those limits.

The cursor separately measures and replays the canonical document stream. Each original-byte feed emits one raw byte per cursor substep, with separate compression, hash and framing turns. This bounds work per substep and leaves throughput for large mounted Open unmeasured. Coalescing paid byte work within the same original scope is a separate encoder performance change.

## Verification

- The existing PDF test compares an exact borrowed stream address and the projected Record with the reference Record. Its Pack comparison uses the ordinary encoder on each Record.
- The new `genuine_pdf_paged_snapshot_export_matches_reference_pack_and_digest` regression runs `OriginalSnapshotPackExport` with one item per original caller turn, using the reference Pack length as its cap. It compares the actual wrapped paged bytes and digest against the shipped reference codec and pays retirement of the output and empty exporter frame. It is pending native package execution.
- MuPDF independently read the reused 769-byte input PDF as one page and decoded its content stream object to 2,688 bytes. The audit did not measure mounted export throughput or establish parity at the reference reader's maximum sizes.
