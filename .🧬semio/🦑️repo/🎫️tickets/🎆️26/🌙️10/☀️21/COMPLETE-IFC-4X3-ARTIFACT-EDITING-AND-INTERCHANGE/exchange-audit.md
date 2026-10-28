# IFC4x3 Exchange Audit

## Scope and Existing Surface

Read-only audit. No production changes or downloads. Read applicable s, stdio, BIM and FEM AGENTS. Bootstrap shell reads used because cached repo.exe is MCP-only and semio-repo.exe is an older CLI; root agent resolves infrastructure.

BIM root: `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`.

- `🚪️io/🦀️.rs` registers IFC2x3/IFC4 import/export and GLB export. No IFC4x3, FEM3d or GLB import registrations.
- `🚪️io/📤️export/🏗️ifc/🦀️.rs` owns staged export, 23 family stages; individual stages can traverse entire families, so stage-based progress is not per-entity bounded cancellation.
- `🚪️io/📤️export/🏗️ifc/🧰️writer/🦀️.rs` Schema enum only Ifc2x3/Ifc4, id and pick helpers. Extend deliberately for 4x3; entity arities/enums cannot be assumed identical to IFC4.
- `🚪️io/📥️import/🏗️ifc/🦀️.rs` builds ModelSnapshot through semantic family readers. Mesh-only geometry explicitly reported and not imported. Parametric roofs etc restored from semio-authored records; foreign models are best effort.
- `🚪️io/📤️export/🧊️gltf` already has codec, document, projection, materials and scene modules. Use these; add graph extraction for IFC rather than duplicate GLB writer.

FEM3d root: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any`.

- IO contains JSON/text imports; JSON/text/CSV/OBJ/STL exports. No IFC/BIM/GLB conversion leaves were found.
- Follow existing `🚪️io/📤️export/🧵️serializers/🗿️artifacts` and import deserializer taxonomy when adding edges; keep paths under 256 characters.

## Proposed Contracts

1. Keep IFC4x3 document as lossless complete STEP graph. Semantic BIM/FEM3d/GLB conversions report losses and unsupported entity counts via IoOutcome rather than silently claiming complete semantic coverage.
2. Introduce reusable domain-neutral geometry extraction contract (placements, units, indexed triangle geometry, source entity identity) owned by IFC4x3 IO; use framework geometry value types, no third-party API types.
3. BIM IFC4x3 serializer/deserializer should reuse family readers/writers after schema-aware attribute lookup. Register distinct dialect `s.stdio.ifc@4x3/*` and emit FILE_SCHEMA IFC4X3_ADD2. Do not alias IFC4 runtime dialect.
4. FEM3d conversion maps structural analysis nodes/members/surfaces, material/profile assignments, supports, load cases/groups/actions with explicit refusal/diagnostics for unrepresentable semantics. Preserve original IFC graph in the IFC artifact; FEM export builds valid structural IFC entities, not opaque metadata only.
5. GLB exporter consumes tessellated extraction with placement and SI scale applied. Reverse GLB import yields IFC triangulated face sets/proxies and preserves identities/materials possible in glTF. It cannot reconstruct authored BIM parameters; fidelity must say so.
6. Every conversion runs as cursor-based job with entity/geometry budgets, progress and cancellation before publication; no mutation of destination until complete. Reuse framework job/IO traits.
7. Schema-first neutral fixtures for each edge, same expected output validated by third-party IfcOpenShell/glTF tooling behind test-only interfaces. Cover roundtrip graph identity and explicit lossy conversion outcomes separately.

## Official Sources and Fixture Plan

- Official IFC4.3.2.0 / IFC4X3_ADD2 publication: https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/
- Normative EXPRESS catalog: https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/annex-a-express.html (some direct requests return 403; obtain published schema through its linked Annex A resource, not current development schema).
- Informative Annex E: https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/annex-e.html
- Verified complete inline IFC-SPF official example: https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/annex_e/basic-geometric-shape/triangulated-item.html . FILE_SCHEMA IFC4X3_ADD2; eight coordinates, twelve triangles, millimetre units, nested local placement. Ideal geometry/scale/GLB oracle fixture. It explicitly has ViewDefinition [notYetAssigned], so do not claim MVD compliance.
- Wall example: https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/annex_e/building-elements/wall-extrusion.html
- Beam example: https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/annex_e/building-elements/beam-parametric-cross-section.html
- Official sample repository: https://github.com/buildingSMART/IFC4.x-specification-models ; former IFC4.3.x-sample-models redirects here.
- Repository README https://raw.githubusercontent.com/buildingSMART/IFC4.x-specification-models/main/README.md warns examples are informative and may not adhere specified MVDs; validation script rewrites file schema identifiers to current development schema. Therefore pin publication-specific IFC4X3_ADD2 originals and hashes, do not blindly download current main or rewrite schemas.
- Official examples index: https://technical.buildingsmart.org/standards/ifc/ifc-examples/ links buildingSMART/Sample-Test-Files. Use only version-verified IFC4x3 files.

First fixtures: triangulated block (GLB), extruded wall (BIM), beam with profile (BIM/FEM), structural analysis model (FEM), plus units and nested-placement cases. Store provenance URLs and SHA256 beside original fixture; validate syntax/schema via full ADD2 catalog and test-only IfcOpenShell. MVD conformance is separate from schema validity and needs actual normative MVD rules.
