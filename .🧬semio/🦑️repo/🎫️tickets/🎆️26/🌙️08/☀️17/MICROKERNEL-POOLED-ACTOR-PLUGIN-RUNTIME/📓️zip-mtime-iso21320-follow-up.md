# Zip mtime + ISO 21320 conformance follow-up (Z1 task_dc49df65)

## Decisions

### `parse_ut_mtime` — removed, no `ZipEntry.mtime`

The logical `ZipSnapshot` schema deliberately forbids native/shadow fields (`unixMtime`, `dosDate`, `flags`, …) — see `shadow_tests` in `✳️any/🧬️schema/📸️snapshot/🦀️component.rs`. Adding `mtime: Option<i64>` would violate that contract.

`parse_ut_mtime` was deleted. Info-ZIP `UT` (0x5455) extra fields are still accepted during decode (`parse_extra_fields`); only the unused mtime projection helper is gone.

### `check_iso21320_conformance` — wire-byte central-directory inspection

Per-entry ISO/IEC 21320-1 checks (encryption, strong encryption/masked headers, data descriptor, version-needed ceiling) operate on **central-directory header fields**, not logical snapshot fields.

Implementation:

1. `inspect_zip_central_entry_headers` in `✳️any/🚪️io` — lightweight EOCD/CD walk, no decompression.
2. `check_iso21320_wire_conformance(data)` — runs hard/soft diagnostics against wire headers.
3. `check_iso21320_conformance(snapshot)` — encodes canonical logical snapshot then delegates to wire conformance (our writer policy is conforming by construction).
4. `ZipIso21320Analyzer` — binary sources checked on raw wire bytes; text/DSL path uses encoded snapshot.
5. `ZipIso21320Validator` — inspects pack-unwrapped or raw ZIP bytes via `check_iso21320_wire_conformance`, not post-decode logical state (decode drops forbidden bits).

## Tests added

- `derived_analysis::tests::*` — seven unit tests for hard/soft diagnostic matrix (restored from pre-logical-snapshot era, adapted to header structs).
- `derived_composition::tests::encrypted_wire_archive_composes_to_clean_logical_output`
- `derived_composition::tests::subset_validator_flags_real_violations_without_normalizing`

## Verification

`cargo test -p semio-s-plugin-stdio --lib iso21320` — **11 passed, 0 failed** (root `target/`, ~16m compile). All analyzer hard/soft matrix tests plus composer/validator wire tests green.
