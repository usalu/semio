# 🔒️ Audit: wasm-bindgen Locks, Demonstrator Wasm Components, Churn

Read-only audit for the mit-bestand demonstrator end-to-end run (`@semio-tech/mit-bestand-demonstrator`). No builds, no git-modifying commands, no file edits except this report. Date of audit: 2026-10-28.

## 1. Summary

- Installed wasm-bindgen CLI is **0.2.126** (`/Users/ueli/.cargo/bin/wasm-bindgen`), wasm-pack is **0.15.0** (stamp `wasm-pack.stamp`, created 2026-10-23).
- Root pin `/Cargo.lock` and every entry in the wasm lock registry are **0.2.129**, so the registry's lock-drift check passes. The mismatch is between the locks (0.2.129) and the installed CLI (0.2.126).
- Blocker (a) is still live, but the cause is the CLI, not the hub lock: `🌎️hub/Cargo.lock` now pins 0.2.129 while the CLI is 0.2.126. `wasm-pack --mode no-install` refuses that pair.
- Blocker (b) is no longer a lock problem: `✏️s/🧑‍💻dev/📐️cad/Cargo.lock` now pins 0.2.129 (the 10/24 note of 0.2.126 is stale).
- Demonstrator wasm chain is 4 explicit wasm projects plus the hub demonstrator composition crate. No demonstrator Cargo.toml uses `default-features = false`. Blockers (c) and (d) were not investigated in this audit.

## 2. Installed Tooling and Registry

| Item | Value | Source |
|---|---|---|
| wasm-bindgen CLI (installed) | 0.2.126 | `wasm-bindgen --version` in `/Users/ueli/.cargo/bin/` |
| wasm-pack | 0.15.0 | `/Users/ueli/.cargo/bin/wasm-pack.stamp` |
| Root pin | 0.2.129 | `/Users/ueli/Documents/semio/Cargo.lock` |
| Registry wasm locks | 6 entries, all 0.2.129 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🕸️wasm-bindgen/🔣️.json` |
| Registry enforcement | Every listed lock must equal the root pin; `requireWasmBindgenSourceLockV1` checks the nearest workspace lock per wasm source owner | `…/🦀️cargo/🕸️wasm-bindgen/🟦️.ts` |
| Registry test | `…/🧪️tests/🕸️wasm-bindgen-locks/🟦️.ts` | (exists) |

Registry refusal message (from `🟦️.ts`): `wasm-bindgen.lock-drift`, regenerate with `repo cargo -- update --offline -p wasm-bindgen -p js-sys -p web-sys -p wasm-bindgen-futures`.

### Registry lock status

| Registry lock | wasm-bindgen | Matches root (0.2.129) |
|---|---|---|
| `Cargo.lock` | 0.2.129 | yes (root) |
| `🧰️framework/Cargo.lock` | 0.2.129 | yes |
| `🌎️hub/Cargo.lock` | 0.2.129 | yes |
| `✏️s/Cargo.lock` | 0.2.129 | yes |
| `✏️s/🧑‍💻dev/📐️cad/Cargo.lock` | 0.2.129 | yes (was 0.2.126 on 10/24) |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/Cargo.lock` | 0.2.129 | yes |

### Mismatch against installed CLI (0.2.126)

| Scope | Result |
|---|---|
| Registry locks (6) | All 0.2.129, all differ from CLI 0.2.126 |
| Demonstrator wasm lock (`🌎️hub/Cargo.lock`, used by `semio-hub-puzzle` and `semio-hub-demonstrator`) | 0.2.129 vs CLI 0.2.126: **refused** |
| Fix direction (not run) | Align the CLI to the locked version (`cargo install wasm-bindgen-cli --version 0.2.129 --locked`, or the repo's tool setup), then verify with `repo tools verify`. Do not lower the lock pin: the registry's root pin is the source of truth |

## 3. Cargo.lock Pins (all 292 locks)

Counts (292 Cargo.lock files, excluding node_modules, .cache, target, .🧬semio):

| wasm-bindgen pin | Lock count | Equals CLI 0.2.126 |
|---|---|---|
| 0.2.126 | 121 | yes |
| 0.2.127 | 54 | no |
| 0.2.129 | 37 | no |
| (no wasm-bindgen crate) | 80 | n/a |

Per-lock detail is in section 7. Notable mixed families (wasm-bindgen differs from js-sys/web-sys):

| Lock | wasm-bindgen | js-sys | web-sys |
|---|---|---|---|
| `🎓️teaching/Cargo.lock` | 0.2.126 | 0.3.103 | (none) |
| `✏️s/🧑‍💻dev/🌊️flow/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 |
| `✏️s/🧑‍💻dev/🧩️composition/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 |
| `✏️s/🔌️plugins/📕️norm/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 |
| `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | (none) |

Most 0.2.127 locks are plugin `🏭️bridge` and stdio artifact codec locks, none of them on the demonstrator wasm path.

## 4. Demonstrator Activation: Wasm Components

### 4.1 `📋️project.json` dependsOn (`/Users/ueli/Documents/semio/♻️mit-bestand/🧺️demonstrator/📋️project.json`)

| Target | dependsOn |
|---|---|
| `prepare-prototype` | 9 × `@semio-tech/framework-os-dev:prepare-<app>-react-prototype` (aggregator, aussuchen, bearbeiten, energy, fem3d, generation3d, generator, koordinator, verfolgen); `@semio-tech/framework-surface-rs:wasm`; `@semio-tech/framework-editor-rs:wasm`; `@semio-tech/framework-os-node-graph-rs:wasm`; `@semio-tech/puzzle-plugin:wasm` |
| `build` | `prepare-release`; the same four wasm targets; `semio-framework-os-flow-browser-package:wasm`; `semio-framework-canvas:stage-runtime-assets` |
| `activate-prototype` | `prepare-prototype`; `@semio-tech/mit-bestand-demonstrator:activate-lanes-prototype` |
| `activate-lanes-prototype` | `@semio-tech/framework-os-dev:activate-{energy,fem3d,generator}-react-prototype` |
| `prepare-e2e` | `activate-prototype`; `workspace:deps-browsers` |

Note: the nine `prepare-*-react-prototype` and `activate-*-react-prototype` targets are not defined in any checked-in `📋️project.json` (only the demonstrator project and a compile-closure fixture reference them). They are presumably generated by the framework-os-dev tooling and were not traced further.

### 4.2 Wasm projects and manifests

| Project | Crate path | Executor |
|---|---|---|
| `@semio-tech/puzzle-plugin` | `🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust` (crate `semio-hub-puzzle`) | `repo ./📜️script.ts wasm` |
| `@semio-tech/framework-surface-rs` | `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust` | same |
| `@semio-tech/framework-editor-rs` | `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust` | same |
| `@semio-tech/framework-os-node-graph-rs` | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/📦️packages/🦀️rust` | same |

### 4.3 Demonstrator plugin crate

- `✏️s/🔌️plugins/🎪️demonstrator` has no root `Cargo.toml`. Its manifests are: `🏭️bridge/Cargo.toml` (`semio-demonstrator-mutation-bridge`, native bin, no default-features) and the `🗿️artifacts/🎪️playground` crates (`semio-s-artifact-demonstrator-playground` and subsets). None uses `default-features = false`.
- The wasm bundle for the six demonstrator panes is `🌎️hub/🧩️compositions/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml` (crate `semio-hub-demonstrator`, in the hub workspace, so it resolves against `🌎️hub/Cargo.lock` at 0.2.129). It has **zero** `default-features = false` entries. The compile-closure test (`🧪️tests/🧪️demonstratorcompileclosure/🟦️.ts`) expects this crate, not the plugin folder.

Component crates linked by the demonstrator wasm bundle (name → workspace dependency):

| Component crate | Role |
|---|---|
| `semio-s-artifact-procedural-generation3d` | generator pane |
| `semio-s-artifact-puzzle-3d` | puzzle3d pane |
| `semio-s-artifact-gis-gismap` | aussuchen/gis pane |
| `semio-s-artifact-cad-cad-editor` | cad pane |
| `semio-s-artifact-process-process3d-editor`, `…-process3d-viewer` | process3d pane |
| `semio-s-artifact-sourcing-curation` | sourcing/curation pane |
| `semio-s-artifact-demonstrator-playground` | demonstrator artifact |
| `semio-s-artifact-stdio-csv`, `-json`, `-xlsx`, `-zip` | in-process codecs |
| `semio-framework-plugin`, `semio-framework-os-kernel`, `semio-framework`, `semio-framework-pack`, `semio-framework-schema`, `semio-framework-job`, `semio-framework-dispatch-macros`, `semio-framework-ui-locale`, `semio-framework-value-derive` | framework SDK |

Plus the separately built wasm projects listed in 4.2, plus `semio-framework-os-flow-browser-package:wasm` and `semio-framework-canvas:stage-runtime-assets` from `build`.

## 5. Git Churn Since 2026-10-24

Paths in the brief that do not exist: `🌎️hub/🌀️procedural` and `🧰️framework/🔨️modules/🌊️flow`. The real paths are `✏️s/🔌️plugins/🌀️procedural` and `✏️s/🔌️plugins/🌊️flow`.

| Area | Commits | Unique files | Note |
|---|---|---|---|
| `🌎️hub/🧩️compositions/🧩️puzzle` | 1 | 1 | Blocker (a) crate; little churn |
| `🌎️hub/🧩️compositions/🎪️demonstrator` | 0 | 0 | Untouched since 10/24 |
| `✏️s/🔌️plugins/🧩️puzzle` | 2 | 221 | Heavy |
| `✏️s/🔌️plugins/🌀️procedural` | 2 | 123 | Heavy |
| `✏️s/🔌️plugins/🗄️stdio` | 4 | 2588 | Sweep-scale, mostly generated/fixture churn |
| `✏️s/🔌️plugins/🎪️demonstrator` | 2 | 4 | Light |
| `✏️s/🔌️plugins/🌊️flow` | 2 | 24 | Moderate |
| `✏️s/🔌️plugins/📐️cad` | 2 | 29 | Moderate |
| `✏️s/🧑‍💻dev/📐️cad` | 1 | 1 | Lock bump only (cad to 0.2.129) |
| `✏️s/🧑‍💻dev/🌊️flow` | 0 | 0 | Untouched |

## 6. Findings and Open Items

1. Blocker (a) root cause is the installed CLI at 0.2.126 against locks at 0.2.129. Fix the tool, not the locks.
2. Blocker (b) is resolved in lock terms; the CAD lock is 0.2.129.
3. Registry lock-drift check passes today, so a drift refusal now means a lock regressed below the root pin, not the CLI.
4. Blockers (c) `os-plugin-describe-rs:build` custody and (d) puzzle3d/generation3d compile errors were out of scope and not checked.
5. Stdio churn (2588 files since 10/24) is large enough that it may explain unrelated lock or fixture noise; check before attributing failures.

## 7. Full Per-Lock Pins

See the table appended below (generated from every Cargo.lock in scope, `—` means the crate is absent from that lock).

| Cargo.lock (relative to repo) | wasm-bindgen | js-sys | web-sys | wasm-bindgen-futures |
|---|---|---|---|---|
| `Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `🧰️framework/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `🎓️teaching/Cargo.lock` | 0.2.126 | 0.3.103 | — | — |
| `✏️s/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `🌎️hub/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `✏️s/🧑‍💻dev/🌊️flow/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🧑‍💻dev/📐️cad/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `✏️s/🧑‍💻dev/🧩️composition/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🧑‍💻dev/💡️services/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/📕️norm/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🧩️puzzle/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `✏️s/🔌️plugins/🏗️fem/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `✏️s/🔌️plugins/🔋️energy/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `✏️s/🔌️plugins/🧱️block/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `✏️s/🔌️plugins/🗄️stdio/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `✏️s/🔌️plugins/🀄️wfc/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `🌎️hub/🧩️compositions/💠️lowpoly/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/📜️imperative/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🏙️bim/🏭️bridge/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🪐️space/🫀️catalog-documents/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/💡️reasoning/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/✒️writer/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/🎬️sequence/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/🌿️vcs/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🌀️procedural/🫀️foundation/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/🪐️space/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/🎞️animate/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/🌍️gis/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/🎪️demonstrator/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🕸️dag/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/💠️lowpoly/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🧱️block/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/📖️playbook/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/📏️layout/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/📐️cad/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🔋️energy/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🖍️draw/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🏗️fem/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🎥️shooting/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🏛️architect/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/📋️forms/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/🗒️note/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🪵️sourcing/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📣️effect/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🏗️fem/⚙️engine/🧱️material/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `✏️s/🔌️plugins/🔋️energy/🧩️extensions/🌦️epw/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🧊️mesh/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏭️bridge/📦️packages/🦀️rust/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🎒️archive/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📊️tabular/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📃️document/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📰markup/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔤️part21/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔊️audio/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `🧰️framework/🛍️products/💻️os/🎚️config/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `🧰️framework/🛍️products/💻️os/🧪️testing/🧩️jcoprobe/👽️guest/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing/📦️packages/🦀️rust/Cargo.lock` | 0.2.129 | 0.3.106 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | 0.2.129 | 0.3.106 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️testing/🌊️actor-import/👽️guest/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/📦️packages/🦀️rust/Cargo.lock` | 0.2.129 | 0.3.106 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/📖️reader/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖼️raster/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | 0.2.129 | 0.3.106 | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏭️bridge/📦️packages/🦀️rust/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏭️bridge/📦️packages/🦀️rust/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏭️bridge/📦️packages/🦀️rust/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏭️bridge/📦️packages/🦀️rust/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏭️bridge/📦️packages/🦀️rust/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏭️bridge/📦️packages/🦀️rust/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🏭️generator/🧪️tests/🧰️support/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🔮️oracles/📦️package-boundary-classification/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🔮️oracles/📦️packages/🦀️rust/Cargo.lock` | 0.2.129 | 0.3.106 | — | — |
| `✏️s/🔌️plugins/🌊️flow/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/🏭️process/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/📕️norm/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/📸️remodel/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🔱️trinity/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | 0.4.77 |
| `✏️s/🔌️plugins/🖨️raster/🏭️bridge/Cargo.lock` | 0.2.127 | 0.3.104 | 0.3.104 | — |
| `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧪️tests/🔬️unit/🔮️oracle/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | — |
| `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | — |
| `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/Cargo.lock` | 0.2.126 | 0.3.103 | 0.3.103 | 0.4.76 |
| `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/📦️packages/🦀️rust/Cargo.lock` | 0.2.129 | 0.3.106 | 0.3.106 | 0.4.79 |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🔬️probes/📖️reader/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🔁️codec/📦️packages/🦀️rust/Cargo.lock` | 0.2.127 | 0.3.104 | — | — |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧪️tests/🔬️catalog-json/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/🧩️json/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🪆️graph-query/📦️packages/🦀️rust/Cargo.lock` | (none) | — | — | — |
