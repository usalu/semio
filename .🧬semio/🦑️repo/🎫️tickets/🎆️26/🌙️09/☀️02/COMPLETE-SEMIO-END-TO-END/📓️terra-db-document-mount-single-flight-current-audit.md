# DB Document-Mount Single-Flight Current Audit

## Scope and qualification status

Read-only review of the current production mount registry in [db engine](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs), the schema fixture, and the four registered native laws. No Cargo command was run. The reported `document-mount-single-flight-native` receipt is currently RED before its selected laws because of the known non-`Send` `Emit` future and two concurrent typed API migrations; this audit does not treat any of the four laws as qualified.

The selected native selector set is exact and unique in [the OS Rust script](../../../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:534):

1. `db_engine::tests::database_concurrent_ensure_mounts_one_actor_and_one_writer`
2. `db_engine::tests::database_published_opening_joins_without_actor_overwrite`
3. `db_engine::tests::database_cancelled_ensure_waiter_does_not_cancel_mount_owner`
4. `db_engine::tests::database_document_mount_failure_waiters_share_terminal_cleanup_and_retry_generation`

## Verified source properties

- An `Opening` registry entry owns the one boxed mount future through `Arc<DatabaseDocumentMountOwner>`; the initiating waiter only owns a `ReplyReceiver`. Dropping an unresolved waiter clears its exact `[generation, slot]`, never the owner ([lines 7453–7488](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7453)). This is the correct cancellation ownership split.
- The fixed per-document waiter array is actually 32 elements and admission rejects a 33rd waiter without replacing the `Opening` owner ([lines 7428 and 8052–8068](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7428)).
- A completing owner takes waiters and swaps `Opening` to `Ready` under the generation check before it fans out replies. A stale owner cannot overwrite a newer generation ([lines 7639–7671](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7639)).
- Catalog publication happens inside the retained owner, not a caller; an arrival after catalog publication but before the actor is ready still finds the same `Opening` entry and joins it ([lines 7949–8033 and 8052–8069](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7949)). The published-window native law exercises that seam.
- Construction rejection retains the exact `ArtifactEngineOpenRejected` and retries `close_step` on its same retained WAL until terminal before the owner reports its error ([lines 7896–7906](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7896)); `ArtifactEngineOpenRejected::retry_close` in turn retains the same `wal` on a failed close ([artifact lines 1220–1234](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1220)). No replacement writer can be acquired during that ownership chain.

## P0: lifecycle emission belongs to the retained mount owner, not an arbitrary waiter

The owner currently installs `Ready` and sends replies without emitting. Instead, whichever `mount_document` caller happens to receive the `DatabaseDocumentMountReply` first flips `reply.emission` to true and awaits `self.emit.emit` ([lines 8040–8050 and 8112–8117](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:8040)). This has three concrete failures:

1. A caller can be cancelled after `swap(true)` and before the `Emit` future completes. All later callers observe `true`, so `document_created`/`document_opened` is lost.
2. A concurrent caller can obtain and use the authority while the elected caller is still awaiting emission. Therefore visible live authority is not ordered after its lifecycle event.
3. The same awaiting `Emit` future makes Hub's request future non-`Send`: `Emit` promises only `Send + Sync` for the receiver, not a `Send` future ([version-graph lines 146–155](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🕸️version-graph/🦀️.rs:146)). That is the present selected-build failure rather than a test-only type annoyance.

The bounded correction is to pass `Arc<E>` into `run_document_mount`, make the `Emit` trait's returned future explicitly `Send`, then await exactly one emission after `ArtifactAuthority::spawn` succeeds and before `DatabaseDocumentMountOwner::complete` publishes `Ready`. Delete `event` and `emission` from `DatabaseDocumentMountReply`; its sole public payload becomes the authority. A cancelled waiter then cannot cancel, duplicate, or overtake publication. An `Emit` implementation remains observational (the trait has no error return), so it cannot manufacture a mount failure.

Required native law: use a `Send` controlled emitter that blocks after the actor is built. Drop the initiating mount future while emission is pending; a second ensure must remain pending, releasing the emitter must yield two handles pointing at one authority and exactly one event. Repeat with two surviving waiters and prove neither sees a handle before the event witness. This both repairs the Hub `Send` compilation requirement and qualifies the correct ownership order.

## P0: mount cleanup has no cancellation/deadline boundary

`close_mount_rejection` retries retained close forever with only `yield_once` ([lines 7896–7906](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7896)). It correctly never loses the writer, but a permanent writer-controller or storage-release fault leaves the only `Opening` owner and every waiter pending indefinitely. `DatabaseShutdownControl` cannot interrupt that retry: shutdown merely calls `owner.schedule()`, yields, and reports progress while the `Opening` remains in the registry ([lines 8181–8195](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:8181)).

Do not release or replace the writer on cancellation. Give `DatabaseDocumentMountOwner` a retained cleanup phase that owns the rejected engine/WAL, and a control-aware scheduled retry boundary. A shutdown cancellation/deadline returns `Interrupted` while that exact cleanup owner remains mounted in `Opening`; a later `shutdown_step` or mount wake resumes the same owner. Only a terminal `retry_close` may remove the generation and fan out the original mount error.

The existing failure law injects a construction append/sync failure and proves eventual terminal cleanup once the injected failure is one-shot. It does **not** inject a failed writer-release attempt or prove a timeout leaves the identical cleanup owner mounted. Add a controller/fault-backed native law that forces the first release attempt to fail, captures the owner generation/key, interrupts shutdown, then resumes it and proves one terminal release with no second actor or writer acquisition.

## P1: acceptance coverage gaps

- The neutral fixture says `waitersPerDocument: 32`, but its independent interpreter never tests admission at the boundary: it does not branch on `catalog-published` either, so that label is presently decorative ([fixture script lines 485–523](../../../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts:485)). Add explicit `fill-waiters`, `reject-over-capacity`, and `release-slot` transitions; native coverage should park 32 callers, reject number 33, cancel one, admit a replacement, then complete one actor.
- The cancelled-initiator law immediately performs a second `ensure`. Add the stricter no-waiter variant: drop the only waiter, permit the owner to finish, inspect that the registry reached `Ready`, then issue the first later ensure. This validates owner scheduling independent of any joining caller.
- The native published-window law is good evidence for no local actor overwrite, but there is no native stale-generation wake law. Use the test hook to hold generation N, cause its terminal cleanup, then begin N+1 and release the obsolete wake; N must be ignored and only N+1 may become `Ready`.
- Shutdown currently retains an `Opening` through the registry and does not silently drop it, but no selected law proves the contract. Add an opening gate, invoke `shutdown_step`, then cancel its control; assert the same owner/generation is still in `Opening`, no ready actor was closed or replaced, and a later uncancelled shutdown reaches terminal acknowledgement.

## Runtime acceptance boundary

After the four compile errors are repaired, the exact executable target remains `os-hub:document-mount-single-flight-native-check` (the native target's command is registered at [project.json lines 20–26](../../../../../../../../🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json:20)). A passing source schema/oracle alone does not qualify the Hub concurrent-document-open recovery path; the four selected laws, the emitter-order law, the capacity law, and the retained-cleanup/shutdown-resume law are the minimum native packet.
