# Playbook Cache Repair And Retained Media Entrypoints

Read-only current WIP audit; no edits, broad tests, builds, or descriptor freshness claims. Earlier accumulation and canonical-handle findings were repaired in source and are not repeated here.

## Current Cache And Read Order

In `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs`, canonical handle validation now accepts exactly 64 lower-case hex bytes. Imported result parsing reads the actual handles string array and rejects invalid elements. Same-source replay returns cached handles. Replaced source text/handle strings enter ValueRetirement, which maintenance and close drive. Successful preview collects the current output handles, resolves imported geometry, then publishes their union before host retirement. This fixes the original duplicate replay and ownership order in source.

Two narrower lifetime gaps were sent to root:

- `imported_geometry_handles` keeps the prior cache when a changed source fails field validation or import. Preview suppresses that error then retain_current includes the prior cache, preserving stale claims despite a different durable source. A changed-source failure must invalidate/retire the prior cache rather than treating it as current.
- Export's evaluated_preview_geometry_handles publishes retain_current before imported_geometry_handles updates or clears the import cache. Export never republishes the final resolved union. Export immediately after source removal/undo therefore preserves previous imported claims until another operation. Republish the final union after resolving import, while preserving newly evaluated handles through temporary host close.

The replay law now compares exported geometry instead of requiring equal session label hashes; this is the appropriate geometric invariant. Native execution remains pending with the owner; no pass asserted.

## Actual Incremental APIs

Framework Flow host source `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:1436` provides `evaluate_step(&mut self, EvalStepBudget) -> Vec<String>`, returning remaining neurons and updating public last_eval_json (field line 230). A retained media work object can keep a supplied temporary host across calls, invoke a bounded number of dispatches, then extract handles only when remaining is empty. Host retirement must stay with that work owner on completion/failure/cancel.

`🧠️neural/⚙️engine/🦀️.rs:2159–2180` defines EvalStepBudget { dispatches, deadline } with dispatches(count) and until(count, clock, deadline). These bound dispatch admission between nodes; they cannot preempt one synchronous native BREP operator. evaluate_step also rebuilds tree/seeds/dirty projections and cold-retires temporary dictionaries within a call. Do not describe it as complete geometric or teardown slicing.

S Session source `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs:863` provides actual `tessellate_step(handle,tolerance,budget) -> TessellationStepOutcome`; cancel_tessellation at line 818 retires the exact handle/tolerance job. A supplied sibling authority isolates its jobs. Native `🧊️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️.rs:296` exposes `TessellationJob::step(&Body,budget)` and cancel at line 278. Its unit is one edge discretization/face triangulation; one pathological face remains a whole unit, explicitly documented in that file. It can replace synchronous tessellation work between media phases, with that granularity limit.

## Missing Retained Media Codecs

Native `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/⚙️engine/🦀️.rs` has export_step_sync at line 1566, export_stl_sync around 1574, export_obj_sync around 1579, and import_step_sync at 1601, plus synchronous STL/OBJ import. STEP in these names is the interchange format, not a retained-work turn. Session.export_solid_json/import_solid_json call these whole operations and whole JSON/base64 packing paths.

Bounded search of that engine and the artifact-owned `🚪️io` serializers/deserializers found no retained import/export media job/cursor. The actual IO artifact composers use complete snapshots/codecs; they do not offer a ready replacement for ModuleGeometryCommandWork's synchronous callback. Native boolean_job_sync/step_boolean_job_sync exist at lines 1077/1091, but are a boolean operation API rather than import/export.

The viable present partial improvement is retained Flow dispatch and retained tessellation with explicit owner/cancel phases. Full end-to-end bounded STEP/OBJ/STL/GLB import/export additionally requires retained decoding, geometry registration, serialization, packing, and output-publication cursors. A bounded wrapper cannot manufacture that guarantee around the current synchronous media methods. Root received these exact APIs and scope limits.
