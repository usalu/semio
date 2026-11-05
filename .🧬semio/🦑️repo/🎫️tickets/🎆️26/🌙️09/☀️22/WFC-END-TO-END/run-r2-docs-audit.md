# WFC Documentation Audit

Read-only review2026-11-04 of WFC README, browser documentation/cases and current source behavior. No source/doc edits outside this report and no verification queue.

## Required Corrections

1. Ticket wfc-browser.md19 still names old queued native sessions57596/61555. Those attempts terminated before compiler in earlier state; replace with current run references or point to active state/native report. It must not present dead queues as active.
2. wfc-browser.md8 describes second-run Abort only when nonterminal is observable. Current scenario also aborts Complete before Finalize, correctly; align the main Coverage paragraph with R2 correction51 so the same document does not describe two different behaviors.
3. wfc-browser.md43 says a larger prepared problem is needed to guarantee nonterminal control coverage. Finite size alone cannot guarantee browser scheduling; require an actual observed Running→Paused witness with bounded escalation, as controls audit explains. Describe larger setup as improving observability rather than a guarantee.
4. Update wfc-browser.md47 after actual execution with the real result; until then its pending-verification qualification remains correct. Do not replace it with success based on module imports.

## README And Cases Correspondence

README uses fresh dev-*-react-prototype targets with parallel1, correct five ports6041-6045 and English/German instructions. No obsolete warm serve launch command remains. It correctly distinguishes authored model/seed/pins from derived preview and identifies bitmap Pin Solution as durable output-pin operation. Empty example selection behavior matches current lookup repairs.

The keep description is valid for current pin-preview implementation; useful precise wording is to select the Output pane and pin a finished preview of the unchanged document. Current source refuses unsolved, contradictory and stale previews, so documentation should not imply that an arbitrary old result can be kept.

Grid3d now owns completed results in instance-local transient and renders unsolved when no current cache exists. README's preview/model distinction remains correct; it makes no claim that grid3d auto-solves at boot or persists a process cache. Existing historical audit descriptions of its former cold render path should remain historical evidence, not current usage instructions.

Browser case table uses current action spellings, proper author prefixes, bitmap keepWindow and distinct grid3d second colour. It intentionally does not exercise all author forms or guaranteed mid-search controls. Its solvePersistence=ephemeral describes solve caches while explicit bitmap keep creates authored pins; that distinction is already explained in prose. No case table correction found.

No runtime/browser pass is asserted by this audit.

## Bitmap Keep Confirmation Check

The ActionPane no-argument row path window-panes/🟦️.tsx74-75 directly invokes onExecute; submitStagedVerb merely clicks a staged execute control and does not confirm dialogs. Shell onActionStable232 forwards to regular input dispatch. No generic human destructive confirmation or Apply/Anwenden primary label was established in this path. action_destructive builder100-104 documents admission approval semantics; that metadata alone does not prove a human dialog.

Current scenario treats the dialog as optional: after10second wait it proceeds to ledger if none exists. Therefore no verified confirmation-label blocker was found, though documentation claiming a required localized confirmation is stronger than demonstrated source behavior. Actual browser evidence should determine whether a dialog is present.


## Removed Synchronous Marker/API Documentation Refresh

Read-only recursive current WFC Markdown/examples and source call search. No current WFC Markdown instructions reference removed SolveResult/Contradiction inferred markers or synchronous solve helpers. All ten host solve/solve_with_job/solve_with_clock wrappers have adjacent cfg(any(test,feature=testkit)); graph3d transient::testing module is gated likewise. Actual production callers identified earlier have been removed/migrated; no remaining executable production call found in this bounded scan.

One exact stale source API reference remains: graph3d editor preview module 🧊️3d/standards/1/any/editor/modes/edit/windows/preview/🦀️.rs5 names removed crate::inferences::solve_assignments in its module doc. It should describe current revision-bound retained transient projection. This is a documentation correction, not an executable unresolved symbol or native acceptance blocker. Fill payload docs saying matches solve_with_job refer to extant testkit oracle helper and are not production cold-fallback claims.

Historical ticket reports preserve earlier source states and failures; do not erase them or mistake their old marker mentions for current API examples. Current provenance408worked/366inputs/105JSON/noerrors/max194 was supplied by parent; this auditor did not recapture or mutate immutable evidence. No source edits/queues.
