# Flow Temporary Host Retirement Audit

## Scope

Read-only review of the settled four-file Flow repair for the retained `addWidget` route. No native build, formatter, source mutation, ticket mutation, or generated artifact was performed. This follows the first-cause record in `GEN/flow-recovery-6-worker-first-cause.txt` and `📓️flow-native-fixture-recovery.md`: a temporary `FlowHost` owned a `FlowFixture` whose `OrderedMap<WidgetLayout>` reached strict drop without retirement.

## Finding

The replacement removes the failing owner instead of bypassing its lifecycle. `child_add_widget_mutation` now retains only its parsed descriptor, selected id, one constructed `Widget`, one stack `WidgetLayout`, and one typed `InsertNode` value. It no longer constructs `FlowFixture`, `FlowHost`, DAG state, evaluation cache, or an `OrderedMap`; therefore it cannot traverse the observed `OrderedMap<WidgetLayout> -> FlowFixture -> FlowHost -> add_widget::child_add_widget_mutation` drop chain. It neither calls a cold drain in the interactive callback nor changes `OrderedMap` or `FlowHostRetirement` strict-drop behavior.

The temporary `SharedRegistry` reader is safe in this use: it borrows `OperatorInfo` only while `widget_from_descriptor_with_info` clones the needed port names into the new widget. Its normal reader drop only performs the registry's last-reader handoff; it does not own the retained Flow fixture domains.

## Compatibility Review

| Requirement | Static evidence | Result |
| --- | --- | --- |
| Smallest unused generated id, serial `>= 2` | `generated_widget_id` is the former `FlowHost::next_widget_id` body: same prefix table, numeric suffix parsing, `HashSet`, and increment loop. | Preserved. |
| Host serial used by clusters | The host delegates to `generated_widget_id` and still assigns its returned serial to `next_widget_serial`; cluster creation still increments that field for `cluster_<serial>`. The replaced temporary host was discarded before clustering, so the child route had no persisted serial side effect to preserve. | Preserved for the host; no child-route state is lost. |
| Explicit ids and duplicate rejection | The direct path uses `descriptor_explicit_id`, then rejects any equal child-node id before creating the mutation. | Preserved. |
| Neuron defaults and ports | `flow_host_with_session` used `flow_neuron_kind_infos_json`, which serializes the current `flow_extension_registry`. The direct path reads the same registry's `operator_info` and passes a borrow to the new shared constructor. Named and variadic default ports therefore derive from the same `OperatorInfo`; no metadata/default value is cloned into a temporary host. | Preserved. |
| Node conversion | `flow_content_node_from_working` is the canonical working-widget to Semio node bridge. It produces the variant params, slider label or kind label, and supplied position. | Preserved. |
| Child-only publication and undo | `FlowChildGroupWork` still rejects every lane except one content child group; existing tests require exact `[Child, Terminal]`, unchanged parent content, two inverse applications, and terminal-empty close. | Preserved. |

The previous host route also had no functional dependency on canvas options, host catalogue sections, evaluation baseline, or DAG rebuilding for this operation: those values could affect the transient DAG but not the newly persisted child node. The new path deliberately avoids them.

## Test Coverage Assessment

The repair adds a useful neutral-style direct law for the `inputNote` default and the `note_2`, `note_4` to `note_3` gap. The existing registered-route tests cover exact child/terminal publication, parent-coordinate preservation, undo restoration, and terminal-empty close. `git diff --check` was clean.

At the initial audit, the direct law covered only the `inputNote` default. The requested table-driven descriptor and collision coverage is now present; the follow-up records the exact scope and its remaining non-default-input boundary.

## Validation Status

Static source and diff review complete. Native acceptance is intentionally pending the coordinator's ordinary Flow retry; this audit did not run Cargo, Nx, or formatting.

## Follow-up: Descriptor and Child-Work Coverage (2026-09-09)

The expanded unit suite closes the descriptor-coverage gap identified above for the `AddWidget` route's supported default construction. `WidgetDescriptor` has exactly eight variants in the framework snapshot: `Neuron`, `InputSlider`, `InputNote`, `InputImage`, `OutputPreview`, `OutputAction`, `OutputExport`, and `Variable`. The table-driven child mutation law now constructs each one and compares the complete encoded Semio node after applying the mutation: generated id, kind, label, ordered params, and position.

It includes two independently shaped `OperatorInfo` values: named inputs map to `["a","b"]`, while variadic input/output minimums map to `["0","1"]` and `["0"]`. That directly exercises the borrowed registry-info path passed to `widget_from_descriptor_with_info`, rather than the former temporary-host map. The direct ID law retains the `note_2`/`note_4` gap and asserts generated `note_3`. The explicit-ID law asserts the shared helper rejects a duplicate child id before node construction.

The registered operation law still verifies the ownership-sensitive contract: each of two admissions yields exactly `[Child, Terminal]`, preserves the parent's content coordinate, publishes only the content child, and applies two undos back to the prior child snapshots. This is sufficient static coverage for the repair's prior gaps: all descriptor variants and both port-default forms, smallest available generated id, duplicate explicit id rejection, node payload/position conversion, and child-only ACK/terminal/undo behavior.

The descriptors in the table deliberately use route-reachable defaults (apart from the slider's required label). The public `AddWidget` command accepts only `kind`, optional `neuron_kind`, and coordinates; it does not expose custom slider bounds, note text, action/export strings, or variable fields. The table therefore does not independently exercise custom optional descriptor-field propagation, but that is outside this command's input surface and is not a compatibility hole in the temporary-host retirement. The framework constructor retains those branches unchanged.

No Rust test or native Flow route was run for this follow-up. Native acceptance remains contingent on the coordinator's pending ordinary Flow retries.
