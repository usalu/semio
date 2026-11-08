STATUS: PARTIAL. VERIFIED native lib: engine-owners, initialization-runtime (W-engineinit-104), initialization-replay (W-engineinit-109 rc 0). plugin catalog/ingress, window-config, plugin genesis callers EDITED-unverified; wasm not run.

## Edits (all EDITED-unverified)
- Hydration callers: document hydration `step_close_envelope_owners` (`cx.with_original_effect` + `record_member_step_performed`); document hydration close ladder (`retire_initial_snapshot_owned(&mut self.initial, &mut effect.nested(child))`, final catalog close nested effect; Pending returns an empty progress and keeps the slot); config hydration close ladder (macro `initial_snapshot!` for pending_snapshot/validation/current, catalog close nested effect); config `advance` snapshot retire via a cold `AuthoringWallet` (no turn there, tests only).
- Runtime layer `🏃️runtime/📥️lifted`: `retire_step(effect)`, `settle_current_retirement_step(effect)`, `close_step(factory, effect)` (phase 0 via `FactorySharedRetirement::admit_original` slot form); replay: `retire_current` helper, `adopt_native_next(.., effect)`, `adopt_next_funded(.., effect)`, `settle_native_read_step(.., effect)`, `close_reads(.., effect)`.
- Store `initialization/🔁️replay/🦀️.rs`: `settle`, RetireIndex phase under `cx.with_original_effect`; `runtime.close_step`/`edit_index.retire_step` get nested effects.
- Plugin `OriginalCatalogConstructor::{advance,close,close_refusal}(effect)`; ingress `drive_store_constructor_unit(constructor, cx, grant)` (in-turn, performed work = fuel), `_cold` twin (artifact-app-laws fixture), `close_store_constructor_unit(constructor, effect)` under `original_lifecycle_effect_step` in plugin-app-impl `close_step`.
- Window-config `catalog_step(effect)`, `admit_catalog(.., effect)` (trait, impl, registry), `original_close_step` catalog close.

## Open
- `initialization/🔁️replay/🧬️native/🦀️.rs` (NativeInitializationFold) is still on the old `with_native_receiving(|control|..)`, grant-shaped `physical`/`physical_work`, `adopt_native_next(.., grant)` and `settle_native_read_step(.., grant)` closures; it cannot compile until ported (not claimed by anyone).
- codec `📦️codec/🎮️operation` `close_uninstalled_owners_step(grant)` (store6) and tests naming old shapes.
Rerun: `chk.sh W-engineinit-102 Cargo.toml native -p semio-os-store-engine-owners -p semio-os-store-initialization-runtime -p semio-os-store-initialization-replay -p semio-plugin-window-config`

## Update
- Native fold `🔁️replay/🧬️native/🦀️.rs` ported: `physical` on `with_native_receiving(&mut outcome, |control,_|..)` + `refuse_scope`/`post_receiving`; `physical_work` and `close_poll` on `with_original_effect`; fns now return `OriginalTurnFailure`; `retire_native` stores a close failure in `original_failure`. EDITED-unverified.
- Fixed after W-engineinit-102: owners `Err((failure,receipt))`, runtime `turn.result?` (poll error type is ValueError).
- `InitializationEnvelope::awaiting_envelope` made pub (typedpack request), uncompiled.
- Rerun: `chk.sh W-engineinit-106 Cargo.toml native -p semio-os-store-initialization-replay -p semio-plugin-window-config -p semio-plugin`, then the same with `wasm`. Needs engine-bundle (gateway) and engine-close (store7) green first.
W-engineinit-105 rc 1: only foreign semio-os-store-engine-stage (13) failed; none of my crates reported. Running W-engineinit-106 native.

## Final (04:4x)
- W-engineinit-106 (native, initialization-replay + window-config + plugin) rc 1: only foreign `semio-os-store-replay` failed, E0425 at `🧱️replay-owned-state-and-result-types/🦀️.rs:83`: `semio_framework_value::NativeAllocationIdentity` is not exported at value's root (it lives at `semio_framework_value::retained_clone::NativeAllocationIdentity`). I changed that path in the replay file (EDITED-unverified; alternative is restoring the root re-export in value).
- W-engineinit-107 started with that fix (same crates, native); it was queued/running at wrap-up: read `🗑️generated/1105b/W-engineinit-107.{txt,rc}`. My crates remain EDITED-unverified until it finishes; wasm not run.
- Rerun after 107: same `-p` set with `wasm`.

## Update (initialization-replay port, EDITED-unverified)
- Root `🦀️.rs`: construction now uses the resident `OriginalConstructionAdmission` protocol (`construction` + `construction_failure` fields; `built(cx, admission, failure, output, demand, construct)` = admit + `Ready -> reset_step`, `Pending -> false`, `Failed -> take_failure_into_step` + invariant error; failure retired via `artifact_retirement_admit_owned` in `close_original`, quoted in `close_demands`, drained in `retire_native`); `admit_awaiting_envelope` now takes the caller's admission and failure slot (plugin genesis, still on the old `admit_original_construction`, must pass them when it is ported). Also: `receive_digest_turn` fuel u128, `bind_genesis` on the new receiving scope, registry birth close ledger, actor `step(effect)`.
- Native fold: `.into()` on invalid, `demands(..., u64::MAX)` for closing quotes, `value_poll_outcome` for `admit_original`.

## Final (W-engineinit-109/110)
- W-engineinit-109 (native, semio-os-store-initialization-replay): rc 0, VERIFIED (native lib; wasm and tests not run).
- W-engineinit-110 (native, window-config + plugin): rc 1, only foreign `semio-os-store-engine-replay` (135 errors, not mine) failed, so window-config and plugin (catalog constructor, ingress, genesis callers) are still unverified. Rerun: `chk.sh W-engineinit-111 Cargo.toml native -p semio-plugin-window-config -p semio-plugin` once engine-replay compiles, then `wasm`.
- Plugin genesis (`🌱️genesis/🦀️.rs`) still calls the old `admit_original_construction`/`admit_awaiting_envelope` shape and needs an admission + failure slot (owner: genesis lane).

## Plugin genesis (🌱️initialization/🌱️genesis/🦀️.rs): NOT ported, scoping finding
- Not a local edit. Nine `built`/`built_with` sites sit on the old `admit_original_construction(cx,output,demand,|control|..)`; the new protocol needs a resident `OriginalConstructionAdmission` plus a failure slot per construction.
- Two births are static and happen before any owner exists: `OriginalStoreGenesis::admit(window,actor,cx)` and the trait method `OriginalStoreInitialSource::admit(cx,output)` (implemented by `FixedStoreInitial` and by app crates: energy, raster and others). Their admission/failure slot must be owned by the caller, so the trait signature and every implementor/caller change.
- The module also still uses the grant-shaped `retired(cx,d,|grant|..)` over `close_owned_store_step(grant)`, `cursor.close(grant)`, `outcome.close_step(grant)` (store slot `📦️store/🦀️.rs` and genesis `close_step`), which are effect-shaped now. `built_with` closures read `control.ledger_identity()`/`retirement_recipient_identity()`; `NativeConstructionScope` provides both.
- Proposed design for whoever takes it: a public `OriginalStoreBirth{admission,failure}` owned by the mounting caller and threaded into both static `admit`s; genesis owns its own admission/failure for later constructions and drains them in `close_step`/`terminal_is_empty`; `retired` becomes `close_original_effect_step`.
- Claim released; `admit_awaiting_envelope(.., admission, failure, output)` (store replay initializer) is ready for it.
- W-engineinit-111 not run: engine-replay (enginestage) had not posted green.
- initialization-replay resident construction outputs (edit_out/checkpoint_out/phase_out/unit_out/commit_out): a local output slot would be dropped between the constructing turn and the Ready turn and rebuild twice; W-engineinit-111 compiles it.
- genesis core ported (EDITED-unverified): OriginalStoreBirth{admission,failure,active} with built/built_with/close_step, trait OriginalStoreInitialSource::admit(cx,birth,output), FixedStoreInitial owns a birth (drained in close_step), OriginalStoreGenesis::admit(window,actor,cx,birth,output)->bool, genesis owns birth+unit slot, close ladder on retired_effect, store slot close on retired_effect. Callers/implementors next: energy state source, ephemeral lanes, instance stores, window-config birth, tests.
- energy source EnergyNativeInitialSource ported (EDITED-unverified).
- instance stores set (lane_birth, advance_lane) and window-config store-owners (birth field, admit call, close drain) ported (EDITED-unverified). Same applies to the genesis frame binding: OriginalConstructionAdmission keeps its Source binding until close_step, so every owner of an OriginalStoreBirth must drain it in its close ladder (done for FixedStoreInitial, genesis, energy source, instance set, window owner). W-engineinit-111 (initialization-replay with resident outputs) rc 0, verified.

## Genesis port status (EDITED-unverified; nothing compiled, blocked by engine-replay)
Done: genesis core (`OriginalStoreBirth`, `FixedStoreInitial`, `OriginalStoreGenesis::admit(window,actor,cx,birth,output)->bool`, close ladder on `retired_effect`, store slot), energy `EnergyNativeInitialSource`, instance stores set (`lane_birth`), window-config store owner (`birth`), ephemeral source (`birth` for the two `Initial::admit` calls). Each owner of a birth drains it in its close ladder (the frame keeps its Source binding until `close_step`).
Not done (call sites still use the old `admit(..., cx)` -> Option shape; each needs a resident `OriginalStoreBirth` + output slot and a loop until the slot is Some):
- tests: energy `🌱️initialization/🧪️tests/🏪️store.rs:10`, energy config `🎚️config/🌱️initialization/🧪️tests/🦀️.rs:13`, energy `🌱️state` tests `🧪️tests/🦀️.rs:10` (`EnergyNativeInitialSource::admit(cx,&mut source)`), raster `✏️editor/🌱️initialization/🧪️tests/🦀️.rs:7`, genesis `🧪️tests/🎟️admission/🦀️.rs:12,18`, declaration `🕳️empty-lanes/genesis-tests.rs:8`.
- `OriginalArtifactStoreSet::admit` (instance stores) and its construction `step`/`take`, the instance construction factory, and the ephemeral source's other four constructions are still on the old `admit_original_construction(cx,output,demand,|control|..)`; they are separate ports (owner: instance-construction lane), not caused by this change.
- Claims: genesis, instance stores released.
- energy tests ported: 🌱️initialization/🧪️tests/{🦀️.rs (admit_source, close_frame helpers),🏪️store.rs}, 🎚️config/🌱️initialization/🧪️tests/🦀️.rs (EDITED-unverified)
- raster 🌱️initialization/🧪️tests/🦀️.rs ported (EDITED-unverified)
- genesis 🧪️tests/🎟️admission/🦀️.rs ported (born/finish with frame; EDITED-unverified)
- declaration 🕳️empty-lanes/genesis-tests.rs ported (EDITED-unverified)
- instance stores set admit(birth)/step/take ported with resident moved/handed slots (EDITED-unverified)

## Stop point (06:xx)
Done since the last block (all EDITED-unverified): energy/raster/genesis/declaration tests moved to a resident `OriginalStoreBirth` + output slot loop with a frame drain; `OriginalArtifactStoreSet::{admit(birth),step,take}` on resident `moved`/`handed` slots.
NOT done, needs a decision beyond this lane: the instance construction factory (`🧱️construction/🏭️factory/🦀️.rs`) has seven constructions on the old `admit_original_construction(cx,output,demand,|_|..)`; its static `begin` is the `OriginalPluginAppFactory.begin` fn pointer owned by plugin-reactor-app and invoked from `admit_original_app` (program builder), so the top-level `OriginalStoreBirth` must be threaded through the reactor's per-operation construction state and that fn-pointer signature. Beyond it the same shape is in `R::admit` (registration), `OriginalArtifactInstanceAdmission::{step,take}`, `A::admit_controller`, and four ephemeral-source constructions (`🫧️ephemeral/🦀️.rs`: admit, set_phase, ...). Pattern for each: field `birth: OriginalStoreBirth` + resident output slot per site (`Option<()>` for unit sites), `ready=self.birth.built(cx,&mut slot,demand,|| ..)?`, take the slot after `ready`, drain `birth` in the close ladder.
Run `W-engineinit-112` (native then wasm: window-config + plugin + initialization-replay) only after engine-replay compiles; no such post seen in the claims.

## Residue rule (found while threading the reactor birth)
A construction leaves its frame in flight after the constructing turn; the next `built` on the same admission with a different output slot only clears it. A site that is entered again after another site ran therefore needs its own slot per phase/stage and must not see a stale `Some`. Applied: genesis `unit[4]` (prefix, cursor-before-length, cursor-output, envelope), instance set `moved[4]` by stage, store replay `unit_out` cleared once the backing quote is 0. One-shot sites (births with resident outputs) are unaffected.
Reactor threading done (EDITED-unverified): `OriginalPluginAppFactory.begin(cx,&mut OriginalStoreBirth,output)`, `admit_original_app(app_id,cx,birth,output)`, `PendingNativeCreation.birth` (init, passed, drained in the opening ladder once the construction is dropped).
- factory 🧱️construction/🏭️factory/🦀️.rs ported: birth+units+finishing fields, receive_parts/source/actor_text/actor/controller/final/begin on birth.built; stores admit and R::admit take &mut self.birth (EDITED-unverified; R::admit signature change pending)
- registration `OriginalInstanceRegistration::admit(.., cx, birth, output)` (trait in 🧾️services, impl in 🎟️registration/🦀️.rs) takes the factory's birth (EDITED-unverified). The registration's own later constructions (`phase`, ▶️advance x5, 📤️take, 🪟️windows x4) are still on the old protocol.
## Scale finding
`git grep -c admit_original_construction` over the plugin instance crate: ~70 call sites in ~30 files (registration 14, typed-operations 19, jobs 13, sources 4, dispatch 6, ephemeral 5, ...). The whole crate is on the old `(cx,output,demand,|control|..)` shape; each site needs the same recipe plus a per-site slot and the residue rule. Done: factory, instance store set, registration `admit`, reactor threading. Open: everything else in that list; ephemeral source is five sites in one file (next smallest).
