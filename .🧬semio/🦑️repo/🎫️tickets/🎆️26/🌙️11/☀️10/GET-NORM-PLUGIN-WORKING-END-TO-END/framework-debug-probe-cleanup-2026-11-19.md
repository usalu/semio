# Framework Debug Probe Cleanup

Retired the temporary Norm Play probes for command ingress, turn phase and admission, reconcile drive, manifest and metadata staging, surface capture and retirement, and the self-waking turn driver. This also removed their probe-only counters, state, and helper functions. The existing command fault, turn refusal, and self-wake bound remain in effect.

The archive and capacity investigations still use the archive deferred, checked-out cleanup-body, and poll diagnostics. Their source and the close-demand fault stayed in place. The separate lane5d, window preparation, value retirement, UI test, and repository markers had no confirmed ownership in this handoff and were left in place. The staged bridge and shared Play process were untouched.

The surface and self-wake probes had introduced direct `semio-plugin-abi` dependencies in `semio-plugin-plugin-program` and `semio-plugin-runtime-ops-wire`. Only those direct edges were removed from the manifests and 41 affected tracked Cargo locks. Other consumers still reference the ABI package. Bun parsed 175 tracked Cargo lock graphs containing the affected crates and found no remaining direct edge in either crate. The pinned Rust compiler parsed all 11 changed Rust files in Rust 2021 mode. No competing framework rebuild was started.

## Framework files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🎞️command/📥️custody/🛂️manifest/metadata.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🎞️command/📥️custody/🛂️manifest/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🛡️wire-limits/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🛡️wire-limits/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🌀️poll-kernel-turn/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/📨️event-dispatch/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🩹️reconcile-drive-and-publication-receipts/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🪜️entry-lifecycle-and-retirement/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/native.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/read.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🔌️program/📦️packages/🦀️rust/Cargo.toml`

## Cargo lock files

- `Cargo.lock`
- `✏️s/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/Cargo.lock`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/Cargo.lock`
- `✏️s/🔌️plugins/🏗️fem/Cargo.lock`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.lock`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.lock`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/Cargo.lock`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/Cargo.lock`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/Cargo.lock`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/Cargo.lock`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/Cargo.lock`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/Cargo.lock`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/Cargo.lock`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/Cargo.lock`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/Cargo.lock`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/Cargo.lock`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/Cargo.lock`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/Cargo.lock`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/Cargo.lock`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/Cargo.lock`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/Cargo.lock`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/Cargo.lock`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/Cargo.lock`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/Cargo.lock`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/Cargo.lock`
- `🌎️hub/Cargo.lock`

## Final Norm Source Pass

Removed the remaining Norm archive-deferred, cleanup-body and poll traces after the archive build had compiled its Job context. Typed poll refusals, retained semantic failures, archive stage 24 admission, and close-demand faults still use their existing paths. Removed the maintenance ladder's probe-only ABI dependency. The pinned compiler parsed the six changed Rust sources. Unrelated lane5d, window and value markers remain owned by their separate work.

- `Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/Cargo.lock`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/Cargo.lock`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/Cargo.lock`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/Cargo.lock`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/Cargo.lock`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/Cargo.lock`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/Cargo.lock`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/Cargo.lock`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/Cargo.lock`
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/Cargo.lock`
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/Cargo.lock`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/Cargo.lock`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/Cargo.lock`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/Cargo.lock`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/Cargo.lock`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/Cargo.lock`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/Cargo.lock`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/Cargo.lock`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/Cargo.lock`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/Cargo.lock`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/Cargo.lock`
- `🌎️hub/Cargo.lock`
- `🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📥️staging/🪜️maintenance-ladder/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪜️maintenance-ladder/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪜️maintenance-ladder/🦀️.rs`
