"""📚 Planned file ownership of the new os-flow crates (relative to the flow module dir)."""
import os
FL="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/"
H="🖥️host/"
W="🕸️wasm/"
LIB={
 "geometry":["🌐️geometry/🦀️.rs"],
 "mesh":["🎒️mesh/🦀️.rs"],
 "drawing":["🖍️drawing/🦀️.rs"],
 "operators":["🗃️operators/🦀️.rs","📔️registry/🦀️.rs","📔️registry/🌱️seed/🦀️.rs","🗂️catalogue/🦀️.rs","🌉️bridge/🦀️.rs","🧩️extensions/🧬️schema/🦀️.rs"],
 "vcs-laws":["🌿️vcs/⚖️laws/🦀️.rs"],
 "vcs":["🌿️vcs/🦀️.rs","🌿️vcs/♻️retirement/🦀️.rs","📝️forms/🧬️schema/🦀️.rs"],
 "extensions-laws":["🧩️extensions/🕸️wasm/⚖️laws/🦀️.rs"],
 "extensions":["🧩️extensions/🕸️wasm/🦀️.rs","🧩️extensions/🕸️wasm/📥️request/🦀️.rs","🧩️extensions/🕸️wasm/📥️request/🎟️context/🦀️.rs","🧩️extensions/🕸️wasm/📨️result/🦀️.rs","🧩️extensions/🕸️wasm/⚠️failure/🦀️.rs"],
 "host-source":[H+"📥️evaluation-source/🦀️.rs",H+"📥️evaluation-source/🌱️seeds/🦀️.rs",H+"📥️evaluation-source/🧵️work/🦀️.rs",H+"📤️publication/🖼️display/🦀️.rs"],
 "host-core":[H+"🧱️core/🦀️.rs",H+"⚠️errors/🦀️.rs",H+"🏠️flow-host/🦀️.rs",H+"✏️host-authoring/🦀️.rs",H+"🌳️host-evaluation-history/🦀️.rs",H+"🏁️host-close/🦀️.rs",H+"🛍️host-catalogue/🦀️.rs",H+"🔌️ports/🦀️.rs",H+"📐️widget-size/🦀️.rs",H+"🧹️retirement/🦀️.rs"],
 "host-read":[H+"📥️evaluation-source/🔎️read/🦀️.rs",H+"📥️evaluation-source/🔎️read/⚖️compare/🦀️.rs",H+"📥️evaluation-source/🔎️read/📨️request/🦀️.rs"],
 "host-session":[H+"🎞️session/🦀️.rs",H+"🧾️eval-session-state/🦀️.rs",H+"⏱️eval-session/🦀️.rs",H+"🔭️preview-status/🦀️.rs",H+"🚦️eval-status/🦀️.rs",H+"🌊️node-graph-extras/🦀️.rs",H+"📤️publication/🎟️session/🦀️.rs",H+"📤️publication/📡️invocation/🦀️.rs",H+"📤️publication/📡️invocation/🪪️identity/🦀️.rs",H+"📥️preview/🦀️.rs",H+"📥️evaluation/🦀️.rs",H+"🚪️io/🦀️.rs",H+"🚪️io/📥️evaluation-response/🦀️.rs"],
 "wasm-domain":[W+"🧩️domain-adapter/🦀️.rs",W+"🌿️vcs-feature/🦀️.rs",W+"🛠️adapter-ops/🦀️.rs",W+"🔤️argument-helpers/🦀️.rs",W+"🖼️surface-presentation/🦀️.rs",W+"📡️protocol/🦀️.rs"],
 "wasm-actions-a":[W+"🎬️actions-a/🦀️.rs",W+"🧵️dag-input/🦀️.rs"],
 "wasm-actions-b":[W+"🎞️actions-b/🦀️.rs"],
 "wasm":[W+"🦀️.rs",W+"🌉️domain-impl/🦀️.rs",W+"🧭️dispatch/🦀️.rs",W+"🔗️linear-memory/🦀️.rs",W+"📡️protocol/🌉️bridge/🦀️.rs",W+"📡️protocol/🪢️domain/🦀️.rs"],
 "host-laws":[H+"⚖️laws/🦀️.rs"],
 "host":[H+"🦀️.rs"],
}
TESTS={
 "geometry":["🌐️geometry/🧪️tests/♻️ownership/🦀️.rs"],
 "drawing":["🖍️drawing/🧪️tests/🔬️drawing-kernel/🦀️.rs","🖍️drawing/🧪️tests/🔬️path-segment-json/🦀️.rs"],
 "operators":["📔️registry/🧪️tests/🧬️contribution/🦀️.rs","📔️registry/🧪️tests/📔️registry/🦀️.rs"],
 "vcs-laws":["🌿️vcs/🧪️tests/🔬️flow-vcs/🦀️.rs"],
 "vcs":["🌿️vcs/🧪️tests/🔬️flow-canonical/🦀️.rs","📝️forms/🧬️schema/🧪️tests/🦀️.rs"],
 "extensions-laws":["🧩️extensions/🕸️wasm/🧪️tests/🔬️unit/🦀️.rs"],
 "extensions":[],
 "host-source":[H+"📥️evaluation-source/🧪️tests/🦀️.rs",H+"📥️evaluation-source/🌱️seeds/🧪️tests/🦀️.rs",H+"📤️publication/🖼️display/🧪️tests/🦀️.rs"],
 "host-read":[H+"📥️evaluation-source/🔎️read/⚖️compare/🧪️tests/🦀️.rs"],
 "host-session":[],
 "host-laws":[H+"🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs",H+"📥️evaluation-source/🧵️work/🧪️tests/🦀️.rs",H+"📥️evaluation-source/🔎️read/🧪️tests/🦀️.rs",H+"📤️publication/📡️invocation/🧪️tests/🦀️.rs",H+"📥️evaluation/🧪️tests/🦀️.rs",H+"📥️preview/🧪️tests/🦀️.rs"],
 "host":[H+"🧪️tests/🔬️unit/🦀️.rs"],
 "wasm":[W+"🧪️tests/🔬️protocol-unit/🦀️.rs",W+"🧪️tests/🔬️component-domain-laws/🦀️.rs",W+"🧪️tests/🎬️draw-list/🦀️.rs"],
}
def absl(cid):return [FL+p for p in LIB[cid]]
def exists(cid):return [p for p in absl(cid) if os.path.exists(p)]
