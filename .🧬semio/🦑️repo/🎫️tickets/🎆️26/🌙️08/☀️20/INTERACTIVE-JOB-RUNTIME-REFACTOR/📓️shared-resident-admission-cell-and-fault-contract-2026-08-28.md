# Retained Admission Cell and First-Fault Contract

## Status

**R27 canonical source/API release:** `bun x nx run @semio-tech/value-resident:test --skip-nx-cache` exited 0 with strict component TypeScript 0. All five resource methods now require the held cell. Actual full logs: `🧪️shared-resident-cell-resource-red-r24-2026-08-28.txt`, `🧪️shared-resident-cell-resource-r25-2026-08-28.txt`, `🧪️shared-resident-alias-red-r26-2026-08-28.txt`, and `🧪️shared-resident-cell-resource-green-r27-2026-08-28.txt`. R26 exposed the real premature `terminalIsEmpty()` result; R27 keeps the resource nonempty and its 8-byte link charged until the separate alias-detach turn.

The following earlier bootstrap paragraph is historical; the remaining resource gap it describes was addressed in R25–R27. Actor and UI caller cutover still remain pending, and this release does not certify either live integration.

Bootstrap source is present and its canonical R23 standalone gate passed after an actual R22 missing-`prepareAdmission` assertion failure. Full stdout is retained in `🧪️shared-resident-cell-red-r22-2026-08-28.txt` and `🧪️shared-resident-cell-bootstrap-r23-2026-08-28.txt`. R23 executed the existing R21 cohort plus prepare/claim wrapper recovery, first-fault handoff, four exact fault-root variants and empty-cell retirement; component strict TypeScript had zero diagnostics. The printed census now includes the previously executed five resource admission-finalizer cases.

At the R23 checkpoint the five resource admissions were not yet routed through cells. R23 alone was not a resource or actor/UI release. The current UI child adoption is still incomplete; the actor preparation/close controller belongs to the Demonstrator.

## Released R27 Typed Recovery and Close

```ts
ledger.beginOwner(partition, cell, grant): ResidentOwnerAdmission;
ledger.reserveRecord(partition, envelope, cell, grant): ResidentRecordAdmission;
owner.reservePage(length, cell, grant): ResidentPageAdmission;
owner.beginRead(source, cell, grant): ResidentReaderAdmission;
owner.reserveExternalBacking(maximumBytes, cell, grant): ResidentExternalAdmission;

cell.result: OwnedResidentAdmissionResult | null;
cell.result?.kind: ResidentAdmissionKind | null;
cell.result?.record: OwnedResidentRecord | null;
cell.result?.owner: OwnedResidentOwner | null;
cell.result?.page: OwnedResidentPage | null;
cell.result?.reader: OwnedResidentReader | null;
cell.result?.slot: OwnedResidentExternalBacking | null;
cell.result?.step: ResidentStep;
cell.retirement: OwnedResidentRetirement | null;
```

Each typed getter performs the module's exact private resource-brand check; wrong-kind access is null. No structural cast or caller-created result is used. `result.root` remains an identity/debug view and is not necessary for typed recovery. The result is installed before resource finalization, is not replaceable by callers, and survives an outer admission wrapper throwing. A caught outer error is still handed into `cell.retainFailure` on a separately granted turn.

Resource intrinsic `retirement` first proves storage/registration retirement. The resource's `terminalIsEmpty()` remains false while it still references its cell. A subsequent `cell.closeStep` returns `pending`, phase `resident-admission-result-detach`, 64 work bytes: it clears both resource↔result aliases and refunds only the retained 8-byte link. Typed result getters then become null. On another turn a healthy cell unlinks/refunds 296 bytes; a fault-held cell instead returns `rejected`, phase `resident-admission-fault-held`, retains its original fault, and has no terminal witness. `OwnedResidentRetirement.matches(cell.retirement, cell)` is the exact intrinsic cell witness check.

| Admission | Resident charge, excluding its separate cell | Construction work |
| --- | --- | --- |
| Owner | 200 bytes / 2 slots / 2 owners | 200 bytes |
| Page | 520 / 3 / 2, including full 256-byte backing | 264 |
| Reader | 136 / 2 / 2 | 136 |
| External | maximumBytes + 328 / 4 / 3 | 328 |
| Record | declared envelope + 264 / 3 / 3 | 264 |
| Admission cell | 296 / 6 / 6 | 296; claim is a later 64-byte turn |

The explicit test fixture capacity increased only its test slots/owners to cover these new real registrations. Control saturation vectors now derive exact simultaneous owner/page plus two-cell envelopes (1312 bytes, 17 slots, 16 owners); no production capacity or control policy changed.

R27 printed: capacity 6, actual overflow 2, owner/reader 1, partial extent 4, simultaneous raw/UI/scratch 1, posted cancellation 1, unsubmitted cancellation 1, transferred-view fault 1, control axes 3, child-close variants 5, domain-record sequence 1, record overflow 3, finalizer frontiers 8, resource admission failures 5, bootstrap cases 7, first-fault variants 4, resource-wrapper cases 5, terminal alias detach 1, strict TS 0. The previous R25 labels retained old cohort names; this report does not reinterpret them as newly printed counts.

First-fault storage does not bound the physical heap of an arbitrary external exception graph. A distinct subsequent exception cannot overwrite it: `retainFailure` rejects and the caller remains responsible; internal repeated catches propagate a distinct second root rather than discarding it. There is no fault-clearing/refund API or physical-heap/fault-flood certificate. Unknown fault handoff to a final concrete domain owner remains a separate future protocol.

## Exact problem

The R21 resource constructor installs its original facade in an intrinsic roster before finalization. A rejected result can therefore return that exact facade. It does not preserve an arbitrary thrown root, and an outer wrapper can call admission successfully and throw before its concrete parent assigns the returned resource. Neither allocating a replacement nor closing unrelated ledger users is an acceptable recovery.

## Proposed neutral capability

An `OwnedResidentAdmission` is a privately branded, separately charged intrinsic cell. It owns one canonical resource result and a preadmitted first-fault slot. It does not mint actor/UI provenance, inspect arbitrary objects, invoke a structural terminal callback, or certify a domain shell's retirement. Concrete composition keeps this capability private.

The cell itself must not depend on its constructor return. Bootstrap is two-phase under the existing ledger root:

```ts
ledger.prepareAdmission(exactConsumer: object, partition: ResidentPartition, grant: ResidentGrant): ResidentStep;
ledger.preparedAdmission(exactConsumer: object): OwnedResidentAdmission | null;
ledger.claimAdmission(exactConsumer: object, admission: OwnedResidentAdmission, grant: ResidentGrant): ResidentStep;
```

The ledger has one fixed pending-bootstrap pointer. `prepareAdmission` reserves the cell envelope, links the exact state into the intrinsic cell roster, and installs that pointer before fallible initialization. A wrapper throwing after preparation cannot lose the cell: repeated read-only `preparedAdmission` returns the same exact cell for the original consumer identity. A foreign consumer cannot retrieve or replace it. No callback is invoked on the identity object.

The concrete parent first assigns the exact cell into its already-owned private preparation slot, then calls `claimAdmission` on a later grant. Claim clears only the bootstrap pointer, not the roster or cell charge. An outer claim wrapper throwing cannot lose the parent-held cell; its private claimed state is observable without replaying mutation. One pending bootstrap can backpressure new bootstrap requests, but existing unrelated resource owners remain usable. Ledger close services this original cell even if its caller vanishes. There is no second registration of the resource facade.

Canonical resource admission then requires the held cell:

```ts
ledger.beginOwner(partition, admission, grant): ResidentOwnerAdmission;
ledger.reserveRecord(partition, envelope, admission, grant): ResidentRecordAdmission;
owner.reservePage(length, admission, grant): ResidentPageAdmission;
owner.beginRead(source, admission, grant): ResidentReaderAdmission;
owner.reserveExternalBacking(maximumBytes, admission, grant): ResidentExternalAdmission;
admission.result: ResidentAdmissionResult | null;
admission.hasFailure: boolean;
admission.failure: unknown;
admission.retainFailure(error: unknown, grant): ResidentStep;
admission.beginClose(): void;
admission.closeStep(grant): ResidentStep;
```

The cell validates exact ledger/partition and unused, claimed state. The exact discriminated result/root is installed before the resource's fallible finalization and before return. A second admission through an occupied cell rejects without allocating, replacing, or consuming it. The original intrinsic resource roster remains the storage authority; the cell is a separately charged strong delivery/fault owner, not another storage or semantic authority.

## Fault custody and retirement

The first arbitrary thrown value is retained by identity, including `null`, `undefined`, proxies and large object graphs; no stringification, replacement `Error`, property enumeration or cloning occurs. The cell's fixed first-fault metadata is reserved before any guarded work. A fault quarantines subsequent forward work. Typed intrinsic backing/reader descendants can still retire in bounded phases, but the cell cannot refund or issue terminal retirement while its unknown fault remains held. This is an explicit held-fault outcome, not a completed close.

`retainFailure` is a granted 64-byte handoff. Until it succeeds the concrete caller retains the caught value. Repeating the same exact value returns a zero-work observation; a different value rejects without replacing the first or discharging the caller's second value. Failed bootstrap finalization exposes the prelinked original facade, rejects claim, and releases the fixed bootstrap pointer only through a separate 64-byte close phase. The fault cell remains charged and rostered, so releasing that pointer is not refund or retirement.

There is no public `clearFailure` or counter subtraction. A later concrete fault-handoff protocol, if required, must use its own exact typed retained owner and witness; this packet does not invent such a terminal proof. A finite test can prove preserved fault custody and released intrinsic backing separately without claiming physical collection of the arbitrary fault graph.

Resource close paths route caught failures into their original admission cell before exposing failure. The Ledger→Admission driver uses a private closure, but R27 Admission→resource and Owner→child dispatch still call public facade methods. Avoiding per-instance method shadows on a facade whose finalization failed is a remaining source boundary, not a proven R27 invariant; an exact private dispatch regression is queued after the independent source hold. A concrete wrapper throwing *after final intrinsic retirement* remains the concrete parent's separately funded fault obligation; it cannot be charged retroactively to a refunded cell.

## Accounting and tests before release

The source cell metadata inventory is 296 bytes / 6 slots / 6 owners: its 13-field state is 120 bytes, facade 24, three-field stable result 40, four-field step 48, final state 40, and final facade 24 (the declared logical convention is 16-byte record plus 8-byte fields). Fault-presence/value are two fields in that state, not an uncharged later allocation. Resource adoption will additionally charge one 8-byte state link per resource. Existing resource prices are not used as dummy cell funding. The pending ledger pointer is part of the explicit composition ledger root, which is already outside descendant accounting; the pointed-to cell is fully charged. This is logical retained accounting, not physical JS heap measurement or a bound on an arbitrary external fault graph.

The source/result alias-detach phase remains to be implemented with resource adoption. It must run on a separate grant after exact intrinsic terminal observation, before cell refund. R23 does not execute that future phase or repeated fault-flood admission.

Required neutral vectors and actual runtime oracles:

- Bootstrap wrapper throws after original preparation; same consumer recovers the original cell while unrelated storage remains readable.
- Foreign/duplicate/replayed bootstrap and occupied-cell admission reject before mutation; zero/short grants conserve all axes.
- Each of owner/record/page/reader/external admissions throws after the original canonical return; the held cell recovers the exact original resource, never a replacement.
- Constructor/finalizer faults preserve the exact arbitrary root and prevent cell refund; distinct `null`/`undefined` cases cannot alias the no-fault sentinel.
- Typed backing retirement progresses while first-fault custody remains funded; final witness stays absent.
- A close child with full-grant work leaves wrapper bookkeeping for another turn; rejected/blocked/over-grant results retain their original accounting.
- Bootstrap caller loss, normal unused cell close and original resource retirement are strongly owned and bounded, without a whole-roster scan.
- Strict Ajv checks the language-neutral vectors; Immer/BigInt provide conservation oracles, and Buffer checks live unrelated byte content.

The staged API names need parent/peer coordination before canonical signature replacement. No compatibility overload, structural receiver, public unchecked factory, per-operation ledger, or new capacity default is proposed.

## R28–R33: Own-Cell Quarantine and Exact Private Drivers

Canonical command: `bun x nx run @semio-tech/value-resident:test --skip-nx-cache`.

R28 executed the page-allocation-after-own-fault regression and failed (ready instead of rejected). R29 passed after guarding each original resource admission independently of the parent: page allocate/write/seal, reader admission/byte/length, external fence/current custody/byte/length. An already-posted late buffer still transfers into retirement-only custody; its original aliases detach, no live receipt or readable access is issued. The exact first fault and its 296-byte cell remain held after intrinsic children retire. R29 did not print the new quarantine count, so its historical stdout is not a printed census claim.

R30 failed the actual failed-finalizer facade shadow law: two replaced public methods were invoked rather than zero. The source now captures each original class close implementation at class initialization and dispatches through the exact private resource state and retained original facade. Parent close sets the private closing state directly. Neither per-instance public method shadows nor later prototype replacements are consulted. Ledger→Admission remains the existing private closure. This corrects the explicitly open R27 dispatch boundary above.

R31 reached the old prototype-injected child-result test and failed because the injection was now correctly ignored. The five neutral child vectors are preserved as ignored-shadow laws; each actual empty page terminal step consumes the entire 264-byte grant, forwards pending/264, and leaves the owner's release for a later turn. The exact source-private `forward` function is selected with the TypeScript AST, transpiled and executed in a test-only VM closure against the original blocked/rejected/over-grant/full-4096 vectors. There is no production export, callback, mutable test capability or substitute runtime driver. R32 encountered Bun's unsupported large data-URL module resolution before these tests; R33 uses the test VM closure instead. This harness error is not a behavioral failure.

Two real child-work faults additionally execute the original page close path: `Uint8Array.fill` throws before work or after the original fill. The original parent cell retains the exact thrown root, the page remains nonterminal, and later bounded intrinsic retirement succeeds without releasing the fault cell. A thrown child has no successful work result; zero reported successful bytes is not a wall-clock or no-work certificate for an arbitrary throwing implementation.

R33 actual Nx exit 0 and strict production TypeScript diagnostics 0. Full retained output: [R33](🧪️shared-resident-dispatch-r33-2026-08-28.txt). Its printed census is capacity6, actualOverflow2, ownerReader1, partialExtent4, simultaneousRawUiScratch1, postedCancel1, unsubmittedCancel1, transferredViewFault1, controlAxes3, childClose5, childFault2, privateDispatch5, quarantine11, domainRecord1, recordOverflow3, finalizerFrontiers8, admissionFailures5, admissionBootstrap7, firstFault4, resourceWrapper5, terminalAliasDetach1. Ajv/Immer/Buffer/BigInt remain the neutral schema/conservation/content oracles. The fixture's intrinsic domain-record charge is corrected to the already-canonical 264 bytes (including its retained 8-byte link).

API signatures and prices remain the released R27 mandatory-cell contract. This is standalone neutral behavior, not actor/UI full-suite or live cutover evidence. UI pool descendants and their authored callers remain under canonical shared-ledger migration. No native input ACK, semantic publication ACK, arbitrary-fault collection, or physical-heap bound is certified here.
