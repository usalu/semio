# R15 Scope and Reuse Audit

Read-only inspection on 2026-10-10. No implementation files changed and no tests run by this audit. Historical verification below is explicitly reported by prior agents, not independently reproduced.

## Existing implementation

The request already has a substantial artifact at `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model`. Its AGENTS.md requires authored snapshot parameters, ModelInference derived data, sparse typed diffs with concrete inverses, generic protocol application, and no independent BIM module or state holder. The model artifact mounts editor, viewer, native codec and exports/imports through generic artifact declarations.

`ModelSnapshot` contains sites/buildings/storeys and walls/curtain walls/columns/beams/slabs/roofs/openings/stairs/railings/ramps/ceilings/spaces; type and material libraries; phases, zones, templates/classifications, views/sheets, energy conditions and coordination entities. `ModelInference` has levels, wall/curtain layouts, openings, solids, rooms, family/component/MEP output, annotations, plans/views/sheets, quantities/schedules/zones, effective properties, clashes/rules and energy output.

The existing storey-level inference propagates height edits through stacking, and wall-layout resolves authored top constraints from inferred levels. Curves use first-party bulge geometry, including concentric offset arcs. Rendering already shares World3d and Canvas2d projections between editor and viewer. Editor taxonomy includes panels, entities, tools/gestures, commands, utilities, config, presence and transient state.

## Concrete reusable infrastructure

- `🧰️framework/🔨️modules/📐️geometry`: first-party bulge, loops, triangulation, placement, mesh, section, skeleton, roof and collision primitives. Reuse rather than adding geometry libraries.
- `🧰️framework/🔨️modules/🧊️3d`: mesh/modeling, surface and collision infrastructure; generic World3d renderer hosts already consume plugin world descriptions.
- The model graph uses generic protocol FieldUpdate, InferenceCache, InferenceSession and touched paths. Its 34 node kinds already form a dependency DAG with incremental projection, progress and cancellation.
- Existing snapshot coordination, property kit, views and sheets give established patterns for new authored records. Existing keyed delta, typed Patch, mutation leaves and inverse leaves give the mandatory change vocabulary.
- Existing IFC2x3/IFC4 import/export plus STEP field support should host new groups, structure and cost records. Use schema-aware export helpers rather than a new importer/exporter state module.
- Existing language-agnostic feature cases and platform oracle manifests provide Shapely, IfcOpenShell, lxml, three, jsonpatch, deepdiff, jsonschema and pypdf only as test hosts. These are declared in `🔮️oracles/🔣️.json`.

## Missing packages confirmed

The coordination log schedules WP-15, WP-21 and WP-22 after the prior batch. A bounded source inventory found no Rust/JSON occurrence of DesignOption, Workset, CostItem, LoadCase, AnalyticalMember, StructuralSupport, option_scope or costs field under the BIM plugin. This is corroborating source evidence of unimplemented packages, not a runtime claim.

1. WP-15 options/worksets: authored option groups, options, worksets and membership references; inferred visibility and option takeoff; editor option selector/workset management; ephemeral shared ownership/locks; IFC group roundtrip. Explicitly keep active-view choices local config and ownership in presence, never in shared snapshot.
2. WP-21 structure: authored supports, load cases and point/line/area loads; inferred analytical members and rigid links from the existing geometry; load/support gestures and structural overlay; IFC4 structural analysis model and solver JSON. Define schema first and do not introduce a solver state holder.
3. WP-22 costing: authored cost items and type links; inferred element/type/storey totals from the existing quantity nodes; localized schedule/cost panel; CSV output. Add an honest dependency on quantity outputs so storey-height changes propagate quantities and costs together.

Each package needs mutation enum/leaf/diff/inverse/schema/codecs, reference validation and deletion cascades/refusals, honest graph node dependencies, text/binary/SQLite facets, editor/viewer integration, example fixture updates and language-agnostic third-party oracle cases. Each must also run its own lib tests and wasm checks once the upstream chain permits them.

## Integration and architectural risks

`r13-exec-integrate.md` records historical lib/tests/wasm checks green on Oct 9 but later failures and stale/blessed fixtures; those are not evidence of current runtime correctness. The coordination tail records active upstream pixel decoder and os-infinite errors, authored-helper mutation-law work, and pending IFC2x3 reblessing. Previously hung house/IFC tests and stack-overflow window mounts need real closure, not permanently skipping cases. Latest reports should supersede older partial tables.

The model-specific inference session currently holds cache, last node values and projected inference inside the artifact inference tree, with generic protocol execution. Its documentation describes a framework-owned mounted instance. Audit the actual registry/session ownership against the latest inference-law report before claiming the user's no separate BIM state/derivation module requirement is satisfied. Domain node definitions and pure inference formulas belong to the artifact; cache/session lifecycle orchestration belongs to the framework. No independent `bim` state engine is justified.

## Recommended completion order

Finish the current compile/law integration first while parallel owners implement WP-15, WP-21 and WP-22 from schema. Then regenerate facets and validate all affected examples once, run complete BIM subject/oracle parity and cross-platform wasm checks, and verify editor/viewer operations with runtime logs including height/curve edits, undo/redo, active option changes, structure and costing propagation. Scope is already broad enough that adding a duplicate plugin would lose substantial completed work.
