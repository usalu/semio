# 🩺️ Compile health — demonstrator runtime closure, `wasm32-wasip2` (2026-09-16)

Scope: `cargo check --target wasm32-wasip2 --profile wasm-dev` over every Rust crate in the
demonstrator's runtime closure (the two pane-hosting plugins, the six host plugins whose apps they
bundle, `flow` + `stdio`, and every `EXTENSION_TARGETS` row that extends one of them).

**Result: 29/29 crates compile clean. Both the multi-package run and the demonstrator-only run
FINISHED (cargo `Finished \`wasm-dev\` profile` line, exit 0).** Six one-line visibility fixes were
needed to get there; nothing else in the closure was broken.

## 🧭️ Closure identification

Profile used by the real plugin build: `pluginWasmProfile()` in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts:79` returns `"wasm-dev" | "wasm-release"`;
root `Cargo.toml:503` defines `[profile.wasm-dev] inherits = "dev", codegen-units = 1`, with
`opt-level = 2` package overrides for `semio-s-plugin-procedural`, `semio-s-artifact-puzzle-3d`,
`semio-s-artifact-lowpoly-lowpoly`, `semio-framework-os-flow`,
`semio-framework-os-kernel-neural-engine`, `semio-framework-replication`, `semio-framework-pack`,
`semio-s-plugin-flow-extension-brep`, `semio-s-plugin-flow-extension-math`. All checks below use
`--profile wasm-dev`, so they share the real build-dir fingerprints.

Registry rows read from
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts`:
every host plugin row (`cad`, `demonstrator`, `flow`, `gis`, `procedural`, `process`, `puzzle`,
`sourcing`, `stdio`) has `dependsOn: []`; every `EXTENSION_TARGETS` row (lines 81–106) carries
`dependsOn: ["<host>"]` / `extends: "<host>"`. The demonstrator's actual crate-level closure is
declared in its `Cargo.toml` (`depends-on = ["cad","gis","procedural","process","puzzle","sourcing"]`
and the matching `default-features = false` path deps at lines 142–147).

## 📊️ Per-crate status

| # | Crate | Role | Status before | Errors | Fixed? |
|---|---|---|---|---|---|
| 1 | `semio-s-plugin-demonstrator` | pane bundle | **13 errors** | E0433 ×3, E0603 ×10 | ✅ yes (fixed in its 6 dependency crates, see below) |
| 2 | `semio-s-plugin-procedural` | pane host | OK | 0 | — (edited to unblock #1) |
| 3 | `semio-s-plugin-cad` | pane host | OK | 0 | — (edited to unblock #1) |
| 4 | `semio-s-plugin-puzzle` | pane host | OK | 0 | — (edited to unblock #1) |
| 5 | `semio-s-plugin-sourcing` | pane host | OK | 0 | — (edited to unblock #1) |
| 6 | `semio-s-plugin-process` | pane host | OK | 0 | — (edited to unblock #1) |
| 7 | `semio-s-plugin-gis` | pane host | OK | 0 | — (edited to unblock #1) |
| 8 | `semio-s-plugin-flow` | extension host | OK | 0 | — |
| 9 | `semio-s-plugin-stdio` | codec host | OK | 0 | — |
| 10 | `semio-s-plugin-flow-extension-bim` | extension | OK | 0 | — |
| 11 | `semio-s-plugin-flow-extension-brep` | extension | OK | 0 | — |
| 12 | `semio-s-plugin-flow-extension-dictionary` | extension | OK | 0 | — |
| 13 | `semio-s-plugin-flow-extension-draw` | extension | OK | 0 | — |
| 14 | `semio-s-plugin-flow-extension-list` | extension | OK | 0 | — |
| 15 | `semio-s-plugin-flow-extension-logic` | extension | OK | 0 | — |
| 16 | `semio-s-plugin-flow-extension-math` | extension | OK | 0 | — |
| 17 | `semio-s-plugin-flow-extension-primitive` | extension | OK | 0 | — |
| 18 | `semio-s-plugin-flow-extension-text` | extension | OK | 0 | — |
| 19 | `semio-s-plugin-process-concrete` | extension | OK | 0 | — |
| 20 | `semio-s-plugin-process-metal` | extension | OK | 0 | — |
| 21 | `semio-s-plugin-process-robotic` | extension | OK | 0 | — |
| 22 | `semio-s-plugin-process-wood` | extension | OK | 0 | — |
| 23 | `semio-s-plugin-sourcing-beams` | extension | OK | 0 | — |
| 24 | `semio-s-plugin-sourcing-slabs` | extension | OK | 0 | — |
| 25 | `semio-s-plugin-sourcing-windows` | extension | OK | 0 | — |
| 26 | `semio-s-plugin-cad-aec-building` | extension | OK | 0 | — |
| 27 | `semio-s-plugin-cad-aec-building-energy` | extension | OK | 0 | — |
| 28 | `semio-s-plugin-cad-aec-building-structure` | extension | OK | 0 | — |
| 29 | `semio-s-plugin-cad-spatial-shape` | extension | OK | 0 | — |

Extension rows **not** in the demonstrator closure and therefore not checked:
`imperative-extension-{control,effect,logic,math,text}`, `playbook-module-procedural` — their hosts
(`imperative`, `playbook`) are not demonstrator panes.

## 🐞️ The one real break: demonstrator surface re-exports (13 errors, all one cause)

`✏️s/🔌️plugins/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🦀️.rs:14-21` imports each pane's app type
through its host plugin crate:

```rust
use cad::editor::cad::{create_cad_app, CadPlayApp};
use gis::editor::gis2d::{create_gis2d_app, Gis2dPlayApp};
use procedural::editor::generation3d::{create_generation3d_app, Generation3dPlayApp};
use process::editor::process3d::{create_process3d_app, Process3dPlayApp};
use process::viewer::process3d::{create_process3d_viewer, Process3dViewer};
use puzzle::editor::puzzle3d::{create_puzzle3d_app, Puzzle3dPlayApp};
use sourcing::editor::sourcing::{create_sourcing_curation_app, SourcingCurationApp};
use sourcing::viewer::sourcing::{create_sourcing_viewer, SourcingViewer};
```

The file's own header comment states the contract: *"module path read off `cad`'s OWN
`📦️packages/🦀️rust/🦀️.rs` `pub mod` nesting"*. Two halves of that contract had drifted:

* **`cad` / `process` / `sourcing`** did declare the surface re-export modules, but as **private**
  `mod editor` / `mod viewer` → `error[E0603]: module 'editor' is private` (10 errors).
* **`gis` / `procedural` / `puzzle`** never had those modules at all; their lib roots name the
  artifact crates directly inside `dyn_enum_close!` → `error[E0433]: cannot find 'editor' in 'gis'`
  (3 errors).

Not a live peer refactor: the manifest was last touched `025ec86a42 2026-09-08 20:58:18`, the
cad/process/sourcing lib roots `599a5d8450 2026-09-09 07:58:58` — a week-old drift, safe to repair.
Fix is additive visibility only, no design change: every host plugin crate exposes its surface
re-export module the way the demonstrator already expects, in the exact shape `cad`/`process`/
`sourcing` already used.

### ✏️ Edits made (6 files, 6 effective lines + doc comments)

| File | Line | Before | After |
|---|---|---|---|
| `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/🦀️.rs` | 27 | `mod editor {` | `pub mod editor {` |
| `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/🦀️.rs` | 33 | `mod viewer {` | `pub mod viewer {` |
| `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/🦀️.rs` | 33 | `mod editor {` | `pub mod editor {` |
| `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/🦀️.rs` | 39 | `mod viewer {` | `pub mod viewer {` |
| `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/🦀️.rs` | 33 | `mod editor { pub use semio_s_artifact_sourcing_curation::editor::*; }` | `pub mod editor { … }` |
| `✏️s/🔌️plugins/🪵️sourcing/📦️packages/🦀️rust/🦀️.rs` | 37 | `mod viewer { pub use semio_s_artifact_sourcing_curation::viewer::*; }` | `pub mod viewer { … }` |
| `✏️s/🔌️plugins/🌍️gis/🦀️.rs` | 13–19 | *(no surface module)* | added `//#region ✏️Editor` + `pub mod editor { pub use semio_s_artifact_gis_gismap::editor::*; }` |
| `✏️s/🔌️plugins/🌀️procedural/🦀️.rs` | 10–17 | *(no surface module)* | added `//#region ✏️Editor` + `pub mod editor { pub use semio_s_artifact_procedural_generation3d::editor::*; }` |
| `✏️s/🔌️plugins/🧩️puzzle/🦀️.rs` | 11–18 | *(no surface module)* | added `//#region ✏️Editor` + `pub mod editor { pub use semio_s_artifact_puzzle_3d::editor::*; }` |

No `viewer` module was added to `gis`/`procedural`/`puzzle` — the demonstrator only imports their
editors, so nothing beyond the failing import was touched.

## 🖥️ Commands run (all foreground, `cd /Users/ueli/Documents/semio` first)

1. **Baseline, 29 packages, `--keep-going`** → log
   `🗑️generated/cargo-check-wasm-closure.txt` (151 lines).
   `CARGO_PROFILE_WASM_DEV_DEBUG=false RUSTFLAGS=-Awarnings CARGO_TERM_QUIET=true cargo check --target wasm32-wasip2 --profile wasm-dev --keep-going -p … (29 packages)`
   → **did NOT finish** (no `Finished` line): `error: could not compile 'semio-s-plugin-demonstrator' (lib) due to 13 previous errors`. All 28 other packages produced zero diagnostics.
   No `exit status: 69` / Xcode-license linker failures appeared in this run.
2. **Same 29 packages after the fixes**, with `DEVELOPER_DIR=/Library/Developer/CommandLineTools`
   (coordinator's Xcode-27-license workaround) → log
   `🗑️generated/cargo-check-wasm-closure-2.txt`. **Exit 0**, empty log (`CARGO_TERM_QUIET`).
3. **Same, without `CARGO_TERM_QUIET`, to capture proof of completion** → log
   `🗑️generated/cargo-check-wasm-closure-final.txt`. Last line:
   ``Finished `wasm-dev` profile [unoptimized] target(s) in 2.25s`` — **finished, exit 0**.
4. **Demonstrator alone, with its real (default) feature set** — the 29-package run unifies features
   across the workspace and would enable each pane host's `plugin-entry`, which the demonstrator
   switches off via `default-features = false`; this run reproduces the shipped feature set →
   log `🗑️generated/cargo-check-wasm-demonstrator-alone.txt`. Last lines:

   ```
   Checking semio-s-plugin-cad v0.1.0 (…/✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust)
   Checking semio-s-plugin-puzzle v0.1.0 (…/✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust)
   Checking semio-s-plugin-procedural v0.1.0 (…/✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust)
   Checking semio-s-plugin-demonstrator v0.1.0 (…/✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust)
   Finished `wasm-dev` profile [unoptimized] target(s) in 2m 25s
   ```

   **Finished, exit 0.**

## ⚠️ Notes / caveats

* **Check only, no link.** `semio-s-plugin-stdio` type-checks clean, but the known
  1M-function component-linker ceiling / `rust-lld` crash is a *link*-stage hazard and is out of
  scope here — nothing in this report says the closure links, only that it type-checks.
* **Xcode 27 license.** The coordinator's `DEVELOPER_DIR=/Library/Developer/CommandLineTools`
  override was applied from run 2 onward. The pre-fix baseline (run 1) happened to contain no
  `exit status: 69` linker failures, so none of the 13 errors classified above is a toolchain
  artefact — they are all genuine `rustc` resolution errors with full diagnostics.
* **Feature unification.** Checking all 29 packages in one invocation turns on each pane host's
  `plugin-entry` feature; that is the *more* inclusive compile and it passes. Run 4 covers the
  narrower shipped configuration. Both are clean.
* Every edit was re-verified on disk after the last build (no peer overwrote them):
  `grep -n '^pub mod editor|^pub mod viewer'` hits all six files.
