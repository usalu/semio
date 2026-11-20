# Bound Graph Authority Receipts

The standalone XLSX dev host finished graph inference and source-fact validation before publication failed. The failed run measured a 50,262,248-byte graph and an 856,037,661-byte authority receipt. The local cache publication limit is 536,870,912 bytes. The selected graph logged 261,004 distinct source observations and more than 4.6 million checks of memoized rows. The uncompressed receipt duplicated semantic analysis dependencies whenever the same source fact appeared beneath several analysis rows.

The authority receipt uses one record for each distinct observation. Each analysis references earlier dependency records by index. The trailing root indexes preserve capture order, including repeated roots. Reuse validates those indexes before resolving the original observations. Selected fleet installation and local graph maintenance use the same receipt format. Maintenance can verify all references and find closure tokens one record at a time. The publication cap stays at 512 MiB.

The generation-based graph reader can also serve selected targets. Host diagnostics reported no published generation at the start of the investigation, so that route would first need a full graph build. The indexed format works with the default selected reader.

## Focused verification

Repository-managed Bun tests passed all 40 cases across seven suites, including seal-under-churn, current and stale source facts, fleet installation and streaming maintenance validation. The indexed receipt fixture checks the wire contract with Ajv and resolves root and dependency indexes independently with SQLite.

A synthetic graph authority with 4,600 roots and 3,680,000 repeated references encoded to 16,269,071 bytes. Its equivalent expanded JSON would occupy 1,118,991,401 bytes, calculated from the serialized row size without allocating the expanded document. Decoding restored shared dependency identity. The live XLSX host replay is tracked separately by the parent task.
