# Exact Domain Metadata Registration

## Chosen Two-Layer Contract

An intrinsic record allowance is a capability held only in a concrete typed composition's private state. The neutral ledger accounts its own fixed registration/witness records plus the consumer's explicitly declared fixed metadata envelope. It strongly retains the exact installed shell but never inspects its properties, calls its methods, enumerates it or claims it is semantically retired.

The concrete typed parent already owns its recoverable pending-construction slot before allocating a shell. The constructor initializes the minimum private state, installs the shell into that typed slot and the exact neutral record, then constructs fields or performs fallible finalization. Original input roots remain owned by the parent until transfer is recorded. Both rosters retain the same shell across a throwing constructor or caller. This is not a dummy owner/page reservation.

## Neutral Signatures To Implement

`ledger.reserveRecord(partition, envelope: ResidentResources, grant) -> {step: ResidentStep, record: OwnedResidentRecord | null}` reserves the full declared envelope plus the fixed intrinsic registration and preadmitted final witness. Each addition is checked before mutation. The record is strongly linked before exposure/finalization. The composition stores this capability privately; public actor/UI facades do not expose it.

`record.install(shell: object, grant) -> ResidentStep` installs one exact shell, once. It allocates no detach capability and calls no shell property. Repeated/foreign installation is rejected. `record.matchesShell(shell: unknown) -> boolean` checks direct identity so the typed construction owner can recover after an actual installation followed by a throwing wrapper. It confers no domain terminal authority.

`record.beginClose()` stops new installation. `record.detach(shell: object, grant) -> ResidentStep` is an operation on the privately held capability, not a ledger API accepting arbitrary roots. It requires this exact original installed shell and a close request; replay/foreign shells fail without releasing credit. The concrete typed owner must first validate its own genuine private terminal witness. The neutral method accepts no witness-shaped object, callback or boolean. Shell detachment and record unlink/refund occur in separate admitted turns.

The fixed `OwnedResidentRecordDetachment` observation is also preadmitted and strongly constructed before shell installation. After detachment, `record.detachment` returns this stable object; `OwnedResidentRecordDetachment.matches(observation, exactRecord, exactShell)` permits exact recovery if the real detach succeeded but its caller wrapper threw before updating phase. Repeating the mutation still rejects. Different shells never match. The observation is not a renewed domain terminal authority. Its exact original reference may retain an already empty domain facade after intrinsic retirement; physical collection is not certified.

The intrinsic record costs 256 logical bytes, three fixed slots and three owner records: registration 128, retirement witness 64 and detach observation 64. The declared domain envelope is added on all three checked axes before construction. A 256-byte work grant admits this fixed registration; shell install/detach each use a separate 64-byte step; intrinsic unlink/refund is a later 256-byte step. Domain field construction has its own real work grant, not permission inferred from the resident allowance.

`record.closeStep(grant) -> ResidentStep` blocks while a shell is installed. Empty never-installed reservations close through a distinct path. After valid detachment, the preadmitted neutral witness becomes valid only when the intrinsic record is unlinked and empty. `record.retirement` and `OwnedResidentRetirement.matches(record.retirement, record)` prove only intrinsic registration retirement, never the shell's domain terminal state. Ledger-wide close also waits on installed records without invoking unknown methods.

The capability has no public-domain getter, callback registration, arbitrary resource decrement or root lookup by key. Its methods are callable only by code retaining that exact capability; the actual actor/UI public API never returns it. Construction does not transfer a newly allocated one-shot detach token whose loss could strand a shell: the original private record capability stays owned throughout.

## Concrete Consumer Join

The actor's existing private pending-dispatch composition owns the record capability, exact response/parser shell and its private retirement witness. Its public close path rejects premature, foreign and replayed domain witnesses before calling `record.detach(originalShell, grant)`. Header/parser/projection envelopes are declared before those objects are constructed. This remains the actor owner's implementation region.

The UI pool constructor will accept the actual shared ledger, with no local capacity authority. UI instance-scope/payload/builder/reader wrapper metadata gets its own declared fixed record allowance; it is not silently paid from the neutral owner/page record price. The UI's current exact private rosters remain the typed construction/close authority. Their private terminal witnesses must prove their children, links and fields retired before detaching their original records. Fixed page backing uses the same ledger's intrinsic page and mandatory registered-reader API. The old unchecked `capture()` is removed, not forwarded to a caller-only neutral handle.

The next UI schema packet will specify the fixed wrapper envelopes and exact grant-bearing admission methods together. Actor's four current UI-pool test constructors and UiDocumentStore's local constructors must move coherently to composition-injected ledgers. There is no compatibility constructor or per-operation ledger fallback.

## Required Executed Laws

Neutral tests must cover charged-but-unused close; exact shell install; foreign/replayed install and detach; global close blocked by installed shell; capacity refusal before shell construction; actual install-then-throw identity recovery; witness finalization failure; and separate terminal child/refund grants. A test-owned typed shell may exercise the neutral protocol, but is not actor/UI runtime proof.

The concrete actor/UI public APIs must separately reject structural/premature/foreign/replayed domain witnesses without changing the record, and recover shells captured during constructor/finalization failures. A legitimate typed terminal witness must detach only the original shell and discharge the intrinsic record once. Both source owners must verify that no public facade property exposes the record capability. Caller loss cannot strand the record because the typed composition and neutral ledger retain their original registered owners.

## Executed Neutral Checkpoint

The neutral source now implements these exact signatures. R17 executed the missing-`reserveRecord` RED. R18 passed exact install, foreign rejection, real install-then-throw recovery, real detach-then-throw recovery, stable exact observation, replay rejection, zero/short final grant conservation and separate intrinsic retirement. R19 adds unused close, all three actual checked-addition refusal paths, ledger-wide close blocked by an installed shell, and record/final-witness/detach-observation finalization failures recovered from the original strong ledger roster.

R19's canonical `@semio-tech/value-resident:test` completed successfully with strict component TypeScript zero. The printed census includes the previous R16 intrinsic laws plus `domainRecord=15 recordOverflow=3 recordConstructor=3`. Full output is `🧪️shared-resident-record-regression-r19-2026-08-27.txt`; the initial missing-method RED and R18 output remain beside it. These are neutral record/capability laws with inert test shells, not concrete actor/UI public-witness tests or a claim of mounted consumer metadata accounting. Those production joins remain assigned to their actual source owners.
