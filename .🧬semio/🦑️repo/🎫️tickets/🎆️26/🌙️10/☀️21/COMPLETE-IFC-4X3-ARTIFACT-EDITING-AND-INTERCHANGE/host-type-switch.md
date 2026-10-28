# IFC4x3 Retained Type Changes

Whole-entity type variants formerly emitted RemoveEntity followed by InsertEntity. They now emit the existing domain mutations for changed name, replaced attributes, descending removed attributes, ascending appended attributes and changed complex constituents. Every central diff retains the entity identity and position, including identities above 2^53. The retained transaction remains the publication authority.

The test-first native scenario consumes `✏️editor/tests.json` (`typeSwitch`) and checks IFCWALL → IFCDOOR, unchanged incoming references, sparse modifications without entity additions/removals, preserved GlobalId/Name/Tag and inverse restoration. The language-neutral feature describes the same behavior. Native tests remain unexecuted while authentic compiler ingress is unavailable; rustfmt parsed and formatted the editor and tests successfully.

The scenario also uses IFCWALL's STANDARD enum, which is absent from IFCDOOR's declared enum. Attribute retention now requires the same attribute name and exact declared type expression. Different domains initialize from the target declaration, making the optional door predefined type unset. Required target attributes do not inherit an unset value, and explicit target attributes do not inherit a derived marker. This avoids carrying an incompatible value merely because two attributes have the same name; it does not guess compatibility between different domains or claim full rule validation during type selection. The independent oracle confirms STANDARD belongs only to the source enum and the target slot is optional.

`details-oracle.py` independently checks all inherited attribute counts and preserved field names against IfcOpenShell 0.9.0.post1, creates both official entity types and confirms equal preserved values. The oracle ran successfully alongside the existing five attribute cases, three GUID vectors and the 301-entity / 257-reference EXPRESS-valid model. IfcOpenShell's native `file.create` accepts a narrower integer ID domain and refused 9007199254740993, so that independent field-value oracle uses ID 1. Exact u64 identity preservation is covered by the written native test, not claimed as an executed third-party identity comparison.

No shared schema, mutation vocabulary, external runtime dependency or build-system code was changed by this correction.

## Exact Real Value Changes

Whole-entity change detection now compares real payloads by their binary64 bits, including nested aggregate/typed values and complex constituents. It uses a depth-sized iterator stack, so wide arrays do not allocate a frontier of all values. This prevents the numeric equality of positive and negative zero from suppressing an explicitly submitted whole-entity change.

Test-first neutral `realEdits` vectors and a written retained native test check a positive-zero → negative-zero argument mutation and exact inverse bits. IfcOpenShell 0.9.0.post1 independently retains both bit patterns when the corresponding Cartesian point coordinate is authored and changed; the extended oracle executed successfully. Rustfmt parses the editor and native tests. Native retained execution remains unverified until authentic compiler ingress is restored.

## Shared Provider and Complex Field Follow-Up

Concurrent IFC work moved the complete provider/reducer into IFC `🔎️details/🦀️.rs` with catalog profiles for each standard. The identity-preserving change logic and domain-aware templates were retained. Root changes follow that shared implementation rather than restoring the former duplicate 4x3 implementation.

The subsequent independent host audit found that shared `complex_edit` still discarded nested complex positive-zero → negative-zero changes through numeric PartialEq. Lossless equality now belongs to the shared IFC snapshot model as `ifc_arguments_identical` and `ifc_complex_identical`; both whole-entity changes and complex-field no-op detection use that same authority. Semantic numeric equality remains separate. A written retained complex-field signed-zero test checks actual admission, emitted SetEntityComplex, central application and inverse bits. Shared sources parse with rustfmt; native execution remains unverified. Context-derived marker repair and bounded large type-selection/reduction work are assigned to the High details worker in response to `host-final-review.md`.
