# R17 — stdio PDF De-Async Repair

## Scope

Exclusive source ownership was limited to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/**`. No stdio registry/Cargo, non-PDF artifact, framework/plugin/derive, renderer, UI, codemod, or journal file was edited in this lane. Shared upstream repairs and the clean `ComponentTree` export were supplied by the parallel renderer/plugin lane.

## Structured diagnostic progression

The required command was run with JSON diagnostics and every error was classified by its primary span:

`cargo check -p semio-s-plugin-stdio --lib --message-format=json`

| Gate | Total | PDF | Non-PDF | PDF attribution |
| --- | ---: | ---: | ---: | --- |
| Iteration 2, first reachable stdio baseline | 3,243 | 188 | 3,055 | diff 31; mutations 30; editors 40; editor windows 20; viewers 20; viewer windows 20; IO 9; inferences 8; schema-other 7; examples 2; root declaration 1 |
| Iteration 3 | 3,217 | 162 | 3,055 | diff 31; mutations 30; editors 40; editor windows 20; viewers 20; viewer windows 20; root declaration 1 |
| Iteration 4 | 2,447 | **0** | 2,447 | no PDF primary diagnostics |

The exact monotonic PDF result is **188 → 162 → 0**. The first 26-error reduction came from the pure parser/IO, inference, schema-helper, example, and schema-root repairs. The final 162-error reduction came from synchronous diff/mutation recursion, surface-builder boundaries, semantic renderer migration, window definitions, and the declaration builder.

Logs:

- `📝️r17-stdio-pdf-baseline.txt`
- `📝️r17-stdio-pdf-iteration-1.txt` and `-errors.txt`
- `📝️r17-stdio-pdf-iteration-2.txt` and `-errors.txt`
- `📝️r17-stdio-pdf-iteration-3.txt` and `-errors.txt`
- `📝️r17-stdio-pdf-iteration-4.txt` and `-errors.txt`

## Repairs

- De-asynced in-memory PDF 1.7 parsing/writing, diff, mutation, and inference helpers deliberately; composition/analyzer and external codec boundaries remain async and are awaited or bridged only where a synchronous callback requires it.
- Removed obsolete `Box::pin` recursion scaffolding after the recursive helpers became synchronous, plus stale `resolve_ready` calls around ordinary `usize`, `ByteReader`, `Result`, and error values.
- Resolved every async stage of the PDF declaration and editor/viewer manifest builders at their required synchronous registration boundary.
- Migrated all 20 PDF window render helpers from legacy `UiNode` to semantic-contract `BuiltNode`, and all 20 editor/viewer trait implementations to `ComponentTree` through the clean SDK conversion seam. No compatibility converter was introduced.
- Preserved async boundaries for artifact builders, composition, analysis, validation, and codec calls. Compiler-identified cfg(test) call sites were updated with explicit awaits after the focused no-run exposed them.

## Focused test reachability

`cargo test -p semio-s-plugin-stdio --lib --no-run --message-format=json pdf` reached test compilation but exited 101 behind 4,436 non-PDF test diagnostics. Its first run also exposed 54 PDF cfg(test)-only stale async-boundary call sites; those compiler-selected sites were repaired. Root requested immediate handoff, so the cfg(test) repair set was not rerun and is not claimed green. Evidence is in `📝️r17-stdio-pdf-focused-test-no-run.txt` and `📝️r17-stdio-pdf-focused-test-no-run-errors.txt`.

Release and runtime-focused PDF tests are not reachable until the remaining crate-wide non-PDF compilation wall is cleared.

## Ratchets and audit

- `bun ./📜️script.ts verify dependencies` exited 0 at **238 → 238**; log: `📝️r17-stdio-pdf-dependency-ratchet.txt`.
- Final static corruption sweep: 0 stacked awaits, 0 duplicated async tokens, 0 nested `resolve_ready`, and 0 known await-token corruption.
- Final PDF legacy-renderer sweep: 0 `UiNode`, `DocumentWindowKit::render`, or `ui_text` residue.
- Scoped status at handoff: 67 PDF files changed across the pre-existing staged sweep plus this repair; unstaged PDF delta 61 files / 391 insertions / 347 deletions, staged PDF baseline 50 files / 736 insertions / 736 deletions.

## Blockers

The PDF lib lane itself has zero primary diagnostics. Full package tests and release validation remain blocked solely at the crate level by concurrent non-PDF stdio diagnostics; no non-PDF source was changed from this lane.
