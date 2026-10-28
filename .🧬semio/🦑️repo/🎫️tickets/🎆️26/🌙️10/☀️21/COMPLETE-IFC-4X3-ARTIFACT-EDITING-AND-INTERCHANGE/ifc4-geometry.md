# IFC4 Geometry Projection

## Implementation

The IFC4 model now owns a representation scene schema and independent Rust/TypeScript projectors in `🧬️schema/📐️geometry`. Both IFC4 main surfaces delegate to the same Rust IO geometry helper. Placeholder-box counting and fake geometry were removed from these two surfaces.

Supported: triangulated face sets including PnIndex; polygonal face sets with simple concave planar faces; faceted BRep, closed/open shells, connected face sets, face/shell surface models; rectangle or arbitrary closed polyline profiles extruded along a valid direction; nested 2D/3D local placements; representation context origins/subcontexts; mapped representations with source origins and uniform/nonuniform operator scales. Mapping axes follow IFC projection rules and mirrored transforms reverse leaf triangle winding. Vertices are welded per representation; indices remain correct when extrusions share vertices.

Empty/unsupported documents produce no manufactured shape. Cycles, missing references, malformed numbers, unsupported representations/profile families, holes and nonplanar polygons produce explicit issues. Opening elements are voids, so the projector omits them. Hosts requiring opening subtraction are also omitted with an explicit unsupported-opening-subtraction issue until boolean subtraction is implemented. Source length units remain source length units; no guessed unit conversion is applied. Product and representation identities are actual IFC entity IDs (u64 in Rust, bigint in TypeScript); scene instance IDs preserve the full decimal identity.

## Work Controls And Surface Boundary

Both projectors enforce work, vertex, triangle and recursion budgets. Original caller callbacks report work and can cancel or yield with distinct cancelled/limited status. Source-list traversal consumes work and avoids whole-list clones. Products publish atomically: cancelled/limited products never publish partial meshes. TypeScript rejects nonfinite/negative budgets. Labels are bounded to 256 Unicode characters.

Both Rust ArtifactViewer/ArtifactEditor render_with_request_context overrides pass the original StepContext to shared render_in_context. Every projection step checks live cancellation and original deadline/yield state and consumes original fuel. A limited/cancelled surface clears completed geometry before normal/JSON generation and publishes explicit status/work/issues. Full normal/JSON generation remains bounded by the global vertex/triangle limits, but it is not itself an incremental retained serialization job.

The default direct render convenience function remains bounded and uses an accepting callback. Request renders are now cancellable through the actual runtime context. The current projector is one-shot: budget yield does not retain/resume a projection cursor across turns. The mounted snapshot preparation and lifecycle hooks were inspected; they require a retained, admitted snapshot/cursor owner, not an ad-hoc global cache. No background-job/build framework refactor was introduced. No user-visible progress bar or richer diagnostic overlay was added; status is published in the world scene status lane.

## Tests And Validation

Schema-first additions include the JSON scene contract, Gherkin behavior and 10 shared source-graph fixtures: placed extrusion; triangle tessellation; empty; unsupported; mapped triangle with nonzero mapping origin; concave polygon; voided host; placement cycle; PnIndex with unused outlier coordinates; faceted tetrahedron BRep.

Rust tests cover all neutral fixtures, bounds, product identity, mesh volume/area, mesh payloads and per-step cancellation/yield with no partial products. Six registered genuine IfcOpenShell/OpenCascade differential handlers consume the same source fixture graphs: placed-extrusion, triangulated, mapped-triangle, concave-polygon, pn-index, faceted-brep. The independent oracle checks expected counts/bounds/measures and compares with subject output. No claim is made that these third-party scenarios have executed on this host.

AJV 2020 strict validation of the new scene schema passed. Static checks passed for all 10 fixture reference graphs, exact synchronization of six Gherkin docstrings with neutral JSON, matching Rust/Python adapter registrations, and new source paths under 256 characters (maximum measured 144 before TS additions).

Native attempt: `bun run repo run '@semio-tech/stdio-ifc-rs:test' --output=stream` exited 1 before IFC compilation/testing. Repository native-build cache directory allocation was denied Windows per-directory case sensitivity. No Cargo/build/cache implementation was changed. Rust compilation and IfcOpenShell parity remain unverified on this host.

TypeScript projector and permanent tests added at `🧬️schema/📐️geometry/🧪️tests/🔬️unit/🟦️.test.ts`; tests consume the real new decodeIfc4 adapter, not a test-only parser. The module now uses the repository node:test/node:assert convention and is registered by the main test router. The permanent source suite was executed through ticket/ifc4-geometry-check.mjs using Node's built-in TypeScript transformation and real source dependencies (decodeIfc4, shared Part21 parser, owned Binary64 helper). Strict TypeScript 5.9.3 compilation produced zero diagnostics. All 15 tests passed: 10 neutral source graphs; cancellation and caller-yield at every extrusion source step; work/vertex/triangle/depth budgets; actual scene mesh identity/topology; bigint product IDs above 2^53; and repeated shared-vertex extrusions. The final rerun after native/TS surface parity review also passed all 15 tests with zero diagnostics. The harness inspects every node:test record because the test registration promise by itself does not propagate assertion failure.

## Source Review

Conformance peer independently reviewed Rust borrow/type/index structure and the IFC mapping/axis/void semantics. Mapping origin is target * source origin, independently confirmed in IfcOpenShell C++/Python implementations. A nonidentity geometric-context origin is treated as a global output-frame transform; independent engine behavior for nonzero context origins still needs an explicit differential frame contract.

## Changed Files

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📐️geometry/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📐️geometry/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📐️geometry/🧪️tests/🔣️.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📐️geometry/🧪️tests/🥒️.feature
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📐️geometry/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/📐️geometry/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📦️packages/🦀️rust/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/📦️packages/🦀️rust/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/👁️viewer/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/✏️editor/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🔺️differential-ifc-4/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🔺️differential-ifc-4/🐍️.py
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🔺️differential-ifc-4/🥒️.feature
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📐️geometry/🟦️.ts
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📐️geometry/🧪️tests/🔬️unit/🟦️.test.ts
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️21/COMPLETE-IFC-4X3-ARTIFACT-EDITING-AND-INTERCHANGE/ifc4-geometry.md

## Final Native / TypeScript Parity Audit

Reviewed matrix composition; projected mapping axes and nonuniform scale; direct mapping-source origin composition; per-leaf reflection winding; world-space vertex welding; triangle/PnIndex bounds; concave ear clipping and planarity; profile closure/placement/direction; shell bounds; product-atomic publication; void handling; labels; graph cycles/depth; and cancellation/limit statuses. The executable TS tests exposed a real shared fixture problem: Axis2Placement3D supplied RefDirection without Axis, contrary to IFC4's joint-provision rule. The source fixtures and six differential docstrings were handcrafted to supply the existing z-axis reference. Rust had already enforced that rule. The audit also corrected Rust extrusion index generation after vertex welding: it now retains actual bottom/top vertex IDs rather than assuming contiguous offsets. TypeScript limited/cancelled camera framing now matches the native surface's cleared-geometry behavior.

Exact native/TS work counters can differ because implementation lookup steps differ; both enforce the same declared finite limits and stop before publishing a partial product. Native binary numeric coordinates round to f64; TS uses the owned Binary64 query helper and retains bigint identities throughout.

Explicit remaining unsupported geometry: circle/ellipse/parameterized profile families beyond rectangles; profiles or faces with holes; boolean/CSG and opening subtraction; revolved/swept disks and general swept/advanced BRep/spline surfaces; curves/wires as body geometry; nonlocal/grid/linear placements; other transformation-operator dimensionalities; and material/color/style projection. Such unsupported representation graphs do not get fallback geometry. A product with any unsupported Body item is omitted as an atomic unit with a diagnostic. Geometry is rendered in source length units, including inches or millimetres; no unit or georeferencing interpretation is guessed. Nonidentity representation-context frame parity needs an explicit engine/output-frame test contract.

Prepared-view limitation remains material: request projection yields with explicit limited status and has no retained/resumable cursor. No progress-bar UI is provided; stage/work/status/issue data are exposed through the existing request and scene channels. Normal/JSON generation for a completed scene is bounded but synchronous. Runtime request cancellation/deadline/fuel now stops projection; cancelled/limited scenes avoid that normal/JSON phase entirely.

The native build and actual IfcOpenShell/OpenCascade differential execution remain blocked/unverified on this host. Passing TypeScript execution and schema checks do not establish native test success or full IFC4 geometry conformance.

Additional input source retained: ifc4-geometry-check.mjs. Generated transformed modules are under the ticket's generated folder and should be removed by final ticket cleanup with the other agents' generated outputs.

## Product Reference Diagnostic Correction

Final review found that an evident product with a missing or incorrect representation reference was skipped before any issue could be recorded. Both native and TypeScript projectors now classify product candidates using close, owned product-name metadata and constrained product attribute hints. Relation/type/style and known nonproduct root metadata are excluded. A valid 22-character IFC GlobalId plus a resolved object-placement reference supplies a strong hint for an uncommon product class; a valid product representation supplies a hint when its placement is explicitly unset. This is a projection classifier, not a claim that the raw snapshot has passed complete IFC EXPRESS validation.

An evident product whose attribute 6 references an absent entity now reports missing-product-representation-#N. A reference to a nonrepresentation record reports invalid-product-representation-TYPE-#N. No substitute geometry is emitted. Metadata's unrelated attribute-6 references stay quiet, including references that happen to point at a real product-definition shape.

New language-neutral contract: 🧬️schema/📐️geometry/🧪️tests/📜️references.json (six graph cases plus owned classifier metadata). The test uses direct owned snapshots after valid IFC4 decode because the strict codec correctly refuses dangling input references before projection. Both Rust and TS regressions consume the same contract. Static validation confirms that both implementation catalogs exactly match that contract. Geometry never reads complex constituents, so optional canonical omission of an empty complex vector requires no special handling.

Actual source execution after this correction: 16/16 permanent TypeScript tests pass; strict compiler diagnostics remain empty. Native regressions remain authored but unexecuted due the previously documented native-build permission blocker. The shared main test router was not edited.

Additional created file: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📐️geometry/🧪️tests/📜️references.json.
