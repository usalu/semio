# IFC2x3 Real Surface Final Audit

Read-only current-source audit2026-10-21. No production edits, native builds or UI execution. Main is concurrently fixing shared numeric normalization/payload normals; those transient blocks were deliberately excluded from defect claims.

## Eight Surfaces Confirmed

All native main wrappers under IFC/🏅️standards/🔖️2x3/🪆️subsets/{🧱️base,🤝️cv20,🏢️cobie,🧮️sav}/{✏️editor/🎭️modes/✏️edit,👁️viewer/🎭️modes/👁️view}/🪟️windows/🪟️main/🦀️.rs now delegate render/render_with/render_in_context to one standard-owned representation_surface module. Exact relative includes resolve on disk to IFC/🏅️standards/🔖️2x3/🚪️io/📐️geometry/🦀️.rs for all8 wrappers. No mesh_from_kind, fallback box constant, manufactured count/id/label or clamped top-level array-count logic remains in these wrappers.

All8 TypeScript main twins now declare arbitrary string meshId and interactionId:string, keeping framework.window.mesh identity and typed transforms/label. None contains meshId:"box". They are payload types rather than separate fake projection implementations. All8 editor/viewer root render_with_request_context methods route main body to main::render_in_context(doc.snapshot,cx) with original request StepContext; editor roots current line293 and viewer roots101. Pure render remains a legitimate separate unbounded-context convenience entry, not the route used by original-context main rendering.

## Ownership and Fidelity

Base schema geometry lives under 🧱️base/🧬️schema/📐️geometry with standard-specific facade and borrowed reader. Its own model package 🧬️schema/📦️packages/🦀️rust/🦀️.rs declares pub mod geometry. Base surface imports semio_s_artifact_stdio_ifc_2x3_base_model::...schema::{snapshot::Ifc2x3Snapshot,geometry}; derived CV20/COBie/SAV schemas explicitly reexport that same base model schema and retain their own mutation modules, so crate::standards::v2x3::subsets::<subset>::schema::geometry resolves to the owned base implementation. There is no source conversion into IFC4 snapshot or dependency on sibling editor from viewer.

Borrowed Rust Ifc2x3GeometryGraph operates over snapshot.document Part21Document, yielding original u64 id, first original type/args and full entities.len cardinality; constituent() exposes each native constituent. GeometryValue uses native Ref/Str/Enum/Real/Int/List getters. TypeScript reader uses own Part21Document/Part21Value, original bigint ids and entities/typeName/arguments; exact decimal coefficient/scale/exponent is converted to finite binary64 only for geometric computation. Unsupported/nonfinite computed geometry is diagnosed; original graph values are not overwritten or narrowed in storage. Integer ids are never converted to Number. Shared first-party GeometryGraph/GeometryValues interface is the reuse boundary, not a4x3 or4 snapshot cast.

Common engine profile discriminant is IFC2X3; IFC4-only tessellated face sets explicitly refuse representation-requires-ifc4. Complex source constituents retain count and are explicitly refused when unsupported, instead of losing extra geometry silently. IFC2x3 reader correctly counts all native Part21 constituents; this differs from IFC4 additional-complex field semantics (separately reported/fixed by owner).

## Progress and Cancellation

Standard surface render_with calls geometry::project_with and render_scene. render_in_context sets stage ifc-geometry; work limit is min(default work,cx.fuel_remaining); original callback checks cx.is_cancelled and cx.should_yield, consumes1fuel per projection tick, and maps controls to Cancel/Limit. It rechecks cancellation/yield after projection before rendering. render_scene clears meshes for every non-complete status and emits localized status/work/issues, so partial scene geometry is not published as complete after limit/cancel. Stable scene ids derive original product/representation, not source-list position. No per-entity replacement StepContext or uncancellable fake scene fallback was found.

Remaining known limitation: Limited/yield is terminal for this render call, with explicit status; prepared projection checkpoint/resumption is not implemented here. Payload normals/serialization are currently part of common engine numeric/payload work owned by main, not independently validated by this static audit. This audit does not claim every expensive payload/assembly phase is resumable or natively executed.

## Evidence Matrix

|Subset|Native Editor/Viewer|TS Twins|Original Context Routes|Manufactured Box|
|---|---|---|---|---|
|Base|2 checked|2 checked|2 checked|Absent|
|CV20|2 checked|2 checked|2 checked|Absent|
|COBie|2 checked|2 checked|2 checked|Absent|
|SAV|2 checked|2 checked|2 checked|Absent|

No new concrete integration defect established in the reviewed8roots/8native/8twins/reader boundary. Actual source integrity/read-path checks passed; native compiler/UI end-to-end not executed due known repository cachegate. Shared TS geometry/rings test execution belongs to main/high reports; this audit did not restate unrun native tests as passes.
