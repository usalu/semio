# Native Build Latency and Target Ownership Frontier

Status: read-only current-source and live-process audit, 2026-09-05. No build, cache cleanup, process termination, or product edit was performed.

## Verdict

The observed extreme duration is a mixture of three different conditions, not one stuck Cargo build:

1. The current Home/space native build is **CPU-active compilation** of the complete `stdio` crate under the Hub's `--all-features` binary gate. It is not blocked by the old document-open test.
2. There are several simultaneous, isolated target roots independently compiling the same large `stdio` rlib/cdylib graph (native debug, WASI dev, and WASI release). Those artifacts cannot share output across target triples/profiles, and the separate native roots also forgo ordinary same-profile reuse.
3. The six-day document-open command is an already-built **runtime test without a terminal result**, not a compiler or Cargo-lock wait. It is isolated from both current Home/WGPU target roots.

The current profile split is materially better than the earlier target-blind configuration: native `dev` has incremental compilation and no one-CGU override; `wasm-dev` and `wasm-release` alone retain the WASI linker mitigation. Do **not** relax the WASI profiles as a latency workaround.

## Live Ownership Snapshot

The process snapshot was taken during this audit. State `U`/`UN` is active/runnable compiler work; `S`/`SN` with no compiler child is waiting or sleeping. CPU percentage is a point-in-time scheduler observation, not a benchmark.

| Process tree / state | Exact target root | Finding |
| --- | --- | --- |
| `bun 48411 -> nx 48558 -> hub script 49065 -> cargo 49443 -> rustc 79325`; `79325` CPU-active for about one hour | `.../🗑️generated/space-public-boundary-sol-target` | Current Home/Hub path. The command is Hub `--all-features` binary compilation and the compiler child is `semio_s_plugin_stdio`; it is real compilation, not a target-lock wait. |
| `cargo 43753 -> rustc 40029`, `CARGO_BUILD_JOBS=1` | `.../🗑️generated/wgpu-directory-retained-home-sol-target` | Current WGPU/Home native-law compilation has its own root and is independently compiling `stdio`. |
| `cargo 94794 -> os_hub test 94953`, elapsed over six days; parent is PPID 1, child state `S` | `.../🗑️generated/open-plan-issuer-target` | The executable is already built and runs only `document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable --exact`. It has no compiler child and no terminal receipt. It cannot own Cargo's lock for either root above because all three target roots are distinct. No termination was performed. |
| `cargo 86722 -> rustc 89367`, `CARGO_BUILD_JOBS=6`, active for over five hours | `/Users/ueli/Documents/semio/target-demonstrator` | A genuine WASI-release stdio component link: `cdylib` and `rlib`, `wasm32-wasip2`, `opt-level=s`, one CGU, link emission and `-Z threads=8`. This is expected expensive isolated publication work, not a native-Hub delay. |
| `cargo 91167`, `wasm-dev` process plugin command, zero CPU and no child in the snapshot | its inherited/default target was not recorded by the process | Waiting/idle only. Neither a lock owner nor an active compiler was observable, so this audit does **not** attribute its wait to a particular target lock. |

Additional active independent roots included `target-block-w1`, `target-block-3d`, `target-lowpoly-e2e`, `target-demonstrator-dev`, `target`, and `target-p3d-e2e`; multiple of them were compiling `semio_s_plugin_stdio`. This is direct duplicate-work pressure, not evidence that any one of those owners may be stopped.

## Verified Causes

### 1. Hub's runtime-selected gate compiles every backend

[`SpacePublicBoundaryCheckScript`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3787) constructs its native test command with `--all-features` at [line 3790](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3790), executes each selected binary law, then repeats an `--all-features --bin os-hub` check at [line 3809](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3809). The Hub manifest makes those features SQLite, Postgres, and Neo4j together ([`Cargo.toml:20-54`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/Cargo.toml:20)); the latter two pull SQLx/rustls and Neo4j in addition to the selected SQLite runtime.

That is intentionally broader than the runtime journey. The selected catalog itself is fixed to 26 stdio plus two GIS codec receipts ([`native-openable-provider:8-12`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:8)) and links only those two providers ([`:25-34`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:25)). Loading every storage driver is not required to prove the Home public-boundary law.

There is an existing correct precedent: the document-open server check explicitly calls its selected binary laws without `--all-features` and labels that a default-feature subset, keeping kernel all-feature qualification separate ([`hub script:3461-3463`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3461)).

### 2. The selected Hub catalog still imports a large plugin graph

The Hub normal dependency graph joins `axum` WS, full Tokio, `directory`, `db`, the plugin host, `stdio`, and GIS in one crate ([`Hub Cargo.toml:31-47`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/Cargo.toml:31)). `default-features = false` on stdio prevents its component installer symbol, but does not make the dependency shallow. Its manifest still has unconditional UI-contract, framework, geometry, graph, math, mesh-engine, OS-kernel, component-guest plugin, schema and pack dependencies ([`stdio Cargo.toml:25-59`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml:25)).

WGPU reaches the same graph through puzzle: the renderer declares `puzzle` as a native dependency, and puzzle declares stdio with `default-features = false` ([`wgpu Cargo.toml:28-62`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml:28), [`puzzle Cargo.toml:80-98`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/Cargo.toml:80)). This explains why independent WGPU and plugin checks repeatedly reach a stdio compile even when no stdio UI is activated.

### 3. Scale mode pays for the full WGPU presentation graph before it can skip plugin work

`NativeBuildScript` always invokes Cargo with `--features native-bin` ([`wgpu script:301-306`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:301)), then only after Cargo returns treats `--scale` as no-plugin-catalog mode ([`:309-322`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:309)). The manifest puts `image`, `vello`, `winit`, `resvg`, `tiny-skia`, `usvg`, puzzle, UI and host dependencies on the native build path ([`wgpu Cargo.toml:28-105`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml:28)). The claimed scale mode therefore avoids runtime activation, but not its compile dependency closure.

### 4. WASI release work is deliberately serialized and fresh

The current root manifest correctly confines one-CGU policy to `wasm-dev` and `wasm-release` ([`Cargo.toml:249-303`](/Users/ueli/Documents/semio/Cargo.toml:249)). The release component also deliberately uses ThinLTO, one CGU, no incremental artifacts and symbol stripping ([`:291-298`](/Users/ueli/Documents/semio/Cargo.toml:291)). This is necessary publication policy, not an accidental native profile regression.

The strict stdio catalog-root operation additionally requires an empty dedicated root, rejects the ambient target, creates a per-process target, disables incremental and disables sccache ([`stdio script:701-750`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:701)). It must remain a cold, isolated receipt operation; sharing its target or accepting a warmed artifact would weaken its freshness claim.

## Bounded Improvements, in Order

### A. Make the Home public-boundary runtime gate SQLite-selected (largest immediate gain)

Change only the native runtime-gate command constructed at [`hub script:3790`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3790) from `--all-features` to an explicit SQLite-selected feature set. Keep the all-feature Hub compile as a separately named backend-matrix/qualification gate after the selected Home result; do not delete it or treat SQLite success as Postgres/Neo4j evidence.

Acceptance:

- The four public-boundary exact binary laws compile and execute with the declared SQLite feature set, retain their hostile private-404/socket assertions, and emit one terminal receipt.
- A source test proves this selected journey does not pass `--all-features` and does pass exactly the selected storage feature.
- A separate all-backend Cargo check still exercises SQLite, Postgres and Neo4j. Its receipt is explicitly backend-qualification evidence, not Home runtime evidence.
- The catalog set remains exactly 26 stdio + 2 GIS receipts; no plugin, descriptor, or provider selection is removed.

This follows the existing document-open default-subset precedent and does not touch agents' current target trees or processes.

### B. Give WGPU scale mode a real minimal compile feature

Split `native-bin` into an explicit presentation feature and a `native-scale` feature. `native-build --scale` must select only the scale binary/modules; full WGPU/Home launch selects the presentation feature and retains the complete plugin/puzzle/UI/GPU graph. Gate source modules and Cargo dependencies so that scale genuinely does not resolve puzzle, the plugin host, UI presentation, `winit`, `vello`, SVG/raster codecs, or native directory transport unless its own source uses them.

Acceptance:

- A source/argument law pins `--scale` to `native-scale`, and an ordinary native launch to the presentation feature.
- A native scale receipt proves the same existing scale result with no plugin catalog/program build claim.
- The retained-Home native laws still compile under the presentation feature and preserve the existing three terminal-receipt laws ([`wgpu script:490-515`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:490)).
- Cargo metadata/source tests prove the scale feature has no puzzle/stdio or presentation-only dependencies. This is a compile-closure claim only; it is not a Home activation claim.

### C. Coordinate native debug roots without sharing publication roots

The existing launch setup already safely shares a target among serial catalog/gate commands: native provider and catalog-selection entries both use `native-openable-provider-sol-target` with one Cargo job ([`launch.json:5550-5599`](/Users/ueli/Documents/semio/.vscode/launch.json:5550)). Preserve that pattern, but introduce a target-root lease/scheduler for **new** selected native-debug gates with the same toolchain, profile and feature tuple. It must serialize owners of a shared root, retain the exact executable hash/list/run receipt validation, and never clean, move, or attach to any live target.

Do not merge this with catalog-root/component publication: that operation deliberately requires an empty per-process root. Do not raise global `CARGO_BUILD_JOBS` while independent agent compiles are live; current activity already combines Cargo jobs and rustc internal threads across many roots.

Acceptance:

- A hostile orchestration fixture proves a second gate waits for the lease, observes the first terminal receipt, then reuses the root; cancellation before lease acquisition produces no Cargo child and no target mutation.
- Each exact law continues to hash and list its own executable; a stale or substituted executable rejects.
- Existing live roots (`open-plan-issuer-target`, `space-public-boundary-sol-target`, and `wgpu-directory-retained-home-sol-target`) are never adopted or deleted by the new coordinator.

### D. Later, isolate catalog authority from Hub transport/backend ownership

The native provider itself is a small fixed closure, but it currently lives under Hub's broad artifact-authority module and imports trusted-catalog vocabulary ([`native-openable-provider:1-75`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:1)). A clean future package boundary can move only trusted catalog loading, exact provider selection, `AuthorityError`, and `OperationContext` into a catalog-authority crate; Hub's HTTP/routes, directory administration, CAS and backend drivers remain in the Hub crate. The Hub binary then depends on that crate, while provider/selection laws compile it directly.

This is not a quick feature toggle: trusted catalog currently uses kernel directory/store types and Tokio file reads, so its exact dependency surface must be declared rather than hand-waved. The payoff is avoiding Axum routes, full Hub test scaffolding, and inactive backend drivers when validating a static 28-receipt catalog closure. Keep one Hub binary readiness law as integration evidence.

## Non-Changes Required for Safety

- Do not terminate orphan `94794`/`94953` or any other agent process under this packet. Diagnose its no-terminal runtime test separately.
- Do not delete or merge the existing target directories. The observed old Hub test target is distinct from the active WGPU/Home targets.
- Do not restore target-blind `profile.dev.package.*` one-CGU overrides or weaken `wasm-dev`/`wasm-release` ThinLTO/one-CGU/freshness policy.
- Do not reinterpret a component catalog-root cold build as a warm native exact-law build, and do not reduce the selected stdio/GIS provider closure.

## Nonclaims

No elapsed-time benchmark or changed build result was produced. This audit does not claim the active Home, Hub, WGPU, stdio, GIS, or document-open runtime test succeeds; it records present ownership, feature/profile dependency causes, and bounded changes that preserve required plugin and backend coverage.
