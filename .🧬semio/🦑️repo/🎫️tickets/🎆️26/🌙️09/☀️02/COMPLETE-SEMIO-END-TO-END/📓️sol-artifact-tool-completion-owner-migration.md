# Artifact Tool Completion Owner Migration

## Boundary

This packet migrates the three generic production callers and the four reserved
Puzzle5d callers identified by
`📓️terra-artifact-tool-completion-owner-returning-rejection-census.md`:

- `RetainedPuzzleCommandJob`
- `DrawingGestureOperationJob`
- `WriterCommandToolJob`
- `Puzzle5dCopyJob`
- `Puzzle5dCutJob`
- `Puzzle5dPasteJob`
- `Puzzle5dImportJob`

Flow and Layout download completion are outside this packet and remain separate
acceptance work. The trusted Stdio+GIS process/native gate remains queued and is
not evidence for this packet.

## Source implementation

Each job now owns an explicit `pending_completion_rejection`. A failed
single-assignment handoff stores the exact returned `Emit`, `EphemeralEmit`, and
fault before returning a terminal job fault. Draw and Writer guard their local
reducers before dispatch/emission when this field is present; Puzzle enters its
existing terminal `Fault` phase. None retries completion or reconstructs output.

The existing `Emit::close_child_one` bounded child-emission closer is now public
so downstream app-owned jobs can honor the owner-returning API. It remains
narrow: it closes only a child emission and does not erase or generically dispose
typed mutation lanes. Each migrated job drives this closer before releasing the
rejection. Writer preserves its existing retained raw-input retirement order,
then closes the rejection before command, snapshot, config, and completion.

Draw and Writer have concrete low-byte-grant lifecycle laws that retain the
normal job owner until the rejected child emission is terminal and then close to
terminal empty. Puzzle has a source-order law because the shared generic module
does not own a concrete `ArtifactApp` fixture; the independent source oracle also
pins its handler and close funnel.

The four reserved Puzzle5d jobs now each retain the exact
`ArtifactToolCompletionRejection<EditorApp<Puzzle5dPlayApp>>`. Their replay
guards precede `commit.prepare` and publication. Their close funnels retire the
rejection before the ordinary work owner:

- composed child emissions are always first;
- Copy retires the exact `ClipboardWrite` fragment;
- Cut accepts only `DisconnectGrips` and `DeletePart`;
- Paste accepts only `CreatePart` and `ConnectGrips`, walking every nested
  string and grip owner;
- Import retains the returned flattened mutation vector as one identity and
  closes `ConnectKindCompatibility`, `DisconnectKindCompatibility`, and
  `ReplaceKindCatalogs` without reconstructing mutation pages;
- ephemeral presence/vector backings and both possible emitted/rejection fault
  graphs close through the same bounded one-owner-per-turn funnel.

Unexpected mutation, effect, event, task, config, draft, or transient lanes fail
closed instead of being silently dropped. Each job's terminal predicate requires
the rejection slot to be empty.

## Neutral fixture and independent oracle

The existing language-neutral awaited-completion fixture/schema now contains
`semio.plugin.artifact-tool-completion-rejection/v1` rows for accepted,
duplicate, and busy admission. The independently modeled Bun/AJV oracle proves:

- accepted assigns the submitted owner and returns none;
- duplicate preserves the existing cell and returns the full submitted owner;
- busy preserves the untouched cell and returns the full submitted owner;
- implicit retry and missing admission-state rows are schema-invalid;
- all three generic production sources retain the rejection, have a terminal replay
  guard, invoke the bounded child closer, and contain no audited `.is_err()` loss.
- all four Puzzle5d source regions retain the exact returned owner after prepare,
  guard replay before prepare, invoke incremental close, and contain no former
  lossy `if let Err(error) = completion.complete` branch.

The earlier generic receipt `7e647a` exited 0 with six assertions. Current-source
receipt `40d058` exits 0 with
`completion-rejection-oracle assertions=10` (three admission states, three
generic callers, four reserved Puzzle5d callers).

## Syntax and hygiene evidence

- The current Puzzle5d source passes parser-only
  `rustfmt --edition 2021 --emit stdout`; both neutral JSON files parse; scoped
  `git diff --check` is green (final combined receipt `666aec`). This changes no
  source and is syntax/format evidence only.

## Runtime status and nonclaims

Five exact Puzzle5d native laws are source-ready: four concrete output-owner
retirement laws plus one four-region replay/retention order law. No Cargo build
or Rust law was launched for this continuation while two Stdio rustc processes
and the hub/trinity/wasm dependency fan-ins were live. Therefore none of these
five laws is credited as executed. This report does not claim Flow, download
completion, or the broader caller census is closed, and does not claim the
queued trusted Stdio+GIS process gate ran.
