# Typed Snapshot Hook Census

Read-only source and existing compiler receipt inspection, 2026-10-03. No Cargo or source modifications occurred. Counts are lexical current-source census, not linked mounted runtime authority; concurrent edits can immediately change signatures.

## Five-Hook Surface

The shared Store trait at line 10892 still exposes String results for `to_sqlite_database`, `from_sqlite_database`, `decode_sqlite_snapshot_native`, `encode_sqlite_snapshot_native`, and `preflight_sqlite_snapshot_encoding` at inspection time. Root assigned High its ValueError conversion.

Repository rg-selected Rust files containing ArtifactSqliteSnapshot implementations: 146 files, 144 lexical impl matches. Excluding physical `🧪️tests` directories gives 131 files and 131 impls, not an independently established 114-impl total. The 114 figure matches production direct decode-hook definitions in those files. Hook declarations counted in selected production files: to/from SQL 132 each, direct decode 114, direct encode 113, preflight 88; all inspected result declarations are String. These counts include declaration/default locations and inline test implementations in production files, so do not interpret them as 131 actual owner registrations. Test-directory census adds 13 impls and to/from13 each, decode2, encode3, preflight6.

## Existing Typed Producer Mappings

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs`: `SemioError::into_value_error` at line34 preserves `DecodingControl(ValueError)` and maps the closed enum: UnknownEnvelope => UnsupportedOwner, InvalidPreamble/InvalidBinaryHeader/AmbiguousEnvelope => InvalidValue. Controlled input envelope functions still return `SemioResult`; use this enum mapping instead of `to_string` or message classifiers. Controlled output wrappers already return ValueError.
- `🧰️framework/🔨️modules/🎒️pack/⚠️error/🦀️.rs`: `PackError::into_value_error` at line28 preserves ValueRefusal, maps TextRefusal by its typed kind/message, LimitExceeded => WorkLimit, unsupported version/flags/codec => UnsupportedOwner, and malformed/checksum/IO forms => InvalidValue. `From<ValueError> for PackError` exists.
- CommonMark private SQL helpers inspected in the previous audit already use ValueError, but five trait terminals still erase it to String. JSON reconstruction closures are likewise typed internally, while direct native framing currently calls `map_err(|e| e.to_string())`, text parsing extracts `e.message`, and direct decode native control still uses the old semantic value ceiling. Root owns JSON's new staging.

## Existing Process Compiler Receipt

The inspected `🗑️generated/root-authentic-process3d10-declared-public-capability-authentic-baseline.log` has completed with Nx failure, 3m51s. Compiler terminal summaries report exactly STEP125 and ZIP52. Primary diagnostic owners: STEP base snapshot SQL119 plus CC1–CC6 one each; ZIP base snapshot SQL47 plus ISO21320 schema3 and base SQL `🚦️native` child2. Concurrent compile progress must not be used to attribute errors: its last 'Compiling' crate misleadingly named OBJ while the actual terminal summary named STEP. The 177 diagnostics are compile prerequisites, not feature assertion failures. High's subsequent STEP/ZIP repair is newer source and has no fresh owning compiler receipt in this audit.
