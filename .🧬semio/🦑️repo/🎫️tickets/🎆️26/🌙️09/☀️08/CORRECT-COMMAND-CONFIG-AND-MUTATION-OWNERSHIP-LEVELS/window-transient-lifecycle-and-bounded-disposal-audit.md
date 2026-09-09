# Window Transient Lifecycle And Bounded Disposal Audit

## Scope And Evidence

Read-only audit of the SDK document-window replacement wiring and the generic transient publication/disposal helpers. No source, test, ticket metadata, or generated output was changed. Native validation was deliberately not run because the shared Cargo target is occupied and the task requested source and queued-log inspection only. The documented TypeScript/Ajv oracle is green. The named `🗑️generated/window-document-generation-green-1.log` is now present: it records the retained-window-input target waiting for the shared Cargo target and then compiling dependencies. It has not yet compiled the plugin test crate or reached a pass/fail result, so no native validation is claimed here.

The document-reset logic itself gets several important things right:

- Text and pack routes parse first, prepare the reset before `Store::reset`, and commit it only after a successful reset (`plugin/🦀️.rs`, `load_document_text` and `load_document_pack` anchors; currently lines 23301-23328).
- Retained candidate publication prepares the registry before candidate publication, restores a rejected candidate, and commits the reset only after the candidate was published (`plugin/🦀️.rs`, `drive_store_replacement_jobs` `CandidateReady` anchor; currently lines 17741-17768).
- The replacement commits a fresh document generation and renews the cancellation scope only after it moves the previous registry to retirement (`plugin/🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs`, `commit_document_window_reset`; currently lines 16-24).
- Captured snapshots, begin admission, advance, and tool context identity all include document generation (`plugin/🪟️window/🫧️transient/🦀️.rs`, `WindowTransientSnapshot`, `WindowTransientOwnerRegistry::begin/advance`; currently lines 62-99 and 279-302; `plugin/🦀️.rs`, `artifact_owned_tool_job_context_identity_digest`; currently lines 12502-12510).
- Window config is not replaced by this path. A same-byte reload still invokes a successful store reset and therefore receives a new document generation.

The retained-window fixture test exercises two concrete windows, same-local-generation replacement identity, stale authority/publication rejection, zero-item close grants, and cancellation scope renewal. It is useful coverage, but it does not exercise a real `VcsArtifactApp` text/pack/candidate reset, a blocked retirement, a root or prepared payload larger than one page, or a retained snapshot alias.

## Findings

### P1 — Retirement Is Not Round-Robin Across Retired Document Registries

`retire_document_windows_step` always selects `next_id_from(0)` and immediately returns that entry's pending or blocked result. See `plugin/🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs`, `retire_document_windows_step`, currently lines 26-40. A blocked retirement in the lowest occupied slot is selected forever; all later document generations receive no maintenance steps. The fixed registry has 64 slots (`plugin/🦀️.rs`, `ArtifactFixedRegistry`, currently lines 14645-14745), so repeated replacement eventually exhausts the slots and rejects otherwise valid replacement requests.

This conflicts with the lifecycle note's statement that ordinary maintenance is round-robin. The adjacent candidate-replacement driver provides the required pattern: it selects from a stored cursor and advances it after selection (`plugin/🦀️.rs`, `drive_store_replacement_jobs`, currently lines 17717-17723).

Add independent `maintenance_window_transient_retirement_cursor` and `close_window_transient_retirement_cursor` fields. Select with the relevant cursor, advance it modulo `ARTIFACT_LIVE_OUTPUT_SLOTS` before driving the selected owner, and retain the cursor even when the owner reports `Blocked` or zero progress. A close cursor is required because close has its own progress schedule.

### P1 — Retirement Is Not Round-Robin Across Partitions Of One Registry

`TypedWindowTransientStoreOwner::close_step` always selects `self.partitions.keys().next()` (`plugin/🪟️window/🫧️transient/🦀️.rs`, currently lines 218-231). Once disposal correctly blocks for a live read or consumes multiple byte pages, the lexicographically first window prevents every other window in that displaced document registry from being retired.

Use an explicit partition cursor/retirement queue and rotate after every selected partition, including on `Pending` and `Blocked`. Removing a complete partition must leave the next eligible partition reachable without restarting at the first map key. This is independent of the fixed-registry cursor above.

### Owner-Kind BTreeMap Fairness — Remediated In The Current Source, Native Result Pending

The owner registry was a third independent starvation layer when `WindowTransientOwnerRegistry::close_step` selected `self.owners.keys().next()`: a blocked lowest window kind prevented all later kinds from disposing. The current source instead records `retirement_cursor`, selects the next lexical owner after it with wraparound, and updates the cursor before driving the owner (`plugin/🪟️window/🫧️transient/🦀️.rs`, `WindowTransientOwnerRegistry::close_step`, current anchors around lines 306-333). The identical progression now exists inside `TypedWindowTransientStoreOwner` for window partitions (anchors around lines 218-240), and the document-level registry has independent maintenance and close cursors (`plugin/🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs`, anchors around lines 26-52).

This is the correct rotation rule: zero-item grants leave the cursor unchanged; every nonzero selection advances before a potentially blocked drive; and removing a selected key still makes its lexical successor reachable on the next call. The retained-window-input fixture now includes source-level behavioral coverage for a paused first partition and a later owner kind (`plugin/🪟️window/🫧️transient/🧪️tests/🪟️retained-window-input/🦀️.rs`, `blocked_partition_and_owner_kind_do_not_starve_later_owners`, anchors around lines 128-149). The target is still compiling, so this audit verifies the implementation shape only, not runtime behavior.

### P0 — The Generic Helper Cannot Retire Any Root Whose Printed DSL Exceeds One Grant

`BoundedTransientRootRetirementFactory::retire` prints the complete root and retains its full byte count (`plugin/🫧️transient/🧵️publication/🦀️.rs`, currently lines 128-135). `BoundedTransientRootRetirement::close_step` cannot release anything until the complete count fits the current grant, then drops the whole `Arc` in one call (currently lines 99-117). The store disposer repeats the same whole-root print and all-or-nothing release for a displaced `TransientStore` (currently lines 161-189).

For Jack Results, `JackResultsWindowTransient` includes a `QueryResult` (`plugins/🔱️trinity/.../📊️results/🫧️transient/🧬️schema/🦀️.rs`, currently lines 5-13) and is wired to all three generic helpers (`.../📊️results/🫧️transient/🦀️.rs`, currently lines 48-63). A root larger than the ordinary 4,096-byte grant can neither publish its displaced root retirement nor retire the document-displaced partition. This is a terminal liveness failure, not merely incorrect accounting.

Remove the claim that these helpers are bounded. Do not use them for any state with unbounded payloads. A concrete owner must release the state in cursor-sized pieces and report no more than the supplied byte grant each step.

### P0 — Preparation Builds And Later Drops Whole Owners Without A Retirement Cursor

The generic preparation encodes the mutation in both `preflight` and `begin` (currently lines 30-39), computes `diff` and `apply` against the whole captured base (lines 47-61), and stores both the request and the entire next root. The request owns an `Arc` to the captured base; the prepared value owns the candidate root. Their sizes are unrelated to the encoded mutation length.

On cancellation or failure, `BoundedTransientPreparation::close_step` takes and drops the request/prepared owners in one call while reporting only the encoded mutation byte count (currently lines 84-96). A compact mutation may produce a large state, and a prepared root may be large even when the mutation footprint is admissible. This gives both an unbounded close operation and false byte accounting. It also offers no cancellation checkpoint inside clone/diff/apply.

The Jack replacement mutation demonstrates the problematic whole-state operation: `ReplaceQueryResult::diff` clones the base and clones the full result (`plugins/🔱️trinity/.../📊️replace-query-result/🦀️.rs`, currently lines 15-25). The general helper has no way to make that work incremental.

Require a concrete preparation owner for each window-state family that holds the mutation, captured base, candidate, and intermediate data behind explicit retirement cursors. Its `advance` must make bounded, cancellable progress; its `close_step` must retire every owned field under the actual grant. Admission must bound both retained input and the produced candidate or reserve a domain-specific streaming representation before construction.

### P0 — Window Snapshot Aliases Are Untracked, So Final Destruction Can Escape Ownership

`WindowTransientSnapshot` is cloneable and directly stores `Arc<dyn Any + Send + Sync>` (`plugin/🪟️window/🫧️transient/🦀️.rs`, currently lines 62-90). Capture uses `TransientStore::current_root()` (lines 188-196), and retained tool input copies the snapshot (in `plugin/🦀️.rs`, currently line 21128). The public `window_transient_snapshot` API also hands the owner to arbitrary callers (currently lines 17393-17396).

The generic root retirement merely drops its own `Arc` (publication helper lines 104-117); the store disposer later drops a whole displaced store (lines 171-182). Neither knows whether a snapshot alias remains. Consequently, a stale tool snapshot can be the final strong owner and recursively destroy a large state outside an owner cursor. Replacing the helper with `shared_retirement` alone is insufficient: it can wait for aliases, but it does not register their eventual return or prove that the remaining alias belongs to the correct retirement order.

The existing Store implementation supplies the needed ownership model. `SnapshotRead` drops the reader alias before marking the exact registry slot returned (`Store/🦀️.rs`, currently lines 249-307); the close cursor pumps returned reads before displaced roots (`Store/🦀️.rs`, currently lines 1870-1903). `ReturnedSnapshotReadRetirement` either passes the unique value to a concrete owned retirement factory or relinquishes its alias while the authoritative root remains (`Store/🦀️.rs`, currently lines 1511-1556). `shared_retirement` is deliberately only a lower-level blocked-until-unique primitive (`Store/♻️retirement/🦀️.rs`, currently lines 290-354); it cannot replace the returned-read registry.

Replace direct retained `Arc` snapshots for window state with a non-cloneable, exact `WindowTransientRead` capability backed by a per-partition read-lease registry. On worker completion/cancellation, return the capability to the partition registry. The partition's retirement cursor must first drain returned capabilities, then displaced roots, then the live root; it is terminal only when the lease registry and each of those owners is terminal-empty. Immediate render paths can borrow state without creating a retained capability; the public retained snapshot API must return this capability or be removed.

## Reusable Owner Interface Plan

1. Move the direct root lifetime mechanics into Store, beside `RetireOwned` and the existing returned snapshot-read facilities. Add a lease-aware transient root owner with a fixed-capacity read registry, a bounded returned-read pump, and a fixed-capacity displaced-root queue. Do not export raw state `Arc`s across retained-work boundaries.
2. Make `WindowTransientOwner::State` and `Mutation` implement `store::retirement::RetireOwned`. This binds each field's destructive path to a `RetirementCursor`; it does not permit a whole-DSl-size estimate as a substitute for actual retirement.
3. Replace the three unrelated `build_*` methods with one exact owner bundle: a domain preparation factory, owned state/mutation retirement factories, and a partition-store disposer. The bundle is installed once for each concrete state. Remove the generic `bounded_window_transient_*` constructors from production use.
4. Keep `ArtifactEphemeralOneItemPreparationFactory` as the scheduling shape, but require each domain factory to retain all input and intermediate owners in cursor-backed slots. It must expose a footprint that reflects every retained owner, advance only within its granted item/byte budget, and on close dispose request, base-read capability, prepared candidate, mutation, and fault text through their own cursors.
5. Make `TransientStore` publish a candidate only after the lease-aware root owner reserves a displaced-root slot. On publication acknowledgement, the publication retires its preparation only; the partition root owner drains returned reads before the displaced root. On document replacement/close, the partition disposer follows the same order for its current root.
6. Give both the document-retirement registry and a concrete owner's partition registry round-robin cursors. Treat blocked owners as eligible on later rounds without letting them starve other slots.

This keeps the reusable interface domain-neutral while preserving domain-specific control over incremental result construction. It is preferable to a generic `P: Clone + ArtifactDsl` helper because those bounds do not provide a bounded clone, diff, serializer, or destructor.

## Required Tests

- Add a neutral fixture shared by TypeScript/Ajv and native tests for a 12,289-byte state under 4,096-byte grants. The expected trace must make forward progress in several steps, never report bytes above the grant, and reach terminal-empty. A one-step whole-state release must fail the contract.
- Add a native owner test with a large state and a compact mutation that expands it. Cancel after preparation has retained both base and candidate. Instrument its retirement cursor and assert that request, candidate, and mutation are released only through bounded steps; no `Drop` path may be the final payload destruction.
- Add a returned-window-read test modeled on Store's returned-read tests. Hold a retained window read, publish or replace its root, and assert retirement blocks while the read is live. Return it, verify the returned alias is pumped before the displaced root, then assert the concrete `RetireOwned` cursor performs final destruction exactly once.
- Add a document-replacement integration test through both `load_document_text` and `load_document_pack`: start a window transient and keyed/single tool work, reload identical document bytes, assert a new document generation with default state and preserved window config, reject old pending publication, cancel old work, and admit fresh keyed work. Repeat for parse/reset failure and rejected retained candidate and assert no generation/config/scope change.
- Add two fairness tests. First, hold the first retired document registry blocked and prove the second registry retires in a later maintenance turn. Second, hold the lexicographically first window partition blocked and prove the other partition retires. Fill distinct fixed slots only as needed to prove later replacements remain admissible while another slot is blocked.
- Preserve the existing zero-grant tests and extend them to preparation, returned-read pumping, document-registry retirement, and partition retirement. Every zero-item or zero-byte path must leave owners intact and report zero release.

## Conclusion

The document-generation fence, commit ordering, config preservation, and cancellation renewal are structurally sound. The two missing round-robin cursors are concrete defects in the new reset retirement path. The generic transient helpers remain unsafe for large or aliased values and must not be relied on to make the new window lifecycle bounded.

## Correction Status

After this audit identified the three independent fairness layers, the implementation owner reported adding the document-registry, owner-kind, and partition retirement cursors with behavioral tests. This report preserves the source-derived findings as the reason for that work. The live `window-document-generation-green-1.log` remains compilation-only at this update; the queued SDK target has no native result yet. A separate DAG native run is reported terminal green, but it does not validate the SDK retained-window-input target audited here.
