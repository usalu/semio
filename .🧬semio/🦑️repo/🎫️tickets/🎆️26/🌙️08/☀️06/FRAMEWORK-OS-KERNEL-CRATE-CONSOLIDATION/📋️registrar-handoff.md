# Registrar Handoff — Framework Os Kernel Crate Consolidation (W8c)

**Status:** `ready-for-registrar` — native + wasm32-wasip2 `cargo check -p semio-framework-os-kernel --lib` GREEN (`check-14.txt`, `check-wasm.txt`).
Dual-tree type identity fixed inside os/**. Old implementations retained as DELETE-READY (dsl-derive sandwich deleted). Host/dev out of scope.

**Ticket:** `26/08/06/FRAMEWORK-OS-KERNEL-CRATE-CONSOLIDATION`
**Goal:** `aioptimizedrepo`

## What landed (finish pass)

1. store + store/sync import protocol IDs from `crate::os_spr::core` (not framework-core reexports).
2. `fault_from_thiserror!` via crate root; macro `$crate` restored.
3. DSL HashMap import ungated.
4. registry `full_resolver()` empty; registry cfg `not(wasm32)`.
5. dsl-derive at Shape V2 packages path; old derive implementations deleted.
6. plugin packages -> `semio-framework-os-kernel` + extern crate aliases.
7. `os_store::{sync,worker}` behind features `sync`/`worker` (not default).

## Root Cargo.toml ADD members

```toml
"🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
```

Already-present non-impl os members for reference:
```
"🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu",
```

## Root Cargo.toml REMOVE members (DELETE-READY)

Count: 80 — also in delete-ready-members.txt

```toml
"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫀️core/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🖋️notation/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📇️registry/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/👪️family/🕸️graph/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/👪️family/📊️sheet/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/👪️family/🗂️catalog/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/👪️family/🧑‍🍳️recipe/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🫀️core/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/📐️format/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔢️value/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔌️io/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⏳️async/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌐️http/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔢️index/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🧪️testkit/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🫀️core/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🎭️actor/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔘️state/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📸️snapshot/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚔️conflict/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📽️projection/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔍️query/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/👁️preview/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔒️security/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️document/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🌐️cluster/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/👁️observe/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️testkit/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/🫀️core/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/🎮️command/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/🔗️causal/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/🔀️crdt/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/📐️format/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/📜️history/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/💎️materialize/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/🔌️io/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/📡️wire/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/🧵️channel/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/🧪️testkit/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/⌨️cli/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/↔undirected/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/➡️directed/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/➕️normal/↔undirected/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/⚡️implementations/🦀️rust",
"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️implementations/🦀️rust",
```

## workspace.dependencies

- ADD `semio-framework-os-kernel = { path = "…/os/packages/rust" }`
- KEEP dsl-derive at Shape V2 packages path
- RENAME/REMOVE these to kernel:
```
semio-framework-os-kernel-dsl = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/⚡️implementations/🦀️rust" }  # 206 refs
semio-framework-os-kernel-dsl-core = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫀️core/⚡️implementations/🦀️rust" }  # 22 refs
semio-framework-os-kernel-pack = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⚡️implementations/🦀️rust" }  # 64 refs
semio-framework-os-kernel-pack-core = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🫀️core/⚡️implementations/🦀️rust" }  # 18 refs
semio-framework-os-kernel-pack-format = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/📐️format/⚡️implementations/🦀️rust" }  # 7 refs
semio-framework-os-kernel-protocol = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/⚡️implementations/🦀️rust" }  # 218 refs
semio-framework-os-kernel-protocol-core = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/🫀️core/⚡️implementations/🦀️rust" }  # 13 refs
semio-framework-os-kernel-protocol-format = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/📐️format/⚡️implementations/🦀️rust" }  # 6 refs
semio-framework-os-kernel-store = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚡️implementations/🦀️rust" }  # 359 refs
semio-framework-os-kernel-vcs = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/⚡️implementations/🦀️rust" }  # 37 refs
[profile.dev.package.semio-framework-os-kernel-store]
[profile.wasm-release.package.semio-framework-os-kernel-store]
```
- `profile.*.package.semio-framework-os-kernel-store` -> `semio-framework-os-kernel`

## Outside-os consumers (45)

- `Cargo.toml`
- `compose/client/lib/rs/Cargo.toml`
- `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📏️layout/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📜️imperative/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/📸️remodel/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🕸️dag/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗒️note/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔨️modules/📜️imperative/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔨️modules/🗣️lang/📦️packages/🦀️rust/Cargo.toml`
- `🌎️hub/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/📚️compiler/⚡️implementations/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/📚️compiler/📖️syntax/⚡️implementations/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/Cargo.toml`

## framework-core retarget (OUTSIDE os — registrar/core owner)

File: framework/packages/rust/lib.rs — matching lines:
```
51|     DocumentVersion, OperationId, PhysicalSize, PluginInstanceId, PresencePeer,
53|     ArtifactId, ArtifactKind, Appearance, Rights, SchemaId, SchemaVersion, Scope, UndoGroup, UndoPolicy,
```

Stop reexporting types from old `protocol_core` / `semio-framework-os-kernel-protocol*`.
Repoint to `semio_framework_os_kernel::os_spr::core` (and related os_spr modules) after old protocol members are removed.

## CLI bins
- `pack` stays; `protocol` bin -> `spr` (kernel already declares `[[bin]] name = "spr"`)

## Residuals deferred
- infinite + flow wiring + gpu feature
- host plane `semio-framework-os`
- physical delete of DELETE-READY implementations after consumer repoint

## Handoff JSON

```json
{
  "owner": "os",
  "ticketPath": "26/08/06/FRAMEWORK-OS-KERNEL-CRATE-CONSOLIDATION",
  "cargoCheckKernel": "pass",
  "wasmAdmission": "pass",
  "oldImplDirsRemainingUnderOs": 85,
  "registrarMustApply": true,
  "newCrates": [
    "semio-framework-os-kernel",
    "semio-framework-os-kernel-dsl-derive",
    "semio-framework-plugin"
  ],
  "status": "ready-for-registrar"
}
```
