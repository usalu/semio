# Computed Plugin Reader Final Review — 2026-09-09

## Boundary

Read-only recheck of the finite repair coordinates reported in [`📓️computed-plugin-reader-audit-2026-09-09.md`](./📓️computed-plugin-reader-audit-2026-09-09.md), plus the Process changes recorded in [`📓️plugin-followup-fixture-fixes-2026-09-09.md`](./📓️plugin-followup-fixture-fixes-2026-09-09.md). No new reader discovery or runtime suite was run.

## Process retained-route correction

The Process fixture and schema both declare 31 routes. The live `✏️editor/🦀️.rs` command match has 31 distinct rows: 25 bounded and 6 resumable. The two proof catalogs contain the same 31 distinct routes. `setActiveUtility` is absent from the command routes; the remaining `set_active_utility_effect` is a host-effect helper, so its omission from the retained-route contract is correct.

The source oracle retains all its other requirements: closed AJV schema objects, fixed limits, exact route partition, publication lanes, fixed document grant checks, empty scan-then-monolith list, and the prohibition on batch-only classification. Its new cancellation assertion matches the two actual retained-job implementations:

- `Process3dConfigStorePreparation::cancel` at `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:816`.
- `Process3dArtifactPreparation::cancel` at the same file, line 1160.

Both set their job-local `cancelled` state, and both `advance` implementations block after cancellation. No unrelated contract requirement was weakened. A nearby source comment still says 33/32 tool IDs at lines 1773-1776; it is documentation-only and conflicts with the live 31-route implementation.

## Computed reader recheck

All previously reported source coordinates now use their canonical owner-relative expression:

- Equation graph and Trinity Rewriting window tests read their direct config and mutation schemas.
- Raster I/O reads `../../🧫️fixtures/${name}`.
- Note action-cohort resolves from the Note owner and reads `🧫️fixtures/🧪️action-cohort/🔣️.json`.
- CAD reads through `source.directory`; Print reads `../../🧫️fixtures/🧫️command-boundaries.json`.
- Each of the 14 Energy BESTEST tests reads `../../../../🖼️assets/🏛️bestest-*/🗣️.dsl.semio`.
- Mesh, Dev config, AgentBridge, Store codec-send, Store backbone-detach, Directory runtime-identity, Plugin codec-caller-source, and Local-interaction mutation-leaf use their repaired owner-relative paths. The mutation leaf has a trailing-slash URL base.

A bounded disk check verified 24 direct canonical targets spanning the Equation, Rewriting, Raster, Note, Print, CAD, Mesh, Dev, AgentBridge, Store, Directory, Plugin, and Local-interaction repairs. All 24 exist. A separate exact check verified all 14 Energy reader expressions and targets. No uncorrected coordinate remains in this finite repair set.

## Verification boundary

This review verified source expressions and physical targets only. Runtime test results are intentionally not claimed here.
