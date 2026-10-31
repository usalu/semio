# Shared Visual Source Snapshot Boundary

The old core reader keyed geometry and membership by id(model), including one permanent None entry. It could therefore return stale default geometry and gave plans no matching source snapshot for original ramp references.

New core geometry_context(model) returns the same read-only source snapshot and matching read-only geometry. source_model(model) resolves source only. Equal canonical JSON content shares snapshots; source content and the canonical native producer closure invalidate geometry. Default source publication is detected by source-file modification timestamp and size plus source-reader identity. Two revisions are retained per source, geometry and membership cache. Explicit immutable source snapshots avoid repeated serialization per actor. Snapshot deepcopy yields ordinary owned mutable dict/list JSON, and the native public model API is unchanged.

Axon primitives, villa_axon and element_axon resolve the pair once. The core public module reexports geometry_context and source_model. Missing owns passing the same pair into plan context; no plan edits were made here.

Portable fixture and schema: visuals/🏛️savoye/🧱️core/🧪️snapshot/{🔣️.json,🧬️.schema.json,🐍️.py}. It verifies content sharing, input and producer invalidation, two-revision bounds, recursive mutation rejection, mutable owned deepcopy, source-only membership and default-source refresh; NumPy independently reads coordinates and jsonschema validates the fixture. Standard BIM loader registration added.

Native locked Python 3.12 normal 2GB admission: original RED 2 missing-interface errors. After implementation 2 functions PASS in 0.083s (exit 0). Existing renderer/revision/assembly/terrain/private-cache/quantity cohort also PASS: 20 tests in 3.385s, exit 0, exact runtime/producers retained in generated/native-r92/renderer-focused-r94.json. No full lifecycle or frozen-model claim.

Production files: core/📐️geometry.py, core/🐍️.py, axon/🧩️primitives.py, axon/🐍️.py. Tests: new core snapshot folder, existing axon ground fixture boundary mock, main test loader. Source model JSON unchanged.

Final bound review: default-source stamp stores only a digest, not a third retained source object; primitive cache also retains at most two geometry revisions. A thousand frozen-source/membership reads with JSON serialization forbidden PASS. Strengthened two-contract rerun GREEN0.087s.

Actual source-only integration now additionally passes: source model discipline lists independently reconstruct the same GUID memberships while building_geometry is forbidden. Three snapshot functions GREEN1.245s. Combined retained native renderer cohort expanded to23 tests and GREEN4.580s, exact runtime/source/producer manifest updated in renderer-focused-r94.json. Missing has integrated the identical model+geometry pair into plan context; its separate plan acceptance is owned there.
