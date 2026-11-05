# Pack JSON Source Stability Audit

Four reads at 250 ms intervals (2026-11-04T23:27:07.358Z through 2026-11-04T23:27:08.115Z) observed identical before/after stat identity, size, mtime, ctime and SHA-256. A later read at 2026-11-04T23:28:31.202Z matched every prior observation.

- Device/inode: 16777229 / 431593299
- Size: 8095 bytes
- mtime and ctime: 2026-11-04T23:13:32.784Z
- SHA-256: `1f7462b2dfcbd08a957101c7aec9980f0fbbc6ca4b7e2abbde272765d78dd261`

No current writes or metadata churn were observed. These observations do not prove what happened during the earlier compiler pass. The mtime predates the repeated refusal sequence; repeated messages alone do not demonstrate repeated writes.

The exact retained cause was `one of its inputs changed` for `framework/modules/pack/json/🦀️.rs`, after the descriptor build published `cargo:artifact-plan-receipt` progress. It exhausted four snapshot attempts, failed the emitter prerequisite, and automatically restarted the existing owner (1 of 6), retaining its capacity grant. No Rust compile error or descriptor emission is claimed.

The native artifact-plan owner raises reason `input` when a completed Cargo observation digest does not equal the admitted planning source digest. This is a provenance comparison, not a raw stat-change verdict. Its map lookup also differs when the completed observation has no entry for an admitted input. The retained error message does not include the expected and observed digests or say whether the observation was absent, so present evidence cannot distinguish an earlier real edit from a stale/missing observation bug. Do not conclude actual ongoing writes or a metadata-instability bug from this generic error.

Source: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts`, `Completed Cargo Source differs from admitted artifact plan` comparison. The bounded read-only evidence is retained in the ticket generated stability JSON files. No source file, receipt, gate, image or grant was changed by this audit.

## Permanent Source Digest Diagnostics

Added a schema-first bounded mismatch DTO/helper in the existing native artifact-plan owner. Mismatches publish `cargo:source-digest-mismatch` with only the exact path (maximum 256 characters), admitted SHA-256, completed SHA-256 or null, and `missingRecord`. Exact source authority and refusal remain enforced; no record, digest, key, receipt or source is substituted.

Six language-agnostic vectors cover match, changed bytes, missing record, observed missing file, distinct noncanonical lookup key and Windows path. The existing suite compares results with independent SQLite exact-path joins and validates DTOs through Ajv. Actual repo-owned tests are pending in `🗑️generated/integration-source-diagnostics-tests.log`.

Pipeline trace: admission hashes raw bytes through `readRustSourceClosureV1`, rather than active-line projections, then resolves each path against the unit root. Completed map keys come from `observation.units[].inputs` with `kind === file`; conflicting duplicate hashes already refuse. Lookup uses the admitted absolute path verbatim. Physical stability alone cannot distinguish absent input from stale digest. New diagnostics expose that actual distinction. No cause is asserted or bypassed without evidence.

Changed owners: native-build/🟦️.ts and its 📋️plan/{🟦️.ts,🧬️schema/🔣️.json,🧪️tests/🔣️.json,🧪️tests/🟦️.ts}.

## Completed Observation Lookup Trace

Completed Source inputs are reconstructed from Cargo dep-info candidates for each artifact. Relative dep-info rows are admitted only when exactly one existing Cargo owner directory resolves every listed source and the target source. When no unique base is found, `baseDirectory` is null and relative rows are excluded from `sourceFiles`; absolute rows remain. Every selected file is observed physically, and its exact resolved path/digest becomes `unit.inputs`. This identifies a concrete place where missing-map diagnostics would need investigation; it does not prove that the pack JSON entry was missing. Candidate discovery also uses the actual invocation profile to locate internal compiler dep-info. No discovery/profile/path behavior was changed without an actual failing record.

## Actual Diagnostic Suite Result

The owned artifact-plan test invocation completed with 17 pass, 0 fail, 117 assertions in 196 ms. The bounded mismatch helper agrees with SQLite exact-key lookup and validates with Ajv. A fresh actual emitter run is still necessary to capture the causal expected/observed/missing-record diagnostic: the prior descriptor automatic retry instead stopped in graph inference on the concurrently corrected private export. No speculative cache or receipt changes were made.
