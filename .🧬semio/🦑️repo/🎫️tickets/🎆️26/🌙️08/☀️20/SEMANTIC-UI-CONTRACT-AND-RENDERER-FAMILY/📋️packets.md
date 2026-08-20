# 📋️ Packet ledger — ownership registry

**`sol` is the only writer of this file.** Executors never edit it. No file may appear in two open
packets. Registrar-only files (see `📌️important.md` U7) never appear in an OWNS list at all —
executors emit a `registrar-request` in their report instead.

States: `queued` · `dispatched` · `blocked-external` · `needs-registrar` · `done-verified`.
`done-verified` means **sol** re-ran the acceptance and saw it pass — never an executor's claim.

## W1 — contract + GPU core (anchor `5e7b8046be`, gated at `cb9bcce7a4`)

| packet | state | files owned | evidence |
| --- | --- | --- | --- |
| `contract-doc` | done-verified | `🧬️contract/…/🦀️document.rs`, `🦀️component.rs` | `📝️gate-w1-contract-test.txt` |
| `contract-layout` | done-verified | `🧬️contract/…/🦀️layout.rs`, `🦀️style.rs`, `🦀️accessibility.rs`, `🦀️surface.rs` | same |
| `contract-action` | done-verified | `🧬️contract/…/🦀️action.rs`, `🦀️presence.rs`, `🦀️limits.rs` | same |
| `render-scene` | done-verified | `🖼️render/…/🦀️scene.rs`, `🦀️tessellate.rs`, `🦀️resource.rs` | `📝️gate-w1-render-test.txt`, `📝️gate-w1-boundaries.txt` |
| `shader-repair` | done-verified | `🎯️targets/🧊️wgpu/🦀️shaders.rs`, `🖼️render/…/🦀️shader_contract.rs` | `📝️gate-w1-render-test.txt` (3 naga tests) |
| `backend-iface` | done-verified | `🖼️render/…/🦀️backend.rs` | same |

Registrar-authored in W1 (sol, not a packet): both crates' `Cargo.toml` / `📦️glue.rs` /
`📋️project.json` / `📜️script.ts`, every region stub, root `Cargo.toml` (2 members + 2
workspace.dependencies), and the seven cross-packet reconciliations listed in `📓️status.md`.

## W2 — headless runtime + GPU frame pipeline (landed, gated at 228 tests)

Runtime crate `semio-framework-ui-runtime` scaffolded by sol before dispatch, same pattern as W1.

| packet | state | files owned | evidence |
| --- | --- | --- | --- |
| `contract-builder` | done-verified | `🧬️contract/…/🦀️builder.rs` | `📝️gate-w2-contract-test.txt` (73) |
| `runtime-entity` | done-verified | `🧠️runtime/…/🦀️entity.rs`, `🦀️context.rs` | `📝️gate-w2-runtime-test.txt` (34) |
| `runtime-present` | done-verified | `🧠️runtime/…/🦀️tracking.rs`, `🦀️present.rs` | same |
| `runtime-gateway` | done-verified | `🧠️runtime/…/🦀️gateway.rs`, `🦀️inbox.rs`, `🦀️presence.rs` | same |
| `render-frame` | done-verified | `🖼️render/…/🦀️element.rs`, `🦀️layout.rs`, `🦀️frame.rs`, `🦀️schedule.rs` | `📝️gate-w2-render-test.txt` (121) |
| `render-dispatch` | done-verified | `🖼️render/…/🦀️dispatch.rs` | same |
| `render-text` | done-verified | `🖼️render/…/🦀️text.rs` | same |
| `render-surface` | done-verified | `🖼️render/…/🦀️surface.rs` | same |

Two reconciliations were run by resuming the original executor with expanded temporary ownership
(`runtime-entity` over all four runtime-foundation files; `render-frame` over its taffy adapter) —
cheaper and safer than a fresh packet, because the author still held the design context.

## W3 — next, queued

| packet | files it will own | notes |
| --- | --- | --- |
| `runtime-reconcile` | `🧠️runtime/…/🦀️reconcile.rs` | keyed diff → minimal `UiPatch`; assigns `UiNodeId`s |
| `runtime-transact` | `🧠️runtime/…/🦀️dispatch.rs`, `🦀️transaction.rs` | the run-to-completion loop; owns the EffectStorm budget |
| `dispatch-tree-seam` | `🖼️render/…/🦀️frame.rs` + `🦀️dispatch.rs` | **must fix the lossy `DispatchTree: From<Vec<Hitbox>>` adapter** before any cutover — `Hitbox` carries no parent/overlay/listener data |
| `conformance-corpus` | `🧬️contract/📚️examples/🧪️conformance/**` + a `conformance` command on the contract `📜️script.ts` | the fixtures React and the GPU renderer must both satisfy |
| `backend-webgpu` | `🖼️render/🎯️targets/🧊️webgpu/**` | new crate; sol scaffolds first |
| `backend-metal` / `backend-d3d12` / `backend-vulkan` | `🖼️render/🎯️targets/{🍎️metal,🪟️d3d12,🌋️vulkan}/**` | new crates; Metal is the only one runtime-testable on this machine |
| `ui-host` | `🖱️ui/🖥️host/📦️packages/🦀️rust/**` | new crate; **W0 open question: check whether the existing `🖥️platform` module already owns window abstractions** |
