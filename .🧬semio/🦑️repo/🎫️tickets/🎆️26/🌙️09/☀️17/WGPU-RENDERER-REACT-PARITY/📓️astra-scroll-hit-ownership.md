# Scrolled Retained Hit Ownership

Terra's final Tree audit identified that painting and the host hit registry subtracted scroll offsets while the retained event router and absolute bounds did not. Root added two regression laws before changing production code.

UI 46 reproduced both failures: a descendant's absolute y remained 72 instead of its painted y of 24, and the physically positioned scrolled Select trigger did not open. The same run explicitly executed the real GPU residency integration law; its success output was captured with the second `--` separator before libtest `--nocapture`.

`UiTree::child_walk_origin` now owns the parent-layout-minus-scroll calculation. Retained paint, inherited clipping, hit-test descent, and absolute bounds all consume it. Absolute bounds stop at a placed overlay after including that overlay's content scroll. Overlay subtree hit tests use window coordinates throughout, preventing both scroll omission and double translation at resolved overlay placements. `node_abs_rect` delegates to the tree's authoritative bounds.

UI 47 passed 638 of 639 tests. The remaining failure isolated a second defect: Select option press used the topmost overlay subtree, but release returned to the ordinary clipped root hit test. Pointer release now invokes the same target resolver after releasing capture. Ordinary ancestor clipping remains intact; overlay escape uses the already registered topmost overlay authority.

UI 48 passed all 639 tests with zero skipped. This includes both new laws and the existing overlay, focus, retained paint and GPU residency tests.

The language-neutral Tree density fixture identifies the terminal `action.engagementAbort` row. Root strengthened its Shell law to scroll, physically press/release the published terminal row with a chrome publication between them, and require exactly that action. This Shell law awaits the full native renderer census. The independent actual React browser baseline passed the same five-step sequence, including `engagementAbort` observed after the real pointer click, then reverse scroll and clipping restoration. Fresh WGPU application acceptance awaits activation 16.

Changed production files:

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🌳️tree/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs`

Changed verification files:

- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-events-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs`
- Ticket `🐍️parity-interact-probe.mjs`
