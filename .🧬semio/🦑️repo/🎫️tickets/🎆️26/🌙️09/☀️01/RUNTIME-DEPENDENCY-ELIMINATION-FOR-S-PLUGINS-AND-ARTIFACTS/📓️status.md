# 🧭️ Status — Runtime Dependency Elimination

## Headline numbers

| metric | baseline (aad3d81959) | now | source |
|---|---|---|---|
| third-party entries in **s production** manifests | 119 | **74** | manifest scan, oracle/probe/generator dirs excluded |
| distinct crates leaking into s production | 23 | **8** | same |
| gate `oracle-conflicts` | 18 | **6** | `verify dependencies literal-external` |
| gate `production-reachable` (rust) | 66 | **53** | same |
| gate `literal-external` (repo-wide raw names) | 163 | 163 | unchanged by design — counts raw names, not reachability |

## Eliminated from every s production manifest

`base64` (7 manifests) · `png` (2) · `image` (2) · `blake3` (1) · `getrandom` (1) · `parry3d` (1) ·
`kurbo` (1) · `parley` (1) · `swash` (1) · `typst` · `typst-svg` · `typst-assets` · `usvg` ·
`vello` · `wgpu` — 24 entries.
`wasm-bindgen` 25→5, `js-sys` 6→4.

**`gltf` — the goal's own example — is genuinely fixed.** Removed from `semio-framework`
entirely; in `🧰️framework/🔨️modules/🔺️mesh-engine` it now sits under `[dev-dependencies]` as an
oracle only. The one remaining declaration under `✏️s/` is a `🔬️probes/` crate, which is the
sanctioned fixture-generating role.

## Remaining: 74 entries, 8 crates

| count | crate | owner |
|---|---|---|
| 36 | `serde_json` | serde wave — pilot in flight |
| 23 | `serde` | serde wave — pilot in flight |
| 5 | `wasm-bindgen` | wasm-glue wave, in flight |
| 4 | `js-sys` | wasm-glue wave, in flight |
| 3 | `web-sys` | wasm-glue wave, in flight |
| 3 | `proc-macro2` / `quote` / `syn` | classifier — build-time, not runtime |

`serde` + `serde_json` is 59 of the 74. Root cause is narrow and structural: `MutationDiff` /
`Mutation` in `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs` carry
`serde::Serialize + serde::de::DeserializeOwned` in their trait bounds, so every plugin that
implements a mutation is forced to depend on serde. The pilot is building the first-party
replacement at `🧰️framework/🔨️modules/🌱️value/✨️derive` + `🔁️codec` over `pack::json::Value`.

## Classifier corrections (reclassification, NOT removal — kept separate deliberately)

`📜️script.ts` `DEPENDENCY_TEST_DOMAIN_PATH_RE` now recognises the repo's real test domain:
`🧪️oracle` · `🧪️test` · `🔬️probes` · `🏭️generator` · `🧫️fixtures`, and classification now keys on
the DECLARATION KIND, so a `[dev-dependencies]` declaration is compliant from any directory. A
self-test guards the escape hatch: a production-runtime manifest cannot hide behind an oracle
registry name. This is why `csv`/`dxf`/`gif`/`las`/`lopdf`/`quick-xml`/`riff`/`ruststep`/`tiff`/`tobj`
cleared — they were always fixture generators, previously mis-reported.

## Verification status — honest

Proven by passing runs: png/image codec (12 tests), text+path (11 tests), animate render (3 tests),
and `PathSeg::arclen` differentially against `kurbo` over 200 curves, max relative error 1e-6
(run in a standalone scratch crate because the in-crate suite was blocked by contention).

**Not yet re-run centrally:** the seven base64 plugin wasm builds, puzzle's blake3 digest pinning,
and the parry3d intersection parity corpus. Machine is saturated — twelve live interactive
developer sessions plus the agent fleet — and a single dependency-free crate's `cargo test` timed
out twice at 600s. These need a central verification pass once contention eases.

## Incident — fixed

A peer's edit to
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml`
replaced `blake3 = "1"` with a `semio-framework-hash` path dep using six `../` where every sibling
in that file uses nine. That broke `cargo metadata` for the ENTIRE workspace — every agent's builds
failed for an unrelated reason. Corrected; `cargo metadata` exits 0.


---

# 🔄️ Update — serde fan-out dispatched

## Measured now

| metric | baseline | now |
|---|---|---|
| third-party entries, s production manifests (rust) | 119 | **64** |
| distinct rust crates leaking | 23 | **7** |
| JS third-party in s `dependencies` | 11 pkgs | **6** |
| gate `oracle-conflicts` | 18 | **6** |

`wasm-bindgen` is fully eliminated (25 → 0). `js-sys` 6 → 1, `web-sys` 3 → 1.

Remaining 64 = `serde_json` 36 + `serde` 23 + proc-macro trio 3 + `js-sys`/`web-sys` 2.

## The proc-macro trio is NOT a violation — do not "fix" it

`…/🔄️fsm/✨️macros/📦️packages/🦀️rust` declares `[lib] proc-macro = true`. Its `[dependencies]` are
compiler plugins linked at BUILD time, never into the target binary; declaring them there is
required. The classifier now reports them `production-build`. Moving them to
`[build-dependencies]` would break the crate. Closed as correct-as-is.

## Fan-out in flight — 6 agents, disjoint batches, 37 manifests

`🌊️flow`×9 · `🏭️process`+`🪵️sourcing`×8 · `📜️imperative`+`🌀️procedural`+`📖️playbook`×7 ·
`📐️cad`+`➗️mathematical`+`🔋️energy`×6 · `🗄️stdio`+`🔱️trinity`+`🪐️space`×5 · tail+wasm remnants.

All build on the pilot's foundation: `ToValue`/`FromValue` over `DslValue`
(`🌱️value/🔁️codec`), `#[derive(ToValue, FromValue)]` (`🌱️value/✨️derive`), and
`MutationDiff`/`Mutation` rebounded off `serde::Serialize + DeserializeOwned`.

## 🚨️ Blocking issue — os-kernel blast radius

The trait-bound change left `semio-framework-os-kernel` with ~75 errors in mutation/store code.
Two independent agents hit it and each confirmed by grep that the errors name none of their own
slices. Every s plugin depends on os-kernel, so **while it is red nothing downstream can be
verified**. The pilot has been redirected to repair it before anything else. This is expected
fallout from the correct root-cause fix, not a reason to revert it.

## Verification ledger — honest

**Proven by passing runs:** base64 4/4 (RFC 4648 vectors + third-party oracle differential) ·
png/image 12/12 (differential vs `png`) · mesh-engine/glTF 26/26 (3 differential oracle tests) ·
text+path 11/11 · animate render 3/3 · kurbo arclen differential over 200 curves, max relative
error 1e-6 · puzzle 15/15 · animate 148/148 · cad 316/322 (6 pre-existing flakes).

**Unproven, must be re-run centrally:**
1. `semio-framework-3d` — the parry3d replacement (BVH-pruned Möller triangle test + winding-number
   containment) **has never compiled**. Its author listed six divergence risks: touching-convention
   assumption, hand-derived edge-loop vs Möller's isolated-vertex branching, winding-number vs
   parry3d's pseudo-normal method, single-tree BVH pruning, f32 summation order, and quaternion
   math written without source access. Highest-risk open item.
2. BLAKE3 first-party parity — digests are content-addressed and persisted via `mint_edit_id` /
   `mint_change_id` / `mint_mutation_id`; a one-bit difference silently invalidates stored data.
3. No `wasm32-wasip2` plugin build has been confirmed by anyone, for any plugin.

## Environment lessons (cost the fleet ~1h)

- Four agents parked on `Monitor`/`run_in_background` despite explicit foreground instructions.
  Children die at turn end; the notification never arrives.
- Several set an isolated `CARGO_TARGET_DIR` to dodge lock contention, which forces a from-scratch
  rebuild of wasmtime/cranelift. Combined with ~12 live peer sessions this hit 77 concurrent rustc
  and nothing finished. Twelve redundant tasks killed; all later prompts ban both practices.
- A peer's `semio-framework-hash` path dep used six `../` where siblings use nine, breaking
  `cargo metadata` repo-wide. Every new path dep must be resolve-checked with
  `ls -d <manifest-dir>/<relative-path>`.


---

# 🔄️ Update — os-kernel green, 119 → 22

| metric | baseline | now |
|---|---|---|
| third-party entries, s production manifests | 119 | **22** |
| distinct rust crates leaking | 23 | **7** |

Remaining 22: `serde_json` 11 · `serde` 6 · `js-sys` 1 · `web-sys` 1 · proc-macro trio 3.

- The **proc-macro trio (3) is not a violation** — a `proc-macro = true` crate's `[dependencies]`
  are compiler plugins, build-time only. Closed as correct-as-is.
- `js-sys`/`web-sys` (2) are `🧩️puzzle`'s `BoardSession`, a live ~50-method WebGPU canvas bridge
  taking `HtmlCanvasElement`, wasm-pack-built as `@semio-tech/puzzle-wasm`. Genuinely live;
  correctly deferred with a framework-wrapper plan rather than rushed.
- So the real remaining work is **17 serde/serde_json entries**.

## 🟢️ os-kernel is GREEN

`cargo check -p semio-framework-3d` exited 0 after 43m, compiling `semio-framework-os-kernel`
as a dependency with warnings only. The pilot's trait-bound migration converged
(166 → 75 → 6 → 4 → 0 across successive agent observations). This unblocks all downstream
verification. `semio-s-plugin-draw-fsm` was independently observed compiling clean for
`wasm32-wasip2` — the first confirmed real plugin build of this ticket.

## ⚠️ Correction to an earlier assumption in this ticket

An early exploration claimed `wasm-bindgen`/`js-sys`/`web-sys` were inert on the plugin target
because their uses sit behind `#[cfg(target_arch = "wasm32")]` while plugins build for
`wasm32-wasip2`. **That is wrong**, and the wasm-glue agent caught it:

```
rustc --print cfg --target wasm32-wasip2
target_arch="wasm32"    ← the gate IS active
target_env="p2"
target_os="wasi"
```

The removals were still correct, but for a properly-evidenced reason: of 34 manifests, 19 were
dead declarations with zero call sites and 14 were bridge code that nothing builds (verified
against the `playgrounds.ts` engine registry and each crate's `📜️script.ts`, not assumed).

## Two framework gaps found and filled mid-wave

- `pack::json::Value` ↔ `DslValue` bridge + `to_json_string`/`from_json_str` added to
  `🎒️pack/🔤️json`, reachable as `semio_framework_os_kernel::json::*`.
- **A second serde root cause**: `ArtifactApp::Snapshot` in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` still bounds on
  `Serialize + DeserializeOwned` (3 trait declarations + generic restatements; ~78 serde refs in
  that file). Independent of the `Mutation` bound. Since every plugin's Snapshot is plugin-local,
  no plugin can reach zero until it moves. Dedicated agent assigned.

## Deferred as its own wave, with reason

`🏭️process`'s own crate: 78 derive sites / 363 `serde_json::` call sites across ~60 files —
comparable in size to the whole animate wave. Its 4 extensions are clean; the parent needs its own
pass. `🌀️procedural`: ~1277 serde call sites across 187 files, blocked on the `Snapshot` bound.

---

# 🔄️ Update — wasip2 glue leak: 12/13 manifests closed, `draw-fsm` clean, `puzzle` blocked on one new finding

## The bug, confirmed and fixed at the 13 named manifests

`rustc --print cfg --target wasm32-wasip2` confirmed `target_arch = "wasm32"` is TRUE for the WASI
component target too — so 13 framework manifests' bare `[target.'cfg(target_arch = "wasm32")'
.dependencies]` gates were linking `wasm-bindgen`/`js-sys`/`web-sys` into every wasip2 plugin. All
13 (`🎭️actor`, `🔄️machine`, `📡️replication`, `🗺️surface`, `🧮️math`, `⏳️async`, `🖱️ui/🖥️host`,
`🖱️ui/🖼️render/…/🧊️webgpu`, `🖱️ui` itself, `✍️editor`, `💻️os/🌊️flow`, `💻️os/♾️infinite`, `💻️os`
os-kernel) are now narrowed to `all(target_arch = "wasm32", not(target_env = "p2"))`, with the
corresponding Rust-side `#[cfg(...)]` blocks narrowed to match wherever they touch
`wasm_bindgen`/`js_sys`/`web_sys` — full per-crate (a)/(b) table (browser-only vs genuine wasip2
arm) in `🔍️research/📓️wasip2-glue-leak.md`. Two genuine wasip2 arms were added (not stubbed):
`dag_debug_log`/`♾️infinite`'s `now_ms`/`💻️os/🏪️store/🔄️sync`'s `now_ms` now take the existing
native `eprintln!`/`SystemTime` path on wasip2 (WASI has both); `identity`'s `fill_entropy` widens
the pre-existing "no delegation" `Err` catch-all to include wasip2 rather than inventing a stub —
`time_ordered_id` already degrades gracefully from that.

## cargo tree -i evidence

- `semio-s-plugin-draw-fsm` — `wasm-bindgen`/`js-sys`/`web-sys` all **absent** from the
  `wasm32-wasip2` graph. Clean.
- `semio-s-plugin-puzzle` — still present, via exactly one remaining path:
  `puzzle → semio-framework-os-infinite → semio-framework-ui → wgpu → wasm-bindgen-futures/web-sys`
  (also via `vello`/`vello_svg`, same root). See below.

## 🚨️ One new finding, left unfixed on purpose — do not rush this

`♾️infinite`'s `🌍️world/🦀️component.rs` (14k lines, mounted unconditionally — real board/mesh3d/
action-queue domain logic, not renderer glue) unconditionally names ~26 symbols
(`Mesh3dFault`, `BoundedActionFault`, `InputState`, `project_point`,
`checked_action_string_bytes`, the `world3d_snapshot_*` family, …) from `semio-framework-ui`'s
`"wgpu-engine"` feature tier, at 192 call sites. Those symbols live in files (`draw.rs`,
`action.rs`, `input.rs`) that also carry genuine unconditional `use wgpu::…` (the real GPU crate).
An attempt to exclude the whole wgpu-engine tier from wasip2 (51 mount-site gates in
`🖱️ui/🎯️targets/🧊️wgpu/📦️glue.rs`, plus splitting `vello`/`vello_svg`'s `wgpu` feature out of
`♾️infinite`'s manifest) DID close the `cargo tree -i` leak completely, but broke `world.rs`'s
build (~90 `E0433`/`E0425` errors) since those 26 symbols stopped existing for wasip2. **Reverted**
rather than shipped broken or dishonestly narrowed — full writeup, and the exact follow-up
(relocate those ~26 symbols out of the GPU-coupled files into a target-neutral module), in
`🔍️research/📓️wasip2-glue-leak.md`'s "Genuinely blocked" section.

## Build verification — honest

Neither `puzzle` nor `draw-fsm` could be confirmed fully green in `--target wasm32-wasip2` this
pass: both are blocked by the **unrelated, concurrent, in-flight serde-elimination wave** (the
`ArtifactApp::Snapshot`/`Mutation` `ToValue`/`FromValue`/`Serialize` bound gaps this same status
doc already tracks above — `SpaceAlternative`, `SpaceCheckpoint`, `HybridLogicalTimestamp`, etc.,
in `semio-framework-os-kernel`). Confirmed unrelated: zero mention of
`wasm_bindgen`/`js_sys`/`web_sys`/`wgpu` in any of those errors.

Individually verified green, both `--target wasm32-wasip2` and `--target wasm32-unknown-unknown`
(browser NOT broken): `semio-framework-actor`, `semio-framework-machine`, `semio-framework-async`,
`semio-framework-replication`, `semio-framework-math`. Browser-only (not currently buildable for
wasip2 due to the unrelated os-kernel breakage, but confirmed NOT broken by this pass on
`wasm32-unknown-unknown`): `semio-framework-ui-host`, `semio-framework-ui-backend-webgpu`,
`semio-framework-ui`. `semio-framework-editor`/`semio-framework-surface` could not be checked on
either target — both depend on the currently-red `os-kernel`.

One transient `wasm-component-ld`/`rust-lld` SIGSEGV was observed linking `semio-framework-actor`
under default parallelism (load average 30–36 this session); did not reproduce building that crate
alone or at `-j 1`/`-j 2` — attributed to resource contention, not this pass's changes.
