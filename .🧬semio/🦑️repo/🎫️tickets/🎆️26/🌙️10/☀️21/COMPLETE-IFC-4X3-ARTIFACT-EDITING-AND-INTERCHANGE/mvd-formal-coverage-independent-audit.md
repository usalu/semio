# IFC4x3 MVD Formal Coverage Independent Audit

Read-only current source/released-primary review,2026-10-21. No executable MVD conformance claim is justified by the reviewed paths. No production edit/native execution.

## Current executable versus informative coverage

IFC4x3 ANY schema/mvd retains876entity and312concept scope rows pinned to official Standards-builds commit610ee3f95db9d0380b1eb8de4dc6e17833531518. Public TS/Rust scope APIs distinguish universal,included,unlisted,unknown and explicitly retain conformanceEvaluated:false. mvd.test.ts verifies identities/badges/labels/immutability, not model constraint execution. Header helpers author/read ViewDefinition through central mutations and roundtrip tests, but that is intended exchange metadata, not validation.

The retained validation tree contains no reviewed production reference to these view IDs or MVD rules. EXPRESS program evaluation is schema semantics, not exchange requirement applicability. Existing geometry/alignment adapters similarly cannot establish all-MVD conformance by accepting their supported projection.

No correctly versioned complete executable ADD2 MVD constraint dataset is pinned in the reviewed artifact. Existing mvd-source-research.md records primary release/API investigation and development-only CSV/XMI generation possibilities. This audit did not locate a new released executable file. That is a bounded source finding, not proof no such input exists externally. Older IFC4 files, development ReferenceView CSV and mvdXML syntax versions must not silently become IFC4X3_ADD2 constraints. Only released ReferenceView and Alignment-basedReferenceView are current catalog identities; no DTV/COBie/other view should be invented from naming similarity.

## Available primary requirement slice

The pinned released [Body Tessellation Geometry](https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/concepts/Product_Shape/Product_Geometric_Representation/Body_Geometry/Body_Tessellation_Geometry/content.html) page explicitly supplies three constraints: the shape representation identifier is Body,type is Tessellation,and items have IfcTessellatedFaceSet type. This is enough for a narrowly named concept evaluator once applicability is explicit; it is not enough to require this geometry on every product or claim a full view pass. Generic diagrams/recommended usage patterns must not be promoted to universal requirements.

The pinned [Alignment Layouts](https://raw.githubusercontent.com/buildingSMART/Standards-builds/610ee3f95db9d0380b1eb8de4dc6e17833531518/IFC/RELEASE/IFC4_3/HTML/concepts/Object_Composition/Nesting/Alignment_Layouts/content.html) page distinguishes horizontal,vertical and cant layouts and directs configuration rules to child concepts. That overview alone does not establish all per-view cardinalities/applicability. Do not infer that every alignment needs all three layouts. Child concept definitions and actual applicable exchange requirements must be pinned before enforcing them.

Direct technical mvdXML page fetch returned403 in this audit; prior ticket source research remains separate evidence. Scope metadata is not executable constraint data regardless of XML format availability.

## Next narrow schema-first slice

Implement an explicitly requested Body Tessellation Geometry concept check, not an all-view validator. First define closed neutral rule/report schemas with release identity,concept URI/source hash,applicability decision,status(pass/fail/not-applicable/incomplete),entity/attribute paths and diagnostics. Separate generic executable concept predicates from the still-missing exchange-requirement→concept bindings. A report must remain incomplete for full ReferenceView/Alignment-basedReferenceView while only a subset is evaluated.

Use the existing declaration/subtype catalog for Items membership, original captured graph/control and progressive reference resolution. No static badge is a prohibition or applicability proof. For complex entities, resolve actual constituent declarations; duplicate IDs/missing targets/cancellation must produce incomplete/refusal, never pass. Native implementation requires original-funded continuation/close and genuine runtime gates.

Test-first neutral matrix: explicitly applicable tessellation with IfcTriangulatedFaceSet and IfcPolygonalFaceSet positive; wrong identifier; wrong RepresentationType; unrelated representation item; mixed valid/invalid Items; missing/ambiguous referenced item; absent body where applicability is unknown versus explicitly required; unrelated Axis representation not falsely selected; multiple shape representations; subtype/complex item; source mutation isolation; every interior/completion cancellation; partial report cannot claim full-view valid. Build physical STEP examples through existing owned tessellation adapter and independent IfcOpenShell traversal/type checks. Full IfcOpenShell EXPRESS validation is supplemental, not an independent MVD oracle; verify the three concept predicates independently and label that narrower scope.

Before a second alignment slice, pin exact child concept text and its exchange applicability. A source manifest/census should distinguish executable EXPRESS, explicit concept predicates, view applicability bindings,informative scope and unsupported requirements. This gives concrete progress without converting complete metadata into false all-MVD coverage.
