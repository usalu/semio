# IFC2x3 Representation Geometry

The eight IFC2x3 native viewer/editor main windows now project the original IFC2x3 representation graph through one standard-neutral engine shared with IFC4. Manufactured box/count scenes have been removed. The eight TypeScript scene contracts now retain real mesh and interaction identities. Existing details and retained mutation routing remain the central authoring surface.

## Architecture

The schema-first borrowed graph seam is at IFC/📐️geometry/📖️graph, with its neutral JSON contract at 📐️geometry/🧬️schema/🔣️.json. Rust and TypeScript readers expose native values and all original type constituents without constructing another standard's snapshot. The complete projector and whole rings module now live under 📐️geometry. IFC4 retains its original public facade and all existing source tests. IFC2x3 has its own schema/📐️geometry entry and reader; IFC4-only tessellated face sets are explicitly refused for IFC2x3.

Both standards diagnose unsupported complex products and representation items rather than silently reading their first constituent. IFC4 counts primary plus additional complex members correctly; IFC2x3 reads the original tuple array. Original entity/product/representation IDs remain full-width, including render interaction IDs and diagnostic JSON strings.

All eight native app roots pass request StepContext into projection. Caller cancellation, yield and fuel limits reach graph traversal. Work, vertices, triangles and recursion depth are finite. Incomplete scenes keep inspectable complete product prefixes, while public native/TypeScript render payloads emit no prefix and use the empty-scene camera. Projection has no resume cursor; callers restart from the retained snapshot. Serialization is synchronous over the bounded completed output.

The IFC2x3 TypeScript exchange entry decodes genuine UTF-8 IFC2X3 STEP into its own native Part21Document, using the shared first-party bounded Part-21 parser. Its writer retains full-width integers, Unicode, binary and exact decimal coefficient/scale/exponent values. It explicitly refuses EDM preambles in TypeScript; the existing native exchange path remains the EDM implementation.

## Neutral and Independent Evidence

Five language-neutral authored IFC2x3 STEP fixtures cover placed extrusion, mapped extrusion, tetrahedron BRep, complex product refusal and complex item refusal. Their manifest specifies original IDs, vertex/triangle counts, source-unit bounds and signed-solid volume. Independent existing Three.js 0.182.0 ExtrudeGeometry and ConvexGeometry reconstruct the same geometry and agree on counts, bounds and volume. These libraries remain test-only.

An unchanged buildingSMART Certification-datasets Architecture source is pinned beside the fixtures with provenance and CC-BY-4.0 attribution. Upstream commit: 80d976a9b193a26a8e928c3e79bff67af1de68a8. SHA-256: c4db65ba847f6b369a95d6c54fa11f4750cbe6d59f021934e8923d8d578e5885. Source: https://raw.githubusercontent.com/buildingSMART/Certification-datasets/80d976a9b193a26a8e928c3e79bff67af1de68a8/IFC%202.3.0.1%20(IFC%202x3%20TC1)/Simple-Scene/Building-Architecture.ifc . The permanent source suite checks the exact hash, parses its original graph and produces source-backed meshes with explicit issues for unsupported representations; it does not claim full model coverage or IFC2x3 EXPRESS validation.

## Validation

- IFC4 existing permanent geometry source suite: 31 groups passed after extraction; zero TypeScript diagnostics.
- IFC2x3 permanent schema/📐️geometry/🧪️tests/🟦️.test.ts: 11 groups passed; zero TypeScript diagnostics. Includes independent geometry oracle, original caller control, incomplete payloads, UINT64/I64 limits and decimal exponent preservation.
- IFC4 borrowed reader permanent 📖️reader/🧪️tests/🟦️.test.ts: 2 neutral complex groups passed; zero TypeScript diagnostics.
- Independent existing Rust grammar parsed all 26 initially owned native engine, reader, model and surface sources without syntax errors. Later native regression additions will be included in the root's final syntax pass.
- Native neutral, official source, original cancellation and incomplete payload regressions are authored. Native compilation and execution remain unverified: the repo-routed IFC package test fails before tests because Windows native-build cache allocation cannot enable per-directory case sensitivity (access denied). No build infrastructure was changed.

Input source loaders are retained in this ticket: ifc2x3-geometry-check.mjs and ifc4-reader-check.mjs. Generated execution modules have been removed after checks. The shared ticket remains open under the root agent.
