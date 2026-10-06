# 📓️ S5-CHANNEL — Wave B In The Plugin Trees: Crate → Change Shape → Owner

Wave B (design §20.7 / §22.4, landed 2026-10-05 06:12–06:47) removed `Edit.description` and every carrier of it. This sheet lists what
changed in each crate of the `✏️s/Cargo.toml` workspace, so an owner who meets one of these shapes in a check knows it is the
wave, not a regression. Source: the driver's apply log (`🗑️generated/s5-channel/waveb-apply-1.txt`) and the positional pass
(`waveb-positional-apply-1.txt`). "verified" = `cargo check --lib --tests` exit 0 inside the wave's lock hold; every other crate
is WRITTEN BUT UNVERIFIED until its batch runs (command at the end).

The four shapes an owner can meet:

- `preflight(&self, input, lane)` — the `description: Option<&str>` parameter and its length check are gone (trait
  `ArtifactStoreOneItemPreparationFactory` / `…Authority`); refusal texts end in `-lane`, no longer `-lane-or-description-envelope`.
- `ArtifactCommand::Apply { mutations, transaction }` / `ApplyInLane { mutations, lane, transaction }` — no `description` field.
- `begin_apply_batch` / `begin_outbound_apply_batch` / `apply_one` / `dispatch_apply_exact` — one argument fewer (the description).
- `…Preparation` structs lost their retained `description` owner (field, literal, close-step arm, `terminal_is_empty` term);
  `rejected.into_owners()` answers `(reason, mutations)`.

## S5-PUZZLE (verified in the hold)

| Crate | Files | Changes | State |
|---|---:|---|---|
| `semio-s-artifact-puzzle-2d` | 6 | 6× description length / owner condition removed; 5× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 2× preparation struct field `description` removed; 2× preparation literal `description: request.description` removed; 2× `preflight(input, description, lane)` → `preflight(input, lane)`; 2× refusal text `…-lane-or-description-envelope` → `…-lane` | verified 06:38–06:41 |
| `semio-s-artifact-puzzle-3d` | 7 | 6× description length / owner condition removed; 6× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 2× preparation struct field `description` removed; 2× preparation literal `description: request.description` removed; 2× `preflight(input, description, lane)` → `preflight(input, lane)`; 2× refusal text `…-lane-or-description-envelope` → `…-lane` | verified 06:38–06:41 |
| `semio-s-artifact-puzzle-5d` | 7 | 6× description length / owner condition removed; 6× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 2× preparation struct field `description` removed; 2× preparation literal `description: request.description` removed; 2× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× refusal text `…-lane-or-description-envelope` → `…-lane` | verified 06:38–06:41 |

## S5-TEXT-STDIO

| Crate | Files | Changes | State |
|---|---:|---|---|
| `semio-s-artifact-stdio-contract` | 2 | 3× `.preflight(m, None, lane)` call → `.preflight(m, lane)`; 1× `preflight(input, description, lane)` → `preflight(input, lane)` | verified 06:38–06:41 |
| `semio-s-artifact-stdio-docx` | 2 | 3× description length / owner condition removed; 2× `begin_*apply_batch(…)` description argument removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× other: self.external_r | verified 06:38–06:41 |
| `semio-s-artifact-stdio-semio` | 2 | 4× description length / owner condition removed; 2× `apply_one(generation, mutation, description, lane)` → 3 arguments; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× other: self.external_r | verified 06:38–06:41 |
| `semio-s-artifact-stdio-wav` | 1 | 3× `begin_*apply_batch(…)` description argument removed | verified 06:38–06:41 |
| `semio-s-artifact-stdio-zip` | 1 | 3× description length / owner condition removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× other: self.external_r | verified 06:38–06:41 |
| `semio-s-artifact-trinity-jack` | 2 | 2× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-trinity-rewriting` | 2 | 3× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-vcs-vcs` | 4 | 4× `.preflight(m, None, lane)` call → `.preflight(m, lane)`; 3× description length / owner condition removed; 2× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× refusal text `…-lane-or-description-envelope` → `…-lane` | WBU — check owed |
| `semio-s-artifact-writer-writer` | 4 | 7× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 3× description length / owner condition removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× refusal text `…-lane-or-description-envelope` → `…-lane`; 1× `Edit`/`HistoryEdit` literal field removed | WBU — check owed |

## S5-TOOLS

| Crate | Files | Changes | State |
|---|---:|---|---|
| `semio-s-artifact-draw-drawing` | 4 | 7× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 3× description length / owner condition removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× refusal text `…-lane-or-description-envelope` → `…-lane` | WBU — check owed |
| `semio-s-artifact-energy-model` | 1 | 3× description length / owner condition removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× refusal text `…-lane-or-description-envelope` → `…-lane` | WBU — check owed |
| `semio-s-artifact-fem-2d` | 4 | 3× description length / owner condition removed; 2× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 2× preparation literal `description: request.description` removed; 2× `.preflight(m, None, lane)` call → `.preflight(m, lane)`; 1× preparation struct field `description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× refusal text `…-lane-or-description-envelope` → `…-lane`; 1× sized-description assertion dropped | WBU — check owed |
| `semio-s-artifact-fem-3d` | 4 | 3× description length / owner condition removed; 3× `.preflight(m, None, lane)` call → `.preflight(m, lane)`; 2× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× other: return Ok(store::SnapshotReti; 1× refusal text `…-lane-or-description-envelope` → `…-lane` | WBU — check owed |
| `semio-s-artifact-forms-forms` | 2 | 3× description length / owner condition removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× other: return Ok(store::SnapshotReti; 1× refusal text `…-lane-or-description-envelope` → `…-lane`; 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-gis-gismap` | 4 | 3× description length / owner condition removed; 2× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× refusal text `…-lane-or-description-envelope` → `…-lane`; 1× `begin_*apply_batch(…)` description argument removed | WBU — check owed |
| `semio-s-artifact-gis-gisterrain` | 2 | 3× description length / owner condition removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× refusal text `…-lane-or-description-envelope` → `…-lane`; 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-layout-layout` | 2 | 2× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-lowpoly-lowpoly` | 3 | 6× description length / owner condition removed; 2× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 2× preparation struct field `description` removed; 2× preparation literal `description: request.description` removed; 2× `preflight(input, description, lane)` → `preflight(input, lane)`; 2× other: if grant.maxi; 1× refusal text `…-lane-or-description-envelope` → `…-lane` | WBU — check owed |
| `semio-s-artifact-note-note` | 3 | 4× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-playbook-playbook` | 2 | 3× description length / owner condition removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× refusal text `…-lane-or-description-envelope` → `…-lane`; 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-procedural-generation2d` | 5 | 6× description length / owner condition removed; 3× preparation literal `description: request.description` removed; 3× `.preflight(m, None, lane)` call → `.preflight(m, lane)`; 3× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 2× preparation struct field `description` removed; 2× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× other: return Ok(store::SnapshotReti; 1× refusal text `…-lane-or-description-envelope` → `…-lane`; 1× sized-description assertion dropped | WBU — check owed |
| `semio-s-artifact-procedural-generation3d` | 5 | 6× description length / owner condition removed; 5× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 2× preparation struct field `description` removed; 2× preparation literal `description: request.description` removed; 2× `preflight(input, description, lane)` → `preflight(input, lane)`; 2× other: return Ok(store::SnapshotReti; 1× refusal text `…-lane-or-description-envelope` → `…-lane` | WBU — check owed |
| `semio-s-artifact-shooting-shooting` | 2 | 2× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |

## S5-GRAPHS-WIRES

| Crate | Files | Changes | State |
|---|---:|---|---|
| `semio-s-artifact-dag-dag` | 1 | 3× description length / owner condition removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `.preflight(m, None, lane)` call → `.preflight(m, lane)`; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× other: let bytes = d; 1× refusal text `…-lane-or-description-envelope` → `…-lane` | WBU — check owed |
| `semio-s-artifact-mathematical-equation` | 3 | 3× description length / owner condition removed; 2× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× refusal text `…-lane-or-description-envelope` → `…-lane` | WBU — check owed |
| `semio-s-artifact-space-home` | 3 | 5× `.preflight(m, None, lane)` call → `.preflight(m, lane)`; 4× description length / owner condition removed; 2× preparation literal `description: request.description` removed; 1× preparation struct field `description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |

## S5-FLOWCAD

| Crate | Files | Changes | State |
|---|---:|---|---|
| `semio-s-artifact-cad-cad` | 3 | 6× description length / owner condition removed; 2× preparation struct field `description` removed; 2× preparation literal `description: request.description` removed; 2× `preflight(input, description, lane)` → `preflight(input, lane)`; 2× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 1× refusal text `…-lane-or-description-envelope` → `…-lane` | WBU — check owed |

## S5-STROKES-NORM

| Crate | Files | Changes | State |
|---|---:|---|---|
| `semio-s-artifact-norm-din18599` | 1 | 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-norm-din4108` | 1 | 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-norm-en1995` | 1 | 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-norm-en1996` | 1 | 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-norm-en1997` | 1 | 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-norm-en1998` | 1 | 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-process-process3d` | 2 | 8× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 6× description length / owner condition removed; 2× preparation struct field `description` removed; 2× preparation literal `description: request.description` removed; 2× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× `.preflight(m, None, lane)` call → `.preflight(m, lane)`; 1× other: let bytes = d; 1× refusal text `…-lane-or-description-envelope` → `…-lane` | WBU — check owed |
| `semio-s-artifact-raster-raster` | 9 | 9× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 6× description length / owner condition removed; 2× preparation struct field `description` removed; 2× preparation literal `description: request.description` removed; 2× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× refusal text `…-lane-or-description-envelope` → `…-lane` | WBU — check owed |
| `semio-s-artifact-remodel-remodeling` | 2 | 1× `begin_*apply_batch(…)` description argument removed; 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-wfc-2d` | 1 | 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-wfc-3d` | 2 | 2× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-wfc-bitmap` | 2 | 3× description length / owner condition removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× refusal text `…-lane-or-description-envelope` → `…-lane`; 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |

## no session-5 owner (S5-INFRA census routes them)

| Crate | Files | Changes | State |
|---|---:|---|---|
| `semio-s-artifact-animate-presentation` | 3 | 5× description length / owner condition removed; 2× preparation literal `description: request.description` removed; 2× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed; 1× preparation struct field `description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× `.preflight(m, None, lane)` call → `.preflight(m, lane)` | WBU — check owed |
| `semio-s-artifact-block-2d` | 2 | 3× description length / owner condition removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× refusal text `…-lane-or-description-envelope` → `…-lane`; 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-block-3d` | 2 | 6× description length / owner condition removed; 2× preparation struct field `description` removed; 2× preparation literal `description: request.description` removed; 2× `preflight(input, description, lane)` → `preflight(input, lane)`; 2× other: return Ok(store::SnapshotReti; 1× refusal text `…-lane-or-description-envelope` → `…-lane`; 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-block-5d` | 2 | 3× description length / owner condition removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× refusal text `…-lane-or-description-envelope` → `…-lane`; 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |
| `semio-s-artifact-demonstrator-playground` | 1 | 3× description length / owner condition removed; 1× preparation struct field `description` removed; 1× preparation literal `description: request.description` removed; 1× `preflight(input, description, lane)` → `preflight(input, lane)`; 1× refusal text `…-lane-or-description-envelope` → `…-lane` | WBU — check owed |
| `semio-s-artifact-sourcing-curation` | 2 | 6× description length / owner condition removed; 2× preparation struct field `description` removed; 2× preparation literal `description: request.description` removed; 2× `.preflight(m, None, lane)` call → `.preflight(m, lane)`; 2× `preflight(input, description, lane)` → `preflight(input, lane)`; 2× other: let bytes = d; 1× refusal text `…-lane-or-description-envelope` → `…-lane`; 1× `ArtifactCommand::Apply{,InLane} { description }` literal/pattern field removed | WBU — check owed |

## Files outside any crate target root (mounted by path from another package)

- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️unit/🦀️.rs` — playbook procedural extension tests (S5-TOOLS)
- `✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs` — space core (S5-GRAPHS-WIRES)

## Owed check (after the coordinator's "LOCKS OPEN"; gate v5, `CARGO_BUILD_JOBS=3`, batches of ≤ 8 crates)

```
cargo check --manifest-path ✏️s/Cargo.toml --lib --tests --keep-going --message-format=short \
  -p semio-s-artifact-animate-presentation \
  -p semio-s-artifact-block-2d \
  -p semio-s-artifact-block-3d \
  -p semio-s-artifact-block-5d \
  -p semio-s-artifact-cad-cad \
  -p semio-s-artifact-dag-dag \
  -p semio-s-artifact-demonstrator-playground \
  -p semio-s-artifact-draw-drawing \
  -p semio-s-artifact-energy-model \
  -p semio-s-artifact-fem-2d \
  -p semio-s-artifact-fem-3d \
  -p semio-s-artifact-forms-forms \
  -p semio-s-artifact-gis-gismap \
  -p semio-s-artifact-gis-gisterrain \
  -p semio-s-artifact-layout-layout \
  -p semio-s-artifact-lowpoly-lowpoly \
  -p semio-s-artifact-mathematical-equation \
  -p semio-s-artifact-norm-din18599 \
  -p semio-s-artifact-norm-din4108 \
  -p semio-s-artifact-norm-en1995 \
  -p semio-s-artifact-norm-en1996 \
  -p semio-s-artifact-norm-en1997 \
  -p semio-s-artifact-norm-en1998 \
  -p semio-s-artifact-note-note \
  -p semio-s-artifact-playbook-playbook \
  -p semio-s-artifact-procedural-generation2d \
  -p semio-s-artifact-procedural-generation3d \
  -p semio-s-artifact-process-process3d \
  -p semio-s-artifact-raster-raster \
  -p semio-s-artifact-remodel-remodeling \
  -p semio-s-artifact-shooting-shooting \
  -p semio-s-artifact-sourcing-curation \
  -p semio-s-artifact-space-home \
  -p semio-s-artifact-trinity-jack \
  -p semio-s-artifact-trinity-rewriting \
  -p semio-s-artifact-vcs-vcs \
  -p semio-s-artifact-wfc-2d \
  -p semio-s-artifact-wfc-3d \
  -p semio-s-artifact-wfc-bitmap \
  -p semio-s-artifact-writer-writer
```
