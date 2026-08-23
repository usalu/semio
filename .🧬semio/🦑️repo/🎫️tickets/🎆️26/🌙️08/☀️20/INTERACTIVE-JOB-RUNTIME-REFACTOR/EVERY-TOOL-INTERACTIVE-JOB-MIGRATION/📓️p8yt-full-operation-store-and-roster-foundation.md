# P8yt Full Operation Store and Roster Foundation

Date: 2026-08-22
Verdict: **PASS only for the fail-closed immutable-root/retirement foundation described below. REJECT for production peer ingress, the typed full-operation route, app/instance terminal disposal, and Phase 8. Production activation remains 0/884.**

## Scope

This packet continued the shared framework/VCS foundation without editing concurrent Layout, Diagram,
browser-worker, UI-contract, or domain-cohort source. It used no Cargo/native/Wasm command, modifying
Git command, or ticket lifecycle operation.

The packet keeps `require_complete_tool_operation_pipeline` before typed-command preparation. None of
the nine owner-local proof rows is activated. Reserved routes and importers remain fail closed.

## Implemented shared store authority

### Exact child snapshot retirement

The store now exposes a required, no-default retirement contract:

- `ErasedSnapshotRetirement::close_step(maximum_items, maximum_bytes)`;
- required `terminal_is_empty()` witness;
- `SnapshotRetirementRejected`, which returns the exact erased snapshot owner on rejection;
- `SnapshotRetirementFactory<P>`, installed exactly once on an `ArtifactStore`;
- required `SpaceMember::retire_snapshot_read_erased`, with explicit forwarding in shared member
  implementations and generated member enums.

The plugin's `ChildContentRetirement` transfers one child snapshot at a time to its exact live member,
propagates `Blocked`, rejects over-budget/lying terminal owners, and removes a retirement only after the
terminal witness. Current and retired child roots are held in fixed registries and pumped by both live
maintenance and app close.

The machine-readable current cohort inventory is
`📊️p8yt-child-snapshot-retirement-cohorts.json`: one production `SemioMembers` cohort with 18 variants,
plus the exact fixture cohort.

### App-typed presence immutable root

`PresenceStore` now publishes an actor-sorted fixed 64-entry `Arc<PresencePeersRoot<P>>` instead of a
resizable string-key map. `PresencePeersPublication<P>` owns a complete candidate and:

1. prunes at most one stale entry per step;
2. adopts one already-decoded peer snapshot per step;
3. retains every created and displaced entry under exact ownership;
4. releases candidate-only references one per step before commit;
5. commits the immutable root by one swap and returns an exact `PresencePeersRetirement<P>` for all
   displaced roots;
6. cursor-disposes aborted candidates, then invokes the domain-owned snapshot disposer for each
   created snapshot;
7. reports terminal only when candidate, created, displaced, and nested disposer owners are empty.

`PresencePeersView` replaces the reducer's former `Vec<(&str, &P)>` projection. Typed command capture
retains one root `Arc`; app code may iterate the already-sorted root lazily.

### Retained peer roster candidate

`PeerRosterPublication<A>` is an admitted fixed-registry owner for generic peer metadata and app-typed
presence:

- raw roster ownership remains in one `ManuallyDrop<Vec<Vec<u8>>>` capped at 64 entries;
- one raw entry of at most 4,096 bytes is decoded per maintenance step;
- metadata is inserted into an actor-sorted fixed root, detecting empty/oversized/duplicate actors;
- app-typed packs are retained in 64 fixed slots, pruned against metadata, decoded one per step, and
  applied to `PresencePeersPublication<A::Presence>`;
- candidate publication revalidates exact generation plus both fixed retirement slots, commits the
  typed root and metadata root in the same non-suspending turn, and transfers displaced owners into
  their bounded retirement registries;
- fault/close drains raw entries, typed packs, metadata entries, rejected decoded snapshots, and the
  app-typed candidate under item/byte grants before terminal removal;
- live maintenance and app close both contain exact cursors for this owner.

The source deliberately does **not** route `PluginApp::adopt_presence` or `plugin_exchange` through this
candidate yet. `decode_app_command` already constructs the owned roster, and a saturated app registry
has no lossless bounded rejection owner at that outer channel boundary. Returning `Err` would
synchronously drop the rejected `Vec<Vec<u8>>`; silently leaking it or acknowledging an unadmitted
roster would be invalid. The current synchronous `from_peers`/typed adoption path therefore remains
the verifier's explicit RED seam until the channel command itself is admitted into a fixed owner before
decode.

## Compiler defect repaired

The newly introduced `snapshot_retirement_factory` field had accidentally been declared on
`TransientStore` while `ArtifactStore` constructors, accessors, clone candidates, and `SpaceMember`
retirement used it. This caused the concurrent Nx probe's E0560/E0609/E0063 errors and a dyn-factory
`Debug` bound error. The field now belongs to `ArtifactStore`; `TransientStore` is restored to its
original field shape. Both shared Rust files parse and format with `rustfmt`.

No Rust type/build gate was run after this repair because the coordinator explicitly retained the
no-Cargo policy and the shared target directory had been removed. Compile success is not claimed.

## Static verifier

`verify interactivity tool-jobs` now requires the child retirement terminal contract, exact child
cohort inventory, immutable peer/app-presence roots, retained per-entry roster publication, and no
whole-roster reducer projection. Its adversarial fixture rejects synchronous whole-roster publication.

Permanent self-tests: **85 clean**.

The full gate remains intentionally RED because production ingress still contains
`PeerPresenceRoot::from_peers`, whole roster validation, `presence_store.peers()` materialization, and
the pre-admission channel decode path. The gate does not award credit for the unconnected foundation.

## Deterministic ledger

The following byte-canonical ledgers were generated twice and compared with `cmp`:

- `📊️p8yt-tool-jobs-ledger-a.json`
- `📊️p8yt-tool-jobs-ledger-b.json`

They are byte-identical.

| Inventory | Count |
| --- | ---: |
| Macro hosts / invocations | 50 / 50 |
| Macro rows / unique rows | 775 / 773 |
| Literal registrations | 656 |
| Complete admitted operations | 0 |
| Production factories / registrations | 11 / 0 |
| Typed dispatches / aliases | 3 / 4 |
| Remaining live commands | 884 |
| Framework-reserved routes | 8 |
| App-owned importers | 35 |
| Global payload stores | 34 |
| Verifier self-tests | 85 |
| Failure classes | 14 |

The 14 exact failure classes are preserved in the JSON ledger. In summary: reserved jobs, importer
pre-serialization, typed preparation/commit, incomplete persistent command authority, peer/interaction
roots, media output, runtime instance close, app deep close, reactor registries, 34 globals, final
freshness, 8 reserved routes, 35 importers, and all 884 live registrations remain rejected.

## Gates executed

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2024` on shared store/plugin source | PASS: parse/format clean |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: `self-tests=85 clean` |
| `bun ./📜️script.ts verify interactivity` | PASS: DENY clean; recorded allowlisted test-only bridge only |
| full tool-jobs JSON generation | Expected RED: 0 admitted, 884 remaining, 14 failure classes |
| second JSON generation plus `cmp` | PASS: byte-identical |
| scoped `git diff --check` | PASS |
| Cargo/native/Wasm/browser/runtime timing | Not run; no pass claimed |

## Exact shared files

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `/Users/ueli/Documents/semio/📜️script.ts`
- `📊️p8yt-child-snapshot-retirement-cohorts.json`
- `📊️p8yt-tool-jobs-ledger-a.json`
- `📊️p8yt-tool-jobs-ledger-b.json`
- this report

## Mandatory residuals

1. **Outer presence ingress ownership.** Admit the encoded `AppCommand::Presence` into a fixed,
   saturation-safe owner before whole command/roster decode. Only then replace the synchronous
   `adopt_presence` route with `PeerRosterPublication<A>`.
2. **Untracked snapshot readers.** Public `SnapshotRead<T>: Clone` and `ErasedSnapshotRead: Clone`
   remain unregistered Arc capabilities. A clone can outlive retirement and later become the last
   deep owner. Global bounded retirement requires scoped/registered read leases with exact handback,
   or definitionally bounded/paged snapshot roots. The current factory alone is insufficient.
3. **Generated member factory bypass.** `space_members!` still generates public
   `MemberFactory::{create,open}` through `create_member_store/open_member_store`; domain wrappers can
   be bypassed by UFCS. The shared macro must require an owner-supplied retirement-factory installer
   for every generated create/open arm, with no default or optional bypass.
4. **Interaction roots.** Interaction state capture must use its immutable store root; hover,
   topology, and peer projections still need event-maintained fixed roots plus bounded retirement.
5. **History and publication.** Command history remains a rebuild/materialization boundary.
   `dispatch_emit` still needs a cursorized candidate and one revision/generation/cancellation-validated
   publication turn.
6. **Typed operation disposal.** `ActiveToolCommand` needs exact cancel/close/terminal-empty stages;
   ordinary deep app/store Drop paths remain rejected.
7. **Independent validation.** After all shared owners stabilize, one serialized Cargo/Nx owner must
   typecheck and run focused Rust tests, then exercise real worker watchdog, saturation, stale commit,
   app close, and browser/Wasm paths.

## Audit readiness

The child and retained roster foundations are ready for independent **source** audit. They are not
ready for compile/runtime acceptance, and the production peer route is intentionally not connected.
The typed full operation and Phase 8 remain RED.

---

## 2026-08-22 — Outer Presence ingress follow-up

This follow-up replaced the reachable synchronous roster route with the deepest source-safe retained
pipeline that the current reactor ABI can support without pretending finite storage is infinite.

### Landed

- `AppCommand::Presence.peers` is a fixed 64-slot, 262,144-byte `PresenceRosterWire`; it is no
  longer `Clone`, returns an exact rejected entry on item/byte saturation, and preserves FIFO order.
- `PresenceCommandCursor` owns the encoded command after a header-only tag/sequence parse. Production
  `plugin_exchange` reserves the app publication, result, and retirement identities before cursor
  construction. One entry of at most 4,096 encoded bytes is copied and decoded per maintenance step.
- Generic public `decode_app_command` now rejects tag 28. Presence is constructible only for encoding;
  interactive decoding must enter through `PresenceCommandCursor`.
- Malformed command bytes after reservation are transferred into a faulted
  `PeerRosterPublication`; they are not explicitly dropped by the exchange callback. Malformed peer
  and typed-pack decodes become observable retained faults and use the same bounded close lane.
- Publication order is exact generation order. Candidate metadata insertion is actor-sorted and
  rejects duplicate/oversized actor identities. Release code validates reservation, processed
  generation, cancellation, result slot, both retirement slots, and the app-typed retirement
  factory before producing a private `ValidatedPeerRosterCommit` authority.
- The app-typed commit and metadata/color root are published in one non-suspending exclusive app
  turn. Displaced typed and metadata roots are transferred to their fixed retirement registries.
- The old `PluginApp::adopt_presence`, `PeerPresenceRoot::from_peers`, and public
  `PresenceStore::{adopt_peer,remove_peer,expire_peers,peers}` mutation/materialization escapes are
  removed.
- Channel source tests cover fixed maximum/+1 exact rejection, FIFO, generic-decoder sealing, and
  grant-size cursor progress. The verifier has six additional adversarial fixtures and reports
  **91 self-tests clean**.

### Honest acceptance blockers

This follow-up is still RED. Three related ownership gaps cannot be closed locally without changing
the reactor/host owner-return contract:

1. When all 64 pre-admitted app slots are occupied, `plugin_exchange` still owns the incoming
   `Vec<u8>`. The current `poll_kernel -> TurnResult` ABI has no rejected-event/owner-return arm and
   no persistent ingress task. A finite in-app overflow queue only moves the loss to its own `+1`.
   The required next seam is an explicit `PresenceIngressOwnerReturn` (or a typed persistent reactor
   ingress task) that returns/retains the exact current command and untouched tail in FIFO order.
2. The current ABI supplies one contiguous `Vec<u8>`. `PresenceCommandCursor::close_release` can
   account logical byte removal in <=4,096-byte grants, but `Vec::truncate` does not release backing
   capacity and final deallocation still releases the original allocation at once. A truthful hard
   boundary needs paged command ownership at or before WIT/reactor lifting, not accounting theater.
   The verifier intentionally rejects the present `truncate` implementation.
3. `decode_presence_peer` receives <=4,096 encoded bytes, but its nested field/count allocations are
   not independently cursorized. The source cap is recorded, but runtime <8 ms evidence and hostile
   zero-length/high-count payload coverage are still required. No timing PASS is claimed.
4. Terminal faults are retained in the ordered app outcome registry and drained by the next
   `plugin_exchange`, but channel v12 has no unsolicited app-frame outbox. A caller that sends no
   later event is not guaranteed to observe the late fault. The reactor owner-return/progress event
   seam must carry this outcome as well as the rejected command owner.

Release-build unexpected `Drop` is therefore not accepted: the cursor has a terminal assertion, but
an incomplete contiguous owner can still be released by ordinary field destruction. This is kept in
the same verifier failure class rather than being described as complete.

### Follow-up gates

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` channel | PASS |
| `rustfmt --edition 2021 --check` shared plugin | Source parses; formatting check remains RED on pre-existing/concurrent style deltas in the monolithic module |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | PASS: `self-tests=91 clean` |
| `bun ./📜️script.ts verify interactivity --format json` | PASS: DENY clean, one recorded test-only bridge |
| full tool-jobs JSON | Expected RED: 0 admitted / 884 remaining; Presence owner-return/paging stays an explicit failure |
| `bun ./📜️script.ts verify diff --format json` | RED on the repository's existing dependency-cruiser violations; not attributed to this packet |
| Cargo/Nx/native/Wasm/runtime timing | Not run by coordinator instruction |

Follow-up evidence files retained in this ticket:

- `p8-presence-current.json`
- `p8-presence-current.stderr`
- `p8-presence-deny.json`
- `p8-presence-self-test.txt`
- `p8-presence-diff.json`
- `📊️p8yt-presence-ledger-a.json` and `.stderr`
- `📊️p8yt-presence-ledger-b.json` and `.stderr` (byte-identical JSON via `cmp`)

Global snapshot read capability/factory ownership, full typed prepare/job/commit, and all 884 command
activations remain fail-closed. This packet is ready for source reattack only, not independent runtime
acceptance.

---

## 2026-08-23 — Fixed-page retained command ABI checkpoint

This checkpoint supersedes the contiguous-`Vec<u8>` ingress residuals above at the source boundary.
It does not change the global RED verdict.

### Landed source authority

- Reactor WIT `poll` accepts at most one `command-ingress-page`. The page has a fixed canonical
  shape: declared `length`, cursor authority, and 64 named `command-page-block` records of eight
  `u64` words each. It contains no dynamic `list<u8>` field.
- Guest lifting validates declared length at 4,096 bytes, reconstructs directly into
  `FixedCommandPage { bytes: [u8; 4096], len }`, and rejects nonzero padding beyond the declared
  length. No post-lift page `Vec` allocation or contiguous-to-page conversion remains on Presence.
- Presence page meaning is carried by trusted cursor authority (`kind=28`, `item_count`, color
  metadata). One raw peer is one fixed page; a zero-peer roster owns one explicit zero-length page.
  Presence no longer uses generic fixed-offset `byte_at`/`copy_range` logic.
- `CommandPageSet` and `PagedCommand` own `VecDeque<FixedCommandPage>`, with fallible exact 64-slot
  reservation before construction. The channel writer fills `[u8;4096]` pages directly.
- `CommandEnvelope` uses numeric `u32` instance authority. `CommandEnvelopeSet` fallibly reserves
  its exact 64 command slots and incrementally enforces aggregate 64-page/262,144-byte credits before
  `CommandBatch` construction.
- `CommandBatchDriver::next_page` returns a fixed page, not a newly allocated `Vec`. Host ACK releases
  one exact page. A zero-length accepted page is distinct from absence and can reach terminal.
- Generic transport now admits 1..=64 ordered pages. Nonterminal pages must be exactly 4,096 bytes
  and each page receives `PageAccepted`; application decode/dispatch remains RED as described below.
- Presence publication terminal success and terminal fault are distinct. Terminal and pending
  cursors use the exact one-past-last-accepted identity expected by the retained host driver.
- Guest contention retains `ReservedPresence` and `PendingPresencePage` fixed owners and returns
  `CommandPending`; it no longer explicitly drops a rejected lifted `Vec`.
- MCP, native run, and WGPU producers construct pre-admitted numeric command envelopes and drive one
  fixed page per reactor turn. MCP keeps its driver in the pending-exchange registry.

### Permanent adversarial evidence

Kernel fixtures now cover:

- zero-roster empty-page ACK followed by terminal completion;
- malformed first and malformed middle Presence outcomes, retaining the untouched FIFO tail for
  one-page-at-a-time close;
- generic two-page ACK ordering and terminal completion;
- nonzero fixed-page padding rejection.

The static verifier adds a separate paged-ingress contract and rejects dynamic WIT lists,
`VecDeque<Vec<u8>>` page owners, generic one-page-only admission, terminal Presence fault reported as
success, and command drivers without terminal-close registries. The allowed gate result is:

| Gate | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: `self-tests=96 clean` |
| `rustfmt --edition 2021 --check` across kernel/channel/reactor/plugin/host/MCP/run/WGPU | All files parse; formatting check remains RED pending a serialized write over shared monolithic files |
| Cargo/Nx/native/Wasm/runtime timing | Not run by coordinator instruction |

### Exact remaining RED boundary

1. WGPU and native run still keep `CommandBatchDriver` in a cancellable async stack frame, not an
   admitted live terminal-close registry. MCP retains a driver, but its registry and enclosing app
   destruction still need terminal-empty enforcement. Caller cancellation, worker shutdown, and
   registry saturation can therefore still bypass one-page close.
2. Generic application dispatch still calls `PagedCommand::contiguous_if_single_page()` in
   `plugin_exchange`; a multi-page command is returned as retry forever. Acceptance requires a
   retained field-by-field generic decoder over fixed pages. Concatenation followed by the old whole
   decoder would be envelope theater and is explicitly not accepted.
3. `CommandPageSet`/`PagedCommand` final semantic validation scans at most 64 fixed descriptors in
   one call. The data is no longer copied or dynamically paged, but hard 8 ms runtime evidence is not
   claimed.
4. Native actor transport serializes one fixed page as a length-prefixed bounded tuple. It does not
   aggregate a command, but the encode/decode turn still needs runtime timing evidence.
5. `PeerPresenceEntryRetirement`, `PeerPresenceRootRetirement`, `PeerRosterPublication`, and related
   typed presence retirement shells still have debug-assert/no-op incomplete `Drop` behavior. Their
   production registries pump normal close, but unexpected destruction is not structurally accepted.
6. Global snapshot clone/factory ownership, full typed prepare/job/commit publication, reserved
   routes, importers, and all 884 command activations remain fail closed.

Verdict: source foundation progress only. It is ready for focused source reattack, not independent
runtime audit or Phase 8 acceptance.

---

## 2026-08-23 — Retained Generic Decode and Caller Close Checkpoint

This checkpoint supersedes the generic-decode and stack-local caller residuals in the preceding
section. It remains a source-only RED foundation checkpoint: no route is activated and no runtime
timing claim is made.

### Persistent generic decode and exact batch storage

- Generic commands now enter `PagedAppCommandDecodeCursor`, which owns a `PagedCommandReader` and
  advances one header, bounded field, or semantic projection stage per guest turn. It never
  concatenates pages or calls the former whole-command decoder. Malformed/truncated/trailing fields
  move into exact bounded close ownership.
- `CommandBatch` no longer nests `CommandEnvelope`/`PagedCommand` allocations. It stores one
  pre-reserved `VecDeque<CommandBatchEntry>` of Copy descriptors and one pre-reserved
  `VecDeque<FixedCommandPage>` arena. ACK and cancellation remove one real fixed page; a terminal
  descriptor with no pages is removed with zero reported bytes.
- `CommandDriverRegistry<CAPACITY>` uses fixed direct slots, exact key+generation validation, and
  explicit Active/Suspended/Closing authority. Every caller marks the owner Suspended before an
  await, so a cancelled/error future leaves the exact driver on the close list rather than in its
  stack frame.
- `RejectedCommandBuildRegistry` retains the exact partial admitted arena plus the rejected current
  command. Native run and MCP pre-admit singleton close capacity and pump one page per retry. WGPU
  transfers rejected builds to a worker-owned `CloseRejectedCommandBuild` request and requeues a
  `MaintainCommandOwners` ticket until terminal empty.

### Caller-specific closure

- Native run owns `CommandDriverRegistry<1>` and `RejectedCommandBuildRegistry<1>`. Previous
  cancelled/suspended or rejected construction authority is pumped before another passthrough
  command can be admitted.
- MCP owns the same singleton command/build registries plus `PendingExchangeRegistry<1>`. It no
  longer retains `Option<Result<AppFrame, Fault>>`: `PendingResponsePage` admits one exact
  `FixedCommandPage` (4,096-byte maximum), distinguishes success/guest fault/oversize/duplicate,
  and closes the fixed page before a cancelled exchange shell is removed. App-frame decoding occurs
  only on the terminal successful take; larger responses fail closed.
- WGPU state owns an active and queued singleton driver registry plus the rejected-build registry.
  Instance destroy marks matching active/queued authority Closing. The worker request loop drives
  one page per `MaintainCommandOwners` request, including a suspended future failure and a rejected
  producer build, instead of discarding either owner at a return boundary.

### Permanent adversarial source evidence

New Rust fixtures cover:

- same direct-slot collision returning the exact rejected driver;
- suspension followed by one-page-at-a-time close;
- stale generation rejection before direct-slot reuse;
- a 64-command shallow batch arena whose page and descriptor elements have no destructor;
- terminal fault after the last page ACK, proving the empty descriptor shell closes with zero
  fabricated released bytes;
- rejected-build singleton saturation returning the exact colliding owner and draining one exact
  page;
- MCP fixed response close grant, oversize fail closure, and duplicate response rejection.

The static verifier now also rejects nested command-owner batches, callers without suspended
authority, rejected-build owner drops, retained deep MCP `AppFrame` results, and WGPU command owners
without queued worker maintenance.

### Exact gates and census

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` on kernel, run, MCP workspace, and WGPU glue | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: `self-tests=112 clean` |
| `bun ./📜️script.ts verify interactivity` | PASS: DENY clean; one recorded test-only blocking bridge |
| `git diff --check` scoped and whole shared tree | PASS |
| full deterministic JSON ledger | Expected RED: 15 failure classes, 50 hosts, 50 invocations, 775 rows, 656 literal registrations, 0 admitted, 884 remaining, 8 reserved routes, 35 importers, 34 globals |
| Cargo/Nx/native/Wasm/runtime timing | Not run by coordinator instruction |

Exact JSON evidence is retained as `p8-caller-registry-verifier.json`.

### Honest remaining caller/runtime blockers

1. WGPU `KernelRequestQueue` is now a non-growing 64-slot direct ring. Admission uses `try_lock`,
   returns the untouched request on contention/capacity failure, and checks aggregate 64-page and
   262,144-byte command credit before insertion. Its shutdown cursor releases one real queued
   command/rejected-build page per grant. `CreateAppRequestOwner` releases path/plugin/app roots one
   field per grant. The production event path admits exactly one `QueuedKernelEvent::SurfaceVisible`;
   unsupported/whole event batches transfer to `RejectedKernelEvents` and the worker releases one
   event per maintenance turn. The production runtime still lacks a final queue-shutdown caller,
   arbitrary rejected event variants lack nested event-specific byte disposal, and the synchronous
   `destroy_app` facade has no backpressure result when the fixed queue is saturated. P3 owns that
   ProgramBridge/Shell API seam, so this packet leaves it fail-closed and does not overlap those
   files. These residuals keep the broader reactor/runtime close failure RED.
2. MCP, native run, and WGPU now call `persistent_command_completion_port_ready()`, which is
   unconditionally false, before new command construction. Their retained loop implementations are
   source foundation only and unreachable in production until a Started/Progress/Completed
   submit/poll/cancel port replaces them. They receive no typed full-operation credit.
3. MCP terminal response decode is bounded to one 4,096-byte fixed page at the source authority but
   still uses the existing whole `decode_app_frame` on terminal take. No runtime `<8 ms` evidence is
   claimed, and larger responses intentionally fault.
4. Caller-local `effects`/combined outcome collections, other WGPU request variants, MCP instance
   maps, and reactor/app shutdown owners remain in the broader deep-close failure classes.
5. Global snapshot capability/factory ownership, full prepare/job/validate/emit/publish, eight
   reserved routes, 35 importers, 34 global payload stores, and all 884 commands remain fail closed.

The verifier now explicitly rejects public `Clone` on `SnapshotRead`/`ErasedSnapshotRead` and
`entry.snapshot.clone()` extraction. Current source intentionally trips this new fifteenth failure:
an exported read clone can outlive the app/store retirement authority and later become the last deep
snapshot owner. Removing the derives alone is not a fix because cloned `ChildContentView` roots need
read access and retirement concurrently. The required shared follow-up is a registered/scoped lease
whose handback transfers the exact last root into the app/store retirement pump, or a definitionally
bounded paged snapshot root. Domain factories by themselves do not close this capability escape.

Additional fixed-queue fixtures cover capacity/+1 exact rejection, aggregate page-credit +1,
contention returning the untouched command owner, FIFO, and an interrupted then completed two-page
shutdown. Create field closure and SurfaceVisible/rejected-event FIFO closure are also covered. No
Cargo/Nx command was run.

Verdict: **PASS for the fixed-page generic decoder, shallow batch arena, fixed command request ring,
and admitted caller command owner retention described here. REJECT for production final queue
shutdown/non-command disposal, caller operation completion,
runtime timing, typed full-operation activation, and Phase 8. Not ready for independent acceptance.**

---

## 2026-08-23 — Pollable Close and Registered Snapshot Lease Checkpoint

This source checkpoint closes the fire-and-forget native destroy submission and public cloneable
snapshot-read capability. It does not make child stores or app Drop terminal: the exact domain-owned
deep store disposer is still missing, so Phase 8 and the complete close route remain RED.

### Retained native close submission

- WGPU `KernelRequest::DestroyApp` now carries an `Arc<KernelCloseSubmission>` registered in a
  fixed 64-slot, exact `(instance, generation)` registry. Admission uses `try_lock`; contention and
  modulo collisions return the exact untouched owner.
- `KernelCloseHandle` is `#[must_use]` and exposes pollable `AdmissionBlocked`, `Pending`, `Complete`,
  and `Fault` states. The worker completes or faults the retained owner; queue shutdown cannot turn
  owner loss into false success.
- The old unit-returning ProgramBridge call now fails closed if admission was not retained. P3 must
  widen its renderer/Shell seam to keep and poll `begin_destroy_app`; this packet did not touch P3
  files.

### Registered snapshot reads and exact handback

- `SnapshotRead<T>` and `ErasedSnapshotRead` are no longer `Clone`. Every read is issued through a
  fixed 1,024-slot `MaybeUninit` registry with a fixed free ring, exact slot generation, nonblocking
  admission, an atomic returned-owner count, and one-slot cleanup probes.
- Drop marks the one exact lease returned before releasing the read-side Arc; the registry-held Arc
  remains stable. Double return is idempotent and counted once. Explicit child retirement validates
  exact type, registry identity, and generation before transferring the guard to the domain factory.
- `SnapshotReadRef<'_, T>` provides borrowed typed access. Child operation roots no longer clone an
  Arc capability or construct a whole `ChildContentView` from the child map.
- `SnapshotReadReturnPump` is retained by `VcsArtifactApp`. Live maintenance and app close drive at
  most one returned document snapshot disposer per grant. Close blocks while an external lease is
  live and advances only after the registry is terminal-empty.
- Final child-root retirement retains the exact `(slot, child_id)` owner. After the captured root's
  disposer completes it pumps late/returned guards for that member and requires the member lease
  registry to be empty. A reader held across close therefore produces `Blocked`, not a last-owner
  drop.

### Fixed child member authority

- `children: HashMap<(String, String), ...>` is replaced by `ChildMemberRegistry<M>`: one
  pre-admitted 1,024-slot allocation, fixed occupancy/reservation bitsets, per-slot generations,
  and deterministic linear probing. Duplicate/capacity rejection happens before `open` constructs
  a member. `register_child` returns the exact rejected member in
  `ChildMemberRegistrationError<M>`.
- Reservation cancellation is generation checked. A stale admission cannot cancel a reused slot.
  Successful adoption consumes the exact reservation; no occupied slot can be replaced.
- Close detaches at most one fixed slot per step into `ChildMemberRetirement<M>`. The new required
  `SpaceMember::{close_owned_step, close_owned_terminal_is_empty}` contract has no trait default;
  generated heterogeneous member enums forward it exactly. Final member drop is allowed only after
  the concrete owner reports terminal-empty.
- `ArtifactStore` currently returns `Blocked` for this deep-owned close contract and never claims a
  terminal witness. A domain-owned disposer must cursor its history/envelope/cache/current and all
  other nested roots before child/app close can complete. This is the exact remaining child-store
  blocker, not a permissive blanket proof.
- `ArtifactFixedRegistry`, `ChildMemberRegistry`, `ChildContentRetirement`, snapshot return pumps,
  and child-member retirement now use release-mode terminal assertions. Their `MaybeUninit` or
  `ManuallyDrop` storage prevents implicit nested destruction on an invariant fault; ordinary
  incomplete Drop is observably fail closed. The broader `VcsArtifactApp` still owns other ordinary
  deep fields, so app Drop is not accepted yet.

### Permanent source evidence

Rust fixtures now cover:

- snapshot lease capacity/+1 with exact rejected Arc;
- try-lock contention returning the untouched owner;
- double return counted and reclaimed exactly once;
- stale generation and same-slot ABA rejection;
- dropped/late read remaining observable until one-slot cleanup takes the guard;
- child member capacity/+1, deterministic collision probing, reservation-generation ABA, one-slot
  close detachment, and release-mode incomplete-Drop faulting without nested destructor traversal;
- retained WGPU close registry collision, contention/retry, and queue-shutdown fault.

The verifier adds four adversarial fixtures rejecting a fixed immutable child view backed by a
resizable deep member map, a fixed child registry without generation/terminal Drop, a blanket member
close proof, and no-op fixed-owner registry Drop.

### Exact gates

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2024 --check` on store and plugin shared source | PASS |
| WGPU targeted rustfmt/check from the preceding close submission boundary | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: `self-tests=119 clean` |
| full deterministic JSON verifier | Expected RED: 14 failure classes, 0 admitted, 884 remaining, 8 reserved routes, 35 importers, 34 globals |
| scoped `git diff --check` | PASS |
| Cargo/Nx/native/Wasm/runtime timing | Not run by coordinator instruction |

Exact full verifier output is retained in `p8-caller-registry-verifier.json`.

Verdict: **PASS for retained/pollable native close submission, non-Clone registered snapshot leases,
exact returned-root pumping, and fixed child-member ownership transfer. REJECT for concrete deep
`ArtifactStore` member disposal, the remaining ordinary app/runtime nested fields, the final P3
renderer close caller, full typed operation publication, all 884 command activations, and Phase 8.
Not ready for independent runtime acceptance.**

---

## 2026-08-23 — Atomic Member Owner Pair Checkpoint

Generated member creation and reopen can no longer bypass the domain owner through UFCS or install
only one half of its close authority:

- `MemberStoreOwner<Mutation>` is a required, no-default snapshot-type contract.
- Its only constructor result is `MemberStoreOwners<P, Mutation>`, which contains the exact typed
  immutable-root, owned-initial-snapshot, owned-mutation, and whole-store retirement authorities
  together.
- `create_member_store`, `open_member_store`, the blanket `ArtifactStore` member factory, and every
  `space_members!` generated arm require `P: MemberStoreOwner<Mutation>`.
- Both generic construction paths install the pair through one private, infallible fresh-store
  transfer. There is no partial snapshot-first fallible branch whose error could stack-drop the
  newly constructed store.
- The production `SemioMembers` cohort generates all 18 exact snapshot/mutation pair bindings from
  `semio_subset_table!`. The old `install_semio_snapshot_retirement` wrapper and its post-factory
  create/open calls are removed, so direct generated `MemberFactory::{create,open}` and the Semio
  convenience functions use the same mandatory authority.
- Duplicate snapshot factory replacement is rejected, missing whole-store ownership faults before
  close progress, and a blocked disposer remains retained for a later bounded turn.

The Semio whole-store disposer is deliberately not certified terminal. It returns an observable
fault naming the missing per-field cursor and `terminal_is_empty == false`; it does not misuse
`Blocked` for a permanent unimplemented state. The internal `ArtifactStore`
still contains an ordinary `ArtifactEnvelope`, history vectors/maps, causal DAG, index roots,
backbone, current/tail snapshot Arcs, and pending command owners without a completed per-field
transfer cursor. Claiming Complete would make final member Drop monolithic. The next accepted layer
must replace those resizable/string-key close owners or transfer each exact owner into a retained
domain cursor one fixed item/page per grant; it must also prove the final store shell shallow.

The first real store-field cursor layers are now connected behind the unforgeable
`ArtifactStoreCloseView` passed only by `close_owned_store_step`:

- returned snapshot leases are pumped first and a live external reader is the only `Blocked` state;
- every inverse/forward history mutation is popped one at a time and transferred into the exact
  owner-supplied `ArtifactOwnedValueRetirementFactory<Mutation>`;
- after mutations are empty, each edit is detached from the tail into
  `ArtifactStoreEditRetirement`, which cursor-disposes edit strings, dependency ids, metadata,
  provenance, and byte ownership under the item/byte grant;
- incomplete edit retirement uses `ManuallyDrop` plus a release-mode terminal assertion, while a
  nonempty mutation vector is returned intact to the store;
- Semio owned snapshots use the existing field retirement tree. Semio mutation retirement remains
  retained and observably faults because generated per-variant field cursors have not landed; it
  does not drop or falsely complete the exact mutation owner.

The remaining structural phase is explicit: history changes/checkpoints/alternatives, durable
message/conflict roots, the HashMap edit index, causal `MutationDag`, cursor/index vectors,
backbone, current/tail/initial roots, and final envelope/store shell. Until those owners move to
fixed/paged terminalizable storage, the Semio disposer returns the named structural fault and
`terminal_is_empty == false`.

The machine inventory `📊️p8yt-child-snapshot-retirement-cohorts.json` now records
`fail-closed-pending-store-disassembly` and the paired owner symbols. The verifier adds four
adversarial fixtures rejecting a default/optional member owner, a snapshot-only member owner, and a
wrapper-only bypass, plus a whole-history drop without the retained edit cursor.

### Exact files changed in this checkpoint

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️component.rs`
- `/Users/ueli/Documents/semio/📜️script.ts`
- `📊️p8yt-child-snapshot-retirement-cohorts.json`
- this report and `p8-member-owner-verifier.json`

### Gates

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2024 --check` on store, plugin, and Semio owner source | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: `self-tests=123 clean` |
| `bun ./📜️script.ts verify interactivity` | PASS: DENY clean |
| full tool-jobs JSON | Expected RED: 14 failure classes, 0/884 admitted, 8 reserved, 35 importers, 34 globals |
| `git diff --check` | PASS |
| Cargo/Nx/native/Wasm/runtime timing | Not run by coordinator instruction |

Verdict: **PASS only for the atomic, mandatory generated member-owner pair and its exact Semio
cohort binding. REJECT for deep `ArtifactStore` disposal, app/runtime terminal Drop, typed full
operation publication, all 884 activations, and Phase 8. Not ready for independent runtime audit.**

---

## 2026-08-23 — Structural Owner Transaction and Generated Mutation Cursor Checkpoint

This rejection-driven layer removes three concrete whole-owner destruction paths while keeping the
activation ledger closed:

- Document root adoption now crosses an explicit
  `ArtifactStoreDocumentRootCommitAuthority<P, Mutation>`. Preparation validates the three exact
  snapshot/mutation retirement factories and reserves the displaced-owner credits before any root
  swap. The commit transfers the old envelope, immutable current root, causal DAG, and optional tail
  root into retained cursor owners without a fallible branch between swaps. Reset, resolution
  candidate adoption, full remote replacement, and remote merge use this seam.
- The live durable diagnostic index has fixed tombstone slots. Removal uses `swap_remove`, removes
  both exact index tickets before moving the tail, updates only the moved ticket, and transfers the
  removed entry to `ArtifactStoreMessageLedgerRetirement`. No `Vec::remove` shift or whole
  `ArtifactEditMessageIndex::rebuild` remains on that path.
- `MutationDag` no longer owns `ManuallyDrop<Vec<MutationEnvelope/String>>`. Its envelope,
  applied-id, and pending-id lanes are fixed 8,192-slot `MaybeUninit` authorities with per-slot
  generations, a fixed LIFO free ring, linked deterministic insertion traversal, exact capacity
  rejection, O(1) ticket removal/tail pop, and terminal-empty Drop. A pending identity is moved into
  the applied lane instead of cloned and dropped. Duplicate seed now returns its exact rejected
  identity owner.
- All 18 Semio member mutation enums implement schema-taxonomy field retirement. Scalar,
  string/byte, nested snapshot, recursive collection, path, geometry, link, and child reference
  fields are transferred into the same per-item/per-byte cursor tree. The permanent
  `semio mutation owner has no generated field-by-field retirement cursor` fault and its opaque
  retained owner are removed.
- Every remaining deep `ArtifactStore` runtime field is now born behind `ManuallyDrop`.
  `ArtifactStore::drop` accepts only a fully detached envelope/current, empty causal/index/string/
  report/lease/displaced-owner shell, no tail/backbone/disposer, and a terminal whole-store
  disposer. Only then does it release the empty shallow authorities. This closes implicit Rust field
  destruction; it does not yet prove that every direct constructor/error path reaches the terminal
  state.

### Permanent adversarial evidence

- causal capacity plus one retains the rejected identity;
- duplicate causal seed retains the exact duplicate owner;
- oversized causal identity returns the whole envelope;
- the verifier rejects a root transaction that still shifts/rebuilds its diagnostic ledger, uses
  `ManuallyDrop<Vec>` causal storage, or retains the opaque Semio mutation disposer;
- the structural predicate now requires fixed causal storage, a fixed live durable ledger,
  generated coverage for exactly 18 mutation enums, and the terminal-only `ArtifactStore::drop`.

### Honest residual rejection

This checkpoint is not a StructuralOwners PASS:

1. `ArtifactEnvelope::edit_messages` is still a resizable wire `Vec`; the runtime index no longer
   shifts or rebuilds, but push/growth and whole wire-envelope materialization are not a fixed
   generation-slot ledger.
2. The causal fixed authority has generation tags and a free ring, but construction initializes its
   fixed control tables, clone still visits every live slot, logical-index lookup walks the linked
   order, and ready/dependency scans remain synchronous rather than retained paged cursors.
3. Reset and resolution candidate preparation can still return an error while stack-local
   envelope/DAG/current candidates exist. The terminal-only store Drop prevents implicit deep
   destruction, but rejected candidates are not yet returned in a reclaimable exact-owner result or
   installed into a live close pump; resolution candidates are still represented as a full
   `ArtifactStore`.
4. Runtime vector/string/backbone/revision replacements outside the new root transaction still
   overwrite some old owners before a displaced retirement cursor is reserved. The fields are
   non-destructive from birth, but terminal progress is not wired for every early return.
5. No Cargo/Nx/native/Wasm/runtime measurement was authorized. Rustfmt proves parsing/formatting,
   not type correctness or the required sub-8ms runtime timings.

### Exact files

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️component.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️component.rs`
- `/Users/ueli/Documents/semio/📜️script.ts`
- this report and `p8-structural-owners-verifier.json`

### Gates

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2024 --check` on store, causal, plugin, and Semio source | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: `self-tests=126 clean` |
| `bun ./📜️script.ts verify interactivity` | PASS: DENY clean |
| full deterministic tool-jobs JSON | Expected RED: 16 failure classes; 50 hosts, 50 invocations, 775 rows, 773 unique rows, 0 admitted, 884 residual commands, 8 reserved routes, 35 importers, 34 globals |
| scoped and whole-tree `git diff --check` | PASS |
| Cargo/Nx/native/Wasm/runtime timing | Not run by coordinator instruction |

Verdict: **PASS only for the pre-reserved retained root commit seam, non-shifting diagnostic removal,
fixed causal owner storage, generated 18-cohort mutation disposal, and terminal-only store shell.
REJECT for exact candidate-error ownership, the resizable live wire ledger, paged causal
scan/clone progress, complete runtime-field retirement, typed full-operation publication, all 884
activations, and Phase 8. Not ready for independent runtime acceptance.**

---

## 2026-08-23 — Exact Candidate Handoff and Runtime Replacement Cursor Checkpoint

The next rejection-driven layer closes the stack-local resolution-candidate drop and direct deep
runtime replacement paths without changing the activation verdict:

- `ArtifactStoreResolutionCandidateAuthority` reserves one exact slot in the store's fixed
  displaced-owner queue before cloning candidate authority. The reservation carries a monotone
  generation and is included in every subsequent capacity calculation.
- Every candidate ingest, validation, early non-acceptance, and adoption-preflight branch now either
  adopts the authority or transfers it to `ArtifactStoreResolutionCandidateRetirement`. The
  retirement advances displaced children, reports, backbone, runtime string/revision lanes, tail
  and current snapshot roots, causal owners, and the final envelope one exact child at a time. Its
  final store Drop occurs only after the terminal-empty store witness is true.
- String vectors, cursor strings, revision accumulators, pending command reports, backbones, and
  persisted cursors have explicit retained retirement cursors. Root transactions reserve their
  runtime-owner credits together with envelope/current/DAG/tail credits before the first swap.
  `set_state`, full remote replacement, remote merge publication, checkpoint/local-actor changes,
  and candidate adoption no longer directly overwrite these deep `ManuallyDrop` fields.
- `CursorRevisionAccumulator::reconcile` pops displaced records into a retained string-vector
  owner rather than truncating and dropping them. Persisted cursor replacement transfers the old
  cursor into `ArtifactStoreCursorRetirement` before adoption.
- A schema-target `ArtifactEditMessageLedger` now exists with 8,192 fixed `MaybeUninit` slots,
  stable linked insertion order, fixed free-ring admission, per-slot generations, stale-ticket
  rejection, exact +1 owner return, and terminal-only Drop. It is intentionally not yet installed
  as `ArtifactEnvelope.edit_messages`: doing so before all serde/pack/error owners use the retained
  decoder/close protocol would turn a source marker into an ordinary-Drop leak or panic.
- Resolved-conflict pruning transfers the removed payload into
  `ArtifactStoreConflictRetirement`; candidate conflict suffixes are likewise retained rather than
  truncated.

### Permanent adversarial evidence

- The verifier now separately rejects a resolution candidate that can cross `?` without an exact
  reserved handoff and terminal retirement owner.
- Structural admission requires the candidate authority, fixed ledger ticket/generation symbols,
  and rejects revision truncation plus applied/redo `remove`/`clear` escape paths.
- `set_local_actor_id` is now fallible; all seven shared plugin callers preserve fail-closure by
  mapping a saturated retirement admission to the exact framework fault path.

### Remaining StructuralOwners rejection

1. The live wire field is still `ArtifactEnvelope::edit_messages: Vec<EditMessages>`. The fixed
   ledger cannot be installed until JSON/SPR/pack decoding returns malformed/+1 owners to a bounded
   close pump instead of relying on serde's ordinary error Drop. Old-wire differential and real
   wire round-trip evidence therefore remain pending.
2. Applied/redo cursor lanes still use `Vec<String>` and three ordered middle-removal paths still
   shift a whole suffix. Those paths move the selected string rather than dropping it, but they are
   not hard-bounded and keep the structural verifier RED.
3. Resolved-conflict removal still shifts the surrounding `Vec<Conflict>` pointer suffix even
   though the exact removed payload is retained. A fixed ordered conflict ledger is still required.
4. Candidate retirement is exact and reclaimable, but candidate work and several preparation maps
   remain run-to-completion rather than a persistent step cursor. The authority is foundation
   progress, not interactive operation admission.
5. Fixed-ledger generation exhaustion, fixed-control-table construction time, decode timing, and
   sub-8ms retirement steps have no runtime evidence because Cargo/Nx/native/Wasm execution remains
   explicitly prohibited.

### Gates

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2024 --check` on store, causal, plugin, and Semio source | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: `self-tests=127 clean` |
| `bun ./📜️script.ts verify interactivity` | PASS: DENY clean |
| full deterministic tool-jobs JSON | Expected RED: 16 failure classes; 50 hosts, 50 invocations, 775 rows, 773 unique rows, 0 admitted, 884 residual commands, 8 reserved routes, 35 importers, 34 globals |
| scoped and whole-tree `git diff --check` | PASS |
| Cargo/Nx/native/Wasm/runtime timing | Not run by coordinator instruction |

Verdict: **PASS only for exact reserved candidate handoff, bounded candidate reclamation authority,
and retained direct-runtime replacement cursors. REJECT for live wire-ledger migration, ordered
cursor/conflict suffix shifts, persistent candidate preparation, typed full-operation publication,
all 884 activations, and Phase 8. Not ready for independent runtime acceptance.**

---

## 2026-08-23 — Fixed-Ledger Admission and Owned-Codec Fail-Closure Checkpoint

This save hardens the fixed edit-message target without pretending that it is already the live
envelope authority:

- `ArtifactEditMessageLedger` now owns a fixed 8,192-bucket identity table in addition to its fixed
  payload slots and linked order. Admission preflights exact item count, 256-byte ids, 4,096 bytes
  per entry, the checked aggregate byte authority, and duplicate ids before moving any entry into a
  live slot.
- Slot generations are `u64` and use `checked_add`; wrapping ABA reuse is rejected with the exact
  entry still owned by the caller. Removal locates and tombstones the exact identity bucket before
  returning the payload and never clones the id.
- `ArtifactEditMessageLedgerRejected` retains the exact rejected vector behind `ManuallyDrop`,
  advances one `ArtifactStoreMessageLedgerRetirement` child at a time under the caller's item/byte
  grant, and accepts Drop only after the vector allocation and every nested message/string owner are
  terminal-empty.
- Permanent fixtures cover duplicate admission, capacity +1, stable order after middle tombstone,
  slot reuse, stale generation rejection, exact rejected-owner retention, interrupted bounded
  reclamation, and terminal Drop.
- The verifier now rejects ordinary-`Vec` rejection ownership and wrapping ledger generations. It
  also has a separate owned-envelope-codec gate: public `ArtifactEnvelope: Deserialize`, any direct
  production `serde_json::from_*::<...Envelope>` caller, missing fixed 4,096-byte decode pages, or
  missing retained decode-error close authority keeps the packet RED.

### Exact codec ingress inventory

The fresh whole-tree scan finds **19 production direct JSON envelope decoders**, plus two store
tests. They are grouped as follows:

- 15 artifact editor/Wasm bridge files: writer, procedural2d, procedural3d, GIS map, Present,
  Shooting, FEM 2d, FEM 3d, Process 3d, CAD, Trinity Jack, Draw, Raster, Puzzle 5d, and Puzzle 3d;
- one Present binary mutation codec;
- Trinity Rewrite's world-side Wasm bridge;
- the framework DAG editor bridge;
- the framework Flow VCS bridge.

The tree therefore differs from the earlier 15-caller estimate: the exact current production count
is 19. None was mechanically rewritten to a run-to-terminal helper. A valid simultaneous cutover
requires an owner-supplied schema decoder for `P` and `Mutation` that accepts fixed pages, advances
one decode/semantic stage per `StepContext` grant, and returns every malformed/partial owner to the
retained error authority. A private serde DTO, `Vec` wire adapter, whole second-pass decode, or
post-lift paging would fail the new verifier contract.

### World3D coordination result

P3 provided the typed snapshot lease/page consumer and an interruptible writer-abort API. An initial
shared producer experiment was rejected during joint source review because it copied already-whole
JSON strings into typed pages, synchronously hashed/built 19 pages, panicked on admission, and only
marked (rather than pumped) writer abort. That experiment and its fixture/imports were fully removed.
`world3d_scene_extended` remains source-coherent with `snapshot: None`; this is an explicit RED
legacy producer boundary, not a claimed typed cutover. The required separate packet has 30 live
domain call sites and must start from typed camera/mesh/vertex/triangle/instance authorities, not
parse or copy their JSON products.

### Gates at this save

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2024 --check` on store, causal, shared plugin, and Semio source | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: `self-tests=131 clean` |
| full deterministic tool-jobs JSON | Expected RED: **17** failure classes; 50 hosts, 50 invocations, 775 rows, 773 unique rows, 0 admitted, 884 residual commands, 8 reserved routes, 35 importers, 34 globals |
| `bun ./📜️script.ts verify interactivity` | RED from three concurrently-owned P1 shard-executor findings; no P8 allowlist was added |
| scoped and whole-tree `git diff --check` | PASS |
| Cargo/Nx/native/Wasm/runtime timing | Not run by coordinator instruction |

Verdict: **PASS only for fixed-ledger preflight, checked generation/identity authority, and exact
bounded rejection ownership. REJECT for the live `ArtifactEnvelope` field, owned schema decoder and
all 19 production call-site cutovers, applied/redo/conflict linked ledgers, persistent candidate
preparation, World3D typed producers, typed full-operation publication, all 884 activations, and
Phase 8. Not ready for independent runtime acceptance.**

---

## 2026-08-23 — Fixed-Page Schema Cursor Checkpoint

This save closes the first truthful layer below the owned envelope decoder without introducing a
serde adapter or claiming a caller migration:

- `OwnedSchemaDecodePages` reserves fixed `MaybeUninit` page slots and exact total byte credits
  before accepting input. Each admitted page is an inline, definitionally shallow 4,096-byte owner;
  saturation and byte +1 return the exact page untouched. There is no `Vec<Vec<u8>>`, no aggregate
  concatenation, and no post-admission paging of a contiguous input.
- `OwnedSchemaTokenCursor` retains operation/generation authority, advances under `StepContext`
  fuel/deadline/cancellation, and validates strings, escapes, Unicode hex, UTF-8 continuation
  ranges, numbers, literals, and truncation directly across fixed pages. Diagnostics carry exact
  byte offset, line, column, and fixed path authority.
- `OwnedSchemaRecordCursor` binds the byte cursor to a no-default `OwnedSchemaRecordSpec`, validates
  the field-ID/key bijection, preserves source field order, rejects unknown/duplicate/missing
  required fields, and returns field tokens with exact stable numeric IDs. Nested delimiter kinds
  are retained in a fixed 256-entry stack; mismatched or excessive nesting faults.
- `ArtifactEnvelopeFieldDecoder<P, Mutation>` is the owner-supplied typed field interface. It exposes
  no serde/serde_json types and requires explicit token, finish, typed-result take, bounded close,
  and terminal-empty methods. The sole 12-field `ArtifactEnvelope` key/ID schema is now declared in
  the same owned taxonomy.
- Cancelled, stale, malformed, or rejected cursors release one real admitted page per close grant.
  The fixed page payload and slot shell have no nested Drop work, so unexpected release of an empty
  or partially parsed byte source cannot walk user payload owners.

### Permanent source fixtures

- exact page capacity and aggregate-byte +1 owner return;
- a string spanning the 4,096-byte boundary and a partial terminal page, driven with seven fuel
  units per turn;
- invalid UTF-8 whose lead byte is the last byte of page one and invalid continuation is the first
  byte of page two, with exact offset 4,096;
- unknown and duplicate envelope field paths, truncated string token, stale generation before byte
  consumption, cancellation, two interrupted close turns, and terminal empty;
- verifier negatives for `Vec<Vec<u8>>` schema pages, a post-lift
  `ArtifactEnvelopeDecodeAuthority::new(Vec<u8>)`, and an unbudgeted token cursor.

### Explicit RED boundary

The field decoder is a required owner contract, but no blanket/default implementation exists. The
current 19 production decoders and two store tests still call `serde_json::from_*`; none was hidden
behind a whole-buffer helper. `ArtifactEnvelope` still derives `Deserialize`, and
`ArtifactEnvelope.edit_messages` is still the live `Vec<EditMessages>`. The fixed ledger remains a
pre-admitted target rather than the live wire field because the nested VCS/P/Mutation/SPR owners do
not yet implement the required token/close protocol. Applied/redo/conflict ordering and persistent
candidate preparation remain RED for the same reason.

### Gates

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2024 --check` on shared store source | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: `self-tests=134 clean` |
| `bun ./📜️script.ts verify interactivity` | PASS: DENY clean; one test-only permanent allowlist entry is structurally outside production |
| full deterministic tool-jobs JSON | Expected RED: **17** failure classes; 50 hosts, 50 invocations, 775 rows, 773 unique rows, 0 admitted, 884 residual commands, 8 reserved routes, 34 globals |
| scoped and whole-tree `git diff --check` | PASS |
| Cargo/Nx/native/Wasm/runtime timing | Not run by coordinator instruction |

Verdict: **PASS only for the fixed-page lexical/schema cursor and the no-default typed field-owner
contract. REJECT for envelope construction, all 21 decoder call sites, live fixed ledger,
applied/redo/conflict fixed ordering, runtime timing, every command activation, and Phase 8. Not
ready for independent runtime acceptance.**

---

## 2026-08-23 — Present Generic Maintenance Gate

The representative Present decoder is no longer only a domain-local job object:

- `ArtifactEnvelopeDecodeOwnerBundle<P, Mutation>` binds the exact owner catalog, initial-snapshot
  retirement factory, and mutation retirement factory in one non-default authority. `ArtifactApp`
  and `ArtifactEditor` return `None` by default, which grants no decode admission; the Present owner
  installs its exact bundle explicitly.
- `VcsArtifactApp` owns a fixed `ArtifactFixedRegistry<ActiveArtifactEnvelopeDecode<...>>`. Public
  submission preflights the exact operation slot before consuming pages, creates a persistent
  `WorkerJobSession`, and returns a generation-qualified handle. Worker saturation yields without
  losing the session.
- A field-registry admission race does not drop the already-tokenized record or field decoder.
  `ArtifactEnvelopeUnadmittedDecodeRejected` retains both and closes one decoder/page owner per
  grant. Its release-mode Drop accepts only terminal-empty authority.
- Maintenance now round-robins the active decode job, returned-field pump, and completed-record
  pump. Application close uses the same three-way interleave, cancels every retained operation,
  and cannot deadlock a decode waiting for its returned field lease or completed output retirement.
- Completed publication revalidates the live store generation in release code immediately before
  the one non-suspending target adoption. A rejected target leaves the exact record and operation
  retained for ordered retry. Cancel/stale/app-close requests output retirement instead of dropping
  an unpublished envelope.

Present's owner-local fixed registry now has permanent fixtures for malformed pack, explicit cancel,
64-slot capacity and +1 modulo collision, same-operation duplicate collision, exact rejected-page
return, publication backpressure and retry, stale generation, and close of a ready but unpublished
output. The source-bounded `<=4096` snapshot pack decode still lacks runtime timing evidence because
native execution remains prohibited.

### Exact production caller ledger

The initial inventory contained 19 direct whole-buffer callers. The obsolete Present binary
`materialize_present_projection_json`/`materializePresentProjectionJson` route is removed. The exact
residual is therefore **18**, all fail-closed placeholders rather than claimed migrations:

| # | Owner | Source boundary | Residual |
| ---: | --- | --- | --- |
| 1 | Procedural3d | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:23` | whole `String` create |
| 2 | Dag | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs:8925` | whole `String` create |
| 3 | Draw | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:497` | whole `String` create |
| 4 | Flow | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs:844` | whole `String` create |
| 5 | Shooting | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:31` | whole `String` create |
| 6 | Procedural2d | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:23` | whole `String` create |
| 7 | GisMap | `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:27` | whole `String` create |
| 8 | Fem2d | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:22` | whole `String` create |
| 9 | Fem3d | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:22` | whole `String` create |
| 10 | Process3d | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:36` | whole `String` create |
| 11 | Present | `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:28` | store create still bypasses generic app registry |
| 12 | Writer | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:36` | whole `&str` create |
| 13 | Cad | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:24` | whole `String` create |
| 14 | Raster | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:23` | whole `&str` create |
| 15 | Puzzle5d | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:27` | whole `String` create |
| 16 | Puzzle3d | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:27` | whole `String` create |
| 17 | Trinity Rewrite | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌍️world/🦀️component.rs:697` | whole `String` create |
| 18 | Trinity Jack | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:23` | whole `String` create |

### Honest populated-history boundary

Present still accepts only exact empty `edits`, `changes`, `checkpoints`, and `alternatives` in its
owner-local VCS authority. `ArtifactVcs` still exposes four `Vec` owners, and converting a decoded
fixed ledger into those vectors would recreate the prohibited deep-drop escape. This checkpoint
therefore does not claim populated history or decrement the 18 residual callers. The next required
source packet is a schema-owned fixed generation-keyed VCS ledger from construction, with retained
per-entry decode/retirement and no ordinary `Vec` publication.

### Gates

| Gate | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: `self-tests=139 clean` |
| full deterministic tool-jobs JSON | Expected RED: **18** failure classes; 50 hosts, 50 invocations, 775 rows, 773 unique rows, 0 admitted, 884 residual commands, 8 reserved routes, 35 importers, 34 globals |
| `bun ./📜️script.ts verify interactivity` | PASS: DENY clean |
| rustfmt parse of shared plugin plus rustfmt write/check on store and Present-owned files | PASS; shared plugin has pre-existing whole-module formatting drift and was not mass-formatted |
| scoped `git diff --check` | PASS |
| Cargo/Nx/native/Wasm/runtime timing | Not run by coordinator instruction |

Verdict: **PASS for the generic retained Present decode maintenance/close foundation and its exact
owner-return gates. REJECT for populated VCS history, all 18 residual caller cutovers, full store
terminal disposal, typed full-operation publication, all 884 activations, and Phase 8. Not ready
for independent runtime acceptance.**

---

## 2026-08-23 — Fixed VCS History and Populated Present Checkpoint

`ArtifactVcs` now owns four schema-level `ArtifactHistoryLedger<T>` authorities from construction
instead of resizable `Vec` histories. Each ledger preallocates exactly 64 stable slots, keeps live
order through fixed previous/next indices, tombstones removed slots, advances a checked generation
before reuse, and returns the exact rejected owner at capacity. Ordinary Drop accepts only a fully
drained logical authority; the empty `MaybeUninit` allocation is then shallow.

History adoption now has an explicit two-phase `ArtifactHistoryReservation`: the exact issuing
ledger, slot, and next generation are fixed before an entry is decoded or cloned. A second
reservation, wrong-ledger token, stale token, or capacity +1 cannot consume the owner. The retained
schema decoder holds the token across per-token work, commits through `insert_reserved`, and cancels
the exact token during bounded close. Rejected entries transfer to their domain retirement factory;
the decoder never uses `try_push(...).map_err` as an owner-dropping admission boundary.

The store owns concrete bounded retirement cursors for the full VCS aggregate: edit mutations first,
then edit metadata, changes, checkpoints, alternatives, and the initial snapshot. Store mutation and
metadata insertions use exact reserved commit helpers; impossible stale-token returns enter the
existing fixed displaced-owner maintenance pump rather than stack Drop. The remaining free
`reconcile_alternative` helper and legacy `.spr`/`.ops` parsers are still RED because they stage
temporary vectors or lack an app-owned rejection pump; no acceptance credit is claimed for them.

Present's retained materializer now accepts populated edit history. It owns the completed envelope,
current snapshot, prior-snapshot retirement, and full-envelope retirement in `ManuallyDrop`
authorities. One worker step takes one mutation in deterministic ledger order, computes and applies
its diff, retires the displaced snapshot before advancing, and publishes only the exact final
snapshot after the history and envelope owners reach terminal empty. Cancellation, stale generation,
fault, and unpublished output all use the same retained close phases. The permanent populated-history
fixture applies a real `ReplaceTiles` edit through submit, maintenance, completion, and close.

The obsolete standalone Present Wasm whole-string ABI was removed: there is no
`createPresentEnvelopeJson`, `envelope_json` constructor parameter, or fail-closed whole-buffer
placeholder in that owner. Source authority now reports **17** remaining production placeholder
sites (the prior table's Present row is gone): Raster, Cad, GisMap, Shooting, Procedural2d,
Procedural3d, Dag, Flow, Trinity Rewrite, Trinity Jack, Draw, Fem2d, Fem3d, Writer, Puzzle3d,
Puzzle5d, and Process3d. This is deletion of an obsolete parallel ABI, not credit for another live
domain migration; Present's live migration is the app-owned retained submit/pump/consume path above.

### Exact changed source in this checkpoint

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️component.rs`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
- the four Draw command literals `commit-document`, `set-fixture-json`, `set-active-example`, and
  `set-snapshot` (mechanical empty fixed-ledger initializers only)
- `📜️script.ts`

### Gates

| Gate | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | PASS: `self-tests=141 clean` |
| full tool-jobs JSON | Expected RED: **18** failure classes; 50 hosts, 50 invocations, 775 rows, 773 unique rows, 0 admitted, 884 residual commands, 8 reserved routes, 35 importers, 34 globals |
| scoped rustfmt and `git diff --check` | PASS |
| Cargo/Nx/native/Wasm/runtime timing | Not run by coordinator instruction |

Verdict: **PASS only for fixed history storage, exact reservation/handback, bounded VCS retirement,
and the populated Present retained materializer. REJECT for the 17 remaining caller placeholders,
legacy vector parsers, free-history rejection ownership, terminal-fault quarantine, runtime timing,
the full typed operation, all 884 activations, and Phase 8.**

---

## 2026-08-23 — Writer Retained Edit and Mutation Decoder Checkpoint

The Writer cohort's nested history decoder no longer reconstructs a bounded entry and invokes
domain serde at the closing token:

- 'ArtifactOwnedHistoryEntryDecoder<T>' is now a factory for a retained
  'ArtifactOwnedHistoryEntryAuthority<T>'. The fixed history array gives that authority every exact
  schema token, preserves a pending token for replay, inserts only after the authority's
  terminal-empty take witness, and drives partial/rejected owners through the existing one-owner
  retirement lane.
- Repository Change, Checkpoint, and Alternative entries use an explicit retained
  source-bounded adapter. It still performs one at-most-4,096-byte serde decode after token
  collection and therefore remains RED pending runtime timing and schema-owned field cursors; it is
  no longer an implicit domain-edit fallback.
- Writer owns a concrete ten-field Edit catalog. Scalar values use fixed inline string
  authorities; forwards and inverse preallocate exactly 64 slots and retain their partially
  populated owners; mutationMeta is presently admitted only as the exact empty array. Missing,
  duplicate, unknown, mismatched, stale, cancelled, and malformed fields fault without dropping
  partial strings or mutations.
- Each Writer mutation is created through the catalog's real begin_mutation method. The exact
  tagged schema accepts only renameWriter/newId, changeUri/newUri,
  changeLanguage/newLanguageId, or editText/text. Tag/payload mismatches, extra payload fields,
  and semantic byte capacity faults retain the exact authority for bounded close.
- Writer snapshot, mutation, mutation-array, mutation-target, and edit authorities use terminal
  Drop assertions over ManuallyDrop owners. Mutation and snapshot retirement release one exact
  string/child-reference owner per item+byte grant.

The live Writer Wasm constructor is deliberately unchanged at this save: it still accepts a whole
&str and invokes the fail-closed placeholder. The generic app can publish a completed envelope
to an exact target, but it cannot yet adopt that record as a document store: ArtifactStore::new
consumes the envelope across fallible, whole-history async validation/folding, and replacing the
app's initial store has no pre-reserved displaced-store retirement transaction. Removing the
placeholder before an owner-returning retained initialization/replacement job exists would be a
deletion-only false migration. The exact residual caller count therefore remains **17**.

P1 coordination in the same shared store boundary is also complete:

- ChannelBackboneRemote::try_pop_front() returns one exact FIFO message or nonblocking
  contention; the legacy bulk drain() -> Vec<_> surface is removed after native and Wasm sync
  consumers migrated.
- ArtifactWorkerEntry::cmd_tx now uses P1's fixed ArtifactMailboxSender; send rejection retains
  the exact ArtifactActorMsg through ArtifactMailboxSendError::into_message.

### Current gates

| Gate | Result |
| --- | --- |
| bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json | PASS: self-tests=142 clean |
| rustfmt parse/write on shared store, Writer binary, and Present binary | PASS |
| scoped git diff --check on shared store and the touched Writer/Present sources | PASS |
| full tool-jobs verifier | Expected RED: activation remains 0/884 and Writer remains one of 17 caller placeholders |
| Cargo/Nx/native/Wasm/runtime timing | Not run by coordinator instruction |

Verdict: **PASS only for the retained token-to-Edit/token-to-WriterMutation ownership foundation.
REJECT for Writer live Wasm submit/pump/consume, populated mutation metadata, retained store
initialization/replacement, legacy .spr/.ops parsers, every activation, and Phase 8.**

---

## 2026-08-23 — Writer Live Retained Load Cohort

Writer is the first caller cohort whose old whole-string Wasm constructor has been removed. The new
greenfield surface owns one operation/generation handle and never exposes a direct `ArtifactStore`
constructor:

1. `beginEnvelopeLoad(maximumPages, maximumBytes)` pre-admits one fixed registry slot and exact page
   and byte credits before any envelope byte owner is copied.
2. `admitEnvelopePage(handle, Uint8Array)` rejects a hostile browser array before the bounded
   4,096-byte copy. The page moves into the exact app-owned ingress authority or returns untouched.
3. `sealEnvelopeLoad(handle)` transfers the sealed page authority to the existing schema decoder.
   Decoder saturation restores the exact sealed ingress owner to its original operation slot.
4. `pollEnvelopeLoad(handle)` advances only one app maintenance turn. Decode completion moves the
   exact envelope into Writer's domain initializer; it does not run either worker to completion in
   the callback.
5. Writer validates identity, duplicate edits, history membership, scalar snapshot fields, mutation
   application, and history hashes one retained phase at a time. Candidate generation uses checked
   `base + 1` before any owner detaches.
6. Candidate construction consumes the domain owner catalog atomically through
   `ArtifactStore::from_initialized_runtime_with_owners`; there is no separately fallible install
   statement that could drop or strand envelope/runtime/catalog ownership.
7. Publication revalidates the exact base generation, swaps once, and retains the displaced store
   behind its domain disposer until terminal empty. Only then does the poll surface `Ready`, and the
   operation remains retained until `acknowledge_artifact_store_replacement` succeeds exactly once.

The shared recovery path was tightened at the same boundary. Missing-candidate and false-terminal
worker outcomes rewrap the exact `ArtifactStoreInitializationJob`, signal its owner-local
cancellation authority, and keep any already-extracted candidate in the replacement registry for
bounded rejection. Closed worker response channels take the same retained cleanup path. No recovery
branch synthesizes a replacement envelope/runtime/catalog or stack-drops a retryable initializer.

### Destruction and adversarial ownership evidence

The app close hierarchy now drains Writer-related owners in this order: ingress pages, decoder and
returned field owners, completed records, initializer/candidate, and displaced store. Each ingress
close releases one real page only when the item and byte grant covers it. The Wasm surface exposes
`closeStep`; it does not synchronously drain in Rust `Drop`.

Permanent source fixtures cover:

- fixed ingress capacity and `+1` modulo collision with exact rejected-owner return;
- zero-item and insufficient-byte interrupted close, followed by one-page terminal close;
- initializer cancellation and stale-generation cleanup to terminal empty;
- checked next-generation candidate construction and cursorized candidate disposal;
- live submit → decode maintenance → initializer → swap → displaced-store retirement → exact ACK;
- partial-ingress cancellation without publication; and
- verifier mutations for a post-lift dynamic byte slice, missing ACK, and false-terminal job drop.

These Rust fixtures were authored but not executed because Cargo/native/Wasm remains explicitly
serialized by the coordinator. Their runtime verdict is therefore **unproven**, not PASS.

### Files changed in this cohort

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️component.rs`
- `📜️script.ts`
- this report and `p8yt-tool-jobs.json`

### Gates and exact census

| Gate | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: **146** self-tests |
| Writer retained-route verifier assertion | PASS in the full verifier; no Writer-specific failure |
| full deterministic tool-jobs JSON | Expected RED: **18** failure classes; 50 hosts, 50 invocations, 775 rows, 773 unique, 0 admitted, 884 residual, 8 reserved, 35 importers, 34 globals |
| broad interactivity DENY | Concurrent RED: one P1-owned testkit constructs an extra WorkerPool; no new Writer finding |
| scoped Rust formatting and diff check | PASS |
| Cargo/Nx/native/Wasm/runtime timing | Not run by coordinator instruction |

The structural direct-caller placeholder count is now **16** (Writer was 1 of 17 at the preceding
checkpoint). This decrement means only that the source route is live and ownership-retained; it is
not runtime or Phase-8 activation credit.

Verdict: **PASS for independent Terra source audit of the Writer retained-load cohort. REJECT for
runtime/Cargo/Wasm proof, ordinary whole-app terminal destruction beyond the newly cursor-owned
lanes, the remaining 16 envelope callers, the global ArtifactEnvelope/store structural failures,
the typed full operation, all 884 activations, and Phase 8.**

---

## 2026-08-23 — Trinity Jack Shared `.spr`/`.ops` Retained-Load Cohort

Trinity Jack is the next live caller on the Writer-proven app-owned ingress, maintenance,
initializer, replacement, and acknowledgement lifecycle. This cohort also closes the first shared
domain-neutral `.spr`/`.ops` edit decoder seam instead of giving Jack a private whole-entry parser.

### Shared schema-first edit authority

`artifact_owned_spr_edit_history_decoder` now consumes the repository-owned token cursor directly.
It has an exact ten-field Edit catalog and retains string, mutation-array, mutation-target,
reservation, published value, and rejection retirement owners. Its mutation array:

- fallibly reserves the fixed 64-item capacity before constructing or copying a mutation owner;
- delegates every scalar/object entry to the domain's exact `begin_mutation` catalog;
- advances at most one schema token or one publication/retirement owner per grant;
- validates operation, generation, and cancellation before mutation admission/publication;
- restores a rejected owner to the exact target on capacity/publication failure; and
- uses terminal Drop assertions over `ManuallyDrop` authorities, while empty fixed-capacity vectors
  release only their shallow allocation after every mutation has been cursor-retired.

The outer ingress remains the established fixed 4,096-byte page protocol with exact page and total
byte credits. Jack's snapshot and mutation scalar packs are each limited to one 4,096-byte field
owner. `ArtifactPack::decode_pack` and `OpBinary::decode_op` therefore process one source-bounded
semantic field, but their under-8ms runtime timing remains unmeasured and is not claimed here.

### Jack domain ownership and initialization

`JackEnvelopeOwnedFieldCatalog` supplies concrete snapshot, mutation, VCS, conflict, Edit, snapshot
retirement, and mutation retirement owners. It calls the shared retained Edit decoder; the old
bounded repository serde fallback is absent from the Jack catalog.

Jack retirement disassembles nested graph ownership one exact item/string at a time. In particular,
the composed graph child keeps its exact ownership through decode, retry, candidate creation, and
close: `child_id`, `artifact_id`, dialect `artifact_kind`, `standard`, and `subset` are distinct
grant-accounted retirement phases. Node/edge/port kind definitions, property definitions,
`ValueType::List` boxes and schema strings, property maps/arrays, ports, nodes, edges, entity ids,
and mutation payloads have the same retained terminal witness.

`JackStoreInitializationAuthority` validates the envelope and edit pairs, clones the initial graph
into pre-admitted target storage one field/manifest item per turn, seeds the four fixed VCS history
ledgers, applies forward mutations, hashes inverse mutations incrementally, builds the exact
checked `generation + 1` candidate, and hands all domain owner factories into
`ArtifactStore::from_initialized_runtime_with_owners`. Cancel, stale generation, decode fault,
candidate rejection, and close all drive the same retained owner graph to terminal empty. Store
publication remains the shared atomic generation-validated swap, followed by cursor retirement of
the displaced store before acknowledgement.

### Live ABI and zero-reachability census

The Jack Wasm bridge no longer accepts an envelope string or calls `ArtifactStore::new`. Its live
surface is `beginEnvelopeLoad` → `admitEnvelopePage(Uint8Array)` → `sealEnvelopeLoad` → one
`maintenance_step` plus `pollEnvelopeLoad` turn → exact replacement ACK, with explicit cancel and
`closeStep`. A hostile page length is rejected before copying into the fixed Rust page. No loop or
run-to-completion callback exists in this bridge.

The census has **16 textual occurrences** of
`reject_whole_buffer_artifact_envelope_ingress`: exactly **one** is the shared fail-closed definition
and **15** are still-live structural caller placeholders. Jack has zero production occurrences; its
only direct `ArtifactStore::new` occurrence is a pre-existing Rust test fixture. The 15 live
residual callers are Raster, Cad, GisMap, Shooting, Procedural2d, Procedural3d, Dag, Flow, Trinity
Rewrite, Draw, Fem2d, Fem3d, Puzzle3d, Puzzle5d, and Process3d. Structural placeholder count is
therefore **15**, while command activation remains separately **0/884**.

### Permanent evidence

Five Jack Rust fixtures were authored:

- checked next-generation initializer publication and incremental candidate close;
- initializer cancel and stale-generation exact terminal ownership;
- nested mutation plus exact graph-child retirement within one item/4,096-byte grant;
- live submit → decode → initialize → swap → displaced-store retirement → exact-once ACK; and
- partial-page cancel without publication.

They reuse the two existing shared ingress fixtures for fixed capacity/+1 collision, exact rejected
owner FIFO close, zero-item interruption, and one-real-page-per-grant cancellation. Six new verifier
mutations reject a private/fallback edit decoder, post-lift dynamic page, missing ACK,
false-terminal initializer drop, whole-buffer string mutation decode, and a missing retained Jack
route. Malformed/truncated/unknown/duplicate/oversized field behavior is additionally held by the
shared owned-schema decoder and exact catalog diagnostics. Rust fixtures were not executed because
Cargo/native/Wasm was explicitly prohibited; only the six verifier mutations executed in this
packet.

### Exact files and gates

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
- `📜️script.ts`
- this report, `p8yt-jack-tool-jobs.json`, and `p8yt-jack-tool-jobs-repeat.json`

| Gate | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | PASS: **152** self-tests clean |
| Jack retained-route verifier assertion | PASS; the Jack-specific failure is absent |
| full deterministic tool-jobs JSON | Expected RED: **18** failure classes; 50 hosts, 50 invocations, 775 rows, 773 unique, 0 admitted, 884 residual, 8 reserved, 35 importers, 34 globals; two independently generated ledgers are byte-identical under `cmp` |
| `bun ./📜️script.ts verify interactivity --format json` | PASS: DENY clean in its declared four UI roots; recorded test-only blocking bridge only |
| scoped Rust formatting | PASS for the shared store and three Jack Rust owners; the graph manifest's new retirement region is formatted, while its unrelated pre-existing whole-file rustfmt drift was deliberately not rewritten |
| scoped and whole `git diff --check` | PASS |
| Cargo/Nx/native/Wasm/browser/runtime timing | Not run by coordinator instruction |

Verdict: **PASS for independent Terra source audit of the Jack shared-decoder retained-load cohort.
REJECT for runtime/native/Wasm proof, the source-bounded field timing claim, the remaining 15 live
whole-buffer callers, global ArtifactEnvelope/store structural failures, full typed operation,
all 884 command activations, and Phase 8.**

### 2026-08-23 — Terra Jack Formatting Re-audit Repair

Terra's focused audit rejected only canonical formatting in three cohort-owned files. No behavior,
authority, fixture, verifier rule, count, or ABI changed in this repair. Canonical
`rustfmt --edition 2021 --config skip_children=true` was applied to exactly:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`;
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`; and
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`.

Inspection found formatter-only import ordering and layout changes. The exact five-file Terra check,
including the unchanged graph manifest and Jack Wasm bridge, now exits 0. Post-repair gates are:

| Gate | Repair result |
| --- | --- |
| canonical five-file `rustfmt --edition 2021 --check --config skip_children=true` | PASS |
| tool-job verifier self-tests | PASS: **152** clean, unchanged |
| Jack retained-route assertion | PASS; no Jack-specific failure |
| deterministic ledgers | PASS: byte-identical, both SHA-256 `7812b2190f74f54d814f1b62124b73deb9a594cd1a747203d86fe6986038c63c` |
| full tool-job census | Expected RED and unchanged: 18 classes, 0/884, 8 reserved, 35 importers, 34 globals |
| structural whole-buffer census | Unchanged: 16 symbols = one shared fail-closed definition + 15 live callers; Jack production zero |
| broad interactivity DENY | PASS in its declared scope; recorded test-only blocking bridge only |
| scoped and whole working/staged/HEAD diff checks | PASS |

Focused verdict: **the Terra formatting rejection is repaired and the Jack cohort is ready for
re-audit. Phase 8 remains RED with the exact previously reported runtime and roster residuals.**

---

## 2026-08-23 — GIS Map Shared `.spr`/`.ops` Retained-Load Cohort

GIS Map is the next live caller on the shared fixed-page envelope ingress, owner-supplied retained
`.spr`/`.ops` Edit decoder, app maintenance pump, store initializer, generation-validated replacement,
displaced-store retirement, and exact acknowledgement path. No tiled-map, World3D, renderer, P1
database, or engine file was touched.

### Domain catalog and exact owners

`GisMapEnvelopeOwnedFieldCatalog` supplies exact snapshot, mutation, VCS, rejected-conflict, snapshot
retirement, and mutation retirement authorities. Its Edit field uses
`artifact_owned_spr_edit_history_decoder`; there is no private Vec/HashMap edit parser or whole-envelope
decoder in the live GIS Map route. Snapshot and mutation packed fields retain
`OwnedSchemaHexAuthority<GIS_MAP_OWNED_FIELD_BYTES>` and are admitted before their domain owner is
published.

`GisMapOwnedRetirement` disassembles the ordered positions, routes, and regions one feature at a time.
Each feature retires its id and recursively retires `DslValue::{String,Array,Object}` ownership. The
exact drawing, optional image, and value child handles each retain and retire `child_id`,
`artifact_id`, dialect `artifact_kind`, `standard`, and `subset` separately. All twelve mutation
variants have explicit retirement taxonomy: create/delete/reorder/replace for position, route, and
region. A zero-item grant returns `Pending { 0, 0 }` without detaching or advancing an owner; empty
phase transitions do not claim a released item.

`GisMapSnapshotCloneAuthority` preserves positions/routes/regions order by indexed source traversal
and ordered target insertion. Child fields are copied separately after exact string admission.
`GisMapStoreInitializationAuthority` validates the envelope and duplicate edits, clones the initial
snapshot, seeds ordered history, applies forward mutations, hashes inverse/redo mutations, and builds
the exact checked `generation + 1` candidate with GIS Map's required owner bundle. Every cancel,
stale-generation, fault, rejected candidate, and displaced snapshot follows retained cursor close;
ordinary incomplete authority Drop asserts terminal-empty.

The individual snapshot/feature pack decode and clone steps are source-bounded by the established
4,096-byte domain field/page limit. Native runtime timing under hostile valid payloads was not measured,
so no `<8ms` runtime claim is made.

### Live ABI and zero reachability

The GIS Map Wasm bridge now owns `VcsArtifactApp<EditorApp<Gis2dPlayApp>>` and exposes only:

1. `beginEnvelopeLoad(maximumPages, maximumBytes)` for fixed slot/item/byte admission;
2. `admitEnvelopePage(handle, Uint8Array)`, rejecting length before the fixed Rust page copy;
3. `sealEnvelopeLoad(handle)`;
4. `pollEnvelopeLoad(handle)`, which advances one app maintenance step and one operation poll;
5. exact replacement acknowledgement after `Ready`, with duplicate ACK returning false;
6. `cancelEnvelopeLoad(handle)`; and
7. one bounded `closeStep`.

The old whole-string constructor/direct `ArtifactStore` route is absent. The exact source census is
**15** `reject_whole_buffer_artifact_envelope_ingress` occurrences: **one** shared fail-closed
definition plus **14** still-live structural callers. GIS Map has zero production occurrences.
Structural placeholder count is therefore **14**. This count is independent of command activation;
the full command roster remains **0/884**.

### Permanent source evidence

Authored Rust fixtures cover checked next-generation candidate publication and incremental candidate
close, cancellation and stale generation to terminal-empty, recursive nested value disposal, exact
drawing/image/value child disposal, all twelve mutation variants under one-item grants, zero-item
ownership preservation, live submit through maintenance/swap/displaced retirement, exact-once plus
duplicate ACK, and partial-ingress cancellation without publication. These Rust fixtures were not
executed because Cargo/native/Wasm remained prohibited.

The verifier now has eleven GIS Map route assertions/mutations. They reject loss of the shared Edit
decoder, a post-lift dynamic byte slice, completion without exact ACK, false-terminal initializer
drop, unchecked generation, drawing-child deep drop, nested-value deep drop, a missing mutation
variant, missing zero-grant/catalog evidence, a whole-buffer ingress bypass, and absence of the full
retained GIS route. Shared owned-schema and ingress fixtures continue to cover capacity/+1 collision,
malformed/truncated/unknown/duplicate/oversized fields, exact rejected owner return, interrupted close,
false terminal, saturation, cancellation, and one-real-page-per-grant cleanup.

### Exact files and gates

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`
- `📜️script.ts`
- this report, `p8yt-gis-map-tool-jobs.json`, `p8yt-gis-map-tool-jobs-repeat.json`, and their
  expected-RED `.stderr` captures

| Gate | Result |
| --- | --- |
| canonical scoped Rust format/check | PASS on the three GIS Map Rust files |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | PASS: **163** self-tests clean |
| GIS Map retained-route assertion | PASS; no GIS Map-specific failure in the full verifier |
| deterministic full tool-job ledgers | PASS: byte-identical, SHA-256 `05dbfee879ca84ef57eb15c932eca39c2f782dcc7261b042ba1f2a7b23a9c04b` |
| full tool-job census | Expected RED: **18** failure classes; 50 hosts, 50 invocations, 775 rows, 773 unique, **0/884**, 8 reserved, 35 importers, 34 globals |
| broad interactivity DENY | PASS: declared scope clean; recorded test-only blocking bridge only |
| scoped and whole working/staged/HEAD diff checks | PASS |
| Cargo/Nx/native/Wasm/browser/runtime timing | Not run; no compile or runtime PASS claimed |

Verdict: **PASS for independent Terra source audit of the GIS Map retained-load cohort. REJECT for
native/Wasm/runtime proof, source-bounded decode timing, the remaining 14 live whole-buffer callers,
global ArtifactEnvelope/store structural failures, the full typed operation, all 884 activations,
and Phase 8.**

---

## 2026-08-23 — Draw Shared `.spr`/`.ops` Retained-Load Cohort

Draw is the next live caller on the accepted shared fixed-page envelope ingress, owner-supplied
retained `.spr`/`.ops` Edit decoder, app maintenance pump, store initializer,
generation-validated replacement, displaced-store retirement, and exact acknowledgement path. No
Procedural2d, Raster, P3 renderer/World3D, P1 database, or engine file was touched.

### Recursive domain catalog and exact owners

`DrawEnvelopeOwnedFieldCatalog` supplies exact snapshot, mutation, VCS, rejected-conflict, snapshot
retirement, and mutation retirement authorities. Its Edit field uses
`artifact_owned_spr_edit_history_decoder`; there is no private Vec/HashMap Edit decoder or
whole-envelope decoder in the live Draw route. Snapshot and mutation packed fields retain
`OwnedSchemaHexAuthority<DRAW_OWNED_FIELD_BYTES>` and are admitted before their domain owner is
published.

`DrawRetirementOwner` disassembles all seven layer variants, their base fields and attributes, fills,
strokes, stops, path segments, polygon points, Group children, image assets, strings, and boolean
operand identifiers one exact owner at a time. All fourteen mutation variants have explicit
retirement taxonomy: visibility, lock, opacity, blend, rename, transform, fill, stroke, boolean,
trace, create, duplicate, delete, and reorder. A zero-item grant returns
`Pending { released_items: 0, released_bytes: 0 }` without detaching or advancing ownership.

`DrawSnapshotBoundsAuthority` preflights the recursive source before construction with fixed
path/frame authority and checked item, byte, and depth credits: 4,096 nested items, the established
fixed envelope byte cap, and depth 64. It advances one layer, child, string, or asset per step.
`DrawLayerCloneAuthority` constructs typed layer skeletons and advances one string, stop, dash,
point, path segment, boolean operand, or Group child per grant. `DrawSnapshotCloneAuthority` first
requires the terminal preflight witness, then advances top-level fields, one nested layer cursor, one
asset field, and the artboard. The production seam has no recursive source clone, whole-tree JSON
materialization, or recursive Drop.

`DrawStoreInitializationAuthority` validates the envelope and duplicate edits, incrementally clones
the initial snapshot, seeds ordered history, applies forward mutations, hashes inverse/redo
mutations, and builds the exact checked `generation + 1` candidate with Draw's required owner
bundle. Every cancel, stale generation, false terminal, fault, rejected candidate, and displaced
snapshot follows retained cursor close. Ordinary incomplete authority Drop requires the
terminal-empty witness.

The individual packed snapshot/mutation decode step remains source-bounded by the established
4,096-byte field/page limit. Native timing under hostile valid recursive payloads was not measured,
so no `<8ms` runtime claim is made.

### Live ABI and zero reachability

The Draw Wasm bridge now owns `VcsArtifactApp<EditorApp<DrawPlayApp>>` and exposes only:

1. `beginEnvelopeLoad(maximumPages, maximumBytes)` for fixed item and byte admission;
2. `admitEnvelopePage(handle, Uint8Array)`, rejecting length before the fixed Rust page copy;
3. `sealEnvelopeLoad(handle)`;
4. `pollEnvelopeLoad(handle)`, advancing one app maintenance step and one operation poll;
5. generation-validated replacement followed by exact acknowledgement after `Ready`;
6. `cancelEnvelopeLoad(handle)`; and
7. one bounded `closeStep`.

The old whole-string constructor/direct `ArtifactStore` route is absent. The exact repository census
is **14** `reject_whole_buffer_artifact_envelope_ingress` symbols: **one** shared fail-closed
definition plus **13** still-live structural callers. Draw has zero production occurrences.
Structural placeholder count is therefore **13**. This count is independent of activation; the full
command roster remains **0/884**.

### Permanent source evidence

Authored Rust fixtures cover a recursive Group containing a Path plus an image asset through live
submit, maintenance, swap, displaced retirement, and exact acknowledgement; duplicate ACK; and
partial-ingress cancellation without publication. The retained authorities themselves encode fixed
item/byte/depth preflight, all seven layer variants, all fourteen mutation variants, zero-grant
ownership preservation, exact cancellation/stale/fault retirement, and terminal-empty Drop. These
Rust fixtures were not executed because Cargo/native/Wasm remained prohibited.

The verifier adds eleven Draw route assertions/mutations. They reject loss of the shared Edit
decoder, a resizable recursion stack, missing nested byte preflight, a recursive source clone, direct
asset Drop, a missing mutation variant, post-lift dynamic page ownership, completion without exact
ACK, whole-buffer constructor ingress, false-terminal initializer Drop, and absence of the full
retained Draw route. Shared owned-schema and ingress fixtures continue to cover capacity/+1,
malformed/truncated/unknown/duplicate/oversized fields, exact rejected-owner return, interrupted
close, false terminal, saturation, cancellation, and one-real-page-per-grant cleanup.

### Exact files and gates

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧰️owned/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
- `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📦️glue.rs`
- `📜️script.ts`
- this report, `p8yt-draw-tool-jobs.json`, and `p8yt-draw-tool-jobs-repeat.json`

| Gate | Result |
| --- | --- |
| canonical scoped Rust format/check | PASS on the three Draw Rust files |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | PASS: **174** self-tests clean |
| Draw retained-route assertion | PASS; no Draw-specific failure in the full verifier |
| deterministic full tool-job ledgers | PASS: byte-identical, SHA-256 `59dfda87dd276d9b1ecd9d7e12782108838e983da977f327254b30f2d2e47b07` |
| full tool-job census | Expected RED: **18** failure classes; 50 hosts, 50 invocations, 775 rows, 773 unique, **0/884**, 8 reserved, 35 importers, 34 globals |
| broad interactivity DENY | PASS: declared scope clean; recorded test-only blocking bridge only |
| scoped and whole working/staged/HEAD diff checks | PASS |
| Cargo/Nx/native/Wasm/browser/runtime timing | Not run; no compile or runtime PASS claimed |

Verdict: **PASS for independent Terra source audit of the Draw retained-load cohort. REJECT for
native/Wasm/runtime proof, hostile-recursive-payload timing, the remaining 13 live whole-buffer
callers, global ArtifactEnvelope/store structural failures, the full typed operation, all 884
activations, and Phase 8.**

---

## 2026-08-23 — Draw Independent-Rejection Remediation

This section supersedes only the earlier Draw source verdict. The independent Sol audit correctly
rejected the cohort because forward replay escaped through `operation.diff(current)` into a whole
`snapshot.clone()`, synchronous serde reconstruction, and an asset-map clone; mutation admission
also called whole-output `encode_op()` before checking its byte bound, and pair validation missed a
single or final edit ID. The remediation keeps Draw at the same zero-placeholder census while
replacing those live transitive paths with retained typed authorities.

### Retained mutation replay and admission

`DrawMutationCandidateAuthority` now owns an incrementally cloned candidate, a fixed-depth layer
locator, typed field-clone cursors, and an owner-moving container rebuild cursor. It applies all
fourteen mutation variants one semantic field, child, stop, dash, path entry, or layer owner per
`StepContext` grant. Structural create, duplicate, delete, and reorder reserve the destination
pointer capacity before moving an owner; the source, reverse buffer, output, pending item, and
removed item remain in `ManuallyDrop` authorities until terminal publication or bounded close.
Layer order is preserved by the reverse-buffer transfer. Publication replaces the initializer's
current candidate only after the mutation authority reports terminal complete; the exact displaced
snapshot enters the existing one-owner retirement cursor.

`DrawMutationDigestAuthority` replaces every live `operation.encode_op()` call. It advances one
typed mutation field, string, gradient stop, stroke dash, or retained layer clone/retirement per
grant and enforces the established 4,096-byte mutation-field credit before owned materialization.
It supplies forward, inverse, redo-forward, and redo-inverse history digests without building a
whole encoded operation. The owned live source has zero occurrences of `operation.diff`,
`diff.apply`, `snapshot.clone()`, `operation.encode_op()`, or serde reconstruction/serialization.

`ValidateEditId` now advances across every history entry independently of duplicate-pair scanning.
It rejects an empty or over-cap ID and over-cap optional actor, start timestamp, mutation timestamp,
or mutation ID, including a history containing one edit and the final edit of a longer history.
The live Draw envelope fixture now contains a real retained `RenameLayer` history edit and verifies
the published nested Path name, so success no longer proves only an empty-history route.

### Exact ownership evidence

Three executable Rust authority fixtures were added to the owned Draw module:

1. `retained_draw_mutation_candidate_covers_all_fourteen_variants_and_returns_exact_owners`
   exercises every semantic mutation through one-fuel steps, including structural layer ownership,
   typed fill/stroke owners, duplicate rewriting, and exact close;
2. `retained_draw_depth_plus_one_and_hostile_fields_fault_then_close_terminal_empty` covers the
   recursive cap +1 and hostile nested-field rejection with retained terminal cleanup; and
3. `retained_draw_container_false_terminal_saturation_and_interrupted_close_preserve_exact_owner`
   proves false-terminal rejection, reserved-capacity saturation, zero/one-grant interruption, and
   exact structural owner handback.

The live editor fixture
`draw_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate` exercises
both ID holes through submit, maintenance, terminal fault, and unchanged generation. Existing live
success/ACK, duplicate-ACK, displaced retirement, partial-ingress cancellation, stale generation,
and terminal-close fixtures remain in place. These Rust fixtures are permanent source evidence but
were not executed because Cargo/native/Wasm remained prohibited.

The permanent verifier now rejects reintroduction of whole snapshot clone, whole operation encode,
diff replay, serde reconstruction, skipped final-ID validation, missing retained candidate/digest/
container authorities, missing authority-close evidence, missing populated live history, or missing
live single/final-ID evidence. Seven new adversarial verifier mutations bring the tool-job suite to
**182** self-tests.

### Exact census and gates

The repository still has exactly **14**
`reject_whole_buffer_artifact_envelope_ingress` occurrences: **one** shared fail-closed definition
plus **13** live structural callers. Draw has **zero** occurrences. The structural Draw decrement
therefore remains 13, but cohort acceptance is deliberately left to an independent re-audit. No
command activation follows from this census; the full roster remains **0/884** with **18** global
failure classes.

| Gate | Result |
| --- | --- |
| canonical scoped Rust format/check | PASS on Draw owned/editor/glue |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | PASS: **182** self-tests clean |
| Draw retained-route assertion | PASS; no Draw-specific failure in the full verifier |
| deterministic full tool-job ledgers | PASS: byte-identical, SHA-256 `21873b21e009a9b82d3a5a6497f8acbaf9e699187a77cd88890811392dcb7ba9` |
| full tool-job census | Expected RED: **18** failure classes; 50 hosts, 50 invocations, 775 rows, 773 unique, **0/884**, 8 reserved, 35 importers, 34 globals |
| broad interactivity DENY | PASS: declared scope clean; recorded test-only blocking bridge and predeclared future entries only |
| Draw placeholder/banned-path census | PASS: Draw zero placeholder and zero live clone/diff/serde/whole-encode patterns |
| scoped and whole working/staged/HEAD diff checks | PASS |
| Cargo/Nx/native/Wasm/browser/runtime timing | Not run; no compile, runtime, or `<8ms` PASS claimed |

Verdict: **READY for independent Draw source re-audit, without self-acceptance. Phase 8 remains RED.**
The native/Wasm/runtime route, hostile-valid-payload timing, 13 remaining live whole-buffer callers,
global snapshot/store ownership, full typed operation, all 884 activations, and Phase 8 remain
unproven or rejected.

---

## 2026-08-23 — Draw Semantic-Digest and Aggregate-Credit Re-Audit Repair

This section repairs only the blockers in
`sol-independent-p8-draw-retained-load-cohort-reaudit-2026-08-23.md`. It does not self-accept the
Draw cohort, decrement another structural placeholder, activate a command, or begin another
cohort. Draw remains pending an independent source re-audit; Phase 8 remains RED at **0/884** and
**18** global failure classes.

### Schema-complete retained semantic digest

`DrawMutationDigestAuthority` now frames every semantic observation with an explicit domain byte,
field tag, and big-endian length, while a repository-owned incremental SHA-256 authority absorbs
the same frames. The terminal SHA-256 is folded into the retained initialization revision digest;
no whole mutation encoding is constructed. One call to `step` observes at most one semantic
field/item.

The cursor taxonomy now covers:

- `None`/`Some` and Solid/Linear/Radial fill discriminants; every color component, coordinate,
  stop count, stop offset, and stop color component;
- stroke presence, every color component, width, cap, join, dash presence/count, and each dash;
- all layer variant discriminants; ID, name, visibility, lock, opacity, blend mode, all transform
  scalars, fill, and stroke;
- Shape kind and every optional Rect/Ellipse/Circle/Line/Polygon discriminator/scalar/point; every
  Move/Line/Quad/Cubic/Arc/Close path discriminator and scalar/flag; Text, Image, Group, Boolean,
  and Trace fields; deterministic child/operand/path/point ordering and asset/source references;
- the exact discriminant and every payload field of all fourteen Draw mutation variants,
  including Create parent/index and its full recursive layer.

`retained_draw_schema_digest_distinguishes_every_nested_semantic_field` is a runtime-free authority
fixture. It mutates formerly omitted fill, stroke, base, geometry, path Arc, layer variant/order,
image/trace reference, and all fourteen mutation payload authorities and requires distinct
semantic digests. Exact mutation owners are then drained through `DrawMutationRetirementFactory`.
These Rust fixtures were authored as permanent evidence but not executed because Cargo/native was
prohibited.

### Aggregate reservation before derived allocation

`DrawMutationCandidateAuthority` now starts in `PreflightSource`, then
`PreflightMutation`. The first fixed-depth cursor calculates source item/byte counts and the largest
live container. The second schema-complete cursor calculates the exact mutation semantic credits.
Only their terminal witnesses may construct `DrawMutationAggregateReservation`; only a successful
reservation may construct the snapshot clone authority.

The checked aggregate includes source ownership, the simultaneously retained candidate copy,
mutation-derived copies, one retained decode page, fixed path/frame/locator/digest backing, source
index backing, and the reverse/output/pending structural container slots. The fixed limits are
**4,096 aggregate items**
and **262,144 aggregate bytes**. Create/Duplicate/Reorder reserve the possible `+1` destination
slot before `Vec::try_reserve_exact`; `DrawContainerRebuildAuthority::new` rejects any capacity not
covered by the exact reservation and returns the untouched source/pending owners.

`retained_draw_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback` proves in
source that the exact 4,096-byte semantic field and exact aggregate boundaries admit, item/byte
`+1` reject, and a real 700-layer source fails aggregate admission before candidate construction
while its last-valid source remains unchanged. `retained_draw_cancel_stale_each_replay_candidate_container_stage_preserves_last_valid`
interrupts and cursor-closes exact authority at source preflight, mutation preflight, clone,
primary/secondary/destination location, owned-value preparation, scalar application, source and
destination rebuild, and terminal candidate stages. It asserts terminal-empty close and unchanged
last-valid source ownership.

### One-entry history preparation

`ValidateEditId` no longer calls `mutation_meta.iter().any`. It advances into
`ValidateEditMeta { edit, meta }` and validates exactly one metadata owner per grant. Applied ID,
optional actor, and redo ID allocations now occur in separate `PrepareApplied`/`PrepareRedo`
grants; cancellation or stale authority transfers any prepared string into the retained one-owner
close pump. Commit no longer creates the outer ID/actor owners in the same grant.

### Permanent verifier and exact gates

The Draw verifier now requires the SHA-256 terminal fold, fill/stroke/path/layer digest cursors,
every previously omitted semantic field marker, aggregate reservation and both cap faults, both
preflight phases, per-entry metadata validation, split history preparation, and all three new
authority fixtures. Forty-seven field-removal mutations plus SHA downgrade/seal loss, aggregate
item/byte loss, clone-before-preflight, whole metadata scan, multi-clone history preparation, and
retained-page credit loss, fixture-erasure mutations must all fail the intended predicate. The suite is now **245** clean
tool-job self-tests.

| Gate | Result |
| --- | --- |
| scoped canonical Rust format/check | PASS on the owned Draw file |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | PASS: **245** self-tests clean |
| Draw retained-route predicate in the full verifier | PASS; no Draw-specific failure |
| deterministic ledgers | PASS: byte-identical `p8yt-draw-tool-jobs.json` and repeat, SHA-256 `c6285afecde02b6005349bc05f24009996ab9c3a4842ce34fd5c9f1008617472` |
| full tool-job verifier | expected global RED: 50 hosts, 50 invocations, 775 rows, 773 unique, **0/884**, 8 reserved, 35 importers, 34 globals, **18** failures |
| broad interactivity self/plain DENY | PASS: declared scope clean; one recorded test-only blocking bridge and two predeclared future entries only |
| forbidden Draw replay/materialization census | PASS: zero source/snapshot whole clone, whole encode, diff/apply, serde reconstruction, or whole metadata scan in the owned file |
| placeholder census | repository **14** occurrences = one shared fail-closed definition + 13 live callers; Draw **0**; cohort count remains unaccepted pending re-audit |
| scoped and whole working/staged/HEAD diff checks | PASS |
| Cargo/Nx/native/Wasm/browser/network/runtime timing | not run by instruction; RED/unverified |

Exact changed evidence in this repair is the owned Draw component, `📜️script.ts`, this report, and
the two Draw ledger JSON files. Verdict: **READY for independent Draw source re-audit, without
self-acceptance.** Runtime behavior, hostile-valid-payload timing, remaining callers, 884 command
activations, and Phase 8 remain RED.

---

## 2026-08-23 — Draw Exact Owner-Census and Staged-Commit Repair

This repair addresses only the blockers in
`sol-independent-p8-draw-semantic-aggregate-credit-final-reaudit-2026-08-23.md`. It does not
self-accept Draw, decrement the independently accepted structural count, activate any tool, or
begin another cohort. The accepted structural placeholder census therefore remains **14** pending
re-audit; the repository still has one shared fail-closed definition plus 13 live callers, and Draw
has zero live placeholder reachability. Phase 8 remains RED at **0/884** and **18** global failure
classes.

### Fixed owner census and reservation

`DrawSnapshotBoundsAuthority` and `DrawSemanticDigestCredit` now fill
`DrawFixedOwnerCensus`, whose 4,096 direct slots exist from construction and cannot grow. Each slot
records separate source and derived item/byte authority. Snapshot traversal accounts source
`String` and `Vec` capacity, the planned candidate capacity, typed layer/fill/stroke/path/container
owners, ordered asset key/value owners, fixed traversal/index state, and the retained page. Mutation
traversal separately records the typed source mutation and every replacement `String`, `Vec`,
`Box`, style, path, layer, and structural owner before candidate construction. The aggregate
reservation exposes each simultaneously retained category explicitly: source, candidate,
mutation-source, mutation-derived, duplicate candidate, authority, structural containers, and
page. The former `semantic_total × structural_copies` approximation and fabricated
`exact_for_test` boundary are absent.

The live aggregate fixture drives the real source-bounds and mutation-digest cursors. It admits an
exact 4,096-byte semantic field, rejects 4,097 bytes, grows real layer owners to the first aggregate
item/byte rejection, verifies the last admitted totals remain within 4,096 items / 262,144 bytes,
and checks that the rejected source keeps the exact same `Vec` backing and unchanged last-valid
document identity. No owner is constructed from fabricated totals.

### One-owner history and duplicate stages

The shared store revision record no longer retains a second `String`. `CursorRevisionRecord` stores
the fixed `[u8; 32]` edit-ID digest; `push_applied` and `push_redo` hash a borrowed `&id` and then
move the sole prepared `String` into its fixed history lane. Revision close removes one shallow
fixed record per grant. Reintroducing the former `id.clone()` form is a permanent Draw verifier
failure.

`DrawDuplicateRewriteAuthority` is persistent across distinct opportunities for ID copy, name copy,
length-framed hash initialization, each at-most-4,096-byte hash page, new-ID reservation/format,
ID replacement, name-suffix reservation, name append, and displaced-ID retirement. Pending and
retired IDs remain in terminal-asserting `ManuallyDrop` slots. Asset preflight uses
`DrawAssetBoundsCursor` plus ordered `BTreeMap::range`; the former `iter().nth` rescan is absent.

### Live cancellation and stale authority

Every `DrawMutationCandidateAuthority::step` validates the exact operation and generation and then
the real cancellation token before source preflight, mutation preflight, clone, primary/secondary
location, owned-value preparation, scalar application, source rebuild, destination location,
destination rebuild, and terminal candidate handoff. The adversarial fixture reaches every named
phase, injects either `cancel_now()` or a mismatched generation, observes the exact fault, pumps
the retained authority to terminal-empty, and verifies that no partial candidate replaced the
last-valid source.

### Permanent verifier and evidence

The Draw predicate now requires the fixed census/slots, separate source and derived owner credits,
fixed asset range cursor, staged duplicate authority, borrowed history-ID hashing, real aggregate
fixture, and live cancellation/stale fixture. It rejects a fabricated boundary, resizable owner
census, semantic-copy approximation, asset `iter().nth`, String revision record, one-step duplicate
rewrite, close-only cancellation, missing item/byte caps, and missing retained page credits. The
unrelated accepted Jack predicate was intentionally left independent of these Draw-only markers.

| Gate | Result |
| --- | --- |
| scoped canonical Rust format/check | PASS on Draw owned and shared store source |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | PASS: **245** self-tests clean |
| Draw retained-route predicate | PASS mechanically; no Draw-specific failure |
| deterministic full verifier ledgers | PASS: byte-identical `📊️p8yt-draw-owner-credit-ledger-a.json` / `-b.json`, SHA-256 `c6285afecde02b6005349bc05f24009996ab9c3a4842ce34fd5c9f1008617472` |
| full verifier census | expected RED: 50 hosts, 50 invocations, 775 rows, 773 unique, **0/884**, 8 reserved, 35 importers, 34 globals, **18** failures |
| broad interactivity DENY | concurrent RED: five P1-owned DB-engine retained-history findings; no Draw finding or allowlist change |
| Cargo/Nx/native/Wasm/browser/network/runtime timing | not run by instruction; RED/unverified |

Exact repair files are the owned Draw component, shared store component, `📜️script.ts`, this report,
and the two `📊️p8yt-draw-owner-credit-ledger-*.json` evidence files. Verdict: **READY for focused
independent Draw source re-audit, without self-acceptance.** Runtime behavior, exact allocator/timing
evidence, hostile-valid-payload timing, remaining structural callers, all 884 activations, and
Phase 8 remain RED.

---

## 2026-08-23 — Draw Fixed-Owner Overlay and Framed-Identity Repair

This section repairs the source blockers recorded by
`sol-independent-p8-draw-exact-owner-staged-commit-reaudit-2026-08-23.md`. It does not
self-accept Draw, change the independently accepted structural count, activate a command, or start
another cohort.

### Pre-admitted owners and overlay publication

The live initializer no longer clones the initial `DrawSnapshot`. After per-entry envelope and
history validation, it hashes bounded identity fields, moves the exact initial snapshot owner out
of the envelope, and installs it into
`ArtifactStoreInitializationRuntime::new_with_owner_catalog`. The catalog pre-reserves the
applied, redo, and both revision ledgers before the initializer authority is constructed; the
runtime consumes those exact returned capacities.

`DrawMutationCandidateAuthority::try_new` now owns, before its admission pass:

- two fixed 64-slot `DrawLayerNode` container arenas;
- sixteen fixed 4,096-byte String/page owners whose allocator-returned capacities are summed;
- the fixed duplicate-ID String owner;
- both 4,096-slot fixed census authorities and the candidate shell itself.

Aggregate admission reads the actual returned `Vec::capacity` and `String::capacity` of these
retained owners. It no longer infers derived capacity from logical length. Snapshot assets remain
the exact moved immutable `BTreeMap` base; the synthetic BTree node-pointer constant and
candidate-map rebuild accounting are gone. Scalar string mutations consume a preallocated page
owner. Structural publication uses `DrawMutationOverlayPatch`, validates the exact source
address on every turn, rebuilds through the retained fixed container arenas, and exposes the
candidate only after the overlay terminal witness. Unused page owners are released one per
candidate/close opportunity.

The live initial snapshot, asset map, and history lanes therefore move into final store
construction; final construction does not rebuild or clone their `BTreeMap`, history `Vec`, or
identity `String` owners. Existing recursive layer/style clone cursors remain one semantic
field/item per grant and are covered by the fixed derived-page credit; native allocator and timing
behavior remain unexecuted under the no-build instruction.

### Duplicate identity and adversarial authority

`DrawDuplicateRewriteAuthority` now hashes these frames in order:

1. `semio.draw.duplicate-id.v1`;
2. big-endian ID byte length;
3. ID bytes in at-most-4,096-byte pages;
4. big-endian name byte length;
5. name bytes in at-most-4,096-byte pages.

The output String is the preallocated fixed owner retained by the candidate, not a post-admission
allocation. The permanent split-boundary fixture proves `("ab", "c")` and `("a", "bc")`
mint distinct IDs.

The exact-boundary fixture now drives the real candidate authority. A live 4,096-byte rename
consumes a preallocated page and preserves the source container pointer; 4,097 bytes faults and
returns that exact pointer without publication. Aggregate growth continues until the first real
item/byte rejection and records only allocator-returned capacities. A live editor fixture
interrupts the retained ingress/initializer after multiple maintenance depths, injects a stale
generation, cancels the exact handle, checks unchanged last-valid generation/document identity,
then separately proves stale ACK rejection, exact ACK, and duplicate-ACK idempotence. Rust
fixtures were not executed because Cargo/native/Wasm were explicitly prohibited.

### Permanent verifier and gates

Thirteen permanent verifier mutations now reject removal or replacement of fixed overlay/container
arenas, the overlay owner witness, pre-admitted store catalog, initial-owner move, any duplicate
domain/ID-length/name-length frame, the split-boundary fixture, or the live staged
cancel/stale/ACK fixture. The verifier also rejects the former synthetic BTree node constant and
`CloneInitial` route.

| Gate | Result |
| --- | --- |
| edition-2021 rustfmt write/check | PASS: Draw owned/editor and shared store |
| tool-job self-tests | PASS: **258** clean |
| Draw retained predicate | PASS mechanically; no Draw-specific full-verifier failure |
| deterministic ledgers | PASS: byte-identical `📊️p8yt-draw-fixed-overlay-ledger-a.json` / `-b.json`, 312,305 bytes, SHA-256 `b24a25c754648a9792fe9348ee141ddf48c1b9d0f5af807d88c4cab44fe481b8` |
| full tool-job census | expected global RED: 50 hosts, 50 invocations, 775 rows, 773 unique, **0/884**, 8 reserved, 35 importers, 34 globals, **18** failures |
| broad interactivity self/plain DENY | PASS: one recorded test-only bridge and two predeclared future entries only |
| production placeholder census | 14 source occurrences = one shared fail-closed definition + 13 live callers; Draw zero; no count change before independent acceptance |
| scoped/whole working, staged, and HEAD diff checks | PASS |
| Cargo/Nx/native/Wasm/browser/network/runtime timing | not run; compile/runtime/`<8ms` remain unverified |

Exact repair files are the Draw owned component, Draw editor, shared store component,
`📜️script.ts`, this report, and the two fixed-overlay ledgers. Verdict: **READY for independent
source re-audit, without self-acceptance.** Phase 8 remains RED at **0/884** and **18** global
failure classes.

---

## 2026-08-23 — Draw Process-Arena Admission and Duplicate-Name Repair

This narrow repair addresses only the two blockers in
`coordinator-independent-p8-draw-fixed-owner-overlay-audit-2026-08-23.md`. It does not begin a
new cohort, self-accept Draw, decrement the independently accepted structural census, or activate
any command. The structural census therefore remains **14** pending independent re-audit: one
shared fail-closed definition plus 13 other live callers, with Draw at zero live placeholder
reachability. Phase 8 remains RED at **0/884** and **18** global failure classes.

### Process-retained fixed arena authority

The container/page/duplicate scratch owners are no longer allocated by
`DrawMutationCandidateAuthority::try_new`. `DrawPlayApp::default` initializes one retained process
pool before any Draw operation can be admitted. The pool owns exactly four generation-tagged
bundles. Each bundle contains two preallocated 64-slot `DrawLayerNode` vectors, sixteen
preallocated 4,096-byte String pages, and one preallocated duplicate-ID String. Pool construction
records the allocator-returned item and byte authority of every bundle and fails closed if the
fixed process claim exceeds four times the 4,096-item / 262,144-byte operation ceiling.

Candidate admission performs only a nonblocking fixed-slot borrow. Capacity `+1`, mutex
contention, generation exhaustion, stale ABA return, occupied-field collision, and nonterminal return are
explicit faults. Both success and close return the exact generation-tagged slot. The container
rebuild now uses the two pooled vectors only as scratch: after the ordered edit is built, it moves
one layer per grant back into the original source `Vec`, publishes that original backing, and
returns both empty scratch vectors. It preflights the original source capacity before any move.
No candidate arena `reserve` or allocation remains.

Pool return is itself a retained four-phase cursor. It moves only one fixed root per grant in this
order: reverse vector, output vector, page catalog, duplicate-ID String. Contention returns
`Blocked` without detaching the next owner; each successful partial handback truthfully returns
`Pending { released_items: 1 }`; only the fourth generation-validated handback makes the slot
available again. An interrupted close therefore cannot deep-drop either a partial bundle or its
remaining candidate owners.

`retained_draw_process_arena_pool_cap_plus_one_returns_exact_slots_and_rejects_stale_aba` uses an
isolated live pool authority, borrows all four slots, proves the fifth admission rejects, closes
each candidate, reborrows all four slots, and compares the exact reverse/output/page/duplicate
allocation pointers while requiring an advanced generation. The fixture is isolated from the
production singleton so parallel tests cannot consume its capacity.

### Duplicate-name staging without live growth

`DrawDuplicateRewriteAuthority` now retains both the fixed duplicate-ID owner and one exact
4,096-byte name page. After the existing domain + ID-length/bytes + name-length/bytes SHA-256
frames complete, it validates the destination ID/name capacities and scratch-page capacity before
modifying the unpublished candidate. ID copy, original-name copy, suffix append, and final name
write are separate fuel opportunities. The live destination String owner is updated in place only
after capacity validation; the scratch owners are cleared and returned to their exact pool slot.
There is no `base.name.try_reserve_exact`, post-admission reserve, displaced scratch leak, or
ordinary owner Drop on success, rejection, cancellation, or close.

`retained_draw_duplicate_name_uses_preadmitted_page_and_returns_exact_rejection_owner` proves the
successful `"Layer copy"` path preserves the original last-valid name backing, then drives an
unadmitted destination through the exact capacity fault and verifies the exact source container
pointer is returned. The framed collision fixture remains intact.

### Permanent verifier and exact evidence

The Draw predicate now requires the process pool, fixed capacity, pre-admission app initializer,
nonblocking borrow, actual capacity claim, generation-tagged return, terminal bundle witness,
original-container restoration, retained duplicate-name owner, destination preflight, pool
cap/+1/pointer/ABA fixture, and the live duplicate-name fixture. Thirteen new adversarial mutations
reject per-candidate allocation, missing pool/borrow, nonterminal return, stale generation,
post-admission name reserve, unretained name authority, missing live-container preflight,
publishing the scratch backing, missing handback fixture, and late pool initialization.

| Gate | Result |
| --- | --- |
| edition-2021 scoped rustfmt write/check | PASS: Draw owned and Draw editor |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | PASS: **271** clean |
| Draw retained-route predicate | PASS mechanically; the full verifier reports no Draw-specific failure |
| deterministic verifier ledgers | PASS: byte-identical `📊️p8yt-draw-process-arena-ledger-a.json` / `-b.json`, 312,305 bytes each, SHA-256 `bac5c05663843af9f70290b449192e6844b59cd8c836b4408761e09ec7e3861f` |
| full tool-job census | expected global RED: 50 hosts, 50 invocations, 775 rows, 773 unique, **0/884**, 8 reserved, 35 importer owners, 34 globals, **18** failures |
| broad interactivity DENY | PASS: one recorded test-only bridge and two predeclared future entries only |
| forbidden Draw allocation/materialization scan | PASS: zero post-admission duplicate-name reserve, candidate arena allocation, whole snapshot clone, serde reconstruction, whole encode, or asset `iter().nth` |
| placeholder census | repository **14** source occurrences = one shared fail-closed definition + 13 other live callers; Draw **0**; no structural count change before independent acceptance |
| scoped/whole working, staged, and HEAD diff checks | PASS |
| Cargo/Nx/native/Wasm/browser/network/runtime timing | not run by instruction; compile/runtime/`<8ms` remain unverified |

Exact files for this repair are the owned Draw component, Draw editor, `📜️script.ts`, this
report, and the two process-arena ledger JSON files. Verdict: **READY for focused independent Draw
source re-audit, without self-acceptance.** Runtime behavior, native allocator/timing evidence, the
13 remaining live callers, all 884 command activations, and Phase 8 remain RED.

---

## 2026-08-23 — Draw Rebuild Rollback and Bootstrap-Fault Repair

This packet repairs only the two blockers in
`sol-independent-p8-draw-process-arena-duplicate-name-repair-audit-2026-08-23.md`. It does not
self-accept Draw or start another cohort.

### Exact rebuild rollback and pool handback

`DrawContainerRebuildAuthority` now records every source/reverse/output/pending/removed owner
move in a fixed 257-entry move log. Move-log capacity and every destination owner are validated
before detaching the next layer. Cancellation, stale generation, and close replay that log in
reverse, one layer owner per grant. Candidate close then restores the exact original live
container `Vec` backing and returns the exact empty reverse/output scratch roots before advancing
the existing page-catalog and duplicate-ID handback phases. The generation-tagged four-slot pool
therefore remains unavailable until all four roots have returned.

Reorder has a retained `DrawContainerSourceUndo`. If cancellation occurs after the removal-side
container completed but before destination publication, close first rolls back any active
destination rebuild and then drives a second retained insertion rebuild to restore the removed
owner at its original parent/index. No fallible validation occurs after the live container is
detached: `start_rebuild` validates reservation and scratch owners first, and
`finish_rebuild` validates the destination container before taking the terminal owner tuple.

The new source fixtures inject cancel and stale authority after a real owner move in every rebuild
phase, compare the exact live `Vec::as_ptr`, FIFO layer IDs, reverse/output/page/String pointers,
terminal witness, immediate slot reuse, and advanced pool generation. A separate reorder fixture
interrupts destination rebuild after source handoff and proves the exact nested container backing
and order are restored before the same pool slot is reborrowed.

### Retained process bootstrap and construction faults

Pool construction is now a persistent process state:
`Building -> Ready` or `Building -> Retiring -> Fault`. One call to
`initialize_draw_mutation_arena_pool` advances at most one allocation, publication action, or
retirement root. The real `DrawStoreInitializationAuthority::InitializeArena` phase pumps that
state once per governed store turn; an incomplete bootstrap cannot publish an operation-admission
pool. The former synchronous loop is compiled only for isolated Rust fixtures.

`DrawMutationArenaOwnerBuilder` retains the two container vectors, page-catalog vector, sixteen
page Strings, duplicate-ID String, and any rejected String in `ManuallyDrop<Option<_>>` fields.
`DrawMutationArenaPoolBootstrap` retains four completed bundles in a fixed array. Allocation,
completed-bundle, arithmetic, and final aggregate-cap failures transition to the retained fault
path. Close releases one page/String/vector/catalog root per grant, keeps the partially built
catalog unavailable, and exposes a terminal fault only after `terminal_is_empty`. The editor
retains the immediate fault witness and its getter also reads the process-terminal fault, so the
failure is not discarded.

Fixtures cover all 80 individual allocation boundaries, all four completed-bundle boundaries,
exact aggregate item/byte caps and each `+1`, one-allocation-per-turn progression, withheld
publication while incomplete, one-root-per-close grant, and terminal-empty cleanup. The
permanent verifier adds fourteen discriminating mutations for rollback removal, source-undo
removal, pre-move-only tests, synchronous production bootstrap, question-mark partial Drop,
missing allocation/bundle/aggregate matrices, and ignored app fault publication.

### Source gates

| Gate | Result |
| --- | --- |
| edition-2021 parser + scoped rustfmt check | **PASS**: owned Draw and Draw editor |
| tool-job source-verifier self-test | **PASS: 285 clean** |
| Draw retained-route predicate | **PASS mechanically**: the full verifier has no Draw-specific failure |
| deterministic ledgers | **PASS**: byte-identical `📊️p8yt-draw-rebuild-bootstrap-ledger-a.json` / `-b.json`, 312,560 bytes, SHA-256 `bc1a768b0f2d4f00848bf9e5fe141d4f4be55801b208e4b344cda8b25a1efad4` |
| full tool-job census | expected global **RED**: 50 hosts, 50 invocations, 775 rows, 773 unique, **0/884**, 8 reserved, 35 importer owners, 35 global payload stores, **18** failures |
| forbidden Draw scan | **PASS**: zero production `DrawMutationArenaOwner::try_new()?`, `rposition(Option::is_some)`, post-admission duplicate-name reserve, fabricated exact helper, structural-copy counter, or asset `iter().nth` |
| placeholder census | **PASS**: repository 14 Rust occurrences = one shared definition + 13 live callers; Draw zero |
| scoped working/staged/HEAD diff checks | **PASS** |
| broad interactivity DENY | **RED outside this packet**: current P1t retained-history synthetic fixture is rejected before the broad audit reaches Draw |
| whole-worktree diff checks | **RED outside this packet**: existing trailing whitespace in `🐙️ueli.md` and a blank EOF line in the staged P3 raster audit |
| Cargo/Nx/native/Wasm/browser/network/runtime timing | **not run by instruction**; compile/runtime and allocator/timing evidence remain unverified |

Verdict: **AUDIT-READY, not accepted.** Both independently rejected Draw source seams are repaired
and their scoped non-build gates are clean. Phase 8 remains RED at **0/884**, **18** global failure
classes, and runtime unverified until independent re-audit and serialized build/runtime evidence.

## Draw governed-bootstrap final re-audit repair

This addendum supersedes the preceding bootstrap description. The production
`initialize_draw_mutation_arena_pool` entry point no longer exists. The process begins in inert
metadata, and `request_draw_mutation_arena_pool` only coalesces an atomic request and observes the
process with `try_lock`; neither the Draw app default path nor an early arena borrow can allocate,
reserve, or advance bootstrap state. An early borrow returns typed `NotReady`, `Contended`, or
`Fault` authority.

`DrawMutationArenaBootstrapJob` owns the exact operation, generation, fixed admission, cancellation,
deadline, and terminal witness. Only its `step(&mut StepContext)` can move the process from `Inert`
to `Building`, advance one admitted allocation, publish `Ready`, or retire one retained root after
cancellation/fault. Budget exhaustion and lock contention report truthful `Blocked`; retained
retirement reports only the ownership actually released. `DrawStoreInitializationAuthority` pumps
this job once in its real `InitializeArena` phase before envelope validation. Default construction,
a second app request, and borrow-before-ready therefore remain allocation-free and only schedule or
coalesce the governed job.

The permanent verifier now matches the live `InitializeArena => match
self.arena_bootstrap_job.step(cx)` block and the exact source-undo
`start_rebuild(..., DrawMutationRebuildRole::CloseSourceUndo)` call. Mutating that actual role to
`Destination`, replacing the actual store-phase pump with `Ok(true)`, reintroducing allocation in
the request/borrow/default paths, or deleting the governed cancellation/saturation fixtures is
rejected. Runtime-free fixtures cover default plus second-app requests, borrow under contention
without allocation, zero budget, lock contention, cancellation during partial bootstrap, one-root
retirement, pool saturation, and terminal slot reuse.

### Final governed-bootstrap source gates

| Gate | Result |
| --- | --- |
| edition-2021 scoped rustfmt write/check | **PASS**: Draw owned schema and editor |
| tool-job source-verifier self-test | **PASS: 285 clean** |
| live Draw predicate and discriminating mutations | **PASS**: baseline accepted; actual source-undo-role and actual store-pump mutations rejected |
| broad interactivity deny | **PASS**: clean; one test-only bridge and two predeclared future entries reported |
| deterministic tool-job ledgers | **PASS**: two byte-identical 312,560-byte JSON results, SHA-256 `8e38a992ffd6c810b5caf6f02f374ac4c3854e0f6f04656804e2dc479962fcf0` |
| full census | expected global **RED**: 50 hosts, 50 invocations, 775 rows, 773 unique, **0/884**, 8 reserved, 35 importer owners, 35 global payload stores, **18** failures; Draw-specific failures zero |
| scoped working/staged/HEAD diff checks | **PASS** |
| whole working/staged/HEAD diff checks | **RED outside this packet**: existing trailing whitespace in `🐙️ueli.md` and blank EOF whitespace in the staged P3 raster audit |
| Cargo/Nx/native/Wasm/browser/network/runtime | **not run by instruction** |

Verdict: **AUDIT-READY, not accepted.** The governed-bootstrap rejection is source-repaired and its
permanent non-build evidence is green. Phase 8 remains fail-closed at **0/884** pending independent
re-audit and separately serialized compile/runtime evidence.

## Trinity Rewrite retained-load cohort

The next-smallest live placeholder cohort is Trinity Rewrite. All remaining owners tied at one raw
whole-buffer symbol, and Rewrite was selected because its bridge already targets the independently
accepted Trinity Jack snapshot/mutation domain, requiring no second codec or domain owner.

The whole-String constructor and direct `RefCell<TrinityGraphStore>` command/projection bridge are
removed. `TrinityRewriteArtifactVcs` now owns the accepted
`VcsArtifactApp<EditorApp<TrinityJackPlayApp>>` and exposes only generation-tagged
begin/page/seal/poll/cancel/close. Each page is an inline 4 KiB owner; credits are capped at 64 pages
and 256 KiB; polling grants one maintenance item/page; terminal results require exact ACK; close
grants one item/page and withholds terminal completion until the shared retained authorities are
empty. The default/request path does not parse or construct an envelope and no compatibility API
recreates the old whole-buffer route.

The permanent verifier adds a live Rewrite predicate and fourteen baseline/mutation checks for
whole-buffer resurrection, dynamic pages, generation erasure, growable buffers, rejected-page
loss, missing seal, unbounded polling, missing ACK/cancel, bulk close, and cap/saturation/close
fixture removal. The source cap fixture covers exact maximum, zero, and both `+1` boundaries.

### Trinity Rewrite source gates

| Gate | Result |
| --- | --- |
| edition-2021 scoped rustfmt/parser | **PASS** |
| permanent tool-job verifier self-test | **PASS: 299 clean** |
| deterministic tool-job ledgers | **PASS**: byte-identical outputs, SHA-256 `f12335f43c5f7e2fc790aa11282cf2f2525062ce76cbe71e8571a1aac6ecb5ce` |
| full verifier | expected global **RED**: **0/884**, 18 failure classes; no Rewrite-specific failure |
| broad interactivity DENY | concurrent **RED outside this packet**: one P1 DB-engine replay-history census finding; no Rewrite finding |
| raw whole-buffer symbol census | **PASS**: decreased from 14 to **13** = one shared definition + twelve live callers; Rewrite zero |
| scoped diff checks | **PASS** |
| Cargo/Nx/native/Wasm/browser/network/runtime | **not run by instruction** |

Verdict: **SOURCE-AUDIT-READY, not accepted.** The exact implementation and caller census are in
`p8yu-trinity-rewrite-retained-load-cohort-2026-08-23.md` and
`p8yu-next-live-placeholder-census-2026-08-23.md`. No second cohort was started.
