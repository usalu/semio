# Store Constructor Producer Caller Source Roster

Historical lexical source roster: it initially classified 20 Wasmtime calls incorrectly. Those producer arguments were removed after verifying the actual fully qualified type or defining Host import. This table records the initial scan and is not a complete defining-type audit. This roster is a source observation, not an immutable Source capsule, syntax/type test or native acceptance. Two-argument actual Store constructors now select the original native default producer; named FlowStore constructors select the concrete child FlowSnapshotPackProducer. Existing one-argument test fixture wrappers remain their own fixture constructors. No constructor actor/envelope input, ordering, request source or caller grant was changed.

Initial selected calls: 156 in 112 files. Twenty verified unrelated Wasmtime calls were subsequently repaired; current selected actual calls require a separate defining-type audit, which is delegated read-only.

| Source | Line | Constructor | Arguments | Selected Producer / Observation |
| --- | --- | --- | --- | --- |
| 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🏟️arena/🦀️.rs | 102 | Store | 0 | unchanged non-two-argument call |
| 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🏟️arena/🧪️tests/🎟️growth/🦀️.rs | 10 | Store | 0 | unchanged non-two-argument call |
| 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🏟️arena/🧪️tests/🔬️unit/🦀️.rs | 13 | Store | 0 | unchanged non-two-argument call |
| 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🏟️arena/🧪️tests/🔬️unit/🦀️.rs | 26 | Store | 0 | unchanged non-two-argument call |
| 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🏟️arena/🧪️tests/🔬️unit/🦀️.rs | 41 | Store | 0 | unchanged non-two-argument call |
| 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🏟️arena/🧪️tests/🔬️unit/🦀️.rs | 48 | Store | 0 | unchanged non-two-argument call |
| 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🏟️arena/🧪️tests/🔬️unit/🦀️.rs | 56 | Store | 0 | unchanged non-two-argument call |
| 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🏟️arena/🧪️tests/🔬️unit/🦀️.rs | 68 | Store | 0 | unchanged non-two-argument call |
| 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🏟️arena/🧪️tests/🔬️unit/🦀️.rs | 82 | Store | 0 | unchanged non-two-argument call |
| 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🏟️arena/🧪️tests/🔬️unit/🦀️.rs | 93 | Store | 0 | unchanged non-two-argument call |
| 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🏟️arena/🧪️tests/🔬️unit/🦀️.rs | 103 | Store | 0 | unchanged non-two-argument call |
| 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🏟️arena/🧪️tests/🔬️unit/🦀️.rs | 110 | Store | 0 | unchanged non-two-argument call |
| 🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🏟️arena/🧪️tests/🔬️unit/🦀️.rs | 123 | Store | 0 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs | 227 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs | 2759 | FlowStore | 2 | ::semio_framework_artifact_flow_flow::io::binary::snapshot::FlowSnapshotPackProducer |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📥️evaluation-source/🔎️read/🧪️tests/🦀️.rs | 90 | FlowStore | 2 | ::semio_framework_artifact_flow_flow::io::binary::snapshot::FlowSnapshotPackProducer |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs | 1579 | FlowStore | 2 | ::semio_framework_artifact_flow_flow::io::binary::snapshot::FlowSnapshotPackProducer |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧪️tests/🔬️flow-vcs/🦀️.rs | 1843 | FlowStore | 2 | ::semio_framework_artifact_flow_flow::io::binary::snapshot::FlowSnapshotPackProducer |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧪️tests/🔬️flow-vcs/🦀️.rs | 1941 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs | 415 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/📤️result/🧪️tests/🦀️.rs | 9 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs | 2030 | Store | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/🧪️tests/🐎️wasmtime/🦀️.rs | 13 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/🧪️tests/🐎️wasmtime/🦀️.rs | 48 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/🧪️tests/🐎️wasmtime/🦀️.rs | 59 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️shared-wasmtime-engine/🦀️.rs | 44 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️shared-wasmtime-engine/🦀️.rs | 68 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️shared-wasmtime-engine/🦀️.rs | 76 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🔬️shared-wasmtime-engine/🦀️.rs | 155 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🏗️instance/🧪️tests/🦀️.rs | 20 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🚪️io/📤️checkpoint/🧪️tests/🦀️.rs | 7 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🧪️tests/🔬️unit/🦀️.rs | 235 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/🧪️tests/🦀️.rs | 14 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/🧪️tests/🦀️.rs | 25 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/🧪️tests/🦀️.rs | 39 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/🧪️tests/🦀️.rs | 45 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/🧪️tests/🦀️.rs | 52 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🏗️startup/🧪️tests/🦀️.rs | 12 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🎟️call/🧪️tests/🦀️.rs | 12 | Store | 2 | verified Wasmtime type; producer argument removed |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs | 1374 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs | 1580 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs | 1666 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs | 4949 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs | 4961 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️tool-run-member/🦀️.rs | 122 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs | 160 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️bounded-reload/🦀️.rs | 172 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧾️document-archive-load-legs/🦀️.rs | 255 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️node-drag-history/🦀️.rs | 32 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📃️query/🧪️tests/📃️query/🦀️.rs | 72 | InteractionStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/🧪️tests/📡️live/🦀️.rs | 20 | TestStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs | 516 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📖️capture/🧪️tests/📖️capture/🦀️.rs | 15 | InteractionStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 1644 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 1649 | ConfigStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 1650 | DraftStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🦀️.rs | 1651 | ConfigStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs | 399 | ConfigStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧪️tests/🪟️retained-window-config/🦀️.rs | 65 | ConfigStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚖️authoring-law/🦀️.rs | 49 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚖️authoring-law/🦀️.rs | 75 | ConfigStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/♻️retirement/🧪️tests/🔬️unit/🦀️.rs | 15 | Store | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs | 1850 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️.rs | 454 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🧪️tests/🔬️unit/🦀️.rs | 733 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🧪️tests/🔬️unit/🦀️.rs | 743 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🧪️tests/🔬️unit/🦀️.rs | 770 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs | 56 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧪️tests/⚖️retirement-law/🦀️.rs | 53 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs | 56 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧪️tests/⚖️retirement-law/🦀️.rs | 42 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs | 659 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs | 680 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs | 1113 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs | 1258 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs | 10765 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs | 11456 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🧪️tests/🔬️unit/🦀️.rs | 127 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/✂️detach/🧪️tests/✂️detach/🦀️.rs | 17 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs | 87 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 53 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 220 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 263 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 562 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 610 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 618 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 661 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 678 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 780 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 784 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 836 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 949 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 1017 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 1021 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 1052 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🦀️.rs | 1074 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-law/🦀️.rs | 131 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-law/🦀️.rs | 181 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-law/🦀️.rs | 244 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/⚡️hot-path/🦀️.rs | 78 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️viewer-head/🦀️.rs | 19 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️viewer-head/🦀️.rs | 114 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️viewer-head/🦀️.rs | 454 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️viewer-head/🦀️.rs | 469 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️viewer-head/🦀️.rs | 477 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️viewer-head/🦀️.rs | 557 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 38 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 1483 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 1489 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 2091 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 6062 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 6337 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 6350 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 6367 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 6459 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 6560 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 6975 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 6990 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 6999 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7008 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7037 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7079 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7105 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7124 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7135 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7150 | InteractionStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7166 | InteractionStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7188 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7243 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7252 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7263 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7276 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7296 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7321 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7335 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7353 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7439 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7448 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7449 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7501 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7502 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7517 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7772 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7805 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7839 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7841 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7846 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7853 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7855 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7888 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7920 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7937 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7950 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 7973 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8000 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8010 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8024 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8044 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8054 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8173 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8184 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8216 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8317 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8319 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8321 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8323 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8354 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8357 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8394 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8416 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8438 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8461 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8464 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8490 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8665 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8770 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8794 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8818 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8843 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8865 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8905 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8928 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8967 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 8984 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9004 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9020 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9037 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9187 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9210 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9242 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9256 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9279 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9305 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9318 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9330 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9342 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9355 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9366 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9379 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9392 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9414 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9431 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9489 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9521 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9541 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9570 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9722 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9726 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 9742 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10231 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10262 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10570 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10571 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10611 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10612 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10644 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10648 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10652 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10691 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10692 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10731 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10736 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10766 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10770 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10774 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10805 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10810 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10845 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10846 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10894 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10898 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10902 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10934 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10935 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10973 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 10974 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11015 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11016 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11047 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11048 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11108 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11111 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11165 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11167 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11207 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11209 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11255 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11257 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11458 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11459 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11535 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11552 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11559 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11628 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | 11997 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️deferred-reprojection/🦀️.rs | 20 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️tool-transaction/🦀️.rs | 27 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️replay-retirement/🦀️.rs | 263 | ArtifactStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🧪️tests/🔬️unit/🦀️.rs | 26 | Store | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🌿️vcs/🦀️.rs | 588 | DagStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs | 4569 | ProbeStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧪️tests/🔬️long/🦀️.rs | 88 | ProbeStore | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs | 853 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🔬️host-unit/🦀️.rs | 466 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🔬️host-unit/🦀️.rs | 910 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧪️tests/🔏️export-content-hash/🦀️.rs | 71 | Store | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧪️tests/🗃️store-append-sequence/🦀️.rs | 66 | Store | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧪️tests/🗃️store-append-sequence/🦀️.rs | 70 | Store | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧪️tests/🗃️store-append-sequence/🦀️.rs | 92 | Store | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧪️tests/🗃️store-append-sequence/🦀️.rs | 115 | Store | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧪️tests/🗃️store-append-sequence/🦀️.rs | 145 | Store | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧪️tests/🔬️unit/🦀️.rs | 223 | Store | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧪️tests/🔬️unit/🦀️.rs | 230 | Store | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧪️tests/🔬️unit/🦀️.rs | 239 | Store | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧪️tests/🔬️unit/🦀️.rs | 259 | Store | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧪️tests/🔬️unit/🦀️.rs | 295 | Store | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧪️tests/🔬️unit/🦀️.rs | 316 | Store | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧪️tests/🔬️unit/🦀️.rs | 333 | Store | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/🧪️tests/🔬️unit/🦀️.rs | 426 | Store | 1 | unchanged non-two-argument call |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🦀️.rs | 3152 | Store | 1 | unchanged non-two-argument call |
| ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs | 70 | TrinityGraphStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs | 20 | RewriteRuleStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🪆️child-frame/🦀️.rs | 32 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 19 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs | 457 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs | 123 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 51 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 534 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 657 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪣️fill-region/🧪️tests/🦀️.rs | 233 | RasterStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️paint-stroke/🧪️tests/🦀️.rs | 237 | RasterStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 167 | RasterStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌈️apply-filter/🧪️tests/🦀️.rs | 235 | RasterStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🫗️fill-selection/🧪️tests/🦀️.rs | 217 | RasterStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️transform-image/🧪️tests/🦀️.rs | 259 | RasterStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🔬️unit/🦀️.rs | 32 | Process3dStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🔬️unit/🦀️.rs | 176 | Process3dStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🔨️modules/🏠️host/🧰️owned/🦀️.rs | 17 | CadStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/📦️member-authority/🦀️.rs | 20 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs | 524 | DraftStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 11 | Block3dStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 11 | Block5dStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 11 | Block2dStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🔨️modules/🏠️host/🧰️owned/🦀️.rs | 11 | DagStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs | 153 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/🧪️tests/🔬️unit/🦀️.rs | 298 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs | 16 | SequenceStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🔨️modules/🏠️host/🧰️owned/🦀️.rs | 25 | WriterDocumentStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🔨️modules/🏠️host/🧰️owned/🦀️.rs | 17 | PresentationStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 18 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🧰️support/🔬️store-fixture/🦀️.rs | 18 | Generation3dFixtureStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🧪️tests/🔬️store-fixture/🦀️.rs | 19 | Generation2dFixtureStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs | 21 | VcsStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️unit/🦀️.rs | 43 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 101 | GisMapStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs | 163 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 28 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🗄️store-fixture/🦀️.rs | 13 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/✋️drag/🧪️tests/🔬️unit/🦀️.rs | 187 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️brush-tool/🦀️.rs | 367 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/✋️drag/🧪️tests/🔬️unit/🦀️.rs | 186 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 17 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔨️modules/🏠️host/🧰️owned/🦀️.rs | 11 | NoteStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🔨️modules/🏠️host/🧰️owned/🦀️.rs | 12 | FormsStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 16 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🔬️unit/🦀️.rs | 5 | ShootingStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs | 142 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🔨️modules/🏠️host/🧰️owned/🦀️.rs | 11 | EquationStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🔨️modules/🏠️host/🧰️owned/🦀️.rs | 12 | LayoutStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🔨️modules/🏠️host/🧰️owned/🦀️.rs | 15 | Puzzle3dStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🔨️modules/🏠️host/🧰️owned/🦀️.rs | 14 | Puzzle5dStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🔨️modules/🏠️host/🧰️owned/🦀️.rs | 15 | Puzzle2dStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 36 | Fem3dStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🕹️interaction/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs | 173 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 44 | Fem2dStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🌉️wasm/🧪️tests/🔬️unit/🦀️.rs | 7 | Fem2dStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🕹️interaction/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs | 132 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs | 100 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 20 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 42 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/📦️owned/🧪️tests/🦀️.rs | 20 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🧪️tests/🔬️unit/🦀️.rs | 106 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🧪️tests/🔬️unit/🦀️.rs | 123 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🧪️tests/🔬️unit/🦀️.rs | 144 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🧪️tests/🔬️unit/🦀️.rs | 166 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/📋️prepare/🧪️tests/🔬️unit/🦀️.rs | 176 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 250 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️geometry/🧪️tests/🔬️unit/🦀️.rs | 8 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️export-document/🧵️export/🧪️tests/🔬️unit/🦀️.rs | 13 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs | 33 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧪️tests/🔬️unit/🦀️.rs | 321 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs | 21 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
| ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️transform/🧪️tests/🔬️unit/🦀️.rs | 60 | ArtifactStore | 2 | ::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication |
