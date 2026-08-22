# P8zi Independent Global Payload-Authority Final Acceptance

## Verdict

**PASS — scoped source/static acceptance.** The P8t/P8z/P8zb/P8zf
payload-authority and CAD-transition findings are closed in the current source.
This is deliberately not a Phase 8 completion claim: native/release/Wasm and
isolated-worker execution remain unrun, and the repository-wide fail-closed
tool-job gate remains red for work outside this acceptance cohort.

No production source, generated output, cache, git state, or ticket metadata
was modified by this audit.

## Audit Basis

Read in full before inspecting the current worktree:

- `AGENTS.md`.
- `📓️p8t-independent-remaining-tools-global-audit.md`.
- `📓️p8w-global-payload-authority-repair.md`.
- `📓️p8z-independent-global-payload-authority-audit.md`.
- `📓️p8za-global-payload-authority-repair.md`.
- `📓️p8zb-independent-global-payload-authority-final-audit.md`.
- `📓️p8zc-global-payload-descriptor-coherence-repair.md`.
- `📓️p8zf-independent-global-payload-authority-acceptance-audit.md`.
- `📓️p8zg-cad-preview-transition-closure.md`.

The review was read-only and source-first. No Cargo command, test execution,
native/release/Wasm build, worker launch, descriptor generation/discovery,
cache operation, or modifying git command was run.

## CAD Transition Authority

The source has exactly two persistence authorities in
`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`:

- `snapshot_of` at lines 360-365 repacks a candidate then rejects any
  `engagement_session_json` delta. Ordinary camera, sun, reference, locale,
  node, dislocate, and engagement-input changes therefore cannot persist a
  checkpoint transition.
- `preview_transition_snapshot_of` at lines 370-381 is the sole changed
  checkpoint route. It requires the public operation identity, rejects a
  negative base generation, uses checked increment bounded by
  `CAD_PREVIEW_GENERATION_MAX`, and stamps the serialized identity only when
  the checkpoint actually changes.

The private raw packer (`cad_config_from_runtime`, line 241) is referenced only
by these two authorities (lines 361 and 371). A production command-tree scan
found no raw packer call and no raw `CadConfigMutation::Snapshot` construction.

Every live command-side mutation/clear is covered exactly once:

| Route | Source evidence | Persistence result |
| --- | --- | --- |
| Engagement submit, possible-select, repeat-last, abort, pointer-down, pointer-move | `🎮️commands/🤝️engagement/🦀️component.rs:29,75,96,100,118,150,178` | Each emits the transition authority; equal paths do not increment. |
| Utility clear | `🎮️commands/🧰️utility/🦀️component.rs:28-31` | Clear then exactly one transition authority. |
| Scene-import clear | `🎮️commands/📥️io/🦀️component.rs:45-47` | Clear then exactly one transition authority. The document-reset effect is still local until the helper succeeds. |
| Active-example/default reset | `🎮️commands/🗺️model-definition/🦀️component.rs:42-64` | The newly defaulted runtime is persisted only through the transition authority. |

The remaining core `CadPlayRuntime` session assignments at editor lines
635/670/689 are internal operation-local session helpers; their command callers
are the engagement routes above. The command tree has no other assignment.

`ArtifactEditor::handle` creates `CadDispatchCtx.preview_operation` from the
actual `doc.operation()` at editor lines 1103-1114. The identity comprises app
instance, parent document, operation id, operation generation, and canonical
base revision (lines 775-795). `CadPreviewStamp::is_fresher_than` requires exact
identity equality and a strictly greater generation (lines 798-810). This keeps
the source-level ABA and two-app guarantees intact.

The exhaustion/missing-context routes fail before any `Emit` escapes: the
helper's `?` is evaluated before the handler returns its locally constructed
emit. The source fixtures at lines 2303-2470 cover normal transitions,
utility/import/active-example clears, equal-checkpoint no-op, missing context,
maximum exhaustion, ordinary-snapshot bypass rejection, ABA, and two-app
identity isolation. They were inspected, not executed.

## Descriptor Coherence

The persisted preview generation remains one lossless domain:

- Runtime/config/generated Rust: `i32`.
- Proto: `int32`.
- GraphQL: `Int`.
- TypeScript: documented `number` range.
- JSON Schema: `integer`, minimum `0`, maximum `2147483647`.

`🎚️config/🦀️component.rs:14-28` rejects negative JSON-backed ingestion;
the transition helper rejects overflow before emitting. A Bun static assertion
read the live CAD JSON descriptor and confirmed `integer: 0..2147483647`.

## Payload Ownership And Bounded Re-entry

- Puzzle3d's `FillWorkerState` at
  `🧩️puzzle/.../✏️editor/⏳️precompute/🦀️component.rs:62-74` owns the fill
  request, scene, meshes, fill checkpoint, cursors, revision/generation,
  observation, and emitted checkpoint. Restore at lines 851-892 checks the
  4 MiB byte cap, mesh/count/URL caps, rebuilds collision state and `FillBuilder`,
  and rejects operation/generation mismatch. The shared worker job at
  lines 1001-1033 restores either checkpoint or admission state, rejects a
  mismatched restored request, then checkpoints each incomplete slice. The
  cold-reopen and two-operation/ABA fixtures at lines 1480-1521 are present.
- Sourcing checks the raw document envelope before
  `serde_json::from_str::<CurateSnapshot>` in
  `🎮️commands/📄️set-artifact-json/🦀️component.rs:18-25`. Its shared scanner
  caps bytes, depth, string bytes, and items before typed decode at
  `🧬️schema/🦀️component.rs:576-647`; contribution outer and nested envelopes
  are checked before typed decode at lines 649-675.
- Process applies the same pre-decode ordering at
  `🏭️process/.../✏️editor/🦀️component.rs:678-780`, including the nested
  machines payload before `serde_json::from_str`. Both contributed catalogs own
  `String` fields; the exact target scan found no `Box::leak`,
  `into_boxed_str`, or `leak_str` in CAD, Block, Process, Sourcing, Note,
  Layout, or Puzzle.
- Block, Process, Sourcing, Note, Layout, Puzzle3d, and Puzzle5d have no
  mutable `thread_local!`, `OnceLock<Mutex<_>>`, `LazyLock<Mutex<_>>`, or
  `static mut` match in the scoped source trees. The Note/Layout exact
  vocabulary scan also found no obsolete working-scene, scratch-cache,
  cache-miss, uncached, never-cached, or staleness-gap wording.

## Static Gate Results

```text
Bun JSON.parse of every currently changed scoped JSON descriptor (9 files)
=> exit 0

CAD command-tree raw packer/raw snapshot bypass scan
=> clean

CAD command-tree session mutator and authority-route scan
=> only the audited engagement, utility, import, and active-example paths

Scoped mutable-global/permanent-leak scan
=> clean

Scoped Note/Layout retired-vocabulary scan
=> clean

git diff --check -- <seven repaired cohort trees>
=> exit 0

bun ./📜️script.ts verify interactivity
=> exit 0; DENY mode clean in its declared four UI roots

bun ./📜️script.ts verify interactivity tool-jobs
=> exit 1, expected repository-wide residual: 34 global payload candidates,
   12 framework-reserved routes, and 875 live registrations remain fail-closed
   pending their independent migrations
```

## Mandatory Unrun Gates And Remaining Phase Blocker

This scoped PASS must not be used to close Phase 8. Still required:

- Native and release compile/type/borrow/Send gates for all affected plugins.
- Runtime execution of CAD normal/maximum/missing-context paths, stale preview
  rejection, ABA, two-app isolation, and cold reopen.
- Puzzle3d isolated-worker first tick, checkpoint, cold restart, cancellation,
  and two-operation isolation on native and Wasm targets.
- Process/Sourcing exact boundary execution before typed decode and descriptor
  discovery/regeneration.
- Completion of the repository-wide fail-closed tool-job ledger reported above.
