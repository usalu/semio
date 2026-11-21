# Note Tool Switch Publication

The first canvas owner refusal in the Chromium acceptance run followed activation of Pencil. Earlier block creation, undo, and selection succeeded. A fresh browser test exercised pointer interaction and wheel before activating a utility and reported no publication-owner refusal.

The mounted command runtime delivers the host switch facts and stages an empty original emission for `setActiveTool` and `setActiveUtility`. Original emission looks up the exact action in the mounted application and framework publication registries. Both switch actions were absent from the framework registry. Later input can resume a pending original emission before it selects its own route, so one missing owner can surface on later input IDs.

Both switch actions now register an exact framework factory under the mounted controller with a `HostOnly` publication lane. The added contract fixture records the action IDs, schema IDs, and publication lanes. The mounted registry test checks those rows. An independent SQLite JSON1 comparison verified a single registration for each fixture row and the existing continuation from direct dispatch to the exact mounted lookup. The source diff passed its whitespace check. The native Rust test and an updated browser run remain pending.

## Original Emission Capture Retirement

The attempt 42 Chromium run reached the newly registered owner and later refused the command with `original emission changed its actual source caller or lifecycle`. The mounted handoff transfers the emitted input and caller loan from the capture. Settling its construction resets the completion flag and leaves the construction bound to the original source. The handoff removed the capture's own binding before phase 8 retired that construction. Phase 8 therefore refused the same caller when it tried to retire the capture.

The handoff now copies the original binding into the mounted header and keeps the capture's binding until its paid close. The capture's close quotation names its outstanding construction state. The neutral fixture requires the capture to stay pending after handoff, reject retirement from a different decoder, and retire under the original decoder while the mounted input remains retained. The Bun SQLite oracle passed all four fixture tests. A source check reproduced the lost binding before the edit and passed afterward. The rebuilt browser run and native Rust regression are pending.

## Reserved Interaction Metadata Retirement

The rebuilt attempt 43 reached the mounted utility operation. Its later Chromium trap reported an ordered-map root dropped without explicit retirement. The first affected input was `interactionHover`. The host-reserved interaction route clones `ActionMeta` into a queued operation and needs its full view to identify the hover window. The manifest and window routing cache retire their own view aliases independently. If the reserved commit holds the final alias, popping the sealed commit after retiring its cancellation lease destroys the live ordered map by ordinary `Arc` drop. Instance close had the same commit removal path.

The reserved commit now retains its metadata in a bounded retirement owner. Both normal sealed completion and instance close drain it before removing the commit. The fixture covers last ownership of an active Pencil view after ingress releases its alias. The independent SQLite test passed, and the production and test diff passed its whitespace check. Native and browser confirmation are pending.

## Pending Effect Surface Borrow

Attempt 45 reproduced the ordered-map trap after the reserved commit retirement change. The compact Wasm stack puts the panic in the command exchange path. The exact served core has 73 imported functions. The function table resolves frame 245328 at `0x537500e` to the ordered-map destructor assertion at line 105. Its direct caller, function 13866 at `0x987612`, contains the runtime exchange and finish source locations. Its outer frames contain `plugin_exchange_turn` and the command ingress turn. The two later `interactionHover` and `interactionSelect` failures were issued after the shared guest trapped.

The effect pull in the exchange finish cloned the live surface `ViewModel` into a closure-local value. The clone includes live ordered-map owners and its ordinary scope exit bypassed their paid retirement. The effect poll now borrows the current surface view and the mutable app as disjoint fields under the same exclusive instance guard. The guard still protects the full synchronous effect poll, including preparation and the app callback. The packed pure command path contains a separate view clone and was outside the reproduced window-action route.

The fixture checks that the surface keeps one owner across both a pending poll and a published response. The source assertion reproduced the clone before the fix and passed with the guarded borrow. The independent SQLite test passed. The rebuilt guest and browser acceptance remain pending.

## Consecutive Command Receipt Binding

Each command page on the normal manifest route admits or validates the live instance's `OriginalCommandOrigin` before receiving its payload. The origin pins a single `OriginalSourceBinding` until instance close. A successful construction leaves a source binding after its transient frame has settled. The receipt construction admission compares that binding to the actual actor source, independently of the receiving decoder for the individual command. Consecutive manifest commands can therefore reuse the construction admission under the same live actor source.

If a different ingress route reports `constructor changed its actual Source`, capture the first rejected action, the acquiring and returning source identities, and the live instance's original command scope. That trace determines whether the same admission slot is shared across distinct original sources.
