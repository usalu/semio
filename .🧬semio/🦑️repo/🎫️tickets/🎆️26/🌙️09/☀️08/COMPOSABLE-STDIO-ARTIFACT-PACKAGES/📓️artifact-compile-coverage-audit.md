# Artifact Compile Coverage Audit

## Snapshot

Captured at `2026-09-09T14:02:59+02:00`. This read-only reconciliation joins the current 99-owner declaration inventory with recovery plans, terminal TSVs, and durable reports. It did not invoke Cargo, Nx, tests, or a native build.

The inventory partitions exactly into 36 stdio leaves, 15 Norm leaves, 7 framework artifacts, 24 single-artifact plugin leaves, and 17 multi-artifact plugin leaves. The full per-artifact matrix is `🗑️generated/artifact-compile-coverage-matrix.json`.

## Coverage By Inventory Class

| Inventory class | Count | Compile coverage found | Current acceptance interpretation |
| --- | ---: | --- | --- |
| Stdio | 36 | Earlier all-36 direct leaf sweep; replacement all-36 target | The replacement stopped with `ENOSPC` while creating AVI's fingerprint, before any package result. The earlier sweep is historical only; all 36 remain in the owned replacement aggregate. |
| Norm | 15 | `@semio-tech/norm-plugin:test --lib` completed 16/16 | The parent manifest directly names every one of the 15 leaves, so this is transitive compile coverage. It is not fifteen separate current default-library receipts. The separately failing surface route does not invalidate the completed parent-library receipt. |
| Framework | 7 | Durable per-artifact native receipts | The result ledger records Run, Workflow, Playbook, Flow, Infinite DAG, Space, and Collection as accepted, totaling 171 native tests. The earlier seven-project batch failure is historical; the later individual receipts are the relevant evidence. |
| Multi-artifact | 17 | Owned 15-leaf default/component matrix plus the delegated GIS and Block queues | Each of the 17 is planned or has a partial attempt. The current set is incomplete: the 15-leaf replacement matrix has shared-source interruptions, and the GIS two-project attempt ended in shared replication compilation. No complete current 17-leaf acceptance may be inferred. |
| Single-artifact | 24 | Nineteen current default rows plus five separate receipts | The active leaf plan now has 19 terminal rows. Writer has a current focused default receipt outside that plan. Animate, Demonstrator Playground, Sequence, and Architect have historical accepted leaf receipts outside it. |

No artifact is missing from both a planned gate and accepted/historical coverage. This is a coverage result only; it does not make the aggregate native suite accepted.

## Active 39-Row Default Matrix

The current plan contains 19 leaf default checks and 20 parent composition checks.

| Leaf state | Count | Packages |
| --- | ---: | --- |
| Accepted terminal `0` | 3 | `semio-s-artifact-vcs-vcs`, `semio-s-artifact-process-process3d`, `semio-s-artifact-sourcing-curation` |
| Terminal retry required | 16 | `semio-s-artifact-mathematical-equation`, `-flow-flow`, `-shooting-shooting`, `-lowpoly-lowpoly`, `-reasoning-wires`, `-forms-forms`, `-layout-layout`, `-cad-cad`, `-playbook-playbook`, `-imperative-procedure`, `-remodel-remodeling`, `-energy-model`, `-dag-dag`, `-draw-drawing`, `-raster-raster`, `-note-note` |
| Parent terminal rows | 0 | All 20 parent rows have begun but none has an entry in `native-library-recovery.tsv`. |

The 16 nonzero leaf rows are failed-only retry candidates after the active parent sequence completes. Their outputs are not current package defects: several stopped in graph or shared-library prerequisites. The report intentionally does not reclassify them as individual source failures.

The four historical single-leaf receipts are kept separate from current plan status. Writer's focused `0` is current evidence in `owned-focused-default-retries-current.tsv`; Animate, Demonstrator Playground, Sequence, and Architect are accepted historical ordinary defaults. Optional Animate `preview-window` and Lowpoly `cad-fixtures` results remain feature evidence only and do not replace default checks.

## Evidence Boundaries

- `native-library-recovery-plan.json` and `native-library-recovery.tsv` are authoritative for the active 39-row default matrix.
- `multi-artifact-execution.md` records the prior 36/36 stdio sweep and the failed `ENOSPC` replacement attempt; static package contracts and normal dependency trees are deliberately not counted as compilation.
- `norm-plugin-lib-test-4.txt` is terminal evidence for the parent test, while `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml` lists all fifteen leaves directly.
- `results.md` provides the durable seven-framework-artifact receipt summary. The old failed seven-project batch cannot supersede later successful individual routes.
- The multi-artifact, GIS, Block, and Norm surface queues remain source or runtime work owned by their existing executors. Their incomplete receipts do not create a missing inventory gate.

No source changes, cache changes, or duplicate native work are recommended by this audit.
