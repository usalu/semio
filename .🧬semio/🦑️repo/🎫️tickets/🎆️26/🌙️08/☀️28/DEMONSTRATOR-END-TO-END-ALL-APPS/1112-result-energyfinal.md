# energyfinal lane (11/12)
## Step 1: initialization GREEN
- W-energyfinal-2 (wasm hub, -p semio-s-artifact-energy-model-initialization) rc 0.
- EnergyModelSnapshot: `impl FromValue` (carrier; replaces the inherent `from_value`, delegates to `receive_stepped`; `from_value_controlled` stays the default refusal = debt).
- SnapshotProjection Output: PagedOneItemPreparationFactory requires `SnapshotProjection<Output=S>`; the derive emitted Box<Self>. Added the additive container flag `#[value(by_value_projection)]` to `derive(SnapshotProjection)` (framework value derive: expansion parse_container_attrs + 📽️projection): output is the plain value, take_into moves the box content out, take quote counts size_of::<Option<Output>>. Used on EnergyModelSnapshot.
- EnergyModelMutation: OpBinary now encodes the pack wire value + `dsl::original_pack_value_operation_receiving!` (io/binary/mutations; tagged `.protocol.semio` no longer the op encoding, io binary tests are debt).
Rerun: ck W-energyfinal-N -p semio-s-artifact-energy-model-initialization
## Step 2: energy-model GREEN, editor-commands GREEN
- W-energyfinal-3 (-p semio-s-artifact-energy-model) rc 0 (cascade of initialization).
- W-energyfinal-5 (-p semio-s-artifact-energy-model-editor-commands) rc 0. `✏️editor/🎮️commands/🛬️action/🦀️.rs` rewritten on the sibling `OpBinaryProducer` protocol (framework `📜️declaration/👷️job/🎮️command/🧬️op-binary`): one resident `OriginalConstructionAdmission` + `OriginalConstructionFailure` slot + unit marker; `built`/`built_with`/`stepped`/`unbuilt` settle every birth and state change, a failed construction keeps its causes and is taken then retired in close; receiver advance runs in `cx.with_native_receiving` with the Source witness (STEP_WORK=4096 per-step cap, shared by quote and call), take via `with_original_effect`, receiver/wire/source closes via `close_original_effect_step` (begin_close_into / close_into replies); Wire::admit gets the same resident admission and adopts it.
- Cargo: editor-commands gained `semio-framework-value-native-owned` (LOCK NEEDED logged).
- Debts: action 🧪️tests still on the grant API (not compiled); close of a receiver that `requires_original_birth/control` (original close loan) is not driven; Wire failed-admission causes retire through the shared admission failure slot.
## Step 3: remaining energy reds, component GREEN
- editor-edit / viewer: `EnergyModelWindowConfigMutation` and `EnergyModelViewerWindowConfigMutation` OpBinary now pack wire value + `dsl::original_pack_value_operation_receiving!` (same ruling as the main mutation).
- editor `🎛️retained-commands/🧬️original`: EnergyModelCommandWork ported to the same resident-admission protocol (built/stepped, failure slot, `close_original_effect_step`, `terminal_frame_bound!` removed since the work declares `terminal_frame_release_bytes`); step splits row birth / handoff with `born`/`emitted` flags so a pending turn never repeats a change.
- editor `🧮️reduce-and-helpers`: `retire_temporary` (old grant retirement) removed; `snapshot.retire()`.
- hub composition `semio-hub-energy`: Cargo.toml gained `semio-framework-value` (dyn_enum_close! expands to it). LOCK NEEDED logged for it and for editor-commands' `semio-framework-value-native-owned`.
- W-energyfinal-10 (wasm hub, -p semio-hub-energy-component) rc 0 ("Finished prototype profile").
## Debts / not verified
- No tests run: action 🧪️tests and other 🧪️tests/tests.rs still use the grant API; io binary tests expecting the tagged `.protocol.semio` op encoding are stale.
- `EnergyModelSnapshot::from_value_controlled` is the default refusal (only `from_value` via receive_stepped); receive_stepped/retire_stepped in the snapshot bridge still use the bridge-local AuthoringWallet (pre-existing, energysnap).
- Action producer: receivers that `requires_original_birth/control` on close are not driven through an original close loan; quote==actual not exercised at runtime; STEP_WORK=4096 per receiver advance.
- Framework: new additive flag `#[value(by_value_projection)]` on derive(SnapshotProjection).
Rerun: "$C/1105b-tools/chk.sh" W-energyfinal-N "🌎️hub/Cargo.toml" wasm -p semio-hub-energy-component
STATUS: FIXED-verified W-energyfinal-10
