# Canonical Document Admission Join

## Reviewed Source

Root read the publication owner's capacity report and the actual reconciliation ledger. The document arena currently has8root slots, while reconciliation has64slot/epoch credits, a32MiB aggregate byte limit,131,076aggregate items and8MiB per-surface limit. Reconciler current state already retains a persistent credit; each new job reserves a separate credit before candidate work, and publication splits that credit into exact owner shares. Shrinking a completed small surface returns unused aggregate allowance. Therefore the64slot number is not permission for64simultaneous8MiB allocations, but more than eight small live roots can legitimately fit.

## Approved Direction

Canonical current/candidate document storage must follow the existing exact reconciliation reservation/credit lifetime, not acquire an unrelated eight-slot root permit after payload construction. Root admission is part of pre-materialization admission. A candidate takes its already-admitted job epoch; a published root retains the corresponding persistent share. A replaced root with final readers still alive retains its original credit and cannot release/reuse that epoch until typed root retirement actually completes. Reader and transport aliases do not create another stored content tree.

Factor the neutral root-permit/storage authority as needed so the UI contract does not import the runtime. Use one canonical schema-owned capacity source for the joined reservation; do not merely change the old8constant to a larger literal or retain separate independently-refusing accounting for the same root. Existing standalone/transport admission must remain explicitly represented rather than silently become the live-surface limit. This is an ownership integration, not permission to raise the32KiB work opportunity,4KiB component grant,8MiB surface allowance or32MiB aggregate allowance.

The old record map must be removed as current-content authority. The document producer should alias that exact root, not rebuild/credited-clone every node into a second tree. Fixed root metadata, pages, current/candidate overlap and retired-reader roots require physical resident accounting. Capacity refusal preserves current/source/candidate owners before allocation. A final reader can only hand back a preowned root in constant structural work; it cannot cold-drop its payload or release quota early.

## Required Native Proof

The integration needs language-neutral cases and actual runtime tests for nine small concurrently live surfaces, the true existing slot/aggregate boundaries, replacement while old readers remain, cancellation before publication, exact generation reuse, and resumption after final-reader retirement. A rejected new admission must preserve the original published state and allocate no unadmitted payload. Retain the actual R30/R31zero-credit and final-byte32KiB REDs unchanged as runtime acceptance gates. Standalone assembly passes do not meet these integration gates.

