# UI Adoption of the Composition Resident Ledger

## Current Boundary

The neutral ledger's intrinsic storage and fixed-domain-record protocol passed the member R19 gate and the coordinator's independent record gate. The existing UI pool still has independent byte/page/owner counters and is not adopted. This report does not claim shared raw/UI admission, native page acknowledgement, live renderer mounting or a physical heap bound.

The fixed UI metadata catalogue is staged at `ui/contract/retained/resident/metadata`. Its neutral fixture names each planned fixed record and field. Logical accounting is 16 bytes per record plus eight bytes per declared field, with one slot and owner per record. This is an explicit logical representation, not JavaScript object sizing or Rust `size_of`. Actual implementation must remain within the declared inventory; a changed fixed layout requires updating the catalogue and oracle. Intrinsic record overhead is an additional 256 bytes, three slots and three owners. Backing and registered intrinsic reader charges are separate again.

| Typed record | Domain bytes | Slots/owners | Explicit contents |
| --- | ---: | ---: | --- |
| Pool | 192 | 4 | Fixed state, facade, empty binding table and preadmitted terminal witness |
| Instance scope | 304 | 5 | Fixed state, facade, exact lifetime value, binding entry and terminal witness |
| Payload | 232 | 3 | Fixed state, facade and terminal witness |
| Builder | 264 | 2 | Fixed private fields, record phase and terminal witness |
| Reader | 112 | 2 | Fixed private fields, record phase and terminal witness |
| Page wrapper | 232 | 4 | Fixed state, facade, builder list cell and terminal witness |
| Input evidence | 152 | 3 | Exact evidence state, facade and terminal witness |

The pool is not silently treated as an additional uncharged composition root. Its actual original composition must retain its pending construction slot before the pool shell is allocated. The shared ledger charges its metadata record first. The shell enters both that exact typed slot and the neutral strong record before fallible initialization/finalization. The original composition, not a caller-only factory, drives abandoned construction to terminal. The Demonstrator owns the ShardClient/composition registration seam; no structural callback or arbitrary `isRetired` predicate is proposed. The pool record capability stays solely in that Shard private slot, not in the public UI facade or an exposed preparation token.

## Planned Canonical UI API

The owning composition supplies its existing `OwnedResidentLedger`. No UI capacity object, per-operation pool, guessed default or fallback remains after cutover. The existing 24 UI fixture constructors and four actor fixture constructors must change coherently; actor fixtures remain their source owner's region.

The settled composition-only factory is `OwnedUiResidentPool.begin(client, ledger, grant) -> {step, pool}`. The client must already own that exact injected ledger through required `ShardClientOptions.residentLedger`; `ShardClient.matchesResidentLedger(client, ledger)` verifies its private identity. There is no actor-constructed or per-pool ledger. The UI exports `uiResidentMetadataEnvelope("pool")` for the peer to reserve the actual canonical metadata allowance.

The agreed peer methods are `prepareUiResidentPool(ledger, grant)`, `installUiResidentPool(pool, grant)`, `ownsUiResidentPool(pool)` and `releaseUiResidentPool(pool, witness, grant)`. Preparation owns the neutral record in the original private client slot before shell allocation. The UI forwards a consumed preparation step and returns pending; construction/install only starts on a later ready step that consumed no work. An existing or failed installed shell cannot authorize a second allocation. `OwnedUiResidentPool.matchesComposition(pool, client, ledger)` checks the actual private UI pair. The preadmitted private `OwnedUiResidentPoolRetirement` is the final release authority; the peer validates its exact client/ledger/pool identity before record detach and separate intrinsic refund. These are agreed API names, not yet a source-ready concrete join.

The retained pool then exposes grant-bearing `bindInstance(owner, activation, lifetime, grant)`, scope `beginPayload(grant)` and payload `reservePage(length, grant)`, each returning a retained admission result. Builder and reader admission likewise receive actual work grants. Each admission reserves the declared domain record, retains its pending construction state and installs the shell before exposing a facade. When admission takes several turns, the exact parent retains the pending slot rather than retrying an allocation from scratch.

The UI instance scope owns actual intrinsic storage registrations on the same ledger. Fixed page allocation uses `OwnedResidentOwner.reservePage(logicalLength, grant)` and pays the full 256-byte backing even for partial or empty logical extents. It never allocates a separate UI `Uint8Array`. A page's reader can only be created through an already strongly registered concrete consuming owner, which calls the intrinsic `beginRead`. The unchecked old UI `page.capture()` is removed rather than forwarded. A live older consumer may still read after producer close; the producer cannot retire its backing before that exact consumer closes.

## Exact Terminal Ordering

Every typed owner first closes its own reader/child registrations, clears source, parent and sibling links, and validates its private terminal witness. Only then may its privately retained neutral record call `detach(originalShell, grant)`. A separate granted turn closes the neutral record and observes its intrinsic retirement witness. Record capability and detach authority never appear on the public UI facade.

An actual detach followed by a throwing wrapper recovers through the record's preadmitted `detachment` observation against the exact original record and shell. It does not repeat mutation or allocate a replacement proof. A neutral retirement observation may retain only an already domain-empty facade; it certifies registration retirement, not physical collection. Parent completion waits for both the actual typed terminal proof and the intrinsic registration witness.

Child `blocked` and `rejected` outcomes remain visible. Child work at the full grant is returned unchanged; completion observation, record detachment and wrapper unlink/refund each have their own subsequent admitted phase. Zero or short grants never consume a completed child's wrapper obligations.

## Evidence Is a Separate Shared Lifetime

The actual source graph is `KernelReturnInputRelease.#proof → UI Evidence{fragment, field, builder}` and `Fragment.#field → Field.#state → original source, host and activation`. Clearing `builder.#copyProof` does not release that graph. Each copied/cancelled evidence object therefore has its own charged strong registration, rather than borrowing the builder's fixed record indefinitely.

Its terminal protocol needs both the original UI observation and the original source consumer's private detachment. Only after both can the evidence clear fragment/field/builder references and detach its exact record. This is distinct from the input-copy token, raw page InputAck and semantic publication ACK. The UI owns evidence construction and terminal validation; the peer owns source close/receipt detachment. A small identity facade cannot hide an immutable `InputOwner` graph after source close. No callback-shaped or boolean witness substitutes for the concrete pair.

## Verification Plan and Active Work

The metadata selector exercises seven neutral catalogue entries using strict Ajv, BigInt field accounting, Immer conservation and real `reserveRecord` admission/refusal on one ledger. It deliberately has no installed UI shells and is not the concrete consumer gate. R1 executed one failed test at the unimplemented catalogue; R2 passed one test with 658 skipped, 659 discovered, in 6.43 seconds. R4 additionally includes hostile unknown/prototype names and a non-coercing object rejection; it passed one test with 658 skipped in 4.83 seconds. Full logs are `🧪️renderer-shared-metadata-red-r1-2026-08-27.txt`, `🧪️renderer-shared-metadata-green-r2-2026-08-27.txt` and `🧪️renderer-shared-metadata-green-r4-2026-08-27.txt`.

Canonical strict R3 exited one with exactly seven existing tutorial diagnostics and zero metadata/UI-resident diagnostics, retained in `🧪️renderer-shared-metadata-strict-r3-2026-08-27.txt`. This is not a zero-error whole-renderer claim. The static catalogue is now source-coherent; actual pool/child/evidence constructor adoption remains in progress.

Concrete adoption must additionally execute constructor/finalization and caller-loss recovery at pool/scope/payload/builder/reader/page/evidence boundaries; premature/foreign/replayed domain-witness rejection; actual install/detach-then-wrapper-throw recovery; independent old readers; source/UI evidence handoff; simultaneous pre-Open raw response, destination pages and scratch on one composition ledger; all three capacity/control axes; and bounded teardown without local counter refunds. The existing paged source/reader/fault/continuation tests remain required during cutover.
