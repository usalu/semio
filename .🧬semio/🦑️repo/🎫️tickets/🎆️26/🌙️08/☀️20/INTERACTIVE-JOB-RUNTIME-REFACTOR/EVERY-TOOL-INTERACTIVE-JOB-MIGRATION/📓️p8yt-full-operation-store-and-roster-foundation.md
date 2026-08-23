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
