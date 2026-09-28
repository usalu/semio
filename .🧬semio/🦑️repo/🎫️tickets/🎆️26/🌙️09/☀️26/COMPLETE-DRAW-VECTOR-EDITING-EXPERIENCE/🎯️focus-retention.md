# Inspector Focus Retention

The previous goal turn made progress: keyboard shortcut ownership and native action labels were implemented and browser-tested. The node-row native test and component materialization remain live and are being polled without duplicate launches.

## Reproduction

Enter on Convert to Curve successfully edited the path, but replacing that action with Straighten Segment removed the focused button and left focus on the document. Undo did the same. This interrupts a keyboard user's editing sequence despite correct command activation.

## Contract

The shared Tree records the focused control's id and its authored row ancestry. When DOM reconciliation removes that control, it first tries the same control, then the nearest surviving row with a usable control, then the tree itself. Focus remains untouched when the original control survives, when another control owns focus, after an intentional blur or outside pointer interaction, and when the tree is unmounted. Ancestry is limited to the last 64 rows. No artifact mutation or shared selection event is introduced by focus recovery.

The implementation uses the browser's MutationObserver and focus APIs behind a disposable Tree helper. Action rows publish their existing authored ancestry; no Draw-specific label matching or path-name parsing is used.

## Validation

English/German neutral cases render actual nested Tree rows through Testing Library. The initial run failed the two ancestor-recovery cases and empty-tree recovery, while outside focus and intentional blur already passed. The first implementation passed all 53 component tests, but browser verification still lost focus.

Temporary `[DEBUG]` diagnostics reproduced the missing sequence: the browser emits focusout with no next control while the old button is still connected, immediately before removing it. Eager blur cleanup discarded its owner before the MutationObserver could recover focus. The test DOM did not emit that removal blur. The neutral ancestor tests now explicitly reproduce this event before replacement; intentional standalone blur is allowed to settle separately. Browser trace is saved under `🗑️generated/tree-focus-browser-trace.json`.

The empty-tree fallback also needs to retain ordinary keyboard ownership. A new language-neutral Delete-key case for role=tree failed, and the shared shortcut predicate now includes tree roots. All 22 OS command shortcut tests pass after that change (55.7 seconds through Nx).

The browser-equivalent removal-blur regression failed both locale cases against eager cleanup. Cleanup is now deferred by one microtask when no next control is named: a still-connected old control releases ownership; a removed one remains eligible for recovery. A real focus transition continues to release immediately. The full Tree component suite then passed all 53 tests (41.3 seconds through Nx).

In the actual Draw browser, Enter on Convert to Curve produced the two control handles and returned focus to the named Anchor 2 disclosure. Command-Z with Straighten Segment focused restored the original line, removed the handles, restored Convert to Curve, and again returned focus to Anchor 2. The `[DEBUG]` console sequence records removal followed by ancestor focus in the same update. The browser reported no console errors. Temporary source diagnostics have been removed. Evidence: `🗑️generated/tree-focus-recovery-trace.json` and `🗑️generated/node-focus-verified.png`.

The current goal turn is progress, with native/build validation still waiting on live handles. It does not complete the full editing goal or prove native-renderer focus parity.

## Continuation

Final polling confirmed native node-row test 73342 and describe/materialize 79298 are still live. Keep those handles. Preview 76268 serves port 6065. The old browser tab was unavailable; new tab 5 was created and marked for handoff. It has Orange Wedge restored, Layer and Arrange folded, Anchor 2 expanded and focused. The loaded plugin still predates the pending stroke/node/gradient inspector rebuild. The host React keyboard/focus changes are verified independently of that component build. After the existing jobs finish, run the changed properties corpus including the unrun gradient-row case, rebuild the newer Draw sources, explicitly activate, and verify the inspector layout and stroke choices.

## Further Work

Persistent node selection still needs a separate framework interaction domain and topology-aware reference handling. The current drag session stores only one transient node hit; its release clears that point. It must not become a private duplicated selection authority. Multiple-node drag, keyboard nudge/delete and the remaining geometry workflows remain required by the full goal.
