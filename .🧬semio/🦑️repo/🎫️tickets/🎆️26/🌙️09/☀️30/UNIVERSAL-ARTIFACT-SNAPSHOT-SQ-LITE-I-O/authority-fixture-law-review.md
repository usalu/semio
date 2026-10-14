# Original Authority Fixture Law Review

Read-only authored-test review, 2026-10-13; no builds/tests. Files under `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/⚠️refusal/💰️limits/🧪️tests`, fixture `🧫️fixtures/🛂️authority.json`.

## Concrete Retirement Case Defect

Native `sqlite_snapshot_original_fixed_limits_receiving_scope_preserves_nested_and_stage_authority` creates `recipient` but never installs it on root control. Its earlier `with_workspace_recipient` lends that recipient only inside the closure, then restores the root with no recipient. Subsequent `receiving.with_retirement_owner(...,0,...)` therefore returns existing OwnershipLimit `SQLite snapshot has no explicit retirement recipient` before reserving a slot or entering the retirement-child closure. Expected UnsupportedOwner authority literal cannot match.

Wrap the retirement owner case in `with_workspace_recipient` lending the real parent recipient; inside its reserved owner closure lend a distinct empty child recipient. This exercises true parent reserved state and required-mode inheritance. Zero wrapper admission itself should be accepted with no port and no heap; recipient reserve/settle toggles inline state only. Failure of the nonzero child reservation must settle parent slot None and preserve terminal-empty original recipient.

## Ledger Expectations Reviewed

First law: SQL native-paid initial64 does not change forwarded counter; accepted 2-byte SQL reservation leaves remaining62 and forwarded2. Bound callback expects local `(2,0,2,64)` because remaining room is128-64. Refused unbound reservation leaves remaining64/forwarded0. Zero-byte admission leaves both unchanged. Current four rows have no zero-byte bound-port row, so requests==usize::from(port) is valid only for that corpus.

Second law after corrected refusal cases: standalone admission1 leaves remaining127/forwarded1. `with_original_allocation(3)` lends same ledger with local forwarded0 and child total ceiling currentSQL1+3=4. Port request `(2,0,2,3)` is correct. Native-paid secondary receipt1 produces ledger4, no second port request. Child limits do not replace parent's128 ceiling; parent remaining124. After guard drop standalone1 yields remaining123. Parent forwarded remains1 after original-allocation child (its local counter is not propagated), then becomes2. Add these forwarded assertions to prevent accidental mixing of local native-loan and parent counters.

Zero heap expectation is reasonable for these inline guards/recipient reservation/static literal refusal paths; fixture JSON construction occurs outside observed operation. `allocation_stage` returns declared0 when its reservation rejects, matching unchanged actual stage ledger.

## Meaningful Missing Laws

- Scope guard not implemented at inspection. Current fixture Source only uses independent SQL decisions and source declaration guards; Native authors real control propagation but the original port is a mock closure. Initial nativePaidBytes is SQL receipt injection, not an observed actual Native allocation. Add a separate actual native owner/Port slack law if claiming genuine original-native integration.
- Cancellation before reservation, original forwarded port returning its exact borrowed cause, nested scope error return and panic unwind restoration are absent from these new scope tests. Existing separate forwarded cause test preserves an **owned** message pointer; add genuine static borrowed cause preservation inside required scope.
- No explicit guard error-return nesting or already-required parent after inner-drop counter law beyond ordinary inner-drop refusal. Current direct body exits normally after collecting errors; scoped unwind behavior remains untested.
- Adversarial control replacement through public DerefMut remains outside these laws. Do not infer an unforgeable authority promise from private flag fields alone.

Source42233 outcome unknown to this audit. No runtime pass claimed.

## Corrected Current Authored Laws

Re-read after Root changes: retirement owner now executes inside `with_workspace_recipient`, so the real parent reserve is present and the distinct child recipient tests required-mode inheritance. Previous missing-recipient defect is resolved in source. Parent forwarded assertions now explicitly preserve1 after original-allocation child and2 after final standalone admission; these match the actual local-forwarding contract.

Third Native law forwards a genuine supplied allocation callback's static borrowed Canceled cause, checks exact message pointer and Cow::Borrowed, zero heap, unchanged SQL remaining4/forwarded0, and subsequent standalone admission after scope-error return. This addresses the prior borrowed-cause/error-restoration gap. No concrete remaining test correctness defect was found in these current three laws by source inspection.

Current zero reservation corpus covers unbound required mode only. Its zero call should succeed with no port invocation and no counter change. A zero bound-port row would currently invoke the existing `admit_allocation_bytes(0)` port callback: the requirement permits zero, it does not automatically suppress callback. If adding that row, author explicit callback expectations rather than assuming zero means no callback. Current `requests==usize::from(port)` matches the four current rows, but should become fixture-explicit if expanding zero/forwarded combinations.

The public guard is intentionally a first-party lexical invariant, not a hostile-caller sandbox. Control replacement/forged-controller adversarial laws are not required by this chosen scope. Production must retain actual supplied controls; Source integration guards and real owner integration remain the relevant proof. Actual native slack integration and panic-unwind restoration remain optional additional coverage, not defects in these current control-only laws. No build/test execution performed.
