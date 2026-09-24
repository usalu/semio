# WG6 — wgpu hub sign-in, spaces browser, hub workspace, connection pill (session 10 takeover)

Worker WG6 (session 10, 2026-09-24). Predecessors: `📓️wg6-wgpu-hub-sign-in-and-spaces.md` (fleet 4, killed
mid-slice) and `📓️wgr-wgpu-hub-and-open-relay.md` (WGr, fleet 5: repair + 112 laws + wasm32 green + partial live
journey). This report records only what this session measured on top of them.

Status: **done** (17:45) — every brief item landed or verified; the only unobserved hop is a painted frame (§4.3).

> **16:29 resume.** Cut by the account session limit at ~12:50; every process this slice started (test
> build pid 50461, hub hold pid 53160/hub 53166) was dead at 16:29, source edits intact. Coordinator
> freeze (stdio, gis, kernel pack/store, framework plugin crates) acknowledged: this slice touches only
> the wgpu renderer crate (`semio-framework-os-renderer-wgpu`: `🧱️elements/**` wgpu targets + tests), so
> nothing here is blocked-by-freeze.

## 1. Inherited state (measured 2026-09-24 12:38–12:45)

Inherited vs re-done (coordinator's 16:54 question): this session read the fleet-4 WG6 report
(`📓️wg6-wgpu-hub-sign-in-and-spaces.md`) and WGr's (`📓️wgr-wgpu-hub-and-open-relay.md`) first and wrote
**no parallel code** — every target, contract twin, shell mount and lane below is theirs (or a peer's) and
was only verified, cleaned and exercised. New code this session: one `SpaceBrowserLabel` row pair, one
live law + two test helpers, one script verb and its registrations (§2). Their `files changed` lists are
all committed (auto-commit 2026-09-20…22), none left uncommitted.

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

Items 1–3 of the brief were already on the tree (§1); this session closed the remaining gaps and proved
the lane live. Design decision kept: the hub workspace stays a retained `UiNode` tree
(`build_hub_workspace_ui`) painted inside a glass-card overlay, NOT a `*_paint_ops` program — a paint
program would be invisible to the accessibility DOM mirror, which walks the retained `UiTree`. The
`🛂️SpaceAdministration` paint-op shape the brief names is therefore deliberately not copied.

| file | change |
|---|---|
| `🧱️elements/🏘️SpaceBrowser/🎯️targets/🧊️wgpu/🦀️.rs` | `SpaceBrowserLabel::Refresh` + en `"Refresh spaces"` / de `"Spaces aktualisieren"` (React's own strings, `🔗️HubConnection/🟦️.tsx:167/307`) — the exhaustive two-column match now owns the last hub label |
| `🧱️elements/🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs` | the refresh button reads that label instead of an inline `match locale` literal; the inline `// 🏠️` comment inside `hub_sign_in_section` moved into its docstring (AGENTS: no comments inside definitions) |
| `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`//#region 🔐️HubWorkspaceLane`) | two inline comments inside `handle_hub_workspace_action` / `run_hub_sign_in_turn` moved into their docstrings; no behaviour change |
| `🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs` | **new law** `a_live_hub_signs_in_and_its_spaces_reach_the_retained_workspace` (+ helpers `hub_verb`, `hub_attribute_values`): the real `ShellState` with its real native `DirectoryTransport` walks `os.openHub` → `hubSetAddress`/`hubAddConnection` → `hubSetEmail`/`hubSetPassword`/`hubSignIn` → `hubSetSpaceName`/`hubCreateSpace` → retained tree (en + de) lists the space via `data-semio-hub-space` → `hubOpenSpace` (roster + `/spaces/<id>` uri) → `hubSignOut`. `#[ignore]`d (needs a live hub; env `SEMIO_HUB_LIVE_ORIGIN/EMAIL/PASSWORD`), same convention as the repo's other live-resource laws |
| `🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts` | new verb **`hub-live-journey-check`** (`HubLiveJourneyCheckScript`): uses the hub named by `SEMIO_HUB_LIVE_ORIGIN/EMAIL/PASSWORD`, otherwise boots a fresh credential-sign-in hub from the staged `os-hub:build-dev` binary (provisioning one principal via `os-hub credential set`), runs the live law with `--exact --ignored --show-output` under the crate's exhaustive budget, then tears the hub down and deletes its data root |
| `🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json` | target `hub-live-journey-check` (`cache: false`) after `normalized-presence-rows-native-check` |
| `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` | row `⚖️gate🔐️hub-auth🧊️wgpu-live-journey` (`4_gate`, order `411.107575`) directly after `⚖️gate🔐️hub-auth🤝️live-sign-in`, identical in both |


## 3. Tests (measured)

All with `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/.tmp-ticket-0918/wp-wg6/target`, `-p semio-framework-os-renderer-wgpu --lib`.

| run | filter | result | capture |
|---|---|---|---|
| 16:29→16:33 | `hub_sign_in::tests space_browser::tests hub_connection::tests hub_projection_workspace_tests` | **78 passed, 0 failed, 1 ignored** (25 + 16 + 33 + 4; the ignored one is the live law) — first execution of WGr's 7 unrun `🔗️HubConnection` laws, all green | `wp-wg6/generated/wg6-unit-tests-2{,.raw}.txt` |
| 16:34 | same + `--include-ignored` against hub 7891 | 1 failed — my own assertion expected `signedIn`, the phase wire spelling is `signed-in`; fixed to `HubSessionPhase::SignedIn.as_str()` | `wg6-live-journey{,.raw}.txt` |
| 16:35→16:36 | same + `--include-ignored` | **79 passed, 0 failed**, 0 warnings in the new law | `wg6-live-journey-3{,.raw}.txt` |

| 16:39→16:40 | same + `--include-ignored`, after the lint cleanup below | **79 passed, 0 failed, 0 ignored; 0 warnings in any hub file** (`🔐️HubSignIn`, `🏘️SpaceBrowser`, `🔗️HubConnection`, `🔗️hub-projection-workspace`) | `wg6-final-native{,.raw}.txt` |

Lint cleanup (all `unnecessary qualification`, exactly the spans rustc named): WGr's
`🔗️HubConnection/🧪️tests/🔬️wgpu-unit/🦀️.rs:331,506-508,540-541`, the peer's
`🔗️hub-projection-workspace/🦀️.rs:43,47`.

**Self-inflicted wasm32 break, found and reverted within 5 min (16:53→16:54).** I had also shortened the
lane's `TransportError::Cancelled` arm in `run_hub_redeem_turn` (`🐚️Shell/…/🦀️.rs:11216`) on rustc's native
lint advice; the `TransportError` import at `:77` is `#[cfg(not(target_arch = "wasm32"))]`, so the first
wasm32 check failed with exactly one `E0433` on that line (`wg6-wasm32-check.raw.txt:2933`). Reverted to
the fully qualified path — the qualification is load-bearing for the browser arm; the native lint on it is
the right trade. Native re-run after the revert: 79 passed, 0 hub-file warnings (`wg6-final-native.txt`,
16:59). HEAD never carried the broken line (`git diff` empty after the revert).


## 4. Runtime evidence

### 4.1 Hub used

No session-10 hub was listening (7600–7899, 12:38 and 16:29). The staged `os-hub:build-dev` binary panics
at boot (§1), so the `os-hub:live-sign-in-check` recipe was run by hand with the newest binary that
boots, **without building semio-hub**:

```
B=.🧬semio/🦑️repo/⚡️cache/hs1/os-hub-7611          # 2026-09-20 23:43, credential sign-in capable
D=/private/tmp/wg6-hub-data-7891                    # fresh, canonical (the recipe's own choice of parent)
printf '<pw>' | OS_HUB_DATA=$D $B credential set --email ada@example.org --display-name "Ada Lovelace"
printf '<pw>' | OS_HUB_DATA=$D $B credential set --email bo@example.org  --display-name "Bo Peep"
OS_HUB_CREDENTIAL_SIGN_IN=1 nohup bun .tmp-ticket-0918/🐍️c8-hub-hold.ts 7891 $D $B > wp-wg6/generated/wg6-hub-7891.txt 2>&1 & disown
```

Port 7891 (the original brief's 7890–7899 band; the coordinator's 16:54 correction to 7900–7909 arrived after the hub had served its purpose, and it is stopped at slice end — §6). Readiness `not-ready` only on `artifactAuthority`
(no catalog in a fresh root) with `authentication.publicSessionIssuance: true` and `directory.ready: true`
— admitted by the hold. `curl POST /auth/sessions` for Ada answered `{"token":"session.v1.…","user_id":…}`
and `GET /directory/spaces` answered `[]` (12:45). Pids: hold 51987, hub 51991 (16:30 boot; the 12:44
pair 53160/53166 died with the cut).

### 4.2 The wgpu shell's hub lane, driven live — **observed**

`a_live_hub_signs_in_and_its_spaces_reach_the_retained_workspace` against 7891, three runs (16:34, 16:35,
16:39) plus once through the new registered verb (§2), all with the real `ShellState` and its real native
`NativeDirectoryTransport` (HTTP pool on the renderer worker pool) — nothing stubbed:

```
wg6-live sign-in phase=signed-in error=None user=Some("01a0d3d2-…") display=Some("Ada Lovelace")
wg6-live spaces phase=ready rows=[("wg6 live 1790260523309", "01a0d3d7-b97c-…", "author"), ("wg6 live 1790260478424", …)]
wg6-live tree locale=En phase=["signed-in"] spaces=["01a0d3d7-b97c-…", "01a0d3d7-0c89-…"]
wg6-live tree locale=De phase=["signed-in"] spaces=["01a0d3d7-b97c-…", "01a0d3d7-0c89-…"]
wg6-live open space=Some("01a0d3d7-b97c-…") members=[("Ada Lovelace", true)] uri=Some("/spaces/01a0d3d7-b97c-…")
test result: ok. 79 passed; 0 failed; 0 ignored
```

What that proves, hop by hop: `os.openHub` → `/hub` opens the overlay (`hub_workspace_open`) with the
pill state `SignedOut`; a typed origin becomes the selected remote connection; credential sign-in mints a
session, the `me` read installs identity + display name, the password draft is empty afterwards, the pill
leaves `SignedOut`; a sealed `create-space` command lands and the authoritative list reloads (`Ready`,
the new row first, access `author`); the retained `UiNode` workspace — the tree the a11y DOM mirror
projects — carries every listed space as `data-semio-hub-space` in **both** en and de; `hubOpenSpace`
loads the roster (creator = owner) and pushes `/spaces/<id>`; sign-out clears rows and session.
Captures: `wg6-live-journey-3.txt`, `wg6-verb-live.txt`, `wg6-final-native.txt`.

### 4.3 Not observed

| claim | status | why |
|---|---|---|
| a **painted** wgpu frame / screenshot of the pill + overlay | **not observed this session** | the renderer wasm `dist/` was wiped with the cargo cache at 12:37; a browser boot needs `@semio-tech/framework-renderer-wgpu:wasm` (a wasm32 build → fleet mutex, held by W1's catalog publishes since 16:36) plus a served plugin variant, and a native window boot (`native run s dev`) needs every `s` plugin guest (W1's work). WGr §5 already observed the pill (`framework.hub.signIn`, keyboard-focusable) and the workspace's 13 mirrored controls in a browser on 2026-09-20; the missing hop there — the sign-in request never leaving the page — is exactly what §4.2 now proves on the native transport, but NOT on the browser `directory-http` door |
| browser `directory-http` door carrying the sign-in live | **tested only** (WGr's `the_browser_door_carries_the_same_mint_route_au3_proved_live`, green in §3) | no browser boot |
| two users collaborating in one space through the wgpu shell | **not observed** | out of this slice's scope (C7/C8) |


## 5. Measured vs tested vs unverified

| claim | status |
|---|---|
| Rust twins of the sign-in / spaces contracts agree with the React twins on the shared fixtures | **tested** — 25 + 16 laws read `📇️directory/{🔐️sign-in,🏘️spaces}/🔣️.json` (+ the directory schema source) |
| footer pill fold (U1 §8), retained workspace tree, draft/republish/browser-door laws | **tested** — 33 `hub_connection::tests` (incl. WGr's 7, first run today) + 3 shell fold laws on `🧫️fixtures/🔗️hub-projection/🔣️.json` |
| `/hub` + `os.openHub` open the overlay without a dock tab | **tested** (peer law) and **observed** inside the live journey |
| wgpu shell signs into a real hub, creates + lists spaces, publishes them in the retained tree (en + de), opens a space with its roster, signs out | **observed** against hub 7891 through the native transport (§4.2), 4 green runs |
| crate compiles for the browser target with every hub surface in it | **measured** (§4.4) |
| en + de strings | **tested** (exhaustive two-column matches; both locales asserted in the live tree) |
| keyboard reachability / DOM-mirror announcement | **by construction + WGr's 2026-09-20 browser observation** (13 focusable, actionable mirrored controls); not re-observed today |
| a painted frame of the pill/overlay, and the browser `directory-http` door carrying a live sign-in | **unverified today** (§4.3) — next step: `bun nx run @semio-tech/framework-renderer-wgpu:wasm` through the mutex, serve a variant on 6400–6409, re-run `🐍️wgr-live-hub-journey.mjs` against a hub on 7900–7909 |
| staged `os-hub` build-dev binary boots | **measured broken** — `Overlapping method route … GET /scopes/{scope}/document/ws` panic; for H2/H4 |

## 6. Files changed

Source (all in `semio-framework-os-renderer-wgpu`; nothing in any frozen crate):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏘️SpaceBrowser/🎯️targets/🧊️wgpu/🦀️.rs` — `SpaceBrowserLabel::Refresh` + en/de rows
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs` — refresh label from the table; inline comment → docstring
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️HubConnection/🧪️tests/🔬️wgpu-unit/🦀️.rs` — 6 lint-only path shortenings
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — two inline comments in the hub lane → docstrings (net; the `:11216` shortening was reverted)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs` — live law + helpers; 2 lint-only path shortenings

Tooling / registration:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📜️script.ts` — `hub-live-journey-check`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json` — target
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` — `⚖️gate🔐️hub-auth🧊️wgpu-live-journey`

Ticket: this report; captures `wp-wg6/generated/wg6-*.txt` (all ≤ 400 KB). Processes started and
stopped by pid: test launchers 50216/50461/51086 (cut or finished), hub holds 50791 (staged binary,
panicked), 53160/53166 (cut), 51987/51991 (killed 17:45); wasm mutex wrappers 56998/75372 (exited).
Removed: `/private/tmp/wg6-hub-data-7891`, `/private/tmp/au3-hub-data-IfKLTv` (my failed first boot),
`wp-wg6/target` (8 KB). No git-modifying command, no worktree, no sub-agent.
