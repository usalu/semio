# Sync Detach — Current Funded-Owner Repair Boundary

## Decision

The still-open production compiler blocker is the real `SyncSession::detach` at900: it awaits synchronous `Result<Option<Backbones>,VcsError>`. The repair must replace that ownership-returning path with the already selected original-parent, same-FIFO in-place transfer and close protocol. Removing `await`, ignoring the result, returning a new owner to an async local, or allocating an unfunded retirement Box is not the implementation.

This review makes the Store/session steps and necessary caller edits concrete, but **the funding prerequisite is not implementation-ready in current source**. Dag directly reconfirmed during this review: there is no newer mounted/reviewed RuntimeAppCell→Store private field binding or live-child release API. Resident R11 still supplies whole-root close and structural Option handoffs only. Its25 passing standalone laws do not mint the missing Store/session parent. The actionable next source boundary is the original-parent implementation plus its exact field API, followed by the Store transaction below; not another OS compile of a knowingly unjoined path.

No production/canonical test/script/metadata edit, Cargo, native process, source oracle, target change, cleanup or lease occurred. This new ticket report is the only authored file. The non-Sync compiler proposal remains ticket-only.

## Evidence And Current Compiler Attribution

Read the retained R17 proposal, full OS R1 attribution and exact detach diagnostic, R2 owned candidate, Mutation's Store ownership release and Sync84/Demo73 packets, current Store/Sync/resident/Plugin sources, and Dag's complete post-R11 parent proposal. The earlier full caller census is preserved, not rewritten.

| Boundary | Actual retained evidence | Current source status |
| --- | --- | --- |
| OS6 R1 |92 compiler errors,66 warnings,0 executed tests; original4 library blockers + intended2 Send assertions +86 test-compile diagnostics. | This remains the latest retained owner-crate compiler result. There was no post-repair OS build here, and no current error-count prediction. |
| Directory concrete runtime import | Original E0432. | Correct services-owned import is mounted at Directory482. No rework proposed. |
| Two codec erased futures / actor Send | Original two actor Send errors plus two intentional E0277 assertion errors. | Only compile_dsl/print_mirror slots and necessary thunks are source-repaired; Plugin12 local qualifications are source-GREEN. No further bound/slot/global trait change proposed and no compiler-sufficiency claim. |
| Sync helper visibility/returned closure | Original E0603 and WorkerSubmitError Debug E0277. | R2 test-only scope and exact returned-closure cleanup repair remain mounted. No additional helper edits. |
| Outer Sync84 | Original14 signature errors,68 stale-await/cascade diagnostics,2 channel trait-access errors. | Mutation's exact source-only join is accepted; no inference that all affected tests execute. |
| **Production detach** | Original E0277 at Sync900, exact `Result<Option<Backbones>> is not a future`. | Same expression remains. Ownership repair is still required before any compile claim. |
| **Sync Demo metadata** | Current base77e requires DESCRIPTORS/descriptor; current Sync3850 impl supplies neither. | Separate missing-required-items source finding after the base change, **not an observed new E0046**. Mutation73 has ticket-only source RED; no fake descriptor/default or test exclusion is acceptable. |

The actual R1 JSONL still hashes `654962ed8040bcc4fb3f693e5c827faca180e2f4a332f3532aa900476140f16e`. [Full preserved compiler evidence](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️os-kernel-six-r1-full-compiler-diagnostics-2026-08-28.md) and [R1 attribution](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️os-kernel-six-native-r1-compiler-red-2026-08-28.md) remain historical evidence, not a fresh92-error result.

The Sync Demo is distinct from Store's already direct-leaf `fixture_mutations::demo` imported at19623. Both compile under the owner-crate cfg(test), regardless of a narrow execution filter. The [Mutation73 packet](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️store-sync-demo-direct-leaf-packet-73.md) records its actual source-model results, not a native descriptor pass. Its22 constructor joins/privacy/metadata remain Mutation-owned. Do not conflate that with either Store refusal law.

## Actual Installed Owners And Gaps

| Exact current owner | Current location and required custody |
| --- | --- |
| Original Store | [Store13144](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:13144): ManuallyDrop envelope/backbone, generation/content revision, snapshot-read authority and displaced FIFO. No parent-field binding exists. |
| Descriptor | [ArtifactBackboneRef2065](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:2065) owns a separate URI String in envelope.backbone. It must move with the original Backbones; retaining just channel pointers loses this owner. Descriptor-only/stale-descriptor states must also be covered, not silently treated as empty. |
| Destination FIFO | [1489](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:1489): VecDeque<Box<dyn ErasedSnapshotRetirement>>,1024 logical capacity,8 reservation entries. Exact reservation identity is slot+checked generation+remaining count. Neither capacity nor allocator success funds its backing or a child shell. |
| Existing shell | [16897](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:16897) retains backbone, queue, message, bytes. It does not own the envelope descriptor, original charge or parent binding. Its constructor takes a whole Backbones before Box allocation. |
| Original session | [Sync866](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:866): Store by value, sender, Tokio event receiver, status. No retained detach request/reservation/phase/runner ticket. Only two lexical SyncSession::new uses,3900/3910, both receive fixtures; no actual RuntimeAppCell-owned session is demonstrated. |
| Native app cell | [Plugin27749](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:27749) still wraps a complete AppInstance in Mutex, plus maintenance state. It does not expose the proposed funded source-field association. |
| Mailbox | [Sync223](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:223): send takes the message, uses blocking poison-recovering mutex, writes slot, then invokes waker/callback before returning. Full/Bytes/Closed/Stale return the exact request. No persistent reservation or in-place source method exists. |
| Actor/channel peer | ChannelBackboneRemote holds both queue aliases; accepted Detach starts close, not an alias-release receipt. ArtifactChannels997 also owns the native runner ticket; current attach888 does not retain it in the session. |
| Canonical resident root | [resident148](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs:148) now has actual Destroy/Free/Refund/Clear. close_step232 closes the whole root/head. It cannot retire this one live Store child while the app stays live. |

Mailbox generation, host runner generation, Store generation and Plugin numeric instance ID are different identity domains. Equal values or matching URI/type do not issue authority. Keep the same original source registration captured during construction; do not derive a replacement parent during detach.

## Store Transaction — Exact Implementation Order

Reuse [Dag's chosen single-parent/single-release plan](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️runtime-opening-original-parent-funding-proposal-r1-2026-08-28.md), including its private Store-field association and pointerless ClearedAwaitingBinding residue. The names below refer to that proposed API, not newly mounted methods.

1. **Original construction and destination admission.** Actual Opening registers this Store/FIFO with its actual original app parent. A standalone SyncSession needs its own actual original registered parent; the app's document field cannot be lent to it. Fund the enlarged FIFO entry type/backing, operation state and failure/cancellation custody before any source operation. The unchanged1024/8 logical limits are not a credit source.

2. **Prepare without detaching.** Store reserves exactly one existing FIFO entry for the combined descriptor/backbone shell, with checked reservation generation and checked next Store generation. It asks the original parent for one `ResidentRecord<ArtifactStoreBackboneRetirement>`, replacing the backbone-specific Box, not wrapping it. Allocate and initialize the empty shell under separate actual grants while descriptor/backbone remain installed. The parent/FIFO binding, not a returned local, retains the pending allocation/partial shell. Empty source combinations must have explicitly declared semantics; descriptor-only ownership still requires retirement.

3. **Revalidate every commit receiver.** Validate original root/consumer/cell/closed Store selector, Store registration, original generation and exact FIFO reservation. Confirm the shell is installed and empty, all moved descriptor/enum/entry bytes fit the remaining work grant, and no producer can reacquire the source after revocation. No callback, allocation, global lookup or variable error construction belongs in the commit window. Busy/fault before commit leaves all source fields untouched and preparation retained for retry/cancel.

4. **Include the real snapshot publication authority.** [SnapshotReadLeaseRegistry::publish_authority88](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:88) uses checked sequence+2 and one CAS; it can refuse even if Store generation itself fits. `bump` currently asserts it only after mutating Store. The new commit must obtain/revalidate this same publication window **after other fallible preparation and before any take**, then perform only fixed no-fail writes and terminal publication. Sequence exhaustion, odd/busy sequence and failed CAS are pre-take refusal laws. Do not add an independent second revision authority or publish a new generation before the descriptor/backbone transition. Old read witnesses must see the old authority or reject during the prepared window, never validate a half transition.

5. **Fixed transfer into the installed shell.** Move the original `envelope.backbone` and `backbone` into that exact empty typed record, install/advance the same FIFO entry, set checked next generation and publish the unchanged content revision. Do not call `bump`16348: its cursor clone/reconciliation and unchecked increment are not bounded detach work. Keep content history/cursor and payload allocations identical; only connectivity and its existing commit authority change. A later error is a retained postcommit fault, not rollback.

6. **Typed descendants, then canonical release.** Close the exact shell in the same FIFO; aliases block. On actual typed terminal, revoke temporary record access and begin that exact bound child release into the original root's single Release slot. Busy release retains the entry. Then empty-shell destruction, node destruction, original Layout deallocation, pointerless charge, Refund, Clear, exact admission unlink/release/Clear, and final FIFO pop are distinct granted phases. Retain the binding through every Clear; no public Record alias crosses free, and no freed pointer is read to recover lost completion.

7. **Cancel according to commit state.** Before commit, cancel only prepared empty child/reservation and retain the original attached Store. After commit, close the same committed child; never restore the old channel or refund its backing early. Lost method return/unwind leaves original parent/FIFO state discoverable. Exact error/panic owners need their own funded destination; arbitrary error strings or caught payloads are not zero-byte scalar faults.

### Physical Close Is Still A Separate Defect

Current shell close16925 is not physical-byte proof: Vec::truncate(u8) changes length without scrubbing; final Vec capacity is dropped at0 released bytes; empty-but-capacious strings/vectors can survive the nonempty checks and drop with their container; queue/Arc/Mutex/channel backing and enum moves are unpriced. Port channel destruction is a generic whole drop. `take_unique_queue` also recovers poisoned Mutex contents instead of preserving a typed poison fault. FIFO1601 pops/drops the Box after child Complete without charging its backing or preserving an external receipt.

The planned shell must therefore retain byte/queue/Arc backing descriptors and initialized frontiers, preflight physical movement/release, distinguish logical payload progress from backing free, and supply exact released-byte/item accounting. No busy loop or accumulated “inspection” credit substitutes for actual work. String-to-byte descriptor transfer must be separately granted before any bytewise operation that would invalidate UTF8.

**No4096 fit is asserted.** Existing VecDeque/Vec payloads may have allocations larger than4096; the current resident release charges the whole Layout. A whole large allocation cannot be freed by fictitious chunked deallocations. Measure the original allocation/transfer/free extents, retain no-fit refusal, and review a source representation change if needed. Merely transferring a small handle or funding the small shell does not retroactively fund existing channels/messages or prove their bounded final free.

## Exact Same-Shell Entry Points That Must Not Bypass It

The owned Store release includes the backbone-specific admission/forwarding joins. These are current source facts, not authorization to rewrite unrelated envelope/decoder domains:

- `replace_backbone_retained`14177 moves old backbone before Box allocation; its incoming `next` is also a live owner. Attach15668 overwrites descriptor before this call. Require original installed incoming and outgoing sources; refusal must not drop either. This is not solved by adding a reservation argument while keeping by-value loss on Err.
- Reset13866 takes the old backbone and allocates another shell after document-root commit. Prepare its exact backbone destination before that root commit, and retain the original descriptor in the appropriate original envelope/shell owner exactly once; never duplicate or discard it.
- `close_take_backbone_retirement`14544 returns a newly allocated Box after take. CloseView1713, CursorDisposer1930 and ResolutionCandidate13405 must advance the same installed child/binding, rather than use a raw returned shell as the close protocol. Whole-root close must have its preadmitted final child destination before admission is revoked.
- `pump`16250/16276 and `flush_outbound`16289 take the backbone into an async local before receive/send and call the same replacement helper to restore it. Cancellation/early `?` can lose that source. The least source-preserving join keeps the backbone in its original field while borrowing only the needed disjoint fields; variable received/outbound values still require their own retained request/result cells. Do not label this a fully bounded IO repair merely because the backbone no longer moves.
- The selected same-FIFO representation may retain an existing erased-owner variant for unrelated retirement domains. That is not a second compatibility representation of this backbone. No old Box-backed backbone entry, raw `detach_backbone` owner return, or void forwarding should survive the authored cutover.

## Session Joint Commit And Close

Store-only transfer is not whole Sync detach. Session preparation must retain the actual Detach request, exact mailbox reservation, Store receiver, original sender/event receiver and native runner association in the original registered session **before** publication.

The current mailbox has64 logical slots and1MiB semantic message bytes; Detach's semantic count1 does not price its actual slot/enum/Arc/control backing. A reservation must debit the same authority and all ordinary senders must respect it. Do not hold a MutexGuard across resumed preparation. Reacquire all actual guards via bounded nonblocking access, validate exact mailbox generation/closed state/reservation plus Store commit window, then perform the one fixed Store+mailbox transition. Real readers must either share those actual guards or validate the existing publication frontier.

Record “request committed” in installed session state before releasing the commit window. Run no waker/callback inside it. Wake-tail work is a subsequent retained phase: a panic after queue insertion cannot erase the committed record or trigger another Detach send. Precommit Full/Bytes/Closed/Stale/Busy/Poison leaves Store/sender/events/request unchanged. Once committed, actor closure/alias release remains pending and must not be described as a refused enqueue.

Current actor close1413 drains commands/outbound before Terminal; runner close_one_terminal_owner2378 still drops whole owners and has no reviewed callback-shell terminal receipt. Retain the original runner association through the actual terminal/alias handback. Merely returning the external ticket, observing a flag, or sampling Arc counts is insufficient.

Current Tokio broadcast receiver destruction may walk unread event slots and destroy last-reader payloads; `try_recv` clones stored events. Keep the receiver structurally installed until a real typed event ownership/retirement protocol can close it. Neither `events=None`, an unbounded drain, nor retaining the sender proves that boundary. This required actor/event work is explicit, not secretly supplied by the Store shell.

## Caller Cutover And Scope

Fresh framework Rust census still finds the same authored forwarding boundaries:

| Caller | Required joined outcome |
| --- | --- |
| SyncSession888/896 | Preserve original channels during attach failure and retained detach preparation/commit/close; no async void. |
| Store SpaceHost18214 | Its original meta Store needs its own registered parent/field, not a fabricated document selector. |
| PluginApp11677, VcsArtifactApp24445 | Carry retained progress/refusal/fault and original parent context; no discarded Backbones/result or cache invalidation on refusal. |
| plugin_detach_backbone29863 | Use original retained instance/cell association; numeric lookup + resolve_ready(void)+Ok cannot report pending close as completed. |
| OS host785 | Preserve exact inner outcome instead of void discard. |
| Shell2977, ProgramBridge529 | Shell currently takes channel then ignores send/Plugin results; bridge's native exchange285 returns unavailable, so there is no proven connected end-to-end detach. Retain original shell channel/instance authority through refusal. |
| Store23795/24515 and Plugin35227 fixtures | Migrate actual callers and exact owner cleanup with the production surface, not by restoring an old compatibility API. |

Broader caller signatures and session/event ownership need root/Dag coordination before production edits. The Store-region release does not authorize modifying unrelated FreshField/FreshVcs/registry/envelope grammar. Existing complete caller details and Tokio source evidence remain in [the earlier census](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️sync-detach-original-owner-caller-census-2026-08-28.md).

## Smallest Honest Verification Boundary

No test was run here.

1. Keep the **original two real Store refusal assertions unchanged in meaning**: full actual1024-entry destination and u64 generation exhaustion must preserve descriptor, generation, revision, backbone queue identity and payload pointer/capacity/content, with exact cleanup before assertion. Their current schema explicitly labels zero/short grants, shell allocation/construction unwind, commit unwind and exact one session request as **pending**, not covered.
2. After the actual original-parent source/API exists, add separate genuine laws at real frontiers: wrong/foreign/stale parent and slot; root/FIFO exhaustion; allocation refusal; partial shell initialization; held/poisoned access; publication sequence overflow/CAS refusal; source transfer interruption; successful same-FIFO commit; live-peer blockage; physical backing zero/short/no-fit; lost Free/Refund/Clear return; repeated reuse. Measure actual allocator/byte work and preserve exact original roots before assertions. Do not retrofit a fake parent just to reach the old refusal assertions.
3. The smallest existing owner-crate behavioral selector is the two exact `backbone_detach_refusal_...` laws through current `@semio-tech/framework-os-kernel:test-native`, which already supplies --lib --features sync,ureq. Its execution filter does **not** prevent compilation of Sync Demo or other cfg(test) bodies. Once the joint source is coherent and separately authorized, the unchanged original six-law cohort also verifies Directory2+Send2; no need to re-author them.
4. The existing peer-alias law `backbone_retirement_blocks_for_live_peer_then_drains_one_owned_message_or_byte_grant`23759 is a useful distinct regression, but its counters/17-byte truncation are not physical-free evidence. Add it only under a separately named approved selection. Do not claim a whole suite, all forwarding paths or timing from these tests.
5. Session request/wake-tail/actual actor+event close require their own real-source laws and coherent funding. No such test is currently mounted in the two-law detach leaf. Missing Demo metadata and the known production detach blocker must be repaired by their actual owners, not hidden by different features, a new harness/target, broad defaults or a favorable subset.

The existing runner remains jobs2/master target/3600000 build ceiling/coverage0/exhaustive/no-fail-fast on explicit future GO. This document is not a command/capture manifest or request to execute.

## Current Hash Observations

One fresh shasum observation, no immutable source hold or complete compiled closure. All17 selected reads succeeded; no source writes occurred. Current Plugin51fb/builderfc2/base77e are preserved.

```text
7c71a7bf09b8bac3fbfd8b420b98f3a82ae89d62ebd0c868f5e6e97d8bffc2c4  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs
62f31952ccdc84de0b2d6e63e39374ae1baedaec0f7304ff926836dd203806e6  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs
e23ec4068c261ef56020e4aaafd97e3bd304a6503a58e9dc1b7a3c6de576dbd3  🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs
56208e0ddbd792fe1351d69a920f3c0472a929841524bd656fe5342fc45816ab  🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs
51fb521e81bd6577fc05a912c8d312c5d89c1dcd5fe5ab7da56a4f530ea834ac  🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs
fc2e46b8755fbe08dfcb2690e0a494d77086d1d1aedbc36832a1a9f571d9e6aa  🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs
77e8205a60377a7170fb45276e31b0084ca943385e52fc360904765400cbb1fc  🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs
53e1044cceec42907c6e741230279aa439b806d6bb232c4ef592580c3cd90211  🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs
527aae4b56941a85798cca575f5556f7f4799f5fdb62d6c5bbbd73f17d548ae7  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs
3c472aa946e552b9c19d7fdfec697475e45b9b3a5aaca7695b52066abd0a7edf  🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs
8bce255b2c9e184249c90970d7af33de638008d34e9da8b1eb28596ddbcd7445  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts
79ca4a57c184a41e4d589260322624243fb894e5831bdb3f2431ca0978789c79  🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json
88492fcc126c50628d67ebbd80e39bd766c1204729d0ef987c60dfecacddfc65  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/✂️detach/🧪️tests/🦀️.rs
7620dffca847c9ff585cfc0bff838006a65c3e01118b9ed7e36e33e4ddf46079  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/✂️detach/🧪️tests/🔣️.json
6e727ae7851d23a2606f8b1bb5750c4588da05c899a2d6cfac3d82d59f096a3a  🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/✂️detach/🧬️schema/🔣️.json
654962ed8040bcc4fb3f693e5c827faca180e2f4a332f3532aa900476140f16e  /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️os-kernel-six-r1-compiler-diagnostics-2026-08-28.jsonl
930a3cf899e01a4e776e0d2adfe44fb4b7219c37da35fea07f8784c6526b9004  /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️runtime-opening-original-parent-funding-proposal-r1-2026-08-28.md
```

Dag's same-parent proposal SHA930a3cf8 remains unchanged. His direct message confirms no private binding/live-child release source has landed; current neutral-anchor/Opening work is ticket-only. This report supersedes only stale claims that resident Free→Refund itself is still unfixed: that standalone R11 implementation is GREEN; **the live Store binding, targeted release, session ownership and physical descendant close remain open**.

