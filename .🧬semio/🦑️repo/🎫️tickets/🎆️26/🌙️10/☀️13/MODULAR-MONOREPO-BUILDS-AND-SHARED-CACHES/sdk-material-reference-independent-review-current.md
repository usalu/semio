# SDK Material Reference Independent Review

Read-only review on 2026-10-15. No tests, builds, strict checks, Originals, image opens, SDK changes, or production edits. Runtime and strict validation remain **UNEXECUTED** during the lease-capacity hold. The active db49 SDK was untouched; this review does not independently re-count its captured source closure.

Paths below are relative to `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/`.

## Source conclusion

No concrete blocking defect found in the staged correction's requested identity/reference/guard boundaries. The prior patched-input-derived base selection is replaced by the actual generation-bound material reference.

- `📦️sdk/📊️material/🟦️.ts` 39–42 captures package, lock, declared patch files, authorizer source, authorizer compiler and material bytes under provision/generation SH. `nxSdkGenerationMaterialReferenceV1` at 27 recomputes `nxToolingInstallIdentityV1` with the same insertion-order framing used by generation publication (`📜️script.ts` 125–135), then parses the material bytes. Marker version/digest and actual authorizer receipt are checked at 42. This authenticates the generation input identity; it does not claim a complete package-file content hash.
- The `MaterialReference` schema is closed to version 1, kind `immutable-binary-objects`, and canonical lowercase 64-hex snapshot. The helper does not grant deletion or ownership authority. `nxSdkMaterialSnapshotPathV1` at 29 only projects a canonical path from those inputs.
- The exact referenced digest selects both the canonical base path and snapshot SH resource at 43. The physical `requireNativeStoragePathV1`, base receipt reads, actual base input capture, base identity recomputation, binary verification and inspections all occur inside that SH callback. The returned body promise keeps the shared guard through the awaited caller callback. Generation guards remain around this entire nested operation.
- Generation and snapshot input capture share explicit aggregate file/byte limits at 40, with the existing per-file physical reader ceiling. Snapshot receipt/identity refusal occurs before material inspection at 43.
- No retirement API or physical mutation was added in the reviewed implementation. The reference alone is not used as retirement permission.

## Qualification gap

`📦️sdk/📊️material/🧪️tests/🟦️.ts` 9–12 and `🧪️tests/📥️reference.json` provide authored parser/path/schema and generation-hash cases with Ajv, SQLite, Node paths and Bun hash oracles. They exercise helpers only: they do not invoke `withNxSdkMaterialReportV1`, prove actual snapshot SH contention, or observe guard lifetime through an asynchronous callback. This is a concrete remaining coverage gap, not evidence the inspected lifetime is wrong. The report's UNEXECUTED wording is accurate; there is no demonstrated RED, passing test, strict result, or runtime qualification for the change.

The callback contract still requires callers to await work they start; this correction does not add generic pending-work enrollment for arbitrary detached callback tasks. That does not alter the explicit awaited callback lifetime at line 43.
