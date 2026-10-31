# 1030 review: framework modules diff (waves 1028-1030)

Scope: `git diff HEAD -- 🧰️framework/🔨️modules` (102 files). Read-only; nothing built or run. Only the following store/plugin-instance/decl-app paths named in the brief are absent from this diff (they are not under `🔨️modules` or are already committed): plugin-instance dispatch/producer/resume, decl-app tool catalog, cold replay. Reviewed: value native_owned / derive, pack-record receiving, replication leaf-descriptor const validation, ui text-splice composer (new), tool-machine typing (admit/close, ledger), job `admit_original_construction`, process leases terminal release, native-build storage (funding replacement, funding backing, funding metadata, directory envelope, publication owner envelope), filesystem descriptor publication data. Not deeply reviewed (too large, hand-minified, no concrete defect found in the paths read): `📤️publication/🌐️platform/🟦️.ts` protected-maintenance scope, `📤️publication/🎛️owner/🌅️initial`, `📤️publication/🎛️owner/🧾️metadata`, `📁️directory/⚓️envelope`.

Lane names below are best guesses from file ownership (no lane map was given); coordinator may re-route.

## F1 (medium, confidence 70%) funding claim is stranded or its phase is clobbered after a durable settle/abort
File: `🏃️process/📦️artifacts/🏗️native-build/💾️storage/💰️funding/🔁️replacement/🟦️.ts:27` (`finish`), and `:26` (`recover`).
Mechanism, two distinct paths:
- (a) Concurrent duplicate `settle`/`abort` on one claim (retry timer plus original call, or settle racing abort). `requireClaim` only checks `owner.phase==="reserved"` before the `await`; both calls pass it. The first appends `funding-settled` and sets `phase="settled"`, `sequence++`. The second runs `nativeStorageEventV1` with the stale `expectedSequence`, throws "Exact native funding journal predecessor required", and its `catch` (`entered` is already true) sets `owner.phase="unresolved"`. A durably settled claim is now reported `unresolved` by `readNativeStorageFundingReplacementV1`, and `retireNativeFundingUnpreparedMetadataV1` (`phase!=="settled"`) refuses forever, though the journal is settled.
- (b) `finish` submits `expectedSequence: owner.sequence`, frozen at reserve time. The held writer lease is only held per `port.run` call, not between `reserve` and `settle`. Any other legitimate journal append in between (another owner's reservation, workspace allocation) makes `applyNativeStorageFundingEventV1` throw on the strict sequence compare, `entered` is true, the claim becomes `unresolved` and the reserved delta stays charged against capacity until a manual `recover` with a caller-discovered sequence.
Fix direction: serialize per claim (set an in-flight marker before the `await`, never overwrite a terminal phase in `catch`), and re-read the journal sequence inside the settle transaction (compare predecessor allocation identity, not a global sequence), or fail closed with a distinct retryable refusal that leaves the phase `reserved`.
Owner: bundle (native-build storage); runtime if the journal contract is the right place.

## F2 (medium, confidence 65%) `NativeReceive` derive silently changes `default = "path"` fields
File: `🌱️value/✨️derive/🛬️receive/🦀️.rs:42` (the compile-time refusal for `FieldDefault::Path` was deleted, 1029 extfix).
Before: a path default on a natively received struct failed to compile. Now `opt = !tuple && !attr.required && (optional(ty) || Bare || container default)`; a `Path` default does not set `opt`.
- Non-`Option` field with `#[value(default="crate::f")]` and the key absent: becomes natively required, so `missing` is `invalid()`. Wire input that serde (and the canonical wire) accepts is refused natively. Intended per the 1029 note ("natively required"), but it is a silent divergence from the canonical decode with no test in the diff and existing users (writer `#[value(default="crate::default_uri")]` in the plugin snapshot, generation3d `default_preview_cam_pos`).
- `Option<T>` field with a path default and key absent: `opt` is true, so the receiver fills `<Option<T> as NativeDefault>` = `None`, not the path function's value. Wrong value, no error. This one is a genuine data bug.
Fix direction: restore a refusal for `Path` combined with `optional(ty)` at least, and add a cross-decode law test (native vs serde JSON) for a path-default field in both shapes.
Owner: runtime (value derive); fix is shared with whichever lane first hits a path-default snapshot (writer/generation3d plugin lanes).

## F3 (low-medium, confidence 55%) const-eval uniqueness hash degenerates on common-suffix keys
File: `📡️replication/🎮️mutation/🪪️leaf-descriptor-validation/🦀️.rs:168-231`.
`identity_hash` hashes only the last 48 bytes of each key. Owners/semantic kinds that share a 48-byte suffix and differ in the head all collide to one slot; linear probing then does O(n^2) `mutation_leaf_descriptor_str_eq` calls, which is exactly the loop-progress problem the removed `#[allow(long_running_const_eval)]` was papering over. The new test uses 300 identical-suffix keys and passes only because 300^2/2 is still under the lint budget. A roster of ~1500+ leaves with a shared 48-byte tail (for example a long common leaf directory name) turns back into a deny-by-default const-eval error. Also `first_duplicate::<65536>` loops forever (const eval) at >=65536 hashed entries (table full, no empty slot); unreachable today, but there is no bound check.
Fix direction: hash a head window plus the tail (or fold the length and a stride sample) and assert `descriptors.len() < SLOTS`.
Owner: runtime.

## F4 (low, confidence 60%) fallible allocation replaced by aborting `Box::new` in original-construction closures
Files: `🛠️tool-machine/⌨️typing/✍️edit/🦀️.rs:47`, `🖱️ui/🎬️scene/✂️text-splice/🧬️native/🦀️.rs:43` (admit closure).
The old typing admit used `std::alloc::alloc` and mapped null to `ValueRefusalKind::AllocationFailed`. The new closure uses `Box::new(owner)`, so allocator exhaustion aborts the process instead of refusing; `record_constructed_allocation` only does accounting after the fact. Under the disk/memory pressure this fleet runs in, a refusable error became an abort.
Owner: stepctx.

## F5 (low, confidence 50%) text-splice composer `advance` cannot report "unfunded, zero progress"
File: `🖱️ui/🎬️scene/✂️text-splice/🧬️native/🦀️.rs:54` (`advance`), `🧵️advance.rs`.
`admit_original_construction` returns `Ok(false)` when the demand does not fit or `should_yield`; `advance` discards it and returns `Ok(self.ready())`, which is `false` for both "made one step" and "did nothing". A caller that loops until `ready()` without re-reading `demands()` spins forever when `demands().capacity_bytes` (phase 1 and 10: the whole scalar buffer, `lengths*4`) exceeds `maximum_capacity_bytes`. Also one scalar per turn (one `step` per `advance`), so a long text costs O(len) turns each paying the fixed ~700-byte quote; correct but slow. `Fence` compares string pointers plus lengths only, so a freed-and-reallocated pair of equal length at the same address passes `matches` (contract says the provider retains the pair; not enforced).
Owner: stepctx.

## F6 (low, confidence 50%) accepted terminal lease release is not idempotent against the normal release
File: `🏃️process/🔒️leases/🟦️.ts:107-118` (`releaseResourceLeaseTerminalV1`, `terminalLeaseIdentity`).
If the original body releases the lease normally after `acceptResourceLeaseTerminalV1` (phase becomes `closed`), the later terminal release fails the `issued.phase!=="held"` check with a "required accepted lease" error rather than returning as already released; `scope.released` is only set by the terminal path. A thrown `scope.release` also leaves `released=false` with a possibly non-`held` phase, so the retry throws the identity error and the lease stays retained ("retain-unresolved" is the documented outcome, so this is by design for the throw case; the normal-release-first case is the undocumented one).
Owner: runtime.

## F7 (low, confidence 50%) fabricated close progress
Files: `🎒️pack/🌱️value/🛬️decode/📦️document/🦀️.rs:29,156` and `📜️record-body-cursor/🦀️.rs:532`.
Zero-progress close turns were fixed by reporting `copied_items:1` / `released_items:1` for transitions that copy or release nothing (consistent with existing sibling transitions; guarded by `retained_clone_grant_admits` which needs `maximum_items>0`). Side effect: `consume_fuel(copied_items)` and any items ledger now count phantom items, and `RetainedCloneStep::Complete` from the Document driver reports `copied_items:1` for a pure flag flip. Acceptable, but the accounting laws (`receipt == real work`) are weakened; tests only assert `progress != default`.
Owner: resid.

## F8 (policy, certain) leftover `[DEBUG]` probe
File: `🎒️pack/🌱️value/🛬️decode/🫴️receiving/🧪️tests/🦀️.rs:137` (`debug_size_probe`, `eprintln!("[DEBUG] sizes ...")`). Brief and AGENTS.md require temporary logs removed before finishing. The JSON receiving test removed its `[DEBUG]` prefix from an assertion message (fine).
Owner: testsplit.

## Checked and found sound (no finding)
- `unsafe impl Send for NativeReceiveSlot` (`🌱️value/🛬️decode/🧳️owned/🌳️value/🦀️.rs:21`): all three owned parts are `Send` by `NativeReceiver: Send+'static`, `RetireOwned: Send`, `ErasedSnapshotRetirement: Send`; `I` is not stored. Law test guards the contract.
- `[T;N]` `NativeDefault`: guarded by `retained_clone_grant_admits`, `N==0` refused, `T: Copy`.
- Scalar-enum `DslReceiving` expansion: ordinals use the same declaration order as `DslScalar::emit_scalar`; `take_source` leaves the source intact on refusal. Not compiled here, a duplicate-impl clash with `DslScalar`'s borrowed codegen is the one thing a build must confirm.
- Typing ledger `writing` flag: `begin_original_write` is guarded by `original_write_pending` in `admit`; `end_original_write` is called once in the final `close_step` branch before payment, state stays consistent if `paid` fails.
- Directory envelope / funding backing state machines: ancestry, namespace and intent append-only checks are consistent; no ABA found in the `WeakMap`-keyed private handles (handles are fresh frozen objects, `settled` is one-way).
