# Current Value Native Selector Audit

Read-only source audit. No compiler or test was started. Canonical package script is `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📜️script.ts`; extracted classes also live in `🧪️tests/🔬️rust-package-commands/🟦️.ts`. A public reexport does not import dependency `cfg(test)` modules into the facade test binary.

All manifest paths below are relative to `🧰️framework/🔨️modules/🌱️value`. Law final function names remain the authored current functions; the table identifies actual test-module prefixes from explicit Rust mounts.

| Advertised group | Current physical package owner | Manifest | Actual law prefix / change |
|---|---|---|---|
| BorrowedObservation | semio-framework-value-persistent | 📋️list/🔗️persistent/📦️packages/🦀️rust/Cargo.toml | list::persistent::tests::borrowed_observations; execution owner already corrected |
| BulkVecRetained | semio-framework-value-retained-clone | 🧬️retained-clone/🧬️contract/📦️packages/🦀️rust/Cargo.toml | retained_clone::bulk_vec_tests; already correct |
| NativeSharedReceiving | semio-framework-value-native-owned | 🛬️decode/🧳️owned/📦️packages/🦀️rust/Cargo.toml | owned::shared::tests; remove native_decoding prefix |
| NativePersistentReceiving | semio-framework-value-native-owned | same | owned::list_tests and owned::original::tests |
| NativePayloadEnum | semio-framework-value-native-owned | same | owned::payload_enum_tests (physical test file under derive is explicitly mounted here) |
| NativeRetirementLoan | semio-framework-value-native-decoding | 🛬️decode/📦️packages/🦀️rust/Cargo.toml | native_decoding::retirement_loan::tests |
| NamedRootReceiving | semio-framework-value-edit | 📋️list/🔺️edit/📦️packages/🦀️rust/Cargo.toml | named_edit::index::tests::transfer::receiving |
| NamedKeyIndex | semio-framework-value-edit | same | named_edit::index::tests::transfer, named_edit::index::tests, named_edit::tests |
| NamedEdit | semio-framework-value-edit | same | named_edit::tests |
| IndexedEdit | semio-framework-value-edit | same | indexed_edit::tests, indexed_collection_commit::tests, named_edit::tests |
| FixedKeySlots | semio-framework-value-registry | 📋️registry/📦️packages/🦀️rust/Cargo.toml | fixed_key_slots::tests |
| FixedRequestOwners | split registry + edit | preceding two manifests | fixed_key_slots/fixed_request_owners in registry; indexed_edit in edit |
| ExactKeyLookup | semio-framework-value-registry | 📋️registry/📦️packages/🦀️rust/Cargo.toml | exact_key_lookup::tests |
| PersistentBuild | semio-framework-value-persistent | 📋️list/🔗️persistent/📦️packages/🦀️rust/Cargo.toml | list::persistent::build_tests |
| PersistentTraversal | semio-framework-value-persistent | same | list::persistent::tests::traversal; remove value prefix |
| TypedEquality | semio-framework-value-persistent | same | retained_equality::tests; remove value prefix |
| SliceEquality | semio-framework-value-persistent | same | slice_equality::tests (explicit persistent crate mount overrides nearest directory intuition) |
| OriginalVectorConcat | semio-framework-value-persistent | same | original_vector_concat::tests |
| SealedFrontier | semio-framework-value-shared-issuance | ♻️retirement/🔗️arc-custody/🔒️sealed/📦️packages/🦀️rust/Cargo.toml | frontier_tests and tests at direct lib root; remove retirement::shared::sealed prefix |
| ImmutableRootClone first law | semio-framework-value | 📦️packages/🦀️rust/Cargo.toml | value::snapshot_projection::immutable_clone::list_tests |
| ImmutableRootClone second law | semio-framework-value-tests | 🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml | retained_clone::source_custody_tests |
| CloseDemand | semio-framework-value-tests | same | retained_clone::close_demand_tests |
| OwnedProjection | semio-framework-value | 📦️packages/🦀️rust/Cargo.toml | value::snapshot_projection::tests::owned; preserve correct local selector |
| RetainedUtf8 | semio-framework-value | same | retained_utf8::tests; preserve correct local selector |
| OriginalRetirementDemand | currently unmounted | source ♻️retirement/🎟️demand/🧪️tests/🦀️.rs | No original_demand_tests module/path mount found anywhere in value Rust source. Old imports also reference removed retirement::shared::sealed; needs current explicit shared-issuance integration ownership, not just selector rename. |

Package library names follow actual Cargo lib declarations/default underscore names; native-owned's compatibility vocabulary alias `native_decoding` reexports `owned` but does not change the emitted test-module path from `owned`. The persistent crate similarly explicitly mounts equality/concat sources despite their nearest physical package directories.

The exact map was sent to execution owner before any mass edit. This is source reachability evidence, not a claim that these standalone test crates compile or that any exact law ran. Existing native compiler failures must be resolved and genuine selected suites must run before runtime acceptance.

## Repair Source Acceptance

Controls owner implemented the23remaining route cases after Borrowed release. Re-review finds all table package/library/prefix mappings represented, including split ImmutableRootClone and FixedRequestOwners groups. OriginalRetirementDemand is now mounted in the existing companion as original_demand_tests and imports current crate::shared_issuance, avoiding new runtime dependency cycles. Sole-importer classes moved to the existing canonical script with directory depth/this.root semantics preserved; the old value helper was removed. Correct local UTF8/OwnedProjection and BulkVec contract selectors remain correct.

Shared dispatch fixture/schema and TS/esbuild actual-class tests compare physical TOML package/library and explicit manifests, preserve Original and capped policy, and use delayed native completion gates to prove each returned Promise awaits its native work. No concrete source issue found in bounded review. The table above records the original source findings; implementations are now source-reviewed repairs, while actual native test inventory, compilation and law execution remain required.
