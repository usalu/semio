# Native Default And Parent Acceptance Ledger

## Snapshot

Snapshot: `2026-09-09T12:40:05+0200`. This is a read-only reconciliation of `🗑️generated/native-library-recovery-plan.json` and `🗑️generated/native-library-recovery.tsv`. The frozen machine-readable candidate list is `🗑️generated/native-default-parent-candidates-66632-snapshot.json`.

The active owner is `matrix66632`. It owns the 39-plan matrix (19 default leaves and 20 default parents); this audit neither starts a duplicate gate nor suggests interrupting it.

## Current Matrix Ledger

| Classification | Exact gates | Evidence and consequence |
| --- | --- | --- |
| Accepted current defaults | `semio-s-artifact-vcs-vcs`, `semio-s-artifact-process-process3d` | Terminal `0` rows in the frozen TSV. These two default library gates require no duplicate run. |
| Completed rows requiring a retry | `semio-s-artifact-mathematical-equation`, `-flow-flow`, `-shooting-shooting`, `-lowpoly-lowpoly`, `-reasoning-wires`, `-forms-forms`, `-layout-layout`, `-cad-cad`, `-playbook-playbook`, `-imperative-procedure`, `-remodel-remodeling` | Terminal nonzero rows: `1` except Shooting and Playbook (`130`). None establishes a package default. The raw receipts show Equation's shared OS-kernel `E0277`, Flow's shared plugin `E0308`, and Imperative/Remodel's shared OS-kernel `E0277`; those are consumer failures before their owning package can be accepted. Lowpoly/Wires/Forms/Layout/CAD stopped during project graph construction, and Shooting/Playbook stopped at `framework-graph:generate`, so they are prerequisite interruptions rather than package regressions. |
| Current queued or unrun leaf defaults | `semio-s-artifact-energy-model`, `-dag-dag`, `-draw-drawing`, `-raster-raster`, `-note-note`, `-sourcing-curation` | No frozen TSV row. They remain owned by `matrix66632`. |
| Current queued or unrun parent defaults | `semio-s-plugin-writer`, `-mathematical`, `-vcs`, `-demonstrator`, `-architect`, `-process`, `-lowpoly`, `-reasoning-mindmap`, `-forms`, `-layout`, `-cad`, `-playbook`, `-imperative`, `-remodel`, `-energy`, `-dag`, `-draw`, `-raster`, `-note`, `-sourcing` | No parent has a frozen TSV row. All 20 remain owned by `matrix66632`. |

At this snapshot the required matrix has 37 unaccepted gates: 11 completed retry candidates and 26 queued or unrun gates. The only post-matrix native candidates are the 11 named retry rows, unless `matrix66632` reruns them itself or later source changes invalidate the two accepted rows.

## Prior Terminal Receipts

The earlier ordinary default `0` receipts for Writer, Animate, Demonstrator Playground, Sequence, and Architect, and the earlier parent `0` receipts for Flow, Animate, Shooting, and Sequence are historical evidence only. They are excluded from current-source acceptance because this ledger has no fresh matching terminal receipt after the intervening shared and Flow work. They also are outside the 39-entry recovery plan, so this audit does not create duplicate work from them.

## Feature And Baseline Separation

Animate's `preview-window` and Lowpoly's `cad-fixtures` feature checks have terminal `0` receipts. They do not establish their default library gates; Lowpoly's default remains a retry candidate.

The old default-matrix plugin-reactor `E0382`, old parent-matrix OS/store `E0425`/`E0422`, and the value-derive required-nullable test row are out of scope for this default/parent ledger. The first two are historical baseline diagnostics documented in [📓️native-library-recovery-audit.md](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/COMPOSABLE-STDIO-ARTIFACT-PACKAGES/📓️native-library-recovery-audit.md:13); the last is a separate focused test gate. None changes the 37 remaining package gates above.

No Cargo or Nx command was run. A terminal `0` here applies only to the named default check; it is not a test pass claim.
