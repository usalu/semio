# Live CSV Preview 30 Acceptance

The current Stdio CSV React preview reached localhost:6212 and mounted the demo in the in-app Browser on 2026-10-03. Browser console error reads were empty during the initial checks. This is live runtime evidence, not complete editor acceptance.

One settled cell change to `Edit 20` was committed; Undo restored `Edit 17`, and Redo restored `Edit 20`. Each command updated asynchronously, so the immediate click snapshot still showed its previous persisted state. The subsequent settled snapshots showed the correct values and enabled/disabled history actions.

A rapid repeated input witness filled the same cell with `Edit 02` through `Edit 19`, pressing Tab after each fill. The immediate draft checks passed through 18, then 19 failed. After settling, the source value was `Edit 17` and only ten committed changes were present. This is a failed rapid-input acceptance check. DOM inspection later established that the active controls are retained `InputView` inputs, not the separate `TableEditableTextCell` textareas. `InputView.useCommitDraft` resets local text whenever either published value or serialized command bindings change; CSV carries a changing document revision inside those bindings. It is not a twenty-edit pass.

Two visible rendering gaps remain: the Headers grid reports `Rows 0–0 of 2` with no header fields, while Details shows the existing Records array with only Create valid item and no children. Clicking Source did not reveal a source editor. A Terra read-only agent is auditing the list/viewport initialization routes while root handles cell draft reconciliation.

Preview 30 supersedes earlier compile and publication blocks. The generated browser publication was recreated from its current producer after the old generated publication was preserved under the ticket. No file import/export, EN/DE switch, invalid Apply/Discard, keyboard focus, cancellation, or family-wide browser acceptance has passed yet.

## Read-Only Live-Editor Window Audit

This source audit was performed against the current checkout on 2026-10-03. It made no production or test changes and did not run a build. It supplements the browser observations above; it does not reinterpret an immediate asynchronous click snapshot as a failed undo, redo, or disclosure operation.

### P1: The Headers table has two independent reporters for one body-wide window ledger

The CSV editor renders its structural header grid and its data grid as two sibling tables in the same main body:

- `render_structural_table` emits the header table under `stdio-table-headers`, then appends the caller's data table (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs`, `render_structural_table`, lines 1334–1388).
- CSV creates that data table with `TableWindowKit::render_indexed_matrix_with_id` and passes the same `TreeWindows::for_body` to both (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/.../🪟️main/🦀️.rs`, lines 41–102).
- Every rendered `TableView` mounts its own `useTreeWindowObserver` and reports only containers under that table's local `rootRef` (`🧰️framework/.../Interpreter/🟦️.tsx`, lines 2739–2772 and 2860–2898).

The host API has the opposite contract: `TreeWindowSchedulerV1.reportWindows` explicitly treats every call as the **whole body**, clears `state.windows`, and replaces it with the report passed in (`🧰️framework/.../ShellHelpers/🟦️.tsx`, lines 2766–2791). Thus the headers reporter and body reporter erase each other's window entries. This is a confirmed protocol violation independent of the exact frame which produced `Rows 0–0 of 2`.

The visible header state is consistent with the guest receiving `rows: 0`: `TableView` retains the advertised total and renders no materialized rows, which produces the reported range. `TreeWindows::sliced_capped` also honors a received zero-row request. The audit does not claim which of the two local reporters won a particular browser frame without a trace; it establishes that the current code allows the loss.

**Required repair:** make reporting body-scoped before it reaches the scheduler. Each observer needs a stable source registration and must publish its current local report to a body coordinator. The coordinator must merge all mounted sources' reports and send one complete body report; source cleanup must remove only that source's entries. It must preserve the maximum measured viewport rows. Do not change `reportWindows` into an accidental incremental API: its replacement semantics correctly prune containers that really leave the body, once the caller supplies the complete body picture.

**Focused regression:** mount a structural table with a two-row header table and a windowed data table under one body context. After both observer frames settle, assert the host request retains both `stdio-table-headers` and the data-table key, and assert both header inputs are present. Unmount one reporter and assert only its key is removed. Also trace `{source, nodeKey, offset, rows, viewportRows}` during the browser check; an on-screen header must never settle at `rows: 0` merely because the sibling table reported last.

### P1: Details emits a nested `treeSection` the renderer deliberately treats as malformed

The missing existing Records children have a separate, concrete cause. `detail_node` adds the collection controls and then appends `tree_window_indexed_section(..., "Items", false, ...)` directly beneath a `tree_item` (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🪟️details/🦀️.rs`, lines 1813–1823).

The interpreter only collects direct `treeItem` children as nested tree data; it classifies every other child of a tree item as an inline control (`Interpreter/🟦️.tsx`, `collectTreeItems` and `collectTreeItemControls`, lines 1673–1694). A `treeSection` in that position consequently reaches the explicit malformed-document fallback:

```tsx
case "treeSection":
case "treeItem":
  return <ContainerView ... />;
```

(`Interpreter/🟦️.tsx`, lines 3001–3005). The fallback does not build a Tree section, does not expose an Items disclosure, and does not connect that window's open state. This explains the observed Records row having neither `aria-expanded` nor an Items child section. The `Create valid item` control is still rendered independently, so it can appear alone.

**Required repair:** do not place a `treeSection` under a `treeItem`. Keep sections direct children of the tree root. Represent a collection's entries with a windowed `treeItem` descendant (using `tree_window_indexed_item`, or a native indexed counterpart that preserves the parent window path) so Records becomes an actual expandable row and its entries remain in the tree walk. The new helper must retain the current bounded materialization and the `records-children` window address. Do not make a browser locator click the Records label to compensate for an invalid tree shape.

**Focused regression:** render a typed `records` array with at least two existing entries and a creation template. Assert that the Records row is a `treeitem` with `aria-expanded`, contains a discoverable Items/entries disclosure, and materializes the first two children after opening and the host refresh settles. Preserve the separate Create-valid-item control. Run the equivalent DOM assertion against the native browser surface, not only a built-node census.

### Source Disclosure and the Misleading Reorder Name

Source is deliberately a default-closed top-level windowed section (`render_snapshot_details_provider`, lines 1883–1892). The browser locator `getByRole("button", { name: "Source Reorder", exact: true })` identifies its `CollapsibleTrigger`, not a separate button nested in it: the trigger gains the `Reorder` word from the descendant drag-handle hint. It owns `aria-expanded` and a click is supposed to toggle it (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/↕️Collapsible/🟦️.tsx`, lines 78–153).

The locator is therefore the only currently discoverable Source disclosure control. Acceptance must first assert that its `aria-expanded` becomes true, then wait for the partial body refresh before looking for the source input. If it remains false, instrument `setOpen`; if it becomes true but no source field arrives, inspect the returned `tree_windows` entry for `stdio.snapshot-details.source` and its `open`/`rows` values. A still-closed immediate DOM is not enough evidence of failure.

The Reorder suffix is itself a UI and accessibility defect. The generic interpreter sets `sortableSections={sections.length > 1}` (`Interpreter/🟦️.tsx`, lines 2436–2445), which makes ordinary sibling sections reorderable even when their authored tree declares no section reorder operation. `TreeDataSectionView` then adds the drag handle. Derive section reordering only from an authored reorder capability/handler; Source and Details must not gain a Reorder affordance merely because there are two sections.

### Input Draft Identity Has No Existing Wire Field

The rapid-cell result is a real draft-reconciliation failure, but the current node contract has no safe generic identity field from which to repair it:

- `InputView` currently scopes drafts with `JSON.stringify(record.bindings)` (`Interpreter/🟦️.tsx`, lines 1371–1415).
- `UiNodeRecord` has an id, sibling-local key, component, and bindings; `InputProps` has no target or draft identity (`🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`, lines 197–226 and 786–790).
- `ActionBinding` contains only `trigger`, `action`, untyped `args`, and `capability`. The argument-definition distinction between document/target revision and an address exists in the guest window definition but is not transported with the binding.
- CSV's set-cell binding intentionally contains `{ row, column, revision }` (`window_kit_revisioned_cell_arguments`, contract lines 1298–1308). Its revision is a changing concurrency guard, while row and column identify the target. The renderer cannot know that from arbitrary `UiValue` arguments. `capability` is authorization data, not target identity.

The existing selection-draft regression already proves that a record key alone is insufficient: the same `position` node receives a new layer argument set and its stale draft must be discarded (`🧰️framework/.../🧪️tests/♿️editable-controls/🟦️.tsx`, lines 38–51).

**Required repair:** add an explicit schema-first opaque `InputProps.draftTarget` supplied by the producer. It must change when the semantic command target changes and remain stable across a revised acknowledgement for that same target. CSV should emit a target based on its stable action/address (`set-cell`, row, column), while continuing to send the current revision in binding arguments for concurrency validation. Do not strip a key named `revision` from arbitrary bindings; that would make a renderer guess action semantics it has not received. The reconciler still needs its own queued local-submit versus published-acknowledgement law; `draftTarget` solves identity, not acknowledgement ordering.

### Acceptance Boundary

The settled single edit / Undo / Redo witness remains valid evidence for that sequence. The earlier rapid sequence remains a failure witness for the then-current reconciler, not evidence that an immediate asynchronous click snapshot fails. Re-run rapid typing only after the explicit identity and acknowledgement tests land, and record the settled committed count and final source value.

## Subsequent Browser Checks

The language selector switched English to German, visibly updating table, row/column controls, Details labels and shell settings; switching back to English also completed with no browser console errors in the sampled reads. This supersedes the earlier untested language entry.

Twenty consecutive edits `Confirmed 01` through `Confirmed 20` each completed when the check awaited the corresponding persisted Remove row label. This proves twenty acknowledged edits. A separate rapid twenty-fill/Tab sequence still ended at `Rapid 17` and added only five history entries, so rapid-entry acceptance remains failed.

The separately rendered TableEditableTextCell also had a real missing-acknowledgement/duplicate-Enter-and-blur defect. Its tests were absent from the registered runner. Root registered that suite, observed the duplicate-submit red test, and repaired pending acknowledgement reconciliation. Four focused tests now pass in `table-edit-ack-green-2.log`, including retained later draft, external conflict, Enter/blur deduplication and explicit retry. This is a separate surface path; it does not repair the active retained CSV InputView.

The Terra audit confirmed there is currently no authored input target identity distinct from volatile binding arguments. Inferring target identity by dropping arbitrary revision-named arguments is unsound. A schema-owned explicit target identity and acknowledgement-aware input lifecycle is required. The audit also clarified that Source Reorder identifies the disclosure button itself, so its failure to expand by pointer and Enter remains a live issue requiring host tree-window tracing.

## Mounted Browser Repairs

The body report coordinator now merges all mounted observer reports for the same host body callback and retires only the unmounted source. After browser reload, Headers contains both name/note text fields and reports Rows1–2of2; this is a fresh runtime pass for the formerly blank header grid. Its initial renderer gate passed14/14 in `body-window-reports-green-1.log`.

The Source disclosure failure also had a concrete cause: ShellHost's stable window-body context permanently exposed `openStates: {}`. The shared `createTreeWindowContextV1` now reads the host's current disclosure map through a getter, and both window and panel channels use it. Its red gate failed on the absent function and green gate passed15/15 in `body-window-context-green-1.log`. The live browser now expands Source and exposes its JSON draft, Apply and Discard controls. Arbitrary sibling-count-based section reordering was removed from the generic interpreted tree, so Source and Details have their proper accessible names.

Invalid `{ invalid` source text remained visible after Apply, with the alert `The draft could not be applied: dispatch-failed`. Discard restored canonical JSON and disabled Apply/Discard. This passes preservation/discard behavior, but the generic refusal message still needs a more helpful validation explanation. Browser console error samples remained empty.

The retained input queue has a red witness that previously dispatched two commands while the first was pending. Its sequential queue uses the current store binding for each subsequent command, preserves later drafts across earlier acknowledgements, and resets when the authored target changes. The original renderer gate passed50/50 in `retained-input-queue-green-1.log`; the later publication gate below supersedes that checkpoint.

## Preview31 Integration and Publication Ordering

Preview31 compiled and served native draftTarget and nested Details collection controls. The first boot refused its demo because the retained TypeScript typed normalizer did not recognize draftTarget. The native owner repaired the actual whitelist and added a shared native-wire normalization test; the framework gate passed306/306, then the browser loaded both header inputs and all nested record/field controls.

A rapid20 run at17:45:50Z exposed a second ordering issue: native command settlement precedes retained UI publication. Only Queued01 persisted while the preserved draft showed Queued20; the next command was refused with `stdio.csv.table-conflict`. The earlier unit law only exercised publication before settlement.

Root added both settlement/publication orders, typed refused/superseded outcomes, foreign guard changes with unchanged visible text, and normalized numeric acknowledgements. The new red witnesses failed on the intended conditions. Current `retained-input-publication-green-1.log` passes55/55. The queue now waits for the submitted normalized value to publish as well as the command to settle; foreign guard changes preserve a conflicted draft. ShellHost preserves InputOutcome through the semantic intent channel. Broader continuous/table/window laws passed30/30 with157outside selection in `live-editor-input-window-laws-current-3.log` after preserving synchronous test sinks.

The rapid20 retry at17:52:13Z settled Ordered20 in both the table and independently rendered Details, with20 Set field history entries and no fresh warning/error logs. Eleven consecutive undos published Ordered19 through Ordered09 correctly before a concurrent HMR reload interrupted the run and reloaded the demo. Full20 undo/redo acceptance is still open.

Preview32 uses the existing registered serve target with `SEMIO_VITE_HMR=0` and Nx dependency exclusion. It reuses preview31's already built native activation so other agents can continue editing without replacing the running acceptance document. It is a stable serve of that checkpoint, not a new native compilation. Receipt: `🗑️generated/stdio-csv-preview-current-32-stable-serve.log`.

The Terra audit identified a remaining causal ambiguity: another commit can publish the same value while a local edit awaits its own publication. Current value matching cannot prove that publication belongs to the local command. Multi-user same-value/structural-race acceptance remains open; do not infer it from the single-user rapid20 pass. Current aggregate renderer typecheck also fails on four concurrent World3dHost component-test call signatures; no modified input file is diagnosed. Receipt: `🗑️generated/retained-input-publication-typecheck-1.log`.

## Stable Preview32 Rapid Edit History Acceptance

The stable serve loaded the native preview31 checkpoint with live reload disabled. At17:58:11Z, root submitted20 fill/Tab commits without waiting for individual publications. Stable20 then appeared in both the table and Details. The history panel showed20 pending document edits.

All20 Undo actions were exercised with an observed persisted row-action label after every action: Stable19,18,…,01, then the original alpha. All20 Redo actions then published Stable01,02,…,20 in order. Each action was driven through the visible History control. Browser warning/error entries filtered from the run start were empty. This completes the single-user rapid edit/undo/redo witness; it does not cover import/export, other format families, or the separately audited concurrent-commit ambiguity.

## Confirmed First Action Obstruction

Stable preview 32 paints the editor, but the Add row button at x7.375/y76.984375, 78.359375×44 is covered at its center by the floating Actions pane toggle. DOM `elementsFromPoint` returns `window-pane-chrome-toggle`, then `window-chrome-chip-cap`, before the underlying action label. Its `window-dead-line-scroll` ancestor has zero top padding and scrollTop zero. CSS scroll-padding alone does not move ordinary content below chrome. The whole window scroll surface needs a real measured content inset, preserving an explicitly edgeless body and the user's scroll position. A narrow component regression is being prepared before implementation.

## Measured Content Clearance Checkpoint

The shared ChromeAwareWindowScrollSurface now uses a measured actual top inset, without changing scrollTop. Independent Terra review found nested edgeless children and nested windows needed explicit ownership boundaries; those were repaired and added to the language-neutral fixture. Full registered Window component suite passes 12/12 (`window-content-clearance-window-suite-2.log`), with ten geometry cases covering short/scrolled/partially-cleared content, offscreen/no-overlay/edgeless hosts, and nested surfaces. Browser DOM now finds Add row itself at its center, at y105.984375, with a29px surface inset; screenshot confirms the action is below the floating Actions row.

The preview reload also exposed a separate archive-transfer failure: document archive load fails with no answer to sequence11/operation9. Preview32 still uses preview31's native binary while current frontend transfer code has changed in the shared checkout. This invalidates post-reload document-edit acceptance until a fresh coherent native activation is built; it does not invalidate the earlier stable20/20/20 run. No save/reopen acceptance is claimed.

## Nested Scroll Position Regression

A further neutral vector reproduced a47px unwanted scroll jump in a Scrollable nested under the already-cleared window surface. Its old useWindowContentDeadLineScroll still forced scrollTop to the overlay clearance. Both Scrollable and ChromeAwareWindowScrollSurface now use the same measured inset hook; the old forced-scroll hook was removed, including its public export, rather than retained as an alternate behavior. The fixture explicitly places the inner scroller below the outer chrome and expects zero extra inset and unchanged scrollTop. Existing tests asserting a forced initial jump were updated to preserve the first content. Fresh focused tests are running; browser confirmation remains pending preview33.

Unified scroll validation passed4/4 across the Window DOM and existing host tests (586 unrelated tests excluded), in window-content-clearance-nested-scroll-green-2.log. The first broad quick attempt exhausted its15second test budget during module startup; the rerun selected only the two source files, max2workers, under the registered long level. The aggregate renderer typecheck also passed in window-content-clearance-unified-typecheck-1.log. The language-neutral clearance fixture now contains11cases.

## Current Preview 33 and Table Escape Integration

Fresh native preview33 completed staging and served at localhost:6212, serve pid1796. At 2026-10-03T19:01:46Z reload restored the full CSV grid and Details tree. A real cell edit to `Receipt33` published in both surfaces with no new warnings/errors. The next Demo reload returned the example value `alpha`; this proves archive load and admission, not persistence of that unsaved edit.

Safe temporary metadata tracing on the second reload recorded LoadDocumentArchive seq9/operation9 returning Done(inReplyTo9), PollDocumentArchiveLoad seq11 through23 returning matching DocumentArchiveLoad frames, and AcknowledgeDocumentArchiveLoad seq24 returning Done(inReplyTo24). No payload was logged. Ownership worker removes this instrumentation after capture.

On the same coherent activation, twenty rapid edits `Current01` through `Current20` all published. Twenty Undo gestures produced Current19 through Current01 then alpha; twenty Redo gestures restored Current01 through Current20 in exact order. Fresh warning/error console entries were empty for this run. Actual file download/import acceptance is next.

A new real retained Table + Input DOM regression exposed Escape bubbling into the row handler: the parent focused its row, causing blur to submit the supposedly discarded draft. `retained-table-escape-red-2.log` failed because one command was emitted instead of zero. The Input now consumes Escape propagation after preventing its default and discarding the draft. `retained-table-escape-green-1.log` passed 26 selected actual Table/input laws (225 outside selection); the independent full editable-controls suite in `retained-table-escape-input-green-1.log` passed. Neutral draft/commit-count fixture and Testing Library accessibility/DOM were used; no simulated row handler substitutes for the real renderer.
