# r15 Plugin Exploration

Read-only source audit on 2026-10-10. Read root AGENTS.md, plugin AGENTS.md, coordination tail, and r13-exec-integrate.md. No builds or runtime tests were run by this agent; historical green flags do not establish current correctness.

## Current Integration

BIM already exists at `✏️s/🔌️plugins/🏙️bim`; extend it rather than introducing another plugin or state module. Its one artifact is `🗿️artifacts/🏢️model`, dialect `s.bim.model@1/*`. The local AGENTS requires authored parameters only, ModelInference for derived values, sparse diffs, concrete inverses, central protocol::apply_diff, and inverse sum laws.

The artifact root `🦀️.rs` exports snapshot/diff/mutation/inference and declares artifact capabilities. Standard `🏅️standards/🔖️1`, subset `🪆️subsets/✳️any/🦀️.rs` registers schema/inference descriptors, IO, viewer/editor, and house/office/demo. Actual plugin wiring is `🌎️hub/🧩️compositions/🏙️bim/🦀️.rs`: builder declares model, editor/viewer mutation rosters, activation and artifact-write capability. Its Rust package entry point is wiring only and exports the plugin component.

The editor owns command routing, not document application: `✏️editor/🎮️commands/*`, modes/windows, panels, presence and configuration. Available windows include world, plan, section, sheet, schedule, family. Snapshot schema includes sites/buildings/storeys, architectural and structural elements, types/materials, families/components, MEP, properties/classification, views/sheets, coordination and energy. These are implemented surfaces, not a fresh wishlist.

`🧬️schema/🧬️mutations/📏️set-storey-height/🔺️diff/🦀️.rs` only patches Storey.height; its inverse sets the original height. Derived downstream heights follow inference. `🧬️schema/💡️inferences/🕸️model-graph/🧳️instance/🦀️.rs` now lends sessions from the framework ArtifactInstanceOperationOwnerHandle; old thread-local registry gap is fixed in current source.

## Immediate Verification Priorities

1. Re-establish current lib/tests/wasm compilation. Coordination reports newer upstream pixels PNG and os-infinite errors after the older compile flags. Do not present those flags as current success.
2. Reproduce mounted editor/gesture tests. Current source still contains 8 MB stack wrappers, BIM_STACK_MB, and debug_stack_probe in `✏️editor/🧵️gestures/🧪️tests/🧷️app/🦀️.rs` (lines 14, 231). The integration report describes recursion/overflow; this remains an observable leftover needing root-cause verification and probe removal after diagnosis.
3. Reproduce house inference tests with bounded execution: the integration report lists hangs in levels/openings, zones/finishes, ramp invariance and attic mirroring. New graph/instance source means the historical cause cannot be assumed.
4. Verify actual UI mount plus progress/cancel and peer/undo re-inference. Instance source now documents cancellable steps and retaining last complete inference on faults; runtime behavior still needs confirmation.
5. Re-check post-IFC4 IFC2x3 committed exports against independent oracles. Coordination explicitly calls for re-blessing after newer writer changes. Likewise re-check opening-frame oracle mullion change, schedule values, annotations and wall solids; earlier failures are historical evidence only.

## Test Entry Points

Nx owner `@semio-tech/bim-model-rs`, package `📦️packages/🦀️rust/📋️project.json`, delegates to 📜️script.ts. Targets: check, test, test-snapshot-sqlite, verify-snapshot-sqlite, test-oracle, test-subject, test-parity. Script routes oracle/subject/parity through repository test platform with owner `🏙️bim`; supported role level and --case provide bounded focus.

Language-neutral cases live under subset `🧫️fixtures`; subset `🔮️oracles/🔣️.json` owns catalogs/oracle decisions, plugin oracle file owns test-only host packages. Existing independent libraries include Shapely, IfcOpenShell, three, jsonpatch, deepdiff, jsonschema, lxml and pypdf. Exact relevant cases include `infer-bim-1-levels-and-wall-heights`, wall joins/depth/solids, plan-and-diagnostics, views, quantities, components, opening-frames, schedules and IFC/IFC4/sheet exports. A release-only ignored incremental benchmark is present in model-graph incremental tests and should remain separate from functional gates.

No implementation files changed. This report is the only created file.
