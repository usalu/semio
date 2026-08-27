# Renderer Contract Repair Checkpoint

## Verified Gates

- Coordinator baseline typecheck: 385 diagnostics, captured in coordinator r2 output. The earlier 112 count was a truncated-stream mistake, not a passing gate.
- Executor typecheck r5: 163 diagnostics; r7: 133 diagnostics. Full captured files are retained. r6 typecheck did not finish with a retained capture and has no claimed count.
- Executor full React r5: 470 passed, one failed. The old tutorial command fixture still expected removed `ActionArgDef.control`; it now asserts the actual stored string schema and options.
- Executor full React r6: invalid run, two suites failed while opening existing dependency files with EINTR. Subsequent ordinary source reads also reported EINTR. No pass credit.
- Executor full React r7: 473 passed, zero failed, four files. Coordinator independently repeated 473/473 in its full r3 log, 21.27 seconds. This does not validate Wasm runtime or the unfinished parameter backends.

## Contract Repairs

Current patches use the existing schema vocabulary for action argument storage, explicit resolved action/tool/argument labels, bilingual manifest-label resolution, current menu references, control IDs and icons, actual Three constructors, numeric projection variants, and native event types. The action-semantics fixture covers six action kinds against the existing native defaults; interactive jobs remain unclassified. Its strict Ajv test runs in the React suite; the native parity test is source-ready but unrun.

The authoritative schema metadata now includes the native execution `interactiveJob` field and exact nested staged-argument maps. Empty separator props use an empty-key record, avoiding the impossible intersection of `Record<string, never>` with the component discriminator. No native runtime behavior was changed for these metadata corrections.

## Remaining Cross-Owner Seams

- Tutorial capture/restore cannot use broadcast-only Presence or invented `selectionJson`. Dag owns the captured full-local interaction producer, including mode, granularity, selection anchors and exact revision authority. Consumer work waits for the coherent schema.
- `PatchWorld3dChrome` has no application producer. Only its host method, transport mappings and obsolete renderer consumer remain. Coordinator approved cohesive removal after the current native source hold, with live Presence replacement regression coverage and a fresh Wasm rebuild.
- The shared surface Wasm module has Graph, Raster, Map and Terrain sessions, but no BoardSession. BoardSession remains puzzle-owned and no built/linked Puzzle wasm-bindgen package is currently present. Coordinator chose an app-owned registered factory and actual product build routing; no fake shared declaration or permanent undefined stub is acceptable.
- Dag owns authoritative Flow browser ABI declarations. The existing handwritten consumer inventory incorrectly includes `renderFrame`; the actual browser entry exposes canvas operations and task results. No ambient renderer shim is being added.
- Repository discovery has five transitive diagnostics and is actively edited by another lane; no unrelated taxonomy changes were made here.

## Scope and Credit

This checkpoint repairs runtime contracts and tests; it is not evidence of bounded large-document interactivity. Whole-map patching, hashing, validation and notification remain the next renderer packet. P2/P3 retained parameter factories, allocation ownership, targeted undo and runtime latest-wins publication remain unfinished.
