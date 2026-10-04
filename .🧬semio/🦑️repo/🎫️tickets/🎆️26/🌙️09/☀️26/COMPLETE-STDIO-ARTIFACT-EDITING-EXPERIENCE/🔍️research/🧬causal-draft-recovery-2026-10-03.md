# Causal Draft Recovery

## Behavior

Blur/Enter commits retain one in-flight command and a queue for newer drafts. Native inputs carry an exact decimal-u64 publication revision. Only an applied operation completion receipt at that exact revision and normalized display value releases the next queued command. Hash-derived revisions are compared for equality only. A foreign publication, even with the same field value, preserves the draft and blocks the queue. Missing/refused/superseded completions preserve the draft. Escape discards queued drafts while retaining an already-applied command's pending publication barrier. Changing the authored semantic target resets the local draft.

Conflicts expose an accessible description and warning control. Its shared popover explains that the draft remains preserved and offers a discard action; Escape remains available. Both explicit English and German bundles include normal/beginner labels. The recovery surface is portaled through the existing shared Popover so fixed-height table cells do not clip its content.

## Validation

- Receipt integration59/59, then Escape publication-barrier red58pass/1fail, then59/59.
- Bilingual recovery red59pass/4fail for missing descriptions, then63/63 green in retained-input-recovery-green-2.log. Tests use the native receipt neutral fixture, actual retained UiNodeView, i18next per-locale instances, React Testing Library and dom-accessibility-api.
- Shared translation totality2/2 green in retained-input-recovery-i18n-2.log (Ajv, independent JSON Pointer and i18next).
- Aggregate typecheck1 correctly caught missing mandatory Button icon on the new discard action; icon fixed, full aggregate typecheck2 passed (`retained-input-recovery-typecheck-2.log`).
- Browser confirmation against current native publication contracts remains pending preview33. No complete editing-experience or save/reopen claim.

## Changed Owners

- Renderer Interpreter: queue acknowledgement and mounted recovery controls.
- Renderer editable-controls test/fixture: native receipt interleavings and bilingual recovery.
- Shared UI I18n contract and React locale bundles: two host labels.
- Shared UI translation-totality fixture/schema: explicit bilingual vectors.
- Related native receipt/producer files are owned by the Media execution lane and recorded in its report.

## Integration Notes

During concurrent flow registration changes, Nx first lacked semio-framework-os-flow, then temporarily accepted @semio-tech/framework-os-flow. Another agent changed the dependency to the temporary name before root's first patch, so root made no change at that stage. The temporary package/project manifests were subsequently removed, restoring Cargo-inferred semio-framework-os-flow. Root then changed flow-core's implicitDependencies back to semio-framework-os-flow and verified the canonical graph with nx show project, whose JSON reports the existing flow Rust package root. This one string is the only root-owned flow integration hunk.

A new real retained Table + Input DOM regression exposed Escape bubbling into the row handler: the parent focused its row, causing blur to submit the supposedly discarded draft. `retained-table-escape-red-2.log` failed because one command was emitted instead of zero. The Input now consumes Escape propagation after preventing its default and discarding the draft. `retained-table-escape-green-1.log` passed 26 selected actual Table/input laws (225 outside selection); the independent full editable-controls suite in `retained-table-escape-input-green-1.log` passed. Neutral draft/commit-count fixture and Testing Library accessibility/DOM were used; no simulated row handler substitutes for the real renderer.

Full post-Escape editable-controls verification: `retained-table-escape-input-green-1.log`, 63/63 tests passed, 0 skipped. The real Table integration is independently covered by the26 selected green laws above.

## Source Explicit Apply Ownership

The current browser exposed a Source Apply refusal with retained local text and working Discard, but its footer exposed `dispatch-failed`; the console explained a native retained-command rejection without a user-facing parse diagnostic. The previous TextEditor Apply implementation could also remain pending after a thrown dispatch, treated superseded as success, and treated any equal published text as acknowledgment.

Root is replacing this lifecycle with required exact native publication revisions and one Apply owner. Receipt equality acknowledges native-normalized source, pending edits preserve newer local text, foreign equal text remains a conflict, stale owner completion cannot touch another document, and thrown/refused/superseded/missing-receipt outcomes preserve a recoverable draft. The footer now uses the authored EN/DE failure label without the internal dispatch code, wraps its message, and associates it with the textarea via aria-describedby/aria-invalid. Native producer propagation is Media-owned, including all generic TextWindowKit consumers rather than only Stdio Source.

The old explicit-draft test was absent from the React runner's include list. It is now permanently registered alongside the new mounted DOM suite, with a launch seed entry. `source-explicit-causal-red-1.log` selected no tests and is not a behavioral red. After registration, `source-explicit-causal-red-2.log` reproduced the former equal-text acknowledgment condition and failed1/1 as expected. Pure neutral schema/causal laws pass14/14 in `source-explicit-causal-green-1.log`. Mounted DOM and aggregate typecheck are in progress; the first DOM attempt used an unavailable queryByRole wrapper method and is not a feature failure.

Cancellation remains on the native operation/Tasks route; this cut does not invent a local Cancel that only abandons the promise. Browser acceptance of these new Source changes requires a fresh native producer build. Current preview33 remains the earlier source lifecycle checkpoint.

Mounted Source controls now pass31/31 (14pure +17DOM) in `source-explicit-causal-dom-green-2.log`. Aggregate renderer typecheck passes in `source-explicit-causal-typecheck-2.log`. The first typecheck exposed the old third-party accessibility package's missing ESM type export; tests now use the typed Testing Library role-description query, preserving the independent accessibility oracle. Launch generation succeeded and generated `⚖️test-source-draft📝️react🟦️` is present.

Real preview33 keyboard checking also exposed Select All ignoring capital `A` with Command/Control; lower-case `a` correctly replaced the whole source with a subsequent typed key. Four neutral modifier/key cases drive the actual canvas host key handler: clean red `source-select-all-red-2.log` passed both lower-case cases and failed both capital cases, without unhandled errors. The key handler now normalizes the key's case; the full Source/echo gate is rerunning. The first red attempt additionally lacked the test environment ResizeObserver stub and is not the clean receipt.

Source explicit lifecycle, mounted accessibility controls, capital/lower-case Command/Control Select All, and echo packing pass40/40 in `source-explicit-causal-dom-green-3.log` (35explicit +5echo, no skipped tests). The latest aggregate renderer typecheck is green in `source-explicit-causal-typecheck-2.log`. Required native producer propagation and fresh browser source acceptance remain in progress.

Additional live preview33 failure: a dirty local Source draft `x` disappears after collapsing and reopening the Source section; canonical JSON replaces it. Media lane will implement lifecycle retention using the actual document owner, including pending completion while hidden and explicit retirement cleanup. A UiDocumentStore-only cache would be insufficient because primary BuiltNode window stores are reused across sessions. This is not fixed by the causal Apply state machine alone.
