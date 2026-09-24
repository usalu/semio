# WG6 — wgpu hub sign-in, spaces browser, hub workspace, connection pill (session 10 takeover)

Worker WG6 (session 10, 2026-09-24). Predecessors: `📓️wg6-wgpu-hub-sign-in-and-spaces.md` (fleet 4, killed
mid-slice) and `📓️wgr-wgpu-hub-and-open-relay.md` (WGr, fleet 5: repair + 112 laws + wasm32 green + partial live
journey). This report records only what this session measured on top of them.

Status: **(in progress)**

## 1. Inherited state (measured 2026-09-24 12:38–12:45)

Paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/` unless absolute.

| claim | how measured | result |
|---|---|---|
| the three element targets exist, committed, no uncommitted peer hunks | `git status --short` on `🧱️elements/{🔐️HubSignIn,🏘️SpaceBrowser,🔗️HubConnection,🐚️Shell}` + `🎯️targets/🧊️wgpu` | clean; last auto-commits touching `🔗️HubConnection` 2026-09-20/21/22 |
| item 1 (pure contract, Rust twin, shared wire vectors) is **already landed** | `grep include_str` in the wgpu-unit tests | `🔐️HubSignIn/🧪️tests/🔬️wgpu-unit/🦀️.rs:10` reads `📇️directory/🔐️sign-in/🔣️.json`; `🏘️SpaceBrowser/…:13-14` reads `📇️directory/🏘️spaces/🔣️.json` + `📇️directory/🧬️schema/🦀️.rs` — the same fixtures the TS twins (`📇️directory/{🔐️sign-in,🏘️spaces}/🟦️.ts`) read |
| item 2 (wgpu targets) is landed as `UiNode` trees, not paint ops | read `🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs` | `build_hub_workspace_ui` = `UiNode::{Stack,Button,Input,Text}` only, so the a11y DOM mirror projects it (WG6 §2's defended deviation from the `🛂️SpaceAdministration` paint-op shape — kept; see §2) |
| item 3 (mounts) is landed | `grep` in `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` | footer pill `s-hub-connection` (`:23577`), becomes `framework.hub.signIn` while signed out (`:25171`); `/hub` in `apply_shell_uri` (`:12239`) sets `hub_workspace_open`; `os.openHub` (`:20262`); overlay paint phase `ShellChromeFramePhase::HubWorkspace` → `render_hub_workspace_step` (`:25919`, a centered glass card + scrim, the React `/hub` overlay twin); panel body `:20511`; lane `//#region 🔐️HubWorkspaceLane` (`:10989`) |
| a peer already pins the `/hub` route in a shell law | `🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs` | `hub_command_opens_the_route_overlay_without_adding_a_default_dock_tab` + 3 fold laws over `🧫️fixtures/🔗️hub-projection/🔣️.json` |
| WGr's 7 extra `🔗️HubConnection` laws were never executed | WGr §5b | still unrun at takeover |
| no hub listening | `lsof -nP -iTCP -sTCP:LISTEN` 7600–7899 | none at 12:38 |
| shared cargo cache was wiped | `ls -la .🧬semio/🦑️repo/⚡️cache/cargo` | `build/` + `target/` re-created 12:37 — every build is cold; the renderer wasm `dist/` is gone |
| the staged hub binary is broken | `os-hub:live-sign-in-check` recipe on 7891 with `📦️packages/🦀️rust/dist/build-dev/os-hub` (2026-09-23 19:37) | panics at boot: `Overlapping method route. Handler for GET /scopes/{scope}/document/ws already exists` (`🏗️bootstrap/🦀️.rs:10709`) — reported, not mine to fix (H2/H4 own hub builds). Capture: first run of `wp-wg6/generated/wg6-hub-7891.txt` (overwritten) |


## 2. Landed changes

(filling)

## 3. Tests (measured)

(filling)

## 4. Runtime evidence

(filling)

## 5. Measured vs tested vs unverified

(filling)

## 6. Files changed

(filling)
