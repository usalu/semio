# 2026-10-12 product fixes: raw close drain, mounted window-config lane, transient disposer spin

Executor: prod-fix (Sonnet). No git-modifying command, no ticket operation, no sub-agent, no `🗑️generated`. All cargo ran in the foreground of one slot through `🚦️cargo-slot.sh` (`SLOTS=2`) with the private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/play-fleet/prod-fix/target` (deleted; the bins and logs too, `play-fleet/prod-fix/keep/` holds the three result files). Test binaries outside cargo ran with `RUST_MIN_STACK=67108864` (plugin lib) and the editor-catalog script of pub-perf (hub). Every edit left the tree compiling (checked by the next build; one transient `[DEBUG]`-only compile error in the window owner existed for under a minute and was reverted at once).

## Result

| proof | before | after |
| --- | --- | --- |
| plugin `--lib`, one process per test (1 231 tests, 5 parallel, 180 s) vs `pl-tests/r15/results.tsv` (951 ok) | 951 ok (952 with pub-perf's new law) | **957 ok, 0 regressions vs r15**; newly green: the 3 target laws, `a_published_window_config_closes_through_its_own_published_physical_demand` (red at baseline), `window_config_paged_registry_actual_window_partition_custody`, and typed-close's structural-refusal law |
| `retained_latest_wins_raw_capacity_is_not_initialized_byte_retirement` (P1) | red on `partial-large-capacity` | ok |
| `empty_transient_retirement_waits_for_read_release_and_matches_neutral_vectors` (P3) | red (`heldReadBlocks: false`) | ok |
| NEW `a_mounted_window_config_publication_resumes_its_refresh_and_queues_the_retained_receipt` (P2) | red with the exact production fault, see below | ok |
| `NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-plugin:test-retained-raw-source --skip-nx-cache` | exit 1, 2 failing tests | **exit 0, 20 pass 0 fail, `tsc --noEmit` clean** |
| hub stdio `editor_catalog` (177 tests, one process each, 5 parallel, 300 s) | 160 ok / 16 FAILED / 1 TIMEOUT (pub-perf) | **160 ok / 16 FAILED / 1 TIMEOUT**, same classes: 11 editors without an edit fixture (docx x3, las, svg x3, tiff x2, xml x2), 5 `lossy-conversion` (jpg x2, pdf17 ua/vt/x), the 300 s `sqlite_snapshot_semio_any_editor` |
| `cargo check --lib` native, `semio-framework-plugin` + `semio-framework-os-kernel` | - | Finished, 0 errors |
| same with `--target wasm32-wasip2` | - | Finished, 0 errors |
| `cargo check --manifest-path 🌎️hub/Cargo.toml --workspace --exclude semio-hub --exclude semio-hub-auth-client --target wasm32-wasip2` | - | Finished (3 m 49 s), 0 errors |
| plugin lib test build (`--lib --no-run`) | - | Finished, 0 errors |

## P1: the raw `Vec<u8>` drain (retained command job close)

The proposal of `📓️2026-10-12-raw-suites.md` (a `clear()` step of copy 0 before the capacity release) was checked and is NOT the right fix. It removes the copy charge, but it keeps a pointless one-item turn per command: a `Vec<u8>` of `u8` has no drop work, so the capacity-release branch (`drop(std::mem::take(&mut self.raw))`) already frees initialized and uninitialized bytes in one deallocation, whatever `len()` is. The fixture v3, the TS oracle and the Rust law say copy 0 and one release of the whole capacity; a `clear()` turn is a logical drain by another name.

Fix (`🧵️retained-command/🦀️.rs`): the two `if !self.raw.is_empty()` branches (`close_demands`: `copy_bytes:1`, `close_step`: truncate by `min(len, grant.maximum_copy_bytes)` charging `copied_bytes`) are deleted. Consequences: no `ceil(len / copy grant)` turns per command (32 for 1 MiB at the shipped 32 KiB copy grant), no zero-copy-grant unfundable demand (#72a shape), and a non-empty `raw` is retired exactly like an empty one with capacity.

Oracles (source twins) updated to the new truth, each proven:
- `🧵️retained-command/🧪️tests/♻️raw-allocation-close/🟦️.ts`: asserts the capacity-release demand and branch, and the ABSENCE of any `raw.is_empty()` / `truncate(` / `len().min(copy)` branch (`bun test` green inside the nx target).
- `🧪️tests/🔬️tool-job-latest-wins/🟦️.ts` (raw close section of `verify interactivity tool-jobs`): `rawClose` now requires no drain branch and the release branch; the old `len().min(...)` hostile was replaced by two drain hostiles (the truncate drain and the proposed `clear()` drain). Verified in isolation (the section extracted into a scratch script: `RAW-OK`, all hostiles refused). That self-test function as a whole already dies EARLIER at `exact(source)` on the obligation `release_current: std::sync::Arc::ptr_eq(&scope.publication_claim, &self.publication_claim)`, identically on HEAD's `🦀️.rs`, so it is red independently of this work (stale gate, owner of the cancellation lease).

## P2: the mounted WindowConfig lane

Three independent defects sat behind "the next unit fails with `awaits an ACK without its retained host page`". The law (`🧪️tests/🔬️tool-run/🦀️.rs`, `a_mounted_window_config_publication_resumes_its_refresh_and_queues_the_retained_receipt`) builds a blank mounted operation (`test_blank_mounted_operation`) over the toy app's real window-config partition, installs a real cancellation lease, the captured window authority and an `Emit` with one `WindowConfigMutation`, then drives `publish_mounted_typed_operation_unit` exactly as the host pump does until the result page, ACKs it, drives the terminal page, and retires every owner (`close_step`, granted retirement, lease retirement, `retirement_step`) to `terminal_is_empty`, then closes the app.

1. **Reproduced** (baseline code, only the law added): `Fault { code: "plugin.internal", message: "artifact-store publication awaits an ACK without its retained host page" }`. Cause (as the report said): the `Published` arm stored `pending_window_config_receipt`, called `refresh` (first call always `Pending(copied_items:1)`: it only captures `pending_snapshot`), and returned `Ok(())`; the next unit advanced the already published store (`AwaitingAck`) and nothing re-read the retained receipt (only `retirement_demands/step` read it). Fix (`🔌️plugin/🦀️.rs`): new `advance_mounted_window_config_refresh(mounted, retained)` used by the `Published` arm AND by a re-entry check right after `pending.is_closing()` (`if mounted.pending_window_config_receipt.is_some() { return ...; }`); it continues `refresh` under the unit's own grant (not `mounted_policy.maintenance`), reports the refresh progress as the unit's ownership receipt (`publication_ownership_progress`, so a host turn accounts it exactly) and bumps `publication_progress` (the stall ceiling sees movement), and once `Ready` it clears the receipt, notes the tool run and queues the page of exactly that retained receipt.
2. **Found behind it** (only visible once 1 was in): every window-config partition had NO preparation issuer: the mounted batch path (`TypedWindowConfigStoreOwner::advance` -> `admit_member_apply_batch`) uses the store's catalog factory, but `bounded_window_config_store_owners` built the catalog without it and `WindowConfigOwner::build_one_item_preparation_factory` (declared by every window owner in `✏️s`, e.g. cad) was never called. The fault was `batched publication requires its original typed preparation issuer`; so no real editor could have published a window config through the mounted lane. Fix: `store::funded_bounded_artifact_store_owners_with_one_item_preparation` (new, next to `funded_bounded_artifact_store_owners`; prices the source from the constructed Arc extent plus the factory's quoted ticket tree and installs it through the sanctioned `bounded_artifact_store_owners_with_one_item_preparation` catalog path, so the catalog retires it like every other original factory) and `bounded_window_config_store_owners::<O>()` now passes `O::build_one_item_preparation_factory()`. The window-config owners in `✏️s` need no change (they already call `bounded_window_config_store_owners`).
3. **Found behind that** (the same defect that kept the pre-existing law `a_published_window_config_closes_through_its_own_published_physical_demand` red at baseline): closing a published partition panicked `snapshot registry is unavailable after exact owned close` (`🏪️store/🦀️.rs:749`): after the store disposer had closed the read registry, the window owner's `retirement_demands` still asked `returned_snapshot_read_retirement_demand`, which dereferenced the closed registry. Fix at the root: `ArtifactStore::returned_snapshot_read_retirement_demand` answers the empty demand for a closed registry (`SnapshotReadLeaseRegistryOwner::closed`). Both laws are green with it.
   - With that in, the close of a window partition quotes `copy_bytes: 21 408, depth 3` (the store catalog's factory ticket tree: snapshot, initial, mutation and disposer factories plus the operation-wire digest), which the 16 KiB NATIVE fixture-caller close grant can never fund, and the fixture close loop spun on an empty receipt until its 30 s deadline (the #72a silent-livelock shape again; it reproduces identically without item 2's factory, so it is not caused by it). The shipped policy (`MountedOwnerPolicyV1::INTERACTIVE_COMPONENT`, copy 32 768) funds it. Law side: `toy_window_config_caller_policy()` (fixture-caller policy with the shipped 32 KiB close copy page) in the tool-run tests, used by both window-config laws. Not changed: the shared fixture-caller JSON (other suites read it).

Oracle: `♻️cancellation/📦️lease/🧪️tests/🟦️.ts` "yielded window refresh keeps the original mutation and receipt before ACK" now asserts the new structure (`pending_window_config_receipt = Some(receipt);`, the re-entry call, the retained receipt read, the page queued from that receipt). It was one of the two red suites of the raw-suites report.

## P3: a transient disposer with a held read

`TransientStoreDisposer::close_step` forwarded to `TransientStoreRetirement::close_step`, which, with the root and the returned reads retired, answers `Progress(default)` while `reads` is not terminal-empty because a LIVE (not returned) tracked read holds the registry: a silent no-progress step. Fix: `TransientStoreRetirement::awaits_live_read()` (no root, no returned/active owner, the registry non-terminal and with nothing returned; `🏪️store/🦀️.rs`) and the disposer (`🫧️transient/🧵️publication/🦀️.rs`) answers `PluginLifecycleStep::Blocked { reason: "transient store retirement awaits the live tracked snapshot read that holds its registry" }` (the check precedes the grant quote, a blocked owner is blocked whatever the grant). The same defect existed in `BoundedTransientStoreDisposer` (the displaced store's returned-read drain went to `maintenance_returned_reads_step_root`, which also answers an empty step with only a live read): same fix with `TransientStore::live_read_holds_registry()`, reason "displaced transient store awaits the live tracked snapshot read that holds its registry". The neutral vector (`heldReadBlocks: true`) was right; no vector change.

## Files touched

All under `🧰️framework/🛍️products/💻️os/🔨️modules/` unless noted.
- `🔌️plugin/🧵️retained-command/🦀️.rs` (P1)
- `🔌️plugin/🧵️retained-command/🧪️tests/♻️raw-allocation-close/🟦️.ts` (P1 oracle)
- `🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts` (P1 oracle, raw section)
- `🔌️plugin/🦀️.rs` (P2: `advance_mounted_window_config_refresh`, the `Published` arm, the re-entry check)
- `🔌️plugin/🪟️window/🎚️config/🦀️.rs` (P2: owners installed with their preparation factory)
- `🏪️store/🦀️.rs` (P2: `funded_bounded_artifact_store_owners_with_one_item_preparation`, closed-registry demand; P3: `live_read_holds_registry`, `TransientStoreRetirement::awaits_live_read`)
- `🔌️plugin/🫧️transient/🧵️publication/🦀️.rs` (P3)
- `🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs` (P2 law + policy helper; the closes-through-physical-demand law now uses the helper)
- `🔌️plugin/♻️cancellation/📦️lease/🧪️tests/🟦️.ts` (P2 oracle)
- this report; `📓️2026-10-10-migration-corrections.md` (entry 74)
- scratch (kept small): `⚡️cache/play-fleet/prod-fix/{b.sh,one.sh,toggle_p2.py,keep/}`.

## Notes and leftovers

- A pre-existing, unrelated red stays: `toolJobLatestWinsSelfTests` (see P1).
- The 271 FAILED, 1 ABORT and 2 EXIT133 plugin-lib results are the known baseline: every one of them was non-ok in r15 (0 regressions), nothing new.
- Window partitions now each construct a preparation factory (`RetainedClonePreparationFactory` and its Arcs) at first capture and retire it with the catalog; pricing is the constructed Arc extent plus the quoted ticket tree (an under-pricing of the child Arcs would only show in an unscheduled catalog's ingress check, which funds itself from the same quote).
- OPEN: the window partition catalog's 21 408-byte copy quote is a real closing cost for any caller whose close copy grant is below 32 KiB; paging the factory tickets would remove it.
