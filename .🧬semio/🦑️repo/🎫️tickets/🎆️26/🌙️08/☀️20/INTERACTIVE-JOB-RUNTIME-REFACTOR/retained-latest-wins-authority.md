# Exact Latest-Wins Publication Authority

## Source Boundary

Ordinary non-keyed operation admission retains its existing document supersession policy. Explicitly keyed factories now use separate exact operation slots beneath a document cancellation scope. The factory callback borrows its target from the concrete command; the command and raw-wire owners remain retained before worker construction. Missing exact command-retirement authority denies keyed admission.

## Schema-First Gate

Plugin `🧪️tool-latest-wins.schema.json` and `🧪️tool-latest-wins.json` define six exact scope comparisons: same target supersedes, while a different widget, app instance, document, controller, or tool remains independent. Every target is an actual8192-byte String, including equal-prefix/distinct-final-byte identities. Five publication cases distinguish cancellation during producer/preparation/preflight/publishing from cancellation after commit while awaiting ACK. No target length cap was added.

## Exact API And Retained Admission

The registered `ArtifactOwnedToolJobFactory` supplies `latest_wins_target(&Owner::Command) -> Option<&str>` and `build_latest_wins_command_disposer() -> Option<Box<dyn ArtifactOwnedDisposer<Owner::Command>>>`. Registration retains compiler-bound callbacks alongside its exact owner/factory/key/schema contract. The runtime calls the selector on the already-owned concrete command before handing the command to the job builder; apps receive no mutable cancellation map and cannot mint commit authority. Procedural/Flow executors received this exact API.

The active dispatcher retains keyed commands in a fixed-capacity FIFO. It copies exact instance/document/controller/tool/target identities into an injective length-prefixed hexadecimal key with byte-accounted pages, then uses the owned OrderedMap bytewise update cursor. There is no hash identity or whole-string comparison. Supersession occurs only after exact map admission; a displaced scope is cancelled through its private publication gate. Root changes during key preparation cancel and retire that cursor, then rebuild against the current root. Only accepted key admission captures fresh job snapshots and starts the worker. A bounded rank scan removes entries after their exact operation claims expire; map cursors, displaced roots, final key bytes, cancelled commands, and raw pages have explicit close paths.

## Publication Linearization

The publisher is synchronous and holds private app/document/operation CAS claim permits for exactly one publication unit. Cancellation sets a distinct bit before cancelling the private token. Cancel-first prevents a claim; claim-first permits that already-linearized unit, while the cancellation bit survives permit release and blocks all later units. Producers receive only child worker cancellation tokens, never writable publication-root tokens. A worker Cancelled outcome is promoted through the runtime gate before publication.

Cancellation before commit begins retained candidate close and preserves visible roots. Unconsumed completion ownership transfers into the mounted publication before a cancelled result is queued. Cancellation after commit preserves the existing receipt and ACK rather than undoing the mutation. Exhausted transport retries transition the retained page to a fault; its exact ACK admits bounded pending-owner close.

## Gates

Four new native tests are source-ready and queued by the coordinator under `retained_latest_wins_`:

- Six8192-byte key cases use the real byte-copy/map registry and an independent serde scope-equality oracle.
- Producer-child cancellation cannot mutate the publication root; document/app cancellation invalidates subsequent CAS claims.
- Five Presence cases call the actual mounted publisher at each preparation/publication/ACK boundary.
- Ten Document cases publish a real scalar count mutation (0→42) through the actual publisher: five cancellation boundaries, each with ordinary or exhausted transport ACKs. They assert root identity, count, generation, and bounded terminal close.

Root `toolJobLatestWinsSelfTests()` adds strict Ajv schema/equality checks and hostile removals of active FIFO/admission/rebase/cancellation/claim obligations. Browser transport source assertions now require admission-aware host retirement, exact owner restoration on refusal, and terminal-empty validation before release.

The coordinator's native r4 gate passed all four tests with zero failures and414filtered (44.55seconds compilation,0.06seconds execution). Runtime DEBUG output confirms all six8192-byte scopes, five Presence boundaries, and ten Document/ACK cases reached exact bounded close with the expected count/generation/root identity. The retained evidence is `🧪️coordinator-latest-wins-native-r4-2026-08-27.txt`. The full mounted factory→keyed admission→worker→document publication scenario is not yet covered by a dedicated end-to-end fixture; the native key-registry and actual-publisher gates are separate. Atomic multi-child publication remains fail-closed and is not completed by this work.

## Coordinator Review Follow-Up

The first native latest-wins compile stopped on15namespace errors: this package aliases the kernel as `protocol`, so its ordered owner is `semio_framework_os_kernel::ordered`, not `protocol::value::ordered`. Those references were corrected through one explicit local alias. No tests ran in that attempt. The coordinator is rerunning the four native gates while plugin Rust source is held stable.

The browser worker gate passed32tests in two files after the admission-aware retirement source assertion was corrected and four hostile removals were added. This is native TypeScript transport/source evidence, not a fresh Wasm build.

New language-neutral `🧪️tool-latest-wins-integration.json` and schema establish the next required gates before implementation: exact same/different targets through registered dispatch, cancellation-key rebinding after a root rebase, reserved modulo-colliding operation slots,65sequential completed targets through a64-slot live registry, and a long-running first worker that cannot starve a ready second publisher. These gates are now wired to the expanded native packet; execution remains pending.

The planned correction retains one actual operation-slot reservation throughout pending admission; refreshes both the lease key and fixed cancellation entry before fresh worker handoff; scans at most one immutable map rank per maintenance/admission turn before admitting into a full map; and advances publication/result metadata cursors round-robin. Existing maintenance stage17 already performs key retirement, so the capacity issue concerns admission racing ahead of that maintenance, not an absent cleanup route.

## Native Shutdown Failure And Exact Fixture Repair

The coordinator's second native attempt compiled the plugin tests, then aborted while unwinding the Presence boundary test. The exact third reproduction established the primary fault: `interactive-job.close-owned-disposer-missing` for `draft-store`, followed by the strict Store destructor panic. The retained log is `🧪️coordinator-latest-wins-native-r3-2026-08-27.txt`. This was not a successful cancellation gate.

Source inspection also found that the constructor had no draft-owner installation hook. The repair adds an explicit default-None draft MemberStoreOwners hook to ArtifactApp and Editor forwarding, installs only the returned exact owner at construction, and supplies TestApp's zero-state draft catalog and cursor disposer. TestApp now also supplies exact NoPresence and NoTransient shell disposers; the Presence disposer refuses any nonempty peer roster rather than assuming all presence stores are empty. Document/config/interaction retain their existing cursor owners. No close or destructor guard was weakened, and the tests still close the entire app under one-item/4096-byte grants.

The fixture repair passed the coordinator's four-test native r4 gate. It does not finish real app shutdown: CAD, Flow, Procedural2d, and Procedural3d must explicitly install their draft and ephemeral domain owners. A shared generic or zero-state default cannot substitute for those owners, particularly when a peer roster is populated.

## Expanded Coherent Packet Awaiting Native Verification

The plugin now reserves the exact operation slot throughout pending keyed admission and checks that reservation before transferring to a mounted worker or retained rejection. Ordinary dispatch cannot take a modulo-colliding pending slot. Freshness restart updates both the private lease key and the registered cancellation entry under one nonblocking lock; a contended rebind retains the pending owner and retries later. Worker construction asserts the refreshed revision/generation witness.

A full live-key map first performs a bounded rank scan, one metadata unit per turn, so expired claims are reclaimed before new-target capacity denial. Publication and result-page selection have independent round-robin cursors; an earlier Worker does not prevent a later Publishing owner from advancing or presenting its ACK page.

Contended lease finish now marks a preadmitted finished bit in the existing private publication claim before any lock attempt. The live maintenance stage18 visits one cancellation slot and removes only a finished exact owner, updating its document and global counts. Keyed immediate release checks both the complete operation key and Arc claim identity, so an old lease cannot delete a replacement. Tests hold the mutex through consuming finish and through actual mounted retirement, then exercise the one-slot live maintenance release.

The new KeyedTestApp registers an actual factory, typed proof, and migrated manifest command. Its retained raw-page worker owns a bytewise command disposer, captures the fresh scalar snapshot, and publishes a real count edit through the existing active executor. Two cases dispatch same/different targets, reserve their operation slots, change the Store while the second key is pending, verify exact lease rebind and target supersession, assert the worker used the fresh count, and close the whole app under one-item/4096-byte grants. The complete filter contains ten native tests. Coordinator r5 executed all ten: nine passed and the registered-dispatch fixture failed constructor validation because its manifest omitted a required window kind. The fixture now declares its actual edit mode and main window using the normal builder; no validator changed. R6 execution is pending. Scoped diffcheck passed.

## Empty Raw Allocation Release Is Not Whole-Helper Lifecycle Approval

The shared retained-command close previously required `maximum_bytes >= raw.capacity()` after the initialized length reached zero. A16KiB or64KiB reserved buffer therefore never closed under the production4096-byte grant. Schema-first `🧵️retained-command/🧪️fixtures/🔣️raw-allocation-close.json` separates initialized byte retirement from the final empty allocation: bytes drain in bounded pages, then one allocation item releases the zero-length buffer with zero semantic byte credit. The actual job close test exercises length0/capacity16384 and length8193/capacity65536 against serde output; root verification also compares initialized extent with Node Buffer and validates the strict schema.

The helper's later `retire_one!` macro still drops arbitrary emit/ephemeral/command/snapshot/config/history/interaction/context values as a claimed one-item/zero-byte unit. That is not a correct generic bounded domain lifecycle and was not repaired by the allocation fix. Dedicated Flow/Procedural parameter jobs must retain exact typed disposers; no whole-helper or full-app lifecycle approval follows from this packet.
# Registered Dispatch Native Iteration — 2026-08-27

Native r7 found a real premature terminal return in pending key-copy retirement. Completing the nested copy now returns one metadata progress unit; the enclosing key authority remains owned for its subsequent turn. Native r8 exposed the same false-terminal pattern in the exact synthetic worker's raw-input close; that fixture now retains its command/completion until their own close turns.

Native r9 reached the mounted Store publisher and rejected the rebased candidate with `one-item publication requires preinstalled fixed applied and revision capacity`. The cold `bump` path rebuilt its persisted cursor using `Vec::clone`, which discarded spare admitted capacity. The cold reconstruction now preserves the existing runtime vector capacity; the interactive commit still refuses unreserved capacity and never allocates a larger publication grant. A direct Store regression plus the actual registered dispatch fixture cover cold edit followed by retained publication. Those new native tests are pending the returned compiler lease.

Pending-key close now charges exact UTF-8 scalar bytes and respects zero-item/subscalar grants. Lost reservations cancel admission and retain its rejection until a vacant result slot exists; a foreign reservation is never overwritten. The registered-dispatch fixture now includes both missing and foreign reservation cases plus the seven-byte `aä🧵` retirement oracle. Node Buffer and strict Ajv validate the language-neutral fixture. Canonical Nx source self-test executed successfully with 798 checks (`🧪️member-latestwins-followup-selftest-2026-08-27.txt`); this is not a native execution claim.
