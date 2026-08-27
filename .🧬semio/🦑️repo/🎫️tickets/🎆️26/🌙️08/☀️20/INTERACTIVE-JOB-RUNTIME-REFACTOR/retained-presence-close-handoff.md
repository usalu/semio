# Retained Presence Close Handoff

## Scope

CAD's concrete presence value contains three variable-length strings. Its local-root factory now retains shared snapshots until uniquely owned, moves one exact string field into a byte cursor per metadata turn, retires at most 4096 initialized bytes per turn, and releases empty allocations separately. The actual field shape is unchanged. Exact NoDraft and NoTransient owners are installed explicitly on CadPlayApp; no generic default admission was added.

The new language-neutral fixture contains empty/default/24-KiB Unicode values plus nonempty peer-roster cases. The Rust domain test uses serde_json as an independent field/byte oracle. The root verifier adds strict Ajv validation and Node Buffer byte-count parity. Native and verifier execution remain pending coordinator scheduling.

## Minimal Shared Seam

PresenceStore currently exposes only cloned local/peer roots. A close disposer cannot obtain exact ownership of either root, so app-only code cannot safely close a nonempty roster. Proposed Store-owned `begin_retirement` transfers the old local root and old peer root into a retained owner and installs a caller-supplied, domain-validated empty terminal local root and empty roster. A close-started state rejects subsequent mutation/publication admission. Captured old readers remain unchanged.

The retained owner waits on shared local and peer roots without spinning. Once the peer root is unique, its fixed root metadata moves into the existing PresencePeersRetirement owner. That owner retires one peer entry and its exact domain snapshot factory; it does not drop an entire populated roster. Every incomplete owner remains retained. Completion requires both detached roots and nested retirement owners to be empty.

ArtifactEditor now exposes ArtifactApp's `build_presence_peer_retirement_factory` as an exact default-None hook with EditorApp forwarding, so CAD and other concrete editors can install their actual domain factory. This is not an implicit local-root fallback.

The consumed peer-commit API now returns the exact untouched commit when the Store has closed. That commit carries its exact snapshot factory and can become a retained root cursor. The sole mounted caller returns rejected metadata into its existing metadata-retirement slot and the typed commit into its existing rejected-owner slot; no monolithic payload drop or rollback is used.

## Remaining Shared-Entry Boundary

The existing PresencePeersRetirement uses Arc::try_unwrap while retaining each shared entry. Two independently retired roster roots that share an unchanged entry can therefore wait on each other. The current tests cover shared readers of one detached root and rejected commits containing newly created entries, not arbitrary overlapping-root retirement. The coordinator requires this correction before any CAD close credit.

The concrete peer correction retains every replaced published roster root until its captured readers release it, then retires one entry reference using atomic Arc::into_inner. A shared entry is released without waiting; whichever exact root/entry cursor receives the final owner retires its actor and typed snapshot. Candidate cancellation must use this same entry owner, not ordinary final drops. Public owned-root Clone and Store Clone must be removed, because untracked copies could otherwise become final readers outside that ownership protocol. Temporary candidate replacements may release only references whose exact new-root or retired-entry counterpart remains locally owned.

Local roots have the same problem when a cancelled operation owns a read while the Store remains live. Their proposed correction uses the existing SnapshotRead lease registry for captured reads rather than exposing Arc. Dropping a mounted read returns its lease to the registry, which retains the root until a live maintenance turn retires one owner. Retained Store replacements and preparation reads must participate too. Cancellation must finish without closing the Store, and final domain retirement must occur exactly once. The language-neutral fixture now contains overlapping roster and live-Store cancelled-read laws; their implementation and execution remain pending.

## Checkpoint Boundary

The coordinator's compound envelope-read gate passed both tests. Shared Store/plugin/CAD integration is now source-coherent and held for the executor's native exact registered-dispatch run, followed by the complete ten-test latest-wins filter and two Store close tests. Four CAD close laws and the actual constructor close remain queued. Full CAD constructor close is intentionally unclaimed until executed.
# Shared Alias Prerequisites — 2026-08-27

The current detach seam is not yet an all-app Presence-close approval. Two retired roster roots can share an unchanged entry; retaining both aliases while waiting for `Arc::try_unwrap` strands both cursors. Local-root capture has the analogous problem when an operation must close while the live Store remains open.

The next coherent unit transfers every displaced published peer root to an exact retirement cursor, removes public mutable-owner cloning, and retires each shared entry using atomic `Arc::into_inner`. Captured root readers keep their immutable roster unchanged until returned; only their specific old-root cursor waits. Candidate cancellation and displaced entries must use the same typed retirement path. No sampled reference count is an ownership proof.

Local captures will use the existing opaque SnapshotRead registry rather than public Arc aliases. Returning a cancelled worker's read must finish while the Store remains open; live maintenance owns final payload retirement. Presence preparation base reads also need this opaque capability, otherwise they remain a final-owner escape.

Source validation identified two prerequisite races in the shared reader protocol. Drop/explicit return published `returned` before releasing their reader Arc. Concurrent maintenance could remove and release the registry Arc first, leaving an ordinary reader destructor as the final payload owner. All four return paths now use one private helper that releases the reader alias while the unreturned registry still owns its guard, then publishes the returned flag. `ErasedSnapshotRead::into_typed` now transfers the unreturned slot atomically under the existing try-lock and exact index/generation validation, without first publishing return. A contended transfer restores the unchanged unreturned read.

Three native tests are source-ready: all four public returns raced against cross-worker reclamation; an injected barrier inside the shared helper after alias release proves the guard remains unreturned until publication; and contended cross-worker erased transfer proves restoration and unique final ownership. The language-neutral fixture records the ownership event order, and strict Ajv plus an independent alias ledger validate it. Source mutants remove alias release, publish a premature transfer return, or discard a restored lease. The canonical Nx verifier passed807checks (`🧪️member-presence-reader-return-selftest-2026-08-27.txt`). Native execution remains pending the combined shared source checkpoint; no atomic-final-owner Presence adoption is claimed before those native laws and the remaining root integration.

The authoritative launch seed now includes focused read-return, cold-rebase, latest-wins, registered-dispatch, and CAD Presence-close gates. Canonical `@semio-tech/plugin-registry:generate` completed successfully and regenerated the launch catalog (`🧪️member-presence-launch-generate-2026-08-27.txt`). This only registers native gates; it does not execute them.
