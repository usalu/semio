# Wave B0 — `did not pre-admit the exact operation slot`

Ticket 26/09/02/PUZZLE-3D-END-TO-END. Status: **fixed, laws green**.

## 1. Root cause

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (pre-fix line numbers)

- `:23476` `let operation_id = semio_framework_job::allocate_operation_id();`
- `:23477-23484` the admission test and the fault:
  `!self.can_admit_typed_operation(operation_id.0) || !self.latest_wins_commands.can_insert(..) || latest_wins_order.len() >= 64 || !self.segmented_downloads.can_insert(..) || !self.segmented_closures.can_insert(..)`
  → `FaultCode("interactive-job.typed-operation-capacity")`, message
  `fixed typed-operation and segmented-output authorities did not pre-admit the exact operation slot`.
- `:23464` `can_admit_typed_operation(op) = tool_operations.can_insert(op) && typed_operation_reservations[op % 64].is_none()`
- `:15837-15895` `ArtifactFixedRegistry` — a **direct-mapped, probe-free** table: `slot = id % ARTIFACT_LIVE_OUTPUT_SLOTS (64)`, `can_insert(id) = slot vacant`, `get/remove` verify exact identity at that one slot.
- `🧰️framework/🔨️modules/⏱️trace/🦀️.rs:873-878` `allocate_operation_id()` = `NEXT_OPERATION_ID.fetch_add(1)` — one **process-global** counter shared with envelope ingress, media exports, local-interaction and every other operation minted in the guest.

### The admission model, as it actually worked

1. Ingress (`dispatch_action` / `dispatch_command` → `dispatch_typed_command_inner`) **minted an id first** from the shared counter.
2. It then asked a single yes/no question: *is the residue class `id % 64` free in all five fixed authorities?*
3. There is **no probing and no free-slot search**. If that one class was occupied by an unrelated live operation, the action was refused outright — with 63 slots empty.
4. Only the latest-wins branch (`:23498`) then writes `typed_operation_reservations[slot] = Some(id)`, so the slot is held for the whole pending→worker handoff (`advance_latest_wins_command_one`, `:22783-22799`) and cleared at `:20157 / :22799 / :22841 / :23724`.

So the refusal was never a capacity problem: it was a **hash collision in a 64-entry direct-mapped table whose keys come from a counter the admitting side does not control**. Every puzzle3d action is a typed operation (see §4), the battery fires them continuously (`suggestionsTick` ~4 Hz, hover, camera), and each live/parked operation poisons one residue class for every action minted into it. That is why `suggestionsTick` ×10, `engagementAbort` ×3, `setCamera` ×2, `transformBegin` ×1 and the earlier mesh-mode `translateSelection` (gumball drag → `sceneDelta=false`) all died on the same message while the *same* action ids also settled successfully elsewhere in the same battery (`probe-2026-09-11T12-10-48.md`: `performInvocation settled … actionId":"setCamera"` next to `action failed setCamera … did not pre-admit`). Intermittent, action-agnostic, storm-amplified — exactly a modulo collision, not a classification bug and not a leak.

The identical mint-then-test defect existed at two more sites, fixed with the same primitive:
- `begin_artifact_envelope_ingress` (`:19425`, `artifact-envelope.ingress-saturated`)
- the media-export start (`:21898`, `fixed media export authority collided at the exact operation slot`)

## 2. The fix (architectural, no bigger constant, no swallowed fault)

**The ingress reserves the exact slot before an operation id exists, and mints the id FOR that slot.**

| file | change |
| --- | --- |
| `🧰️framework/🔨️modules/⏱️trace/🦀️.rs` | new `allocate_operation_id_in_slot(slots, slot)` — `fetch_update` advances the shared counter to the next value congruent to `slot (mod slots)`, so ids stay process-unique, monotone and never `0`. |
| `🧰️framework/🔨️modules/🧵️job/🦀️.rs` | re-export `allocate_operation_id_in_slot`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | `ArtifactFixedRegistry::slot_is_vacant(slot)` (`can_insert` now delegates to it). |
| ″ | `VcsArtifactApp::typed_operation_slot_is_vacant(slot)` — vacancy across `tool_operations`, `typed_operation_reservations`, `latest_wins_commands`, `segmented_downloads`, `segmented_closures`. |
| ″ | `VcsArtifactApp::admit_typed_operation_slot()` — checks the latest-wins FIFO, finds the first vacant class, mints the id for it; `None` only when all 64 are live. |
| ″ | `dispatch_typed_command_inner` uses it; the fault now reads **`every fixed typed-operation and segmented-output slot already owns a live operation`** and can only fire at true saturation. |
| ″ | `begin_artifact_envelope_ingress` and the media-export start converted to the same pre-admit-then-mint shape (messages updated to name real saturation). |

`can_admit_typed_operation` is unchanged — it is still the exactness assertion used by the latest-wins handoff and by `start_typed_command_operation`'s asserts, which now hold by construction.

No host (TS) half is required: `🔌️PluginRuntime`, `🏛️ShellHost` and `🌐️World3dHost` carry no slot or capacity notion; the fault was raised entirely inside the guest and only surfaced by the host.

## 3. Law

`typed_operation_ingress_pre_admits_the_exact_slot_before_it_mints_an_operation_id`
— `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs` (entry point),
body `test_typed_operation_slot_preadmission` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`.

It holds 63 residue classes with foreign reservations, leaving slot 11, then:
1. 64 consecutive admissions must EVERY time land on slot 11 (64 consecutive counter values cover all 64 classes, so the mint-then-test model fails this deterministically, not probabilistically);
2. a real `dispatch_typed_command_inner` must succeed and own slot 11;
3. only with all 64 classes live may dispatch refuse, with `interactive-job.typed-operation-capacity`;
4. the pending command is then cancelled, mounted as its terminal fault page, ACKed and closed to terminal emptiness.

**Reproduces the defect first** — with `admit_typed_operation_slot` temporarily reverted to mint-then-test:

```
running 1 test
test component::plugin_runtime::plugin_builder_contract_tests::typed_operation_ingress_pre_admits_the_exact_slot_before_it_mints_an_operation_id ... FAILED
panicked at 🦀️.rs:17312:61:
one vacant residue class still pre-admits an exact operation slot
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 649 filtered out
```

**Passes after:**

```
running 1 test
[DEBUG] actual typed ingress pre-admitted slot 11 under 63 live foreign reservations and refused only at true saturation
test component::plugin_runtime::plugin_builder_contract_tests::typed_operation_ingress_pre_admits_the_exact_slot_before_it_mints_an_operation_id ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 651 filtered out; finished in 0.09s
```

### Neighbouring law set

`cargo test -p semio-framework-plugin --lib -- typed_operation_ingress_pre_admits artifact_fixed_registry_tests artifact_media_export_credit_tests reserved_undo local_interaction`
(`RUST_MIN_STACK=67108864`; see §5)

```
test result: FAILED. 54 passed; 1 failed; 0 ignored; 0 measured; 597 filtered out; finished in 3.70s
failures:
    …local_interaction_dispatch::local_interaction_registered_query_channel_continuation_ack_and_close
```

`cargo test -p semio-framework-trace`:

```
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

`cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly`:

```
warning: `semio-s-artifact-puzzle-3d` (lib) generated 88 warnings (run `cargo fix --lib -p semio-s-artifact-puzzle-3d` to apply 85 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.87s
```
**0 errors.** The 88 warnings are pre-existing (dead code / unused).

## 4. Every puzzle3d action id classified as a typed operation

`host_configuration_mutation` returns `Ok(None)` for every action (`✏️editor/🦀️.rs:7588`), and none of them is framework-reserved or `setActiveTool`/`setActiveUtility`, so **all 66 declared actions** take the typed path through `dispatch_typed_command_inner` and therefore through the operation-slot admission. All are declared `InteractiveJobClassification::Migrated` at `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:8344-8410` — **every declaration is correct; none needed fixing.** The five faulting battery actions are all in this list, and so is every action the battery can drive:

```
acceptSuggestion addBrushObject addObjectKind addTargetVolume cancelFillBuild closeVortexSuggestions
createAttraction cycleBrushCandidate cycleBrushCandidateBack deleteAttraction deleteSelection
deleteTargetVolume duplicateSelection engagementAbort engagementControlSelect engagementInput
engagementRepeatLast engagementSubmit exportFixture fillBuildTick focusSelection hoverSuggestion
importFixture openAddObjectDialog openImportFixture openVortexSuggestions patchInspector
registerBrushMesh relocateTargetVolume rotateSelection scaleSelection selectSameKindSelection
setActiveExample setBrushPlacementOverlapBudget setCamera setChunkSize setFillCount
setGridSnapEnabled setGridSpacing setGridVisible setLodAutomatic setLodDepthVariable setLodManual
setObjectKindWeight setPanelPage setProjection setProjectionParam setProximityRadius
setSelectableKind setSelectionFlag setSunAzimuth setSunElevation setSunIntensity
setTargetVolumeFlag setTransformGumballFlag setVortexDirection setVortexKindWeight setVortexShow
setVoxelDims suggestionsTick toggleSun transformBegin transformEnd translateSelection
worldPointerDown worldRelocate
```

## 5. Pre-existing red in the crate — NOT from this wave

A peer is mid-refactor of the typed publication / emit path in the same file while this wave ran (`git diff` shows their hunks removing the `_ if failed =>` publication guards at `:23161-23241`, moving `Self::mint_extension_invocations` into the effect drain at `:23331`, adding `window_transient_store.refresh` at `:23317/:23446`, and deleting `async fn dispatch_typed_command(`). Their work makes these fail at the moment; each was verified **not** to be caused by this wave:

| test | why it is not ours |
| --- | --- |
| `fixture_contract_is_anchored_to_the_production_retained_factory_publisher_and_host_receivers` | source anchor `output.typed_operation_result.as_ref()` in `⚛️reactor/🦀️.rs`, a file this wave never touched and which is clean vs HEAD. |
| `full_operation_source_rejects_generic_reducers_and_old_monolithic_shells` | anchor `require_complete_tool_operation_pipeline` before the decoder; this wave's diff never touches that symbol. |
| `host_configuration_uses_one_bounded_event_sourced_lane_before_the_generic_gate` | anchor `async fn dispatch_typed_command(` — the peer deleted that function. |
| `retained_latest_wins_registered_dispatch_rebases_worker_and_publishes_real_document` | **reproduced identically (`left: 13 right: 97`) with `admit_typed_operation_slot` reverted to mint-then-test.** |
| `retained_latest_wins_reserved_slots_and_ready_publisher_are_fair` | **reproduced identically (`typed operation lost its persistent worker session before publication`) with the fix reverted.** Builds its mounted operations by hand; never reaches the ingress. |
| `local_interaction_registered_query_channel_continuation_ack_and_close` | `assert!(presence.is_empty())` on an Ephemeral frame from the emit path the peer is rewriting; the local-interaction query path never reaches typed-operation slot admission. |
| `instance_lifetime_close_rejects_foreign_root_and_exhaustion_before_detach` | debug-build **stack overflow** (`VcsArtifactApp` frame size); passes with `RUST_MIN_STACK=67108864`. Unrelated to this wave — use that env var when running this crate's laws in debug. |

## 6. Remaining risk

- The fault can still fire — but now only at real saturation of all 64 slots. If the browser battery shows `every fixed typed-operation and segmented-output slot already owns a live operation`, that is a genuine **retirement leak** (operations mounted and never closed), a different defect, and the message says so.
- `admit_typed_operation_slot` always takes the lowest vacant class, so ids are no longer consecutive per app. Nothing orders by id — `latest_wins_order` is push-ordered FIFO, `Operation::new` only carries the id as a seed — but any future code that assumes dense/consecutive typed operation ids would be wrong.
- The shared `NEXT_OPERATION_ID` now advances by up to 64 per typed admission instead of 1 (irrelevant for u64, noted for anyone reading counter values as a count).
- A coordinator re-probe needs a fresh release wasm: the fix is entirely inside `semio-framework-plugin`, which compiles into the guest component.
