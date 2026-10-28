# Retained Admission and Complex Control Fixes

This follow-up addresses all three actionable findings in `host-mvd-review.md`. Earlier direct resolver tests did not establish that virtual reference inputs passed retained preflight. That limitation is now covered explicitly in written native tests and remains unexecuted until native compiler ingress is available.

## Retained Reference Admission

The editor overrides the existing `snapshot_edit_is_admitted` hook. The same bounded `normalized_reference_edit` function converts reference controls into actual snapshot paths for both preflight and reduction. It checks the original serialized command byte bound, exact decimal/u64 syntax and path depth; then the normal shared admission checker examines the normalized real event. Actual target existence/type, optionality, capacity and uniqueness remain checked during reduction before mutation publication.

The exact-reference input resolves to SetValue on its owning IFC value; append resolves to InsertValue on the actual aggregate. No fake snapshot field, publication grant, generic admission exception or alternate mutation lane was introduced. First constituent insertion into an omitted empty complex list normalizes admission to creation of the real optional complex field as a one-element list. Reduction remains the shared complex resolver.

Tests now call retained admission and `snapshot_edit_emit`, rather than only `snapshot_edit_special`, for exact references and collection insertion. They also assert preflight admits first complex insertion. The actual host scheduler/browser path still requires execution.

## Complex Type Controls

Every additional complex constituent now exposes the complete indexed type chooser, including abstract partial entities. Selecting a type or editing its IFC name creates its direct attribute layout, preserving existing matching attributes by name. Required values receive typed templates, optional values remain unset and root GlobalIds are generated through the owned identity helper. One SetEntityComplex mutation publishes the change; written tests assert identity preservation and exact inverse application.

Primary complex components now include abstract partial types in their chooser. Ordinary simple entities continue to use concrete entity choices and flattened inherited attributes. Shared argument-template construction keeps both component paths consistent. Derived redeclarations contributed by another component remain read-only in the ancestor component; lookup is bounded by the schema's admitted inheritance depth.

## Evidence and Limits

- Extended language-neutral `tests.json` with three direct-attribute/abstractness component cases and added neutral complex-control scenarios.
- Wrote native tests for all three component layouts, name changes creating Coordinates, abstract primary choices, generated root GlobalIds, sparse complex mutation/inverse, retained admission and inherited derived immutability.
- IfcOpenShell 0.9.0.post1 independently confirms all three direct attribute counts and abstractness flags. The same oracle run confirms five attribute controls, three GlobalId vectors and reference insertion into a 301-entity property graph with zero EXPRESS errors.
- All changed Rust editor/provider/tests parse and format with rustfmt. No Rust type-check, native host execution or browser interaction is claimed because authentic compiler ingress remains unavailable. No enforcement or build infrastructure was changed.

Specialized geometry interactions, large-graph reference search/indexing, native semantic diagnostics and full host end-to-end testing remain required goal work. These fixes make each declared partial attribute reachable; they do not claim every intermediate repair state is conformant. See `complex-mapping-research.md` for the later correction: an external-single-chain authoring toggle is not a required conforming IFC feature, because the physical mapping selection uses the internal form for one leaf.

Files: IFC4x3 `✏️editor/{🦀️.rs,details.rs,tests.rs,tests.json,tests.feature}` and retained ticket `details-oracle.py` plus this report.
