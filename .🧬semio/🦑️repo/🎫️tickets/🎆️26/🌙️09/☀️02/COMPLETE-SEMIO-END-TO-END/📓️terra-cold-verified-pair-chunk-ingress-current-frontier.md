# Cold Verified Pair Chunk Ingress Frontier

## Verdict

The earlier single hydrate-event proposal is not admissible. A browser child message is capped at 256 KiB, while the canonical `pack || spr` pair may be 4 MiB. The smallest coherent replacement is a fourth, dedicated `reactor.poll` input: one acknowledged cold-pair page per turn. It must end at the existing typed `plugin_load_document_pack` call; it must not use generic `document-read`, `load-document`, `assets`, or the generic command-page lane.

This is a source audit only. No build, native law, or browser run was performed here.

## Immediate Lifecycle Prerequisites — P0

The pair transfer cannot begin merely because the child has returned `describe`. It needs the existing actor lifecycle to become real production state first.

| Current source | Concrete defect | Required correction |
| --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1130-1138` | `InstanceLifetimeRegistry::open` rejects only a same-instance occupant, then overwrites a different live instance that aliases the same modulo slot. The prior native owner becomes unreachable. | Reject **any** occupied slot before `guest_lifetime.next()`. Do not mint a serial or replace the slot on collision. |
| `.../⚛️reactor/🦀️.rs:1379-1403` | Successful `InstanceOpen` opens metadata, creates the app, and sets the actor but emits no `Captured` receipt. | Install the real lifecycle owner only after all three open steps succeed, retain `Captured`, and return that exact receipt. Roll back metadata/app/slot on a failure before capture. |
| `.../⚛️reactor/🦀️.rs:1408-1411` | `InstanceLifecycleAck` is deliberately ignored. Therefore a host cannot establish that capture was observed before it transfers a document. | Route the event through the existing `GuestLifecycleCell::stage_ack` and `finish_turn`; only its exact `Captured` ACK makes the instance `Live`. |
| `.../🔌️plugin/🦀️.rs:32668-32676` | The generated world export directly calls `reactor::poll`; no outer wrapper repairs the missing lifecycle ownership. | Repair the reactor path itself; do not add a browser-only parallel receipt protocol. |

`⚛️reactor/🚪️lifetime/🦀️.rs:47-164` already contains the appropriate retained protocol: `install_owner` emits `Captured` (78-84), `stage_ack` requires exact identity (117-123), and `finish_turn` moves `Captured → Live` only after a successful timed turn (141-163). The production registry/close registry needs to adopt this owner rather than duplicating its phases. In particular, close must remain rejected until `Captured` is ACKed (`validate_close`, 86-92), and the existing retained close work must become the cell's terminal owner rather than being dropped when a new open aliases a slot.

The sole `Option<lifecycle_receipt>` in `TurnResult` means an open cannot silently share a turn with a different terminal receipt. The bounded first slice should reject/defer an `InstanceOpen` while a retained lifecycle receipt occupies that turn; it must not overwrite `Retired` or `Captured`. A later receipt-page generalization can improve throughput, but is not required to make one document correct.

## Existing Authority and Transport Boundaries

| Boundary | Existing source | What is reusable | What is not sufficient |
| --- | --- | --- | --- |
| Verified cold source | `🧰️framework/🔨️modules/📡️replication/🟦️.ts:172-191,793-1010` | `ArtifactBootstrapAssembler.finish` validates descriptor, pack, SPR, aggregate hashes, and the exact wire lengths before yielding a pair. | The broad bootstrap ceiling is 64 MiB/16,384 × 4 KiB chunks. It is not the browser actor ingress cap. |
| Browser retained copy | `🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts:2007-2018` | Completion copies authenticated `pack`/`spr` into the scoped `ArtifactState`; `assembler.bootstrap` still holds descriptor/frontier/hash metadata at that point. | `currentPack`/`currentSpr` alone carry no retained bootstrap metadata and must not be treated as a fresh authority after rebootstrap. |
| Child envelope | `.../🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts:93-102` | One outstanding invocation and transferable `ArrayBuffer` ownership give useful backpressure. | `BROWSER_ACTOR_CHILD_LIMITS.messageBytes` is 262,144 bytes, so a whole pair cannot enter a generic invocation. |
| Existing document loader | `.../🔌️plugin/🦀️.rs:31095-31103`; `.../🏪️store/🦀️.rs:9363-9391` | `plugin_load_document_pack` is the terminal typed loader for the established `(pack,spr)` format. | Neither WIT `instance-open.config/assets` nor generic `load-document` causes this loader to run today. |
| Generic ingress | `.../📡️spr/🧵️channel/🦀️.rs:45-48`; `.../⚛️reactor/🦀️.rs:1087-1098,1648-1792` | The status/ack pattern is useful precedent. | Command ingress is capped at 64 × 4 KiB = 256 KiB, has command semantics, and already reserves two global owners. It must not be overloaded. |

## Bounded WIT and Kernel Shape

Add an explicit fourth poll parameter, after `command-page`:

```wit
poll: async func(
  events: list<event>,
  command-page: option<command-ingress-page>,
  cold-pair-page: option<cold-document-pair-page>,
  budget: budget,
) -> result<turn-result, plugin-error>;
```

Add `cold-pair-ingress: cold-pair-ingress-status` to `turn-result`, parallel to but independent from `command-ingress`. This updates `📜️.wit:1110-1155`, generated bindings, `reactor::poll`/`poll_kernel` (`⚛️reactor/🦀️.rs:1265-1289`), `kernel::TurnResult` (`🎠️kernel/🦀️.rs:1842-1860`), and the WIT conversion function (`⚛️reactor/🦀️.rs:2400+`). It deliberately leaves the existing `event` variant unchanged (`📜️.wit:782-804`): a page is owned transport, not a reorderable event.

The schema must be domain-specific and fixed:

```text
cold-document-pair-header {
  lifetime: actor-instance-lifetime,
  transfer-generation: u64 nonzero,
  descriptor-sha256: bytes exactly 32,
  baseline-frontier: exact frontier summary,
  pack-sha256: bytes exactly 32,
  spr-sha256: bytes exactly 32,
  aggregate-sha256: bytes exactly 32,
  pack-length: u64 1..4MiB,
  spr-length: u64 1..4MiB,
  page-count: u32 1..64,
}
cold-document-pair-page { header, page-index: u32, bytes: list<u8> 1..65536 }
cold-pair-ingress-status = idle | page-accepted(cursor) | backpressure(cursor)
                           | loading(cursor) | applied(receipt) | fault(cursor, bounded-fault)
```

Constants: `COLD_PAIR_PAGE_MAXIMUM_BYTES = 64 * 1024`, `COLD_PAIR_MAXIMUM_BYTES = 4 * 1024 * 1024`, `COLD_PAIR_MAXIMUM_PAGES = 64`. Validate every header on every page; require `page_count == ceil((pack_length + spr_length) / 65536)`, all non-terminal pages exactly 64 KiB, terminal page exactly the remaining bytes, and `pack_length + spr_length <= 4 MiB` with checked arithmetic. 64 KiB leaves ample room under the current 256 KiB child message cap while avoiding a 4 MiB transient message.

The cursor must identify the exact `{lifetime, transfer_generation, page_index, page_count}`. The `Applied` receipt repeats the lifetime, transfer generation, baseline frontier and aggregate hash. It is not a generic document handle and does not grant a later read/write capability.

## Reactor Ownership and State Transitions

Use a dedicated fixed `COLD_PAIR_INGRESS` registry adjacent to `COMMAND_INGRESS`, with capacity equal to active actor-instance capacity—not the existing two command slots. Each owner is keyed by the exact captured `ActorInstanceLifetime`; a second pair for the same lifetime is `Backpressure`, while a mismatched page is a `Fault` and cannot displace the existing owner.

1. **Admission.** Only accept page zero for an exact lifecycle cell in `Live`, after the host has ACKed its `Captured` receipt. Validate every size/count/hash/header field before reserving pair backing. Allocate exactly `pack_length + spr_length` once, retain the header, and copy page zero.
2. **Progress.** For the expected next page, validate the whole repeated header and exact index before copying. Return `PageAccepted` only after the bytes are held. The browser sends no next transferable page until it receives that exact status.
3. **Terminal verification.** At the final byte, verify pack, SPR, and aggregate SHA-256 over the retained pair. Construct only the existing `store::ArtifactPackFiles { pack, spr, ops: String::new() }` and call `plugin_load_document_pack`; do not introduce a second restore decoder.
4. **Applied / fault.** Return `Applied` only after that loader succeeds. If hashing or loading faults, retain the exact owner in a faulted/closeable state until a bounded close step wipes it; do not discard a pair then report a retryable success. A terminal loader retry must either use the retained verified bytes without accepting a duplicate terminal page, or be explicitly unavailable and require close/reopen.
5. **Close.** Before destroying an app, cancel and wipe its exact pair owner. A page or result from the old lifetime after close/reopen must fail before mutation. The close must not produce `Applied` and cannot free a colliding new lifetime's owner.

The old checkpoint helper (`⚛️reactor/📸️checkpoint/🦀️.rs:72-100`) is evidence that `plugin_load_document_pack` is the right terminal method; it is not a second host restore path.

## Private Backbone Driver

At `installArtifactBootstrap`, add a private opaque `VerifiedColdDocumentPair` immediately after `assembler.finish` and before publishing UI events. It owns the authenticated copies plus a frozen copy of `assembler.bootstrap` identity. Its validity fence must require the current `runtimeKey`, Hub scope, selected descriptor hash, baseline frontier, pair hashes/lengths, and bootstrap generation. `requireArtifactRebootstrap` (`backbone-worker.ts:1973-1983`), socket replacement, lease drop, and document abort wipe it.

`DocumentBrowserActorReservation.activate` (`backbone-worker.ts:1033-1069`) may drive pages only after all of the following are true:

1. the exact accepted Session/socket/lease/reservation checks already in `assertCurrent` hold;
2. the child has returned the verified `describe` result;
3. reactor `InstanceOpen` has returned the matching `Captured` receipt and the driver has sent its exact ACK, then observed the post-ACK live turn; and
4. the private pair owner still matches the same scoped bootstrap.

Copy one 64 KiB segment into a separate transferable buffer for each child call; never transfer the backing of `state.currentPack/currentSpr`, because that would detach the browser's retained checkpoint. After each await, revalidate the reservation/session/lease/pair generation before sending or accepting the next page. On replacement, abort, deadline, descriptor mismatch, status mismatch, or child failure, zero all unsent copied bytes, close the child, and do not emit a render-ready status.

The current `renderer-unavailable` emission at `backbone-worker.ts:1064-1066` remains correct until a later renderer-turn bridge accepts the terminal `Applied` receipt and revisioned UI patches. This slice loads a document; it does not by itself prove GIS rendering, map mutation, or any new effect authority.

## First Executable Laws

1. **Lifecycle collision/capture.** Open two different instance IDs that alias the registry slot: the second open is denied without serial advance or mutation of the first cell. A successful first open emits `Captured`; foreign/replayed ACKs do nothing; only exact ACK makes it live.
2. **Exact 4 MiB stream.** A real typed app receives 64 × 64 KiB pages, each child call stays below 256 KiB, and `plugin_load_document_pack` is called exactly once with byte-identical pack/SPR. `Applied` binds the original lifetime/generation/frontier/aggregate hash.
3. **Hostile page matrix.** Reordered, duplicate, foreign-lifetime, changed descriptor, changed length, non-full middle page, and mutated terminal bytes each produce no loader invocation and no document mutation.
4. **Retained terminal fault.** Inject loader failure after hashes validate: the owner remains reachable for close; duplicate final pages cannot invoke load again; bounded close wipes all pair bytes.
5. **Close/reopen race.** Close while a page awaits; then reopen the same numeric instance with a new lifetime. A late old page/result is denied, no old bytes reach the new app, and both registries have no leaked owner.
6. **Actual child cancellation.** Force child timeout/termination after one accepted page. The Backbone frees copied transfer buffers and child capacity returns to zero without a generic host dispatch effect.
7. **Real GIS progression.** With a genuine materialized GIS component and authenticated pair, require `Captured → ACK → Applied` and a later real `tiled-map` patch through `UiDocumentStore`. A fixture actor, `describe` alone, or `renderer-unavailable` cannot satisfy this law.

## Nonclaims

The authenticated bootstrap already validates pair bytes, and Session activation already verifies actor description; neither currently transfers the pair to the actor or renders it. This packet does not authorize generic document access, network, storage, actions, jobs, or map mutation. The later editor path remains behind the fixed-three Store/DB witness owner.

## 2026-09-06 Current Module Addendum: Detached Load and Baseline Authority

This is a current-source read-only review of the newly added kernel vocabulary and reactor-local registry. It does not claim that the WIT/reactor/child driver is mounted: `ColdDocumentPairIngressRegistry` is still directly tested at `.../⚛️reactor/📥️cold-pair/🧪️tests/🦀️.rs`, rather than reached from an authenticated browser actor turn.

### P0: detached-load ownership is not retained by the registry

`ColdDocumentPairIngressRegistry::begin_load` (`.../⚛️reactor/📥️cold-pair/🦀️.rs:181-197`) moves the verified `ArtifactPackFiles` into `ColdDocumentPairLoad`, leaving only `Loading(header)` in the registry. The earlier report incorrectly said that `close(lifetime)` left that header intact. It does not: `close` takes the matching slot (`:231-240`), removes `Loading`, and returns zero because the detached files are no longer in that slot. A later `finish_load(load, Ok(()))` therefore returns `cold-pair.stale-load`; it cannot emit `Applied`, and consuming `load` then drops/wipes its files.

The real gap is still material. While the load owner is detached, `close` has no owner for its bytes or an in-flight loader callback. If the caller abandons `ColdDocumentPairLoad` without a close, `Drop` wipes the files but cannot remove the registry's `Loading` slot, permanently consuming a fixed entry. If close races a loader which has already entered an app-mutating call, the post-call stale check prevents `Applied` but cannot roll back that mutation. The async bridge must retain one exact load ticket, make loader admission/each mutation verify its still-live lifetime, and retain/cancel that ticket through close before the registry entry is terminally released.

### 2026-09-06 correction: current Home rewrite keeps a shared owner; it does not permit post-close apply

The paragraph above described the superseded `Option`-slot implementation. Current [`cold-pair/🦀️.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📥️cold-pair/🦀️.rs) retains `Rc<RefCell<ColdDocumentPairOwner>>` in both registry and `ColdDocumentPairLoad` (`:114-135,147-151`). `request_close` changes that owner to `Closing` (`:284-291`); `close_step` cannot wipe while the load's extra `Rc` is live, returns `{ wiped_bytes: 0, closed: false }`, and waits for the load reference to disappear (`:90-110`). `finish_load` then detects a phase other than `Loading` and returns `cold-pair.stale-load` (`:246-257`). The current test proves that exact path at [`tests/🦀️.rs:145-153`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/📥️cold-pair/🧪️tests/🦀️.rs:145). There is no current post-close `Applied` transition; the previous claim was false.

The retained-owner rewrite fixes the narrow detached-byte disappearance, but leaves three concrete ownership obligations for the real async bridge:

1. A caller can drop `ColdDocumentPairLoad` while no close has been requested. Its `Drop` changes `Loading` to `Faulted` but neither wipes the pair nor reclaims the fixed slot (`cold-pair/🦀️.rs:130-137`). The current test manually calls `close_all` after that drop (`tests/🦀️.rs:156-164`); no production owner is yet responsible for that terminal close. A retained load-task/ticket must request and drive the exact close to completion on cancellation, loader error, and abandonment.
2. `request_close` fences final publication but cannot stop a loader that already borrowed `load.files()` and entered an app-mutating operation. The later `finish_load` stale result cannot undo that operation. The load ticket must make the loader's admission and its app-facing mutations require the same live lifetime/control ticket, and close must revoke/cancel that ticket before registry retirement.
3. `ColdDocumentPairIngressRegistry::Drop` currently calls `std::mem::forget(owner)` for every mounted `Rc` (`cold-pair/🦀️.rs:323-330`). That permanently retains a strong reference and its pair bytes whenever the registry itself is dropped. This is a direct memory/secret-retention defect, not a post-close application path. Replace it with a bounded terminal close owner or explicitly wipe/release the unique owner; a registry destructor must not forget verified bootstrap bytes.

The rewrite correctly checks `live` on every page (`:160-167`), at `begin_load` (`:230-233`), and at `finish_load` (`:258-263`). It still has no task/cancellation owner tying those local checks to the external loader call; that is the remaining race to solve.

The production integration needs an explicit detached-load owner, not a free-standing `ArtifactPackFiles`:

```rust
struct ColdPairLoadTicket {
    lifetime: ActorInstanceLifetime,
    transfer_generation: u64,
    load_generation: u64,
    revoked: Arc<AtomicBool>,
    // owns the byte/slot permit until files are wiped
}
```

The slot retains the exact ticket while the retained reactor load task owns the same ticket plus the bytes. `close(lifetime)`, replacement, and a failed live check set `revoked` before terminal close. `finish_load` must require the exact live, unrevoked ticket; otherwise it wipes the held files and returns a close/revoked fault without `Applied`. An abandoned task must report/tombstone the ticket in its `Drop`; the next registry maintenance pass reaps only that exact tombstone after its backing permit has returned. Do not admit a new pair into the modulo slot while detached bytes still own their byte permit: otherwise close/reopen can exceed the advertised aggregate cap.

The retained task is the only code allowed to call `plugin_load_document_pack`. It must pass the exact lifetime and ticket through the await, then call `finish_load` or `abort_load` on every success, fault, cancellation, and panic-safe terminal path. The direct registry API should not expose a completion path that accepts an arbitrary `ArtifactPackFiles` or a header reconstructed from current state.

Required laws:

1. begin verified load, close/revoke, complete load: no `Applied`, exact files zeroed, slot/byte permits return only after the retained load terminal path;
2. begin verified load, drop/cancel outer task: one maintenance reap releases the exact slot, then a new lifetime may admit page zero; 
3. close/reopen while old loader completion races: old completion cannot alter the new slot and cannot call the loader for the successor;
4. loader success, loader fault, and loader panic each leave baseline state unchanged until their exact `finish_load` terminal publication.

### P0: every page and load transition requires the exact live lifetime

`accept_page` validates `live == Some(header.lifetime)` only while the slot is empty (`:130-133`). Once a `Receiving` slot exists, an ordinary page compares only the header/lifetime held in the slot (`:147-159`); `begin_load` performs no live check at all. A lifetime that becomes `Captured`, `Closing`, or replaced between page zero and the terminal page can therefore retain and verify a 4 MiB pair, then start the loader.

Pass one authoritative `Option<ActorInstanceLifetime>` (or, preferably, a private `&LiveLifecycleLease`) to **every** `accept_page`, `begin_load`, and `finish_load`. Validate it before allocation/copy and before loader invocation; the retained load ticket repeats the check after the await. The registry must not infer Live from a matching numerical header. The negative law is page zero accepted while Live, then the lifecycle changes before page one/final page or load; each subsequent action is rejected, the owner is closeable, and the loader call counter remains zero.

### Bounded allocation, hashing, and wipe contract

The header caps one pair at 4 MiB, but `ColdDocumentPairOwner::new` performs two `vec![0; length]` allocations (`:11-16`) before any reactor-wide byte admission. This can panic on allocation failure and permits `N × 4 MiB` resident backing without an aggregate per-reactor budget. Use a first-party fixed byte permit / `try_reserve_exact` preflight for the full checked total, retain that permit through detached load and wipe, and allocate only after it succeeds. A rejected allocation must not leave a slot.

`verify()` currently performs three full hashes over the entire backing after the final page (`:39-48`). At the allowed 4 MiB this can violate the interactive turn ceiling precisely on terminal ingress. Retain incremental pack, SPR, and aggregate SHA-256 state in the owner, updating the exact portions copied from each accepted page. The final page compares the three finalized digests only; it must not rescan the pair. The aggregate update order is `pack bytes || spr bytes`, including the page that crosses the boundary.

Use one private `wipe_artifact_pack_files` routine for every owner, detached load, successful load, fault, and close. It must overwrite `pack`, `spr`, and any nonempty `ops` allocation before clearing; current paths clear `ops` without overwriting it. The registry's `retained_bytes`/permit accounting must include detached-load bytes until that wipe completes.

### Actual Hub baseline semantics

The current `last_commit_seq <= head_edit_ordinal` check in `ColdDocumentPairHeader::validate` is source-aligned, not an arbitrary relation: Hub document descriptor validation imposes `commit_seq <= head_seq` at `🌎️hub/📇️directory/🦀️.rs:1298`, and the schema twin does the same. Do not remove it.

An authoritative canonical checkpoint requires a nonempty head identifier and nonzero chain hash (`validate_checkpoint_shape` at `🌎️hub/📇️directory/🦀️.rs:1337-1358`). A DB genesis frontier legitimately has `{ head_seq: 0, commit_seq: 0, chain_hash: 0 }` (`🛢️db/💾️durability/🦀️.rs:66-70`), but it has no active canonical checkpoint suitable for this transfer. Likewise `best_effort_frontier` deliberately fabricates an empty head on a read failure (`🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:3716-3724`) and must never be converted into `ColdDocumentPairHeader`.

The only permitted browser source is the selected `VerifiedArtifactBootstrap`/active `ArtifactCheckpoint` metadata already bound to the lease: map its exact descriptor digest, checkpoint baseline frontier, pack/spr digests and lengths into the header. Do not derive a header from `currentPack/currentSpr`, live telemetry, a tail frontier, or an empty-head fallback. Add a conversion law with a valid active checkpoint, a genesis/no-checkpoint case, an empty-head read-failure case, and a pair whose `commit_seq > head_seq` is rejected before any pair allocation.
