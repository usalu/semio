# 🧪️ W2h — wgpu unit tests: Nx visibility, stub cleanup, wiring law

Packet W2h of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Follows `📓️audit-build-serve-theme.md` §3
(packets 2–3), which found the wgpu unit-test case directories invisible to Nx and three empty stub
directories in the os renderer.

---

## 1. Census — what exists

Full machine-generated map (case directory → the exact production file that mounts it):
`🗑️generated/w2h-case-census.txt`.

| tree | case dirs | crate | Nx target that runs them |
|---|---|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-*` (33) + `🧱️elements/*/🧪️tests/🔬️wgpu-*` (3) | **36** | `semio-framework-ui` | `@semio-tech/ui-rs:test-wgpu-engine` |
| `📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-*` (12 dirs, 11 Rust) + `🧑‍🎨engine/🧱️elements/*/🧪️tests/🔬️wgpu-*` (39) | **50** | `@semio-tech/framework-renderer-wgpu:test-wgpu-unit` | |
| **total** | **86** | | |

(The audit's §3.1 counted 30 `🔬️targets-wgpu-*`; there are now 33 — `🔬️targets-wgpu-theme-token-parity`
and `🔬️targets-wgpu-widget-metrics` landed after it, plus this packet's own law case. It did not
enumerate the ui element cases, nor any of the renderer's 50, whose four `🎯️targets/🧊️wgpu/🧪️tests/`
stubs it mistook for the renderer's whole Rust case inventory.)

Every one of the 86 is mounted with `#[cfg(test)] #[path = "…"] mod tests;` or an `include!("…")`
inside an existing `mod tests { … }`. **Zero** are `[[test]]` entries; `semio-framework-ui`'s and
`semio-framework-os-renderer-wgpu`'s manifests declare none. One case directory holds only a
TypeScript adapter (`🧑‍🎨engine/🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts`) and is run by vitest, not cargo.

### Why the taxonomy test plugin does not see them

`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs` discovers on `**/*.feature`. Its
`canonicalCase` gate *accepts* these directory names — `🔬️` is not in
`pathEmojiPolicy.genericEmojiIdentities` (`📁️`, `📂️`, `📄️` only), the slugs match
`testCaseSlugPattern`, and no owner path crosses `🧪️tests`/`📦️packages`/`⚡️implementations`/`🎯️targets`.
The single reason all 86 are invisible is that none carries a `🥒️.feature` file.

**Decision: they were not converted to `.feature` cases.** That route generates a host that links the
subject crate plus a third-party oracle, i.e. every case would need a gherkin feature, an adapter and
an independent twin. These are `use super::*` unit laws that reach into private module internals —
they have no language-agnostic fixture to answer, and there is nothing to twin. The repo's *other*
convention for Rust unit tests is used instead, the one `@semio-tech/ui-rs` already follows: a named
target in the crate's `📋️project.json` calling that package's `📜️script.ts`.

---

## 2. What was wired

### `@semio-tech/ui-rs` — `test-wgpu-engine` now matches the integrator's invocation

`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts`

- `TestWgpuEngineScript` gained `--lib`, so the target is exactly
  `cargo test -p semio-framework-ui --features wgpu-engine --lib` (the `📓️w1-integration.md` row 5
  command). Without `--lib` the run also dragged in integration targets this lane does not own.
- 🐛 **Both** `TestScript` and `TestWgpuEngineScript` were declared `run(): void` and called the
  `async` `runCargoTestBudgeted` **without awaiting it**. `ScriptRouter.run` does
  `await Promise.resolve(script.run(…))`, so the router returned before cargo had settled and the
  test run's outcome was a floating promise. Both are now `async run(): Promise<void>` with `await`.
  The target already existed in `📋️project.json`; no JSON change was needed.

### `@semio-tech/framework-renderer-wgpu` — new `test-wgpu-unit`

`…/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json}`

- New `WgpuUnitTestScript` → `cargo test -p semio-framework-os-renderer-wgpu --lib` (row 6's command),
  registered as `test-wgpu-unit`. The pre-existing `test-native` runs the crate's *all-targets* suite;
  `--lib` is the selector that compiles the 50 mounted case files and the selector a dangling
  `#[path]` wedges, so it gets a target of its own.
- The auto-generated `build`/`check`/`test` from the library plugin's `cargoTargets` are untouched.

### `semio-framework-os-infinite` — new `test-wgpu-world-terrain`

`…/♾️infinite/📦️packages/🦀️rust/{📜️script.ts,📋️project.json}`

- New `TestWgpuWorldTerrainScript` → `--lib` plus the `world::` and `terrain` filters (row 7's
  command). Filters are passed as nextest positional execution filters, which
  `partitionNextestExecutionFilters` routes correctly; `--lib` lands in the build args.
- The package's `📜️script.ts` previously registered only `fonts`.

### `.vscode/🧩️launch.seed.jsonc` → `.vscode/launch.json`

⚠️ **`.vscode/launch.json` is generated, not hand-maintained.**
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts` renders it from
`.vscode/🧩️launch.seed.jsonc` plus the playground registry. A direct edit to `launch.json` made
earlier in this packet was silently wiped within minutes by a peer's regeneration (file mtime moved,
`git diff` went empty). The edit belongs in the **seed**, followed by
`bun 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts generate` — which is what
was done here; that run reported `.vscode/launch.json regenerated` and left the registry's own
`🤖️generated` catalog byte-identical.

Three entries added to the seed in group `4_build`, continuing the existing wgpu block
(`206.145`–`206.147` → `206.148`, `206.149`, `206.15`) and its `📦️<verb>🧊️wgpu…` naming:

| name | command |
|---|---|
| `📦️test🧊️wgpu🖱️ui` | `bun nx run @semio-tech/ui-rs:test-wgpu-engine` |
| `📦️test🧊️wgpu📺️renderer` | `bun nx run @semio-tech/framework-renderer-wgpu:test-wgpu-unit` |
| `📦️test🧊️wgpu♾️infinite` | `bun nx run semio-framework-os-infinite:test-wgpu-world-terrain` |

---

## 3. Stub directories — all four deleted

`🎯️targets/🧊️wgpu/🧪️tests/` held four dead directories. Every live case in this product lives one
level up, under `🧑‍🎨engine/🧪️tests/` — which is also what the vitest config in that same folder points
at (`🎚️config/🟦️.ts` sets `testRoot` four levels up, to `🧑‍🎨engine`).

| deleted | superseded by | evidence |
|---|---|---|
| `🎮️wgpu-browser-input-wire/🦀️.rs` (7-line placeholder) | `🧑‍🎨engine/🧪️tests/🎮️wgpu-browser-input-wire/{🦀️.rs,🟦️.ts}` — 3 Rust laws + TS twin over `🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json` | the mount in `🎮️input-wire/🦀️.rs:176` is `../../../🧪️tests/…`, i.e. three levels up — it already resolves to the engine-level case, **not** to the placeholder. The placeholder had been orphaned since W1g landed the real laws. |
| `🎮️browser-interactive-job-port/` (empty) | `🧑‍🎨engine/🧪️tests/🎮️browser-interactive-job-port/🟦️.ts` (22 KB, in the vitest `include`) | — |
| `📨️browser-frame-transport/` (empty) | `🧑‍🎨engine/🧪️tests/📨️browser-frame-transport/🟦️.ts` (38 KB, in the vitest `include`) | — |
| `🧩️package-integration/` (empty) | `🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts` (34 KB, in the vitest `include`) | — |

Only the placeholder was ever tracked (`git ls-files` on that folder returned it and `🎚️config/🟦️.ts`
alone — git does not track empty directories). `🎚️config/` stays. A three-line note recording the
removal sits at the mount site in `🎮️input-wire/🦀️.rs`. No assertions were invented for the deleted
stubs: their real assertions already exist one level up.

---

## 4. The wiring law

`🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-case-wiring-law/🦀️.rs`, mounted from
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs` (the wgpu target root — it owns no module
behaviour, only the mounts every other case is reached through). Two tests:

1. `every_wgpu_case_directory_is_mounted_exactly_once` — each `🧪️tests/🔬️*wgpu*/🦀️.rs` under the ui
   and renderer trees is named by exactly one `#[path]`/`include!` in a production source. Zero means
   test code that never compiles; two means the same assertions running twice.
2. `no_module_mount_in_the_wgpu_trees_is_dangling` — no `#[path]`/`include!` names a missing file.
   This is the one that matters: the 26/09/09 incident recorded in the deleted placeholder's own
   header was a dangling `#[path]` failing `cargo test -p semio-framework-os-renderer-wgpu --lib` for
   the **whole crate** with `couldn't read …: No such file or directory`. The law names the offender
   instead of letting the crate's whole test build wedge.

Scanner details that were found the hard way and are encoded in the file:

- `#[path]` resolves relative to the **containing file's own directory** for a top-level `mod`, and
  relative to `<dir>/<file stem>` for one inside an inline `mod` block; both candidates are tried.
- Lines whose trimmed start is `//` are skipped — several docstrings quote `#[path = "…"]` while
  explaining it, and a naive scan reports every one as dangling.
- Only literals ending in `.rs` count. That drops the directory form
  (`🧊️renderer/🦀️.rs:3199`'s `#[path = "."]`, which only re-roots an inline module's children) and
  `include!(concat!(env!("OUT_DIR"), …))` in `🖼️IconRenderHost`, both of which name no file.

---

## 5. Verification

| check | result |
|---|---|
| `NX_DAEMON=false bun nx show project @semio-tech/ui-rs --json` | ✅ lists `test-wgpu-engine` → `bun ./📜️script.ts test-wgpu-engine` |
| `NX_DAEMON=false bun nx show project @semio-tech/framework-renderer-wgpu --json` | ✅ lists `test-wgpu-unit`, cwd = the wgpu TS package, `forwardAllArgs: true` |
| `NX_DAEMON=false bun nx show project semio-framework-os-infinite --json` | ✅ lists `test-wgpu-world-terrain` |
| router registration (`bun ./📜️script.ts __probe__` in each of the three packages) | ✅ usage lines name `test-wgpu-engine`, `test-wgpu-unit`, `test-wgpu-world-terrain` |
| `cargo test --no-run -p semio-framework-ui --features wgpu-engine --lib -j 4` | ✅ exit 0, `Finished test profile in 15.48s` (`🗑️generated/w2h-ui-test-norun.txt`) |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -j 4 -- case_wiring_law_tests` | ✅ **2 passed, 0 failed**, 551 filtered out (`🗑️generated/w2h-law-test.txt`) |
| `bun …/📇️registry/📜️script.ts generate` (launch.json from the seed) | ✅ exit 0, `.vscode/launch.json regenerated`; all three `📦️test🧊️wgpu…` entries present at lines 3547/3558/3569 (`🗑️generated/w2h-launch-generate.txt`) |
| law catches a real violation | ✅ an independent Python twin of the same algorithm flagged this very case directory as `UNMOUNTED` before it was wired, and flagged the comment/`concat!`/directory-form literals as dangling before the `.rs` filter was added |

No wgpu activation, no trunk, no full `cargo test` suite was run (W3c/W3d own the live serve).

---

## 6. Files touched

Wired:
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-case-wiring-law/🦀️.rs` *(new)*
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎮️input-wire/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/📋️project.json`
- `.vscode/🧩️launch.seed.jsonc` (source) and `.vscode/launch.json` (regenerated from it)

Deleted: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/{🎮️wgpu-browser-input-wire,🎮️browser-interactive-job-port,📨️browser-frame-transport,🧩️package-integration}/`

---

## 7. Left open

- The 86 cases are now runnable and cacheable through three named Nx targets, but still not one Nx
  project *per case*. That would need the `.feature` route (§1) and, with it, a language-agnostic
  fixture and an independent twin per case — a separate ticket, not a wiring change.
- The law covers the ui and renderer trees. Other crates mount `#[cfg(test)] #[path]` cases the same
  way and have the same wedge exposure; widening `SCAN_ROOTS` (or lifting the law to a repo-wide
  check) is the obvious follow-up.
- `cargo test -p semio-framework-ui --features wgpu-engine --lib` emits 31 warnings, e.g. an unused
  `crate::wgpu::text::FontAtlas` import in `🔬️targets-wgpu-shell-unit`. Pre-existing, not touched.
