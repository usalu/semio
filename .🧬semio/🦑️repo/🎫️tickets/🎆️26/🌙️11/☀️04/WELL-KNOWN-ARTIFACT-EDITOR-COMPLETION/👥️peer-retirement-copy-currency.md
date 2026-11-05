# Peer Retirement Copy Currency

This is a separate source-level accounting issue from the diagnosed lazy 536-byte Arc allocation. No native execution or production edit was performed.

PeerPresenceRootRetirement contains ManuallyDrop<Option<Arc<PeerPresenceRoot>>> root, ManuallyDrop<Option<PeerPresenceRoot>> owned, ManuallyDrop<Option<SharedControlledRetirement<PeerPresenceEntry>>> entry, and usize cursor. ManuallyDrop preserves the inner type layout.

## Concrete zero-copy state change

When owned is Some and cursor >= root.len, retirement_demands returns copy_bytes 0. close_step then writes root.len=0 and executes self.owned.take(), returning Complete with item1/copy0. A grant with positive items/depth and copy0 therefore removes the real inline owner, including the newly relevant empty root. Option::take moves the Option value and writes None; it is not a no-state-change transition. The precise opt0 memcpy extent cannot be certified from source, but an actual nonzero metadata quote is mandatory even independently of compiler lowering. A direct full Option take should conservatively quote its actual size_of<Option<PeerPresenceRoot>> plus the writes/moves the implementation performs, or be replaced with a bounded final-address clear whose actual extent is quoted. Do not hardcode the observed platform size.

## Other metadata inventory

For cursor < len, demand quotes only size_of<Arc<PeerPresenceEntry>>. The step increments usize cursor and takes an Option<Arc<...>> slot; a missing entry returns item1/copy0 despite that cursor and Option write. A populated entry additionally initializes the inline SharedControlledRetirement Option. These parent metadata writes require actual typed extents and must appear in the returned receipt.

An active entry child step may become terminal, after which the same call executes self.entry.take() without a separately quoted parent-clear extent. Its demand only adds depth to the child demand. The clean seam is either a separate paid terminal-parent removal transition, or a combined quote and receipt including the actual parent metadata operation.

Root Arc extraction currently quotes size_of<PeerPresenceRoot> and actual shared allocation release, but also takes the root Option and assigns owned Some. Those pointer/Option metadata extents should be included in the same inventory rather than assuming a payload-only quote is exact.

## Minimal actual regression law

Construct the real retirement owner from empty and populated roots, drive it to each metadata boundary under the same original caller, and snapshot root/owned/entry/cursor. A zero-copy grant must preserve the exact snapshot whenever that next transition requires copying. Grants just below the actual typed demand must similarly preserve state. The exact funded grant must advance and report the actual copied extent, including sparse entry holes and terminal child removal. Finish each owner with funded closure; do not drop an incomplete owner after an assertion path. Existing physical allocation checks complement this test but do not measure stack-copy currency.


## Permanent neutral transition inputs

Extended existing peer handoff corpus/schema/test with six metadataTransitions: owned-empty final removal, sparse cursor advancement, and terminal entry cursor removal, each denied and paid. Fixtures carry only state/progress booleans and cursor stages, with no assumed Rust sizeof byte counts. SQLite owns the independent mutable state and verifies denied copy admission preserves it exactly, while accepted transitions report nonzero metadata work. Actual Bun execution before production repair: 5 pass, 0 fail, 72 assertions, 445 ms. This green validates the schema and neutral law, not current Rust behavior; root owns three real native regressions and the production repair.


## Genuine red and current source repair

Editor confirms actual instrumented native execution0pass3fail7filtered, not a compiler failure. The three red assertions reproduced empty owned clear under copy0, sparse cursor copied0 versus quoted8, and terminal child cursor removal without funded copy. Root then repaired the production class.

Current source final owned clear quotes/reports2*sizeof<Option<PeerPresenceRoot>> and removes the redundant len write. Sparse cursor advances quote usize plus two Option<Arc<Entry>> moves and report that same demand even for a hole. Terminal child cursor removal is now a separate positive-copy transition; the child step no longer clears its parent on the same turn. Root Arc extraction accounts for root/owned Option moves while retaining the actual shared allocation release quote and zero new capacity. No fake heap receipt, new ceiling or replacement owner was introduced. No concrete source mismatch was found against the three regression boundaries.

Focused permanent neutral recheck after source repair:5pass0fail72assertions295ms. Fresh instrumented native green is still pending and cannot be inferred from source review or this neutral result. No native job launched by auditor.
