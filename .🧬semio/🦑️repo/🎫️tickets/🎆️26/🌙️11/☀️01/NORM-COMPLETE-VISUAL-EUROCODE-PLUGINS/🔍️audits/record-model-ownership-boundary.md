# Record Model Ownership Boundary

The actual record-model production root currently owns both canonical recursive value/schema types and higher typed binding/receiving/encoding machinery. A coherent lower first-party record-ownership crate will own Shape/FieldSpec/RecordSpec, FieldValue/RecordValue/RecordFields, wire/expression values, controlled RecordSpecProducer, borrowed static metadata, and the associated RetireOwned and NativeValueSource implementations. The external traits must follow their canonical type owner under Rust orphan rules.

The higher record-model remains the public binding/receiving/encoding and JSON-schema layer and explicitly reexports the lower-owned types/producer/retirement ports. It does not duplicate type definitions or add a compatibility branch.

Tests stay at their current source paths and remain mounted in record-model. The existing native-source module will keep its original test mount and imports; its genuine production NativeValueSource implementations move with the value types. JSON-schema tests stay with the higher schema functions. No test-only package, test source relocation, cap increase, or source-count suppression is involved.

Root's independent current-source classification is36Rust+8embedded JSON test inputs. Removing the cohesive producer, retirement, field storage, and borrowed metadata production modules from the higher closure restores32Rust. Canonical runtime fixture reads with explicit authored nativeSources can remove the eight embedded data inputs while retaining actual hashed test input custody. The post-edit collector and actual native gate remain required; this document claims design only.

## Authored Extraction And Source Admission

The new lower package is semio-framework-dsl-record-ownership under model/🧳️ownership. Its source is the canonical types plus controlled producer, retirement implementations, original field storage, borrowed static shape metadata and native value-source implementation. The model facade explicitly reexports those exact owners, while typed binding/receiving/projection and JSON-schema generation remain higher. Native-source tests remain at their existing file path and same model::native_receive::tests mount; their original test module is not moved into the new package.

All eight previously embedded neutral JSON inputs now use fully-qualified canonical repo_text! at their existing read sites. All eight exact workspace paths are declared in the existing authored @semio-tech/dsl-record-model-rs project nativeSources. The build input owner at build/inputs/🟨️.mjs merges declarations.nativeSources into nativeTestSources together with the inferred Rust inputs, rather than replacing Rust inputs. Existing source files, fixtures, test counts, laws and constants were not removed.

The fresh independent conservative collector reports model lib23files215844bytes1854lines; model test32files258742bytes2147lines; lower ownership6files38603bytes624lines. No/all authored features agree. Model had3402bytes and zero file slots spare before the subsequent small existing scalar test API correction. See record-model-ownership-census.md for exact method/cfg limitations.

Framework lock update59349 is terminal0 and its actual lock contains the new owner and model dependency edge. Root lock58511 is terminal0, adding only ownership and concurrent pack-json-canonical first-party packages. Norm lock31949 is active. No external version changes were reported.

Low's actual focused shape-red28946 reached model lib-test rustc and accepted the production extraction so far, then stopped at the existing scalar fixture's receiving_issued_grant()? Option-versus-Result use. That exact fixture now requires Some issued original grant or preserves an explicit InvariantViolated value refusal; no fallback wallet/grant exists. Low owns the successor after28946 terminal. No semantic shape-counter red or green has yet been established.

An independently launched duplicate14903 was canceled through its scoped process session and is confirmed terminal130; it is not test evidence. Low28946 remains the authoritative filtered attempt.

Norm workspace update31949 is confirmed terminal0. It adds only the lower ownership and concurrent canonical JSON writer first-party packages and explicitly retains36 unchanged external dependencies. Framework, root and norm locks now carry the ownership/model edge. No lockfile regeneration or pin weakening was used.
