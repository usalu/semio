# Process3d forest archive load

The PID 34318 editor run in `🗑️generated/supervised-editor.log` starts at byte 66,216,420. The Bearbeiten executable recorded two distinct fixture stopping conditions at the same archive entry point.

| Test | Last confirmed boundary |
| --- | --- |
| `pane_smoke::bearbeiten_pane_renders_the_loaded_forest_in_en_and_de` | The bundled DSL and booted OPS parsed and printed to Pack and SPR. The admitted archive still reported `Running` at 125/125 after 1,000,000 turns. |
| `pane_smoke::bearbeiten_pane_loads_applies_undoes_and_closes` | `setActiveExample` was dispatched and its `LoadDocument` effect published. The archive helper then exhausted the fixture's 30-second host lifetime. |
| `context::conflict_history_edits_end_to_end` | A separate history archive reported `DecodeForward: Replay: truncated at offset 6` for a saved mutation payload. |

`setContributions` runs during fixture setup, which both forest tests had already passed. The command example creates an edit-free SPR through `empty_document_spr`, so it does not run a Process3d mutation payload through `DecodeForward`. The text example carries the boot artifact OPS and did not produce a decode fault in this run. Both paths pass the parent Pack and SPR to `document_load_archive` with an empty member roster. They hydrate the parent and derive its candidate member roster through archive maintenance. The retained JSON history-array fix is in the separate envelope ingress decoder. The existing fixes have no demonstrated causal link to the forest plateau.

## Archive acknowledgment

The fixture admits under `LAW_DOCUMENT_LOAD_OPERATION` and renews the original caller before each maintenance and poll turn. The non-merge poll reads status. Maintenance stage 24 drives SPR decode, parent hydration, the candidate handoff, member roster sealing and final input retirement. The store replacement stage independently initializes, validates, publishes and retires the candidate's displaced owners. During archive-owned roster admission the replacement intentionally waits at `AwaitingMembers` until the archive seals it. The archive only reports `Ready` after the replacement is acknowledged and archive inputs are terminal-empty. The testing law acknowledges the archive only after that terminal state. The status counter increments by retained decode progress as well as the parent and member handoffs, so equality in the counters does not identify a specific phase or prove publication.

The saved log has no phase or candidate diagnostics at that boundary, and the linked executable at the path emitted by Cargo is gone. The testing-only `load_document` helper now prints the resident archive phase, counters, terminal target and hydration owner when an error leaves the operation live. It also prints the candidate replacement phase, worker state, commit flag and pending outcome and resume flags, if present. Both existing forest tests use that helper. The repository diff whitespace check and Rustfmt parsing passed. No native result is available for the current source.

## Focused native replay

Run these from the Process3d Cargo workspace when the compiler lane is available. The shared prefix selects both forest tests. Each full label in the table can be used with `--exact` for an independent run. The library control uses an unseeded registered app to load the real example effect through member-less archive admission, independently of contribution seeding.

```sh
repo cargo --store=shared --configuration=prototype --parallelism=1 -- test -p semio-s-artifact-process-process3d-editor --test process3d_bearbeiten_pane_smoke pane_smoke::bearbeiten_pane_ -- --nocapture --test-threads=1
repo cargo --store=shared --configuration=prototype --parallelism=1 -- test -p semio-s-artifact-process-process3d-editor --lib unit_tests::contributions_and_examples::every_example_loads_through_the_member_less_archive_door -- --exact --nocapture
```

## Counter bound and retained grant

The archived `125/125` excludes the upstream history decode and pending parent hydration. The member-less archive starts with a one-unit gap. History discovery adds the same item count to completed and total. Pending hydration adds the owner's progress, whose reported total is at least its completed count. The gap can first close when the hydrated parent transfers to the store replacement. A later stall may therefore be in pre-roster replacement initialization, all-member settlement, or terminal archive cleanup. The bundled forest declares seven composed children. The next failed fixture turn also prints the current archive retirement or lifecycle demand, the original turn ceiling, and the remaining grant and fuel. [Replacement publication audit](process-editor-replacement-publication.md)
