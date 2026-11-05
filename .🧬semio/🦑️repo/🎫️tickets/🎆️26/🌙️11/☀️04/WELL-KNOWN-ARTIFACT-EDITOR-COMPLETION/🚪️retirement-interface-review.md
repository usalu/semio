# Retirement Interface Review

Read-only review of current root retirement source; no heavy tests or product edits. Early identity refs flow through identity-document-ports into mutable core, with fail-closed initial admission/persistence behavior. Late close-document assigns admission and natural scope assigns captured persistence. Canvas, plugin lifecycle and switch wrappers read refs at invocation. Current source switch gate invokes retire synchronously before publish (surface-switch366–377); its initial ownership fence is sound.

Concrete follow-ups reported to root:

1. requestProgramRetirement lease sets started before closing all attachments, then awaits each drain/destroy. A drain or destroy rejection exits the loop after attachments are closed; release ignores started leases, leaving surviving owners sealed and later owners unretired. Define and test cleanup/finalization for this partially committed failure.
2. The started lease clears closeRequestRef before awaited drain. Until the caller removes its panel, another retirement path can capture the same owner. Already-sealed owners are admitted, and canvas close serialization and plugin-operation lock cover separate routes. Add a shared per-owner retirement reservation across canvas and plugin lifecycle to prevent duplicate destroy/module disposal.
3. Archive-file importDocumentArchiveOnce still creates, loads and focuses without the new plugin-operation, exact loaded-handle and captured-host fences used by natural import and kept reopen. A queued archive import can race plugin reload/uninstall and publish from an old handle.

These are source-derived scenarios awaiting root actual callback regressions. Existing252host tests predate the new retirement implementation.

## Latest Reservation Review

Root added exact handle/instance reservations and deferred caller release until drain settles. This closes the earlier duplicate-admission window and releases seals for owners whose destruction was not attempted. Remaining failure issue: all attachments still close before the first drain, so surviving later owners become unbound; a kept owner then hits duplicate catalog ID on first-save fallback. Close per owner at commit and define attachment recovery for a failed owner.

Seals were keyed only by plugin ID and numeric instance. Reloaded handles can reuse IDs and fresh intake is blocked. A required handle-scoped API and precise call-site map are in 🪪️sealed-owner-callsite-map.md. Also found FIFO reseal bug: unseal leaves an old key in the FIFO, so with capacity1 seal/unseal/seal evicts the fresh seal. Root is implementing both fixes.

## Existing Kept Recovery Review

Current recovery writes the edited app current full native archive to its captured existing kept target, checks exact bytes and current owner, and then attaches. It does not load older disk content into the edited app. Cancellation after receipt closes only its exact client and drains its lane. Moving attachment close per owner preserves untouched later owners after an earlier failure.

Concrete follow-up: captured kept catalog record is not part of current(). A concurrent retire/replace of its target/surface/schema/admittedAt can make recovery reattach an obsolete or forgotten target. Recheck captured catalog fields before write, before attach and after attach; allow unrelated concurrent documents. Root notified.

Latest descriptor check remains stale: Office9editors lacks IDML; BIM6lacks IFC4x3; Semio19has text schema s.stdio.semio.text and empty import formats. Authoritative refresh is queued by runtime owner. Current source roster/schema fixture remains91pairs/91declared codecs with6lightweight metadata tests passing.
