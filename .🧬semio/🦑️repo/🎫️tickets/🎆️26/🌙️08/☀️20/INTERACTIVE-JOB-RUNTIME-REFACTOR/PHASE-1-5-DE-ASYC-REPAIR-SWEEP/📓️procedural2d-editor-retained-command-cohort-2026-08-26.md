# Procedural2d Editor Retained Command Cohort

Date: 2026-08-26  
Owner boundary: `✏️s/🔌️plugins/🌀️procedural/**/procedural2d/**/✏️editor/**` only.  
Excluded: Procedural3d, the shared verifier, and every unrelated plugin.

## Initial audit

The editor declared 19 actions and the internal `flowEvalTick` command as `Migrated`, but had no `register_tool_job_factories`, no `build_tool_job`, no exact proof rows, and no retained work implementation. Direct handlers showed that host clone/diff, generation projection, topology/media/canvas traversal, and flow evaluation were not truthful bounded-first-step routes.

## Exact dispositions

| Route | Disposition | Work owner |
|---|---|---|
| `nodeGraphViewport` | bounded first step | shared `BoundedArtifactCommandWork` |
| `setShowMode` | bounded first step | shared `BoundedArtifactCommandWork` |
| `nodeGraphEdit` | resumable | node-graph cursor |
| `addWidget` | resumable | node-graph cursor |
| `removeWidget` | resumable | node-graph cursor |
| `moveMediaNode` | resumable | media cursor |
| `connectMediaPorts` | resumable | media cursor |
| `setEvalOutputs` | resumable | media cursor |
| `reorganize` | resumable | reorganization cursor |
| `addGeneration` | resumable | generation cursor |
| `removeGeneration` | resumable | generation cursor |
| `renameGeneration` | resumable | generation cursor |
| `updateGenerationValues` | resumable | generation cursor |
| `generate` | resumable | generation cursor |
| `selectGeneration` | resumable | generation cursor |
| `canvasPointerDown` | resumable | canvas cursor |
| `canvasPointerMove` | resumable | canvas cursor |
| `canvasPointerUp` | resumable | canvas cursor |
| `canvasWheel` | resumable | canvas cursor |
| `flowEvalTick` | resumable | flow cursor |

The two concrete `Procedural2dBoundedCommandJobFactory` and `Procedural2dResumableCommandJobFactory` types register disjoint exact key sets and contracts. All 20 migrated rows have owner-local proof rows for `EditorApp<Procedural2dPlayApp>`, controller `s.procedural.procedural2d@1/*#editor`, and schema `procedural.2d`. `setLocale` is explicitly `ForbiddenFromUi`, so the complete 21-row typed command enum is classified without granting a twenty-first execution route. There is no fallback factory or compatibility wrapper.

## Bounds and lifecycle

- Raw wire maximum: 8,192 bytes.
- Semantic work maximum: 64 cursor items; extent 64 is accepted and 65 is rejected.
- Work per step: one item.
- Step ceiling: 7,500 microseconds in both contracts; tests assert measured steps remain below 8,000 microseconds.
- Checkpoint and progress cadence: every step.
- Custom checkpoint: fixed 24-byte `P2C1` record containing disposition, terminal bit, cursor, and observation digest.
- Cancellation: per-operation policy; the shared retained job checks cancellation before every work step.
- Close: custom cursors own no heap buffer and become terminal immediately after `begin_close`; the shared job retains and boundedly retires payload, command, snapshot, config, interaction, completion, and wire owners.
- Replay: restore validates the exact disposition and record shape, resumes at the recorded cursor/digest, and produces the same empty canvas result as uninterrupted execution.

## Source changes

- `.../✏️editor/🦀️component.rs`: exact factory registration/build path, proof rows, route dispositions, resumable cursor, checkpoint/restore/close, lifecycle/bound/timing tests, and lockstep synchronization of the editor callbacks. Genuine asynchronous menu and `AppIo` construction remain behind explicit `resolve_ready` boundaries.
- `.../✏️editor/🎮️commands/🕸️node-graph-edit/🦀️component.rs`: factored `apply_selected` so retained execution preserves the exact framework interaction selection semantics without reconstructing a private `InteractionView`.

## Validation ledger

| Gate | Result |
|---|---|
| Handler/static route audit | 20/20 classified: 2 bounded, 18 resumable |
| Direct `rustfmt --edition 2021` over both changed Rust files | PASS, exit 0 |
| Scoped `git diff --check` over Procedural2d editor and this report | PASS, exit 0 |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS, exit 0, `self-tests=460 clean` |
| Full JSON tool-job verifier | Expected repository-wide exit 1; Procedural2d failures `0`, Procedural2d remaining `0` |
| Native Cargo/Nx | Intentionally not run: coordinator holds the serialized Cargo slot |
| Wasm | Intentionally not run: coordinator holds the serialized Cargo slot |

No Cargo or Nx process was started during the serialized hold.

The full report is `📊️procedural2d-tool-jobs-live-2026-08-26.json`. Before the static-recognition repair it recorded `boundedRows=174`, `forbiddenRows=0`, `remaining=765`, and `failures=29`. The final report records `boundedRows=194`, `forbiddenRows=1`, `remaining=744`, and `failures=10`: accepted `+20`, forbidden `+1`, remaining `-21`, failures `-19`. Its Procedural2d slice contains exactly 20 accepted command rows, two explicit factory contracts, zero remaining rows, and zero failures. Every remaining failure is outside Procedural2d: seven exact FEM factory-identity findings plus three repository-wide aggregate deficits.
