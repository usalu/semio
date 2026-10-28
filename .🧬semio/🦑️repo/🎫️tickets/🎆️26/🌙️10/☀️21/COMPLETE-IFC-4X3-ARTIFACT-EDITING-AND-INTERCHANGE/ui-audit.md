# IFC Editor UI Audit

## Scope and Access

Read-only exploration of IFC artifact schemas, editor surfaces and shared details UI. Root, `✏️s/AGENTS.md`, and `✏️s/🔌️plugins/🗄️stdio/AGENTS.md` were read. No source, build system, Git state or AGENTS files were modified. `repo` is absent from PATH; the recorded native path is a stale macOS path; cached binaries expose MCP or an old CLI without script execution. Read-only PowerShell and rg were necessary fallbacks pending a working interface.

All paths below are relative to `C:/git/semio/✏️s/🔌️plugins/🗄️stdio`.

## Existing Editor

- `🗿️artifacts/🏗️ifc/🟦️.ts` exports a schema-agnostic IFC4 graph, IFC2x3 graph and SQLite codecs. No IFC4x3-specific declarations or controls are exported.
- `🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/✏️editor/🦀️.rs` owns the editor. Its `EditSnapshot` command enters `SnapshotEditingEditor::snapshot_edit_emit`; publication is centralized by `snapshot_editing_publication_authority!`. The Details branch at line 285 calls generic `render_snapshot_details` and passes controller `s.stdio.ifc@4/*#editor`.
- The same editor registers a shared details window next to a mesh window. Its TypeScript main-window twin only describes mesh instances and IDs; there is no IFC-local TSX control surface.
- `🏅️standards/4️⃣4/🪆️subsets/🔣️.json` explicitly documents the generic Part21 layer and states that IFC4 MVD-specific mutations do not exist. IFC2x3 has cv20, sav and cobie narrower subsets.
- The wildcard snapshot is lossless raw Part21: header value lists, entity name, argument values and complex constituents. The snapshot JSON schema is a transport graph schema, not an EXPRESS entity/attribute declaration catalog. Therefore generic Details cannot infer IFC attribute names, admissible references, IFC enums, SELECT membership, or IFC cardinality.

## Mutation Constraints

`🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs` maps header lists, entity names and individual argument subtrees to concrete domain mutations. Insert/remove rules cover entities and arguments. Complex constituents, entity IDs and document schema are explicitly refused. Controls generated from unrestricted transport structure can therefore advertise operations the IFC edit vocabulary rejects. IFC declaration-aware capabilities must agree with the command layer.

## Shared UI Infrastructure

`📇️registry/🧬️contract/✏️editing/🪟️details/📐️detail-model/🦀️.rs` defines `SnapshotDetailsProvider`. It already provides lazy path addressing, child counts, object keys, creation templates, missing-property templates, enums, localized presentations, capabilities, collection insertion and variant switching. This is the correct integration seam for IFC semantics.

`📇️registry/🧬️contract/✏️editing/🪟️details/🌲️detail-tree-and-window/🦀️.rs` exports `render_snapshot_details_provider_revisioned` and the generic wrapper. Reusing it retains tree-window bounds and source-draft interactions.

`📇️registry/🧬️contract/✏️editing/🪟️details/🏷️labels-and-controls/🦀️.rs` contains EN/DE labels, scalar/enum/template/variant controls, capacity-aware construction and action arguments. Domain-specific labels should be supplied through provider presentations. A dedicated reference selector additionally needs target identity/name/type presentation; generic enum values alone do not distinguish a graph reference from a scalar enum.

Nearby shared tests cover schema templates, union/source/window laws, lazy/bounded laws, collection/table laws and language-neutral first-paint/derived-control/collection-shape fixtures. These should be extended, not replaced.

## Recommended Contract

Create an IFC4x3 declaration catalog under `🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🧬️schema` with JSON as authoritative schema and explicit TS projection. It should contain schema identifier, entity inheritance/abstract status, ordered inherited attributes, optional/derived status, localized documentation, defined types, enum members, SELECT alternatives and recursive aggregate kind/lower/upper bounds. A type reference should distinguish scalar, entity, enum, select, aggregate and defined types.

Core public exports should include the catalog, inherited attribute lookup, value validation and the centralized mutation/diff/inverse pipeline. The details model must consume these exports rather than rebuild type interpretation. The editor should keep entity identities as exact decimal strings at the UI boundary and convert to owned bigint only in core commands; raw JS number cannot preserve the existing u64 IDs.

Use an IFC-specific `SnapshotDetailsProvider` in the Rust host to present named attributes and declaration-constrained capabilities while retaining existing details rendering and publication. A TS details projection can support direct tests and clients, but a standalone TS panel without the Rust render integration would not reach the existing app.

## Required Controls and Verification

Implement localized entity/type selection, named attribute rows with descriptions, optional unset and derived states, schema enum choices, subtype-filtered reference choices, SELECT alternative switching, aggregate insertion/removal/reordering with cardinality and set-uniqueness rules, nested typed values, header editing and complex constituent editing. Configure capabilities from mutation support and schema restrictions.

Add language-neutral cases exercising representative scalar, enum, reference, aggregate, SELECT, optional and inherited fields. For every accepted UI event, assert concrete domain mutation, sparse diff, inverse, undo/redo and exact source export/reimport. Validate source outputs independently with an IFC-aware third-party implementation; existing ruststep oracles validate Part21 syntax only, which does not establish IFC4x3 declaration/MVD conformance.

Existing IFC editor unit tests only establish role/dialect and invoke a shared history acceptance law. Existing IFC TS SQLite tests independently validate exact values and graph preservation, but they do not demonstrate field controls or browser interaction. Add actual host interaction coverage for keyboard focus, accessible names, English/German labels, invalid input recovery, reference navigation and aggregate editing. Include bounded rendering and cancellation for large models.

MVD completeness must be reported from explicit conformance evidence per view. A wildcard raw-graph editor supports syntax editing across views, but does not itself establish all MVD constraints.
