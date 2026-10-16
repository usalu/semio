# Cache Store Consumer Lifetime Design

## Observed Source and No Execution

Read `artifact-reader-independent-current.md`, the current generic reader, artifact storage helper, normal/maintenance store owners, Core control/refinement implementation and actual Native close/port implementations. No production Source changed, Original was created, image opened, test run or cleanup performed for this design.

Current normal `withRepositoryCacheStoreV1` creates an unsettled synchronous control, opens the real image, recovers its charge, invokes the callback and then closes the image. Core-enrolled detached consumers are not awaited by that callback owner. The held-store branch also invokes its callback after an awaited recheck without synchronous pending enrollment. A Core operation can therefore remain live after its actual backing reader closes.

The artifact reader now enrolls under actual base and phase before SH acquisition and holds producer SH outside its inner consumer loan/drain. Storage's actual `withArtifactStorageOperationV1` similarly authenticates exact invocation, private Native reservation and actual refinement. Those operations can wait for SH after the store callback returns. Merely wrapping the existing callback in a canonical requested loan closes ancestor births at callback return; the accepted waiter can then fail to issue its reader/task child after SH becomes available. The graph proof established this exact lifetime distinction.

## Ownership Invariants

- Preserve the real store object and its stable `control`, `storage`, `image`, invocation, environment and Native port identity. Native ports compare their issuing control by exact object identity; replacing `store.control` with a different child loan would make a false ownership pair even when Native reservation signal checks happen to accept it.
- No ambient Original lookup, structural store/control copy, global registrar, new process birth, deadline renewal, sampled duration, or Core ancestor-admission exemption.
- Add private actual-store issuance indexed by exact store object and exact Native reservation. Retain closed records in the WeakMaps, so an escaped known store cannot become an unknown generic Native fixture.
- Actual cache image custody must outlive both private accepted-store drainage and all Core-enrolled work under its canonical owner control. Known consumer frames must drain while the owner's canonical callback is still active, before Core closes its births.
- Only privately known reservations get cache-store lifetime enrollment. A separately and genuinely owned Native fixture remains in its existing explicit Core/image owner; no path, receipt, owner string or structural fields can adopt it as a repository store.

## Proposed Minimal Owner API

The store owner keeps the issuance maps private and exports one narrow synchronous enrollment seam, tentatively:

```ts
withRepositoryCacheStoreConsumerV1<T>(
 custody: {
  storage: NativeStorageReservationV1;
  control: ResourceLeaseControl;
  invocation: ScriptInvocation;
  phaseControl: ResourceLeaseControl;
 },
 run: (phaseControl: ResourceLeaseControl) => Promise<T>
): Promise<T> | undefined
```

`undefined` means only that the already-authenticated Native reservation has no privately issued repository-store owner; it never means success or permission. A known store returning `T=undefined` still returns a Promise. The caller's existing genuine Core-owned Native path handles absence. A known mismatch, closed issuance, wrong actual invocation/control, missing owner-context ancestry or invalid refinement throws before callback/effects; it never falls through. No token can be cast into `T` and no failure ledger is cleared.

The helper authenticates private actual controls/invocation/refinement and Native reservation before lookup. Known-store matching additionally requires exact captured reservation/control/invocation identity and the actual private frame ancestry, not repository/capability equality or an unrelated live Original. It does not issue a store or accept caller registration.

Every accepted callback gets a real canonical requested child control plus a private frame. Pending registration occurs synchronously before Core's deferred effects. Frames have a live actual owner, accepting flag, pending set and first failure flag/value (including undefined). Author the store pending bound in its closed protocol schema (proposed32 active consumers per actual store issuance, no caller override), and reject before new effects when full. Count private active operations rather than treating the scalar as a physical disk quota. A frame closes only its own births after callback return, drains its accepted children before the canonical callback returns, then performs its final real recheck. An accepted child frame may finish its own registered descendants while a returned parent refuses new births. There is no mutation of a closed Core scope.

The callback receives the actual carried control so lease acquisition, hashing, Native use and owned commands use the same live narrowed phase. This is preferable to returning a changed store or inferring a phase from caller DATA.

## Live Phase Selection

Root/store allocation control remains separate from the current consumer effect control. Use Storage's read-only `requireIssuedResourceControlRefinementV1` to prove actual private relationships; never use numeric remaining-time comparisons.

The effective phase must preserve both the current accepted frame and the explicit caller phase. Choose the explicit phase only if it privately refines the current frame; otherwise choose the current frame only if it privately refines the explicit phase. Refuse incomparable restricted siblings. An unrestricted same-Original zero base may admit its actual narrower sibling per the existing guard. All controls must have the exact required invocation binding. A genuine foreign Original with identical capability DATA is not a borrower.

This requires the lower artifact operation callback to receive the effective phase, instead of closing over an earlier `phase` variable. Root's reader operation and each accepted reader consumer must use the same seam; wrapping only the outer reader leaves a child consumer's independent canonical callback able to close too early. Storage's scratch/publication/binding operation callbacks must carry the returned phase to their waits/effects too. Their physical cleanup remains under their existing preaccepted cleanup owner.

If an API supplies only an invocation and derives a new unrelated control internally, it does not automatically inherit a resource phase. Such actual callsites must either carry this returned control or conservatively refuse incomparable scope use. No claim is made that matching invocation DATA alone propagates deadlines.

## Normal Store Owner Lifetime

Create the actual store/Native-port control with canonical `withIssuedResourceControlV1` rather than an unsettled synchronous factory. Open the image and obtain its genuine reservation under that exact control, preserving existing field/port identities. Construct exactly one frozen store and private issuance record.

Inside the canonical callback, run a private accepted frame for the user body. It closes user births and drains known accepted store consumers before its canonical callback returns. The canonical owner then drains any additional directly Core-enrolled work. Its actual image close happens outside that entire settlement and preserves primary Error/undefined over close errors. Managed Native close is an actual SH release; current Mac and shared fixed implementations do not require issuing new work after owner-control retirement. This exact close contract needs a physical regression; do not assume it for arbitrary fresh-private backends.

Store recheck must require private live issuance, the exact real reservation/control and the actual live owner context; an escaped store must refuse once settled. Internal final fences use a private read-only closing context rather than reopening public births. Ownership stays live through actual Core drainage, then closes before the image reader is released.

## Held/Reentrant and Cross-Store Calls

The same-key/same-invocation branch must register its complete callback synchronously before its first awaited recheck. It passes the same actually issued store; a new private child frame retains the current real scope. An ignored reentrant callback therefore cannot outlive the image, and a returned parent cannot admit a late callback. Frame phase constraints remain live rather than being recreated from a sampled duration.

For nested different cache partitions under the same exact invocation, the new store operation should itself enroll in its current privately accepted parent frame before opening the child image; both image scopes remain held through the child's complete owner settlement. A private frame ancestry chain supports accepted descendants using actual ancestor store resources without treating the closed parent's public birth slot as open.

Different actual invocation identities must not silently borrow a held store. Existing semantics open a separate actual Native reader when a different invocation is explicitly supplied. Whether that independent owner can be nested under another invocation's private phase must be checked conservatively; no path/capability-based reuse or hidden cross-invocation control refinement is proposed. The first implementation should refuse a known-store consumer with a mismatched invocation and report actual cross-alias opening requirements before inventing a bridge.

## Maintenance Boundary

Existing whole-image maintenance has a distinct private task list and actual imageEX callback. Its known accepted consumer callbacks must be drained before that EX callback returns. The new issuance lookup must never mistake a closed maintenance store for an unknown Native fixture. Preserve its exact registry/current-reader/imageEX authorities and primary failure behavior.

Normal owner drainage can cover every directly Core-enrolled base operation by enclosing image close outside the canonical control settlement. For imageEX maintenance, direct unregistered Core work is a separate ordering concern because EX release is inside the Native callback. Do not claim universal maintenance safety from the known-consumer registry alone. The reviewed implementation scope should either extend actual maintenance lifetime ownership coherently or explicitly reject unsupported maintenance borrowing; no new canonical maintenance execution is requested here.

## Genuine RED and Bounded Proof Plan

Only after Root reviews this design, stage a schema-owned neutral corpus and routes in the existing private script/editor catalogs. A bounded private32MiB Native fixture must exercise the actual owner lifetime helper without changing repository capacities or fabricating a RepositoryCacheStore. If a private lower owner extraction is needed for authentic fixture ownership, authenticate its actual Native image/control/reservation and keep creation private to the real store owner; no public structural registrar.

Meaningful RED: while the callback is active, enroll a detached consumer under the actual base/phase and deliberately hold its producer EX so its SH waits. Return the store callback. Independently observe whether image EX becomes available early; then release producer EX and require the accepted reader to create its real child loan/owned command, consume exact bytes and settle before backing release. Current owner should close too early or invalidate that accepted descendant. Parent failure is genuine; expected negative child outcome may be inspected as DATA, never acknowledged into success.

Green proofs must cover:

- same actual callback with detached reader waiting for producer SH; independent Python SQLite checks image/producer guards before and after actual drain;
- an already accepted consumer issuing a genuine nested owned command after the outer callback returns;
- detached held/reentrant callback and nested same-Original different-store callbacks, each retained before first await;
- late public body birth refusal while accepted child descendants may finish; no phase rebasing;
- primary Error and undefined, secondary/detached failure, cancellation and init failure with real Root ledger/final settlement;
- copied/foreign/escaped store and reservation/control/invocation mismatches, including genuine equal-DATA foreign Original;
- known closed reservation refusing rather than becoming the generic Native path;
- actual store remains the same object, Native port still accepts its exact ownership control, and real parent reservation/image cleanup settles last.

The healthy parent must be accepted before a negative child's Root cancellation and retain exact known private allocation/inode facts for cleanup. No late replacement cleanup Original, process killing, image force-detach, raw deletion or unknown-path cleanup. Physical proof is held until Root and Storage grant the small-image window.

## Coordination and Handoff

Root owns the generic reader's descriptor/inventory/opaque-task work. Storage owns scratch/publication/binding and the existing artifact operation helper. This lane owns normal/held cache-store issuance/lifetime and the owner enrollment seam. No reader or Storage Source will be edited in parallel by this lane. Exact helper name/signature and maintenance/cross-alias scope must be agreed before Source changes. No canonical products/Tools/Rust, full graph, SDK, fence, full-size runtime or maintenance is part of this design checkpoint.

## Concrete Small-Image Fixture Option for Review

The normal API derives capacity/path from its closed repository budget. Calling it on a nested workspace without changing fixture data would allocate another multi-GiB partition and is not authorized. A structural fake store, mocked image opener or public owner registrar is therefore unsuitable for the RED.

One truthful one-image option, requiring explicit review, is a standalone test child that opens its funded diagnostics host first, then temporarily tightens only the in-memory diagnostics declaration to a known private path beneath that already-held host and capacity33MiB/filesystem32MiB. Canonical leases remain unchanged. The actual normal store API then opens/reclaims that one real private32MiB image and genuine charge. The policy Source file is never modified, no new caller knob exists, capacities are only tightened, and restoration/finally occurs after exact known private charge/backing retirement. The parent retains its actual healthy host and Original through expected negative child settlement. This follows the earlier allowed data-only reduced fixture limits, but budget/path declaration authority is more consequential than a metadata cap, so it is a proposal rather than assumed authorization.

If this tightening is not accepted, use a reviewed internal lower owner extraction with actual private Native custody; do not export a structural registration function merely to make the test easy. No physical proof has been launched with either approach.


## Implemented Private Owner Checkpoint

The approved proof uses no policy mutation. `withRepositoryCacheStoreFixtureV1(parent, invocation, {version:1,kind:"consumer-lifetime"}, body)` authenticates a privately issued, still-live diagnostics parent, its exact Original/control/reservation and retained consumer frame. It creates its own random `tmp/cc-<12hex>` namespace and actual 32 MiB Native image, obtains a real 8 MiB artifact charge, and invokes the same private store-body function used by the normal opener. The factory never accepts an image, port, path, capacity or registration callback. Its private child key is the real child root; it never replaces the canonical diagnostics partition key.

Normal opening now holds its actual canonical control through Core settlement; image close follows that settlement. Known store consumers register synchronously in private frames before deferred effects. Each actual owner has a cumulative 32-active limit. Frames retain pending tasks and preserve primary Error or undefined failures; a returned parent frame closes births while an already accepted child may enroll its own descendants under a live canonical loan. Retained outer stores are found only in the actual private parent-frame chain. An inner operation may borrow an outer store, but registration stays in the current accepting frame and carried control retains the current actual restrictions. Cross-alias normal opening is conservatively refused.

The bridge returns synchronous undefined only for an independently authenticated Native reservation absent from the private store map. Known maintenance, mismatched, closed or out-of-context store reservations refuse and never fall back. Maintenance's existing own task/drain mechanism is preserved; this checkpoint does not claim direct-Core maintenance imageEX drainage is solved.

The test factory retains the genuine parent across child settlement. It records its created image device/inode/size, closes the child reader only after settlement, rechecks that exact image identity, reopens under the already retained healthy parent, obtains nonblocking real imageEX, verifies the complete exact single allocation census, retires that charge and the exact image, and rechecks the parent. This is diagnostic-only ownership, not an arbitrary Native port registrar.

### Actual Evidence So Far

- Genuine shared-body RED: prototype 49472, exit 1, `retiredEarly:true`; exact own image was retired. No pass is attributed to this run.
- First shared-owner successor: prototype 92856, 2 laws / 5 assertions, exit 0.
- Nested/private/cap/SQLite successor: prototype 86374, 4 laws / 29 assertions, exit 0, 8.67 s.
- Release 70548: 4 laws / 39 assertions, exit 0, 7.97 s. Separate genuine failing child Originals preserve Error and undefined failures after exact image retirement; both children exit 1, parent observes terminal status as data.
- Strict 90871 / 84023: both modes exit 0 for store/tests/private prerequisite closure. These precede final joint reader integration.

The cumulative owner active limit includes outer reader operations. It does not promise that a store-owned reader can accept 32 tasks in addition to its outer scope. The positive store proof independently uses Python SQLite to establish actual resource EX held/free; it does not by itself qualify generic reader producer-SH or command consumption after callback return. That joint proof remains pending until the carried-phase artifact helper is wired.
