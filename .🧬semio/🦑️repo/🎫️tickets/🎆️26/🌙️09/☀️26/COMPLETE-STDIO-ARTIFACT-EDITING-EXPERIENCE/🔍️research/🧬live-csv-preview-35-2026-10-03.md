# Live CSV Preview35

The registered native component-dev preview rebuilt successfully and served at localhost6212. The first page loaded without fresh warning/error logs, mounted Source/Details, and exposed Save File / Open File actions. Save File was clicked without a new console fault; browser download bytes have not been inspected, so this is UI dispatch evidence only.

Valid Source Apply at20:53:38UTC changed canonical note to `Source35Note`, returned Apply/Discard to disabled, and produced no fresh console warning/error. An older conflicted table draft remained visible until Escape correctly restored the newly published source value.

The initial ownerless demo session exposed two failures: quick edits across two fields caused the second command to refuse with `stdio.csv.table-conflict`, and collapsing Source lost an unsaved `x` draft. The media worker mounted an exact primary-session owner. After a fresh browser reload at20:56:36UTC, rapid name/note commits at20:58:00UTC published both `Owned35Name` and `Owned35Note`; both Details fields agreed and no fresh warning/error occurred. Source retention and cross-window acceptance are still underway.

A separate repeated Add row experiment at20:54:47UTC produced one row and refused the second stale command. Root is implementing retained button pending admission with disabled/aria-busy feedback, backed by three mounted red witnesses reproducing duplicate dispatch.

## Pending Button Admission

Retained buttons now publish disabled and aria-busy while their dispatch promise is pending. Repeated activation is refused locally before it can send a stale command; native success/refusal/transport rejection releases the control. Store/key/document replacement gets a new pending owner and late old completion cannot release the new owner. Three red mounted witnesses reproduced two dispatches instead of one. Fresh final suite95/95 passed with cache disabled (`retained-button-admission-green-3.log`), including the additional owner-replacement witness. Aggregate renderer typecheck passed (`retained-button-admission-typecheck-1.log`). Green2 was an unexpected stale Nx94-test cache hit and is not counted as validation of the new95th witness. Native preview acceptance still follows.

## Source Layout and Cross-Window Retry

The refreshed frontend requires newly added typed diagnostic labels; preview35 native output predates those settings, so Source Apply/Discard are absent. Source retention is deferred to a coherent native rebuild, not accepted from a mixed version page. DOM measurement additionally found the source draft column had width0 inside a220px control parent because its native stack did not grow; root added a neutral native red witness before changing the authoring wrapper.

The first cross-window attempt used a treeitem accessible name containing the input value; that name changes while typing. A later isolated Details edit using the freshly observed name successfully published `DebugDetails35` into Table and Details. Root is retrying rapid cross-window commits through the inspected stable input id. Temporary `[DEBUG]` draft typing/publication logs are active only for this diagnostic and will be removed.

The rapid cross-window retry at21:11:20UTC through the stable observed control id succeeded: Table name `Final35Name` and Details note `Final35Note` both published into both windows. Console tracing showed two typing admissions followed by two native publication waves, with no non-debug warning/error or owner replacement. The temporary root draft logs were then removed from source. The earlier accessible-name attempt is not counted as a successful or conclusive cross-window product test.

Repeated Add row through the refreshed ButtonView succeeded at21:12:13UTC: the table grew from2 to4 data rows, with no non-debug warning/error. The immediate snapshot preceded the pending attribute render, so pending ARIA/disabled behavior is supported by mounted tests rather than claimed from that snapshot.

Two further red mounted witnesses caught stale button bindings/disabled state between retained store publication and React paint. Button activation now reads and validates the current store record/key/type/disabled state before dispatch, preserving the pending owner guard. Final fresh suite97/97 passed (`retained-button-dispatch-green-1.log`); both red cases failed before the repair. Root temporary debug logs are absent from source.

The final aggregate renderer typecheck after fresh button dispatch guards passed with cache disabled (`retained-button-dispatch-typecheck-1.log`).
