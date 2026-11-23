# Seal allocation accounting

## Root cause
The seal cursor did not under-report. The harness (`semio_framework_value_clone_source_testing::drive`, heap == reported progress) was correct: a foreign `[DEBUG] phase ... window` eprintln! inside the seal turn loop allocated during the measured turn (392 / 666 bytes observed, 0 reported). The other agent removed it; no cursor change needed.

## Verification (current tree)
- `semio-framework-replication --test paged_seal_digest`: 2 passed.
- `semio-framework-replication-edit --lib seal`: 4 passed (incl. `canonical_native_edit_seal_settles_every_exact_quote_within_its_own_grant_for_large_edits`).

## Remaining `[DEBUG]` eprintln in measured paths
Only `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs:590` (`page allocation occupied`), on the refusal path (returns `original page allocation attempt is occupied` right after). It allocates only when the effect already refused, so it does not break exact accounting on success paths, but it should be removed with that refusal's diagnosis. None remain in seal, clone-source testing, canonical-edit.

## Test fix: `canonical_native_source_handoff_refuses_extracted_original_without_losing_custody`
Two test bugs, no production bug: (1) `maximum_depth:4096` exceeded the Source stock depth 1024 (changed to 1024); (2) `admit_source` was given a 4096-byte copy grant below `source_constructor_demand()`, so it returned `Ok(None)` instead of reaching `try_borrow` (now grants exactly the constructor demand, so the extracted source is refused with `Err`). File: `🧰️framework/🔨️modules/📡️replication/🚪️io/📝️text/🧵️canonical/🔏️seal/🧪️tests/🦀️.rs`.

## Runtime symptom
"canonical original child receipt exceeds caller grant" (3336 vs 3975) is not explained by this: it is a copy_bytes mismatch, not heap under-reporting.
