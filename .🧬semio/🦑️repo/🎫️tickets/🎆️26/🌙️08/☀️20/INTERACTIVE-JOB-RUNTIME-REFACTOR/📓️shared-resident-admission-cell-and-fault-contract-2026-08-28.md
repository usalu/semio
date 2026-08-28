# Retained Admission Cell and First-Fault Contract

## Status

Proposed canonical replacement for the R21 return-only handoff. No implementation or runtime credit yet. The independent R3 neutral gate covers R21, not this contract. The current UI child adoption is incomplete; the actor preparation/close controller belongs to the Demonstrator.

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
admission.beginClose(): void;
admission.closeStep(grant): ResidentStep;
```

The cell validates exact ledger/partition and unused, claimed state. The exact discriminated result/root is installed before the resource's fallible finalization and before return. A second admission through an occupied cell rejects without allocating, replacing, or consuming it. The original intrinsic resource roster remains the storage authority; the cell is a separately charged strong delivery/fault owner, not another storage or semantic authority.

## Fault custody and retirement

The first arbitrary thrown value is retained by identity, including `null`, `undefined`, proxies and large object graphs; no stringification, replacement `Error`, property enumeration or cloning occurs. The cell's fixed first-fault metadata is reserved before any guarded work. A fault quarantines subsequent forward work. Typed intrinsic backing/reader descendants can still retire in bounded phases, but the cell cannot refund or issue terminal retirement while its unknown fault remains held. This is an explicit held-fault outcome, not a completed close.

There is no public `clearFailure` or counter subtraction. A later concrete fault-handoff protocol, if required, must use its own exact typed retained owner and witness; this packet does not invent such a terminal proof. A finite test can prove preserved fault custody and released intrinsic backing separately without claiming physical collection of the arbitrary fault graph.

Resource close paths route caught failures into their original admission cell before exposing failure. Internal parent drivers use private exact state/closures rather than shadowable public close methods. A concrete wrapper throwing *after final intrinsic retirement* remains the concrete parent's separately funded fault obligation; it cannot be charged retroactively to a refunded cell.

## Accounting and tests before release

The cell metadata inventory will declare state/facade, stable result, exact first-fault slot and final witness separately and sum their bytes/slots/owners before allocation. Existing resource prices are not used as dummy cell funding. The pending ledger pointer is part of the explicit composition ledger root, which is already outside descendant accounting; the pointed-to cell is fully charged. This is logical retained accounting, not physical JS heap measurement.

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
