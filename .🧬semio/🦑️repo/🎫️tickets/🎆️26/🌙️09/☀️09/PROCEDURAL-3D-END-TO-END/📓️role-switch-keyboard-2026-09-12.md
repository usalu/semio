# Surface-Role Switching and Keyboard Paths — 2026-09-12

Implements `📓️viewer-eval-chain-2026-09-12.md` §6 items **1–4** (the clean design that lane deliberately
did not build), closing `📓️audit-window-inventory-2026-09-12.md` §4 **P0 item 2** ("no playground control
to open the viewer role") and §4 **P1 item 5** ("no keyboard or menu path to switch modes or reach
viewer") at the source level, with vitest + node-twin + browser proof.

---

## 1. The two defects, restated

### 1.1 `?role=viewer` did nothing on 6018, and `SEMIO_APP_ROLE=viewer` did not either

Two independent breaks, both closed here:

1. **The React dev entry never read the query string.** The wgpu browser boot resolves
   `?plugin=`/`?role=` in `bootDescriptor` (`🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts:44-53`); the React
   entry (`🧑‍💻dev/🟦️.ts`) read only `VITE_SEMIO_APP_ROLE`.
2. **The playground's app pin silently outranked the role.** `🧑‍💻dev/🟦️.ts:20` passes
   `appId = VITE_SEMIO_APP_ID ?? boot.defaultAppId`, and the `generation3d` playground declares
   `app: "s.procedural.generation3d@1/*#editor"`
   (`🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds.ts:54`). `establishPrimarySession`'s pinned branch
   then resolved that id and **never consulted `appRole` at all**. So the two launch rows
   `📓️viewer-eval-chain-2026-09-12.md` §5.2 added (`SEMIO_APP_ROLE=viewer` on 6018 / 6118) could not have
   worked for the React renderer either — that is a second finding of this lane, and it is now fixed.

Even the unpinned branch preferred `defaultAppId` over the role
(`… find(id === defaultAppId) ?? find(role === appRole) ?? apps[0]`), so `?role=viewer` would still have
landed on the editor.

### 1.2 Mode and role switching were mouse-only

`playground.navbar.modes.*` were plain buttons with no chord, no `aria-pressed`, no group `role`, and no
keyboard equivalent anywhere. The role axis had no control at all.

---

## 2. What changed

### 2.1 `?role=` on the React dev entry (deliverable 1)

**New** `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔗️boot-query/🟦️.ts` — the query contract as
named constants plus one pure resolver:

| export | meaning |
|---|---|
| `BOOT_QUERY_PLUGIN_PARAM` = `"plugin"` | the variant the wgpu boot switches its whole plan on; the React server is built for one variant, so the name exists to keep both urls spelled identically |
| `BOOT_QUERY_APP_ROLE_PARAM` = `"role"` | the boot-time surface role |
| `BOOT_QUERY_CAPACITY` = `8192` | the wgpu boot's `LOCATION_SEARCH_CAPACITY`, so both entries refuse the same oversized url |
| `resolveBootQueryAppRole(search, fallback)` | `"viewer"`/`"editor"` only; absent **or** unrecognized falls back |

`🧑‍💻dev/🟦️.ts` now splits the axis in two: `envAppRole` (the per-server default from
`VITE_SEMIO_APP_ROLE`, the `SEMIO_APP_ROLE` projection §5.1 of the viewer lane added) and `appRole`
(per-navigation, `?role=` winning). With no env set the behaviour is byte-identical to the wgpu boot's
`params.get("role") === "viewer" ? "viewer" : "editor"` — asserted by fixture rows
`empty-search-with-no-env-boots-the-editor` / `unrecognized-query-role-falls-back-to-the-env-role`.

The wgpu boot was **not** touched: its `bootDescriptor` is extracted by TypeScript AST and evaluated in a
bare `node:vm` by `🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts:28-36`, so an imported call inside it
would break that oracle. The two entries are kept in agreement by fixture rows instead, and both
modules' doc comments name each other.

### 2.2 Role-aware primary-app resolution

`resolveBootPrimaryAppV1(apps, pinnedAppId, defaultAppId, appRole)` (new, in the owned module §2.3)
replaces the inline resolution at `🏛️ShellHost/🟦️.tsx:3359`. A surface app id is `<dialect>#<role>` by
construction (`surfaceAppId`), so an anchor naming the wrong role is an anchor that named the right
**dialect** — the resolver projects it onto the requested role when the manifest declares that sibling:

- `pinnedAppId` naming no declared app at all → `undefined`, and the caller still throws
  `primary plugin … does not declare pinned app …` (unchanged boot error).
- no sibling for the requested role → keep the anchor (a viewer request on an editor-only artifact is a
  downgrade to the surface that exists, never a dead boot).
- no anchor → first app with the requested role, else `apps[0]` (previous behaviour).
- `appRole === undefined` (an embed that states none) → projection off, anchor untouched.

### 2.3 `switchToPluginApp` + the close ladder (deliverable 2)

**New owned module** `🏛️ShellHost/🔀️surface-switch/🟦️.ts` — no React, no shell imports, the separation
`🛠️ShellHelpers/🧩️contributions` already established, so every law is drivable without the shell's
element graph:

| export | role |
|---|---|
| `RoleSurfaceAppV1` | the two manifest fields role resolution reads |
| `SURFACE_ROLE_ORDER` | `["editor","viewer"]` — focus order |
| `SURFACE_ROLE_CONTROL_IDS` | `playground.navbar.roles.{editor,viewer}` (DOM ids **and** keybinding control ids) |
| `MODE_STEP_CONTROL_IDS` | `ui.shell.mode.{next,previous}` |
| `surfaceRoleAppsV1(apps, dialect)` | both surfaces for one dialect, or `null` — the role group's render gate |
| `roleSwitchTargetV1(apps, dialect, currentRole, requested)` | the app a switch would open, or `null` |
| `resolveBootPrimaryAppV1(...)` | §2.2 |
| `stepModeIdV1(modeIds, activeModeId, step)` | wrapping mode cycle |
| `runSessionAppSwitchV1(ports, request, onRetireFailed?)` | **the ordered switch** |

`switchToManagedApp(appId, viewState?)` is **gone** (no compat alias); `switchToPluginApp(pluginId,
appId, viewState?)` (`🏛️ShellHost/🟦️.tsx:4868`) replaces it and its four `applyShellUri` call sites now
pass `hostConfig.pluginId` explicitly. It resolves the plugin from `loadedPlugins`, and the host-only
bookkeeping (`openSpaceIdRef`/`openInstanceIdRef` reset on the landing app, and the empty studio
`panelJson`) sits behind an explicit `hostMode` guard inside its `seedLayout`/`defaultViewState` ports —
a non-host role switch now produces exactly the view-state shape a non-host boot produces.

**The close ladder.** `retireSessionInstance` (`🏛️ShellHost/🟦️.tsx:4852`) is lifted out of the
shell-unmount teardown effect and runs the same three steps: close every open document session bound to
that instance (which retires its attachment), drain the attachment lane, `destroyApp`.
`runSessionAppSwitchV1` runs it **after `createApp` and before `SET_SESSION`**, so no render ever
observes two live instances of one plugin and the old one-`createApp`-leak-per-switch is gone. A
predecessor that refuses to retire is reported through `onRetireFailed` and never blocks the successor.

Order asserted by fixture (`switch` rows):
`create → retire → publish → seed → refresh`; switching to the already-mounted app creates and retires
nothing (with a `viewState`, it is a publish+refresh republish); an unresolvable target creates nothing;
`created − retired === 0` for every switch that had a predecessor.

#### Every `session.app.role` gate still holds

All of them read the **live session** at call time, so replacing `session.app` with the sibling
`AppDefinition` flips all of them in one step — nothing in this lane reproduces or bypasses a role rule:

| gate | line | reads |
|---|---|---|
| VCS check-in visibility | `🏛️ShellHost/🟦️.tsx:8215` | `canCheckIn(session?.app.role)` |
| VCS check-in row | `:8402-8404` | `!canCheckIn(session.app.role)` |
| space check-in scoping | `:8240` | `liveEntry.session.app.role === "editor"` |
| viewer chrome (`isViewer`) | `:8383` | `session.app.role === "viewer"` |
| mutation-command filtering | `:8626` | `session?.app.role === "viewer"` |
| mutation-command refusal | `:8711` | `session.app.role === "viewer"` |
| viewer mutation-action refusal | `:6174` | `targetSession.app.role === "viewer"` |
| window-action filtering | `:10195` | `filterDefinitionsForRole(…, session.app.role)` |
| `canonicalSurfaceId` document scope | `:2602` | `canonicalSurfaceId(entry.session.app.dialect, entry.session.app.role)` |
| `canonicalSurfaceId` sync target | `:5581` | `canonicalSurfaceId(targetSession.app.dialect, targetSession.app.role)` |
| `canonicalSurfaceId` openArtifact | `:7278` | `canonicalSurfaceId(dialect, role)` |
| surface-role chip | `:9468` | `session.app.role` |

Runtime confirmation that the guest side follows too (`🗑️generated/role-switch/boot-viewer.console.txt`):
after each switch the contributions lane reads the NEW app's document —
`doc "s.procedural.generation3d@1/*#viewer"` → after clicking Editor →
`doc "s.procedural.generation3d@1/*#editor"` → after `⌘️⌥️V` → `…#viewer` again.

### 2.4 Navbar roles group (deliverable 3)

`roleSwitcherElement` (`🏛️ShellHost/🟦️.tsx:8596`, line refs as of this writing), rendered beside `modeSwitcherElement` in the desktop
navbar centre cluster and in the mobile panel's synthetic "App" tab:

```
<ButtonGroup id="playground.navbar.roles" role="group" aria-label="Surface role"|"Oberflächenrolle">
  <button id="playground.navbar.roles.editor" aria-pressed data-state data-role
          aria-keyshortcuts="Control+Alt+E" …>Editor</button>
  <button id="playground.navbar.roles.viewer" … aria-keyshortcuts="Control+Alt+V" …>Viewer</button>
```

- **Render gate**: `sessionRoleApps` (`:8586`) = `surfaceRoleAppsV1(plugin.manifest.apps,
  session.app.dialect)` — `null`, and therefore no group at all, unless the loaded plugin declares BOTH
  surfaces for the OPEN document's dialect. Measured live: the group is present for generation3d, and the
  fixture proves it is absent for generation2d (editor-only), for a single-app plugin, and for an app
  with no dialect.
- **Labels** come from each target `AppDefinition`'s own `LocalizedLabel` through `resolveManifestLabel`
  — `Viewer`/`Betrachter` and `Editor`/`Editor`, authored in the Rust surface builders
  (`🔌️plugin/🦀️.rs:29179,29187`). No shell dictionary, no default language.
- **Icons** are fixed per role (`pencil` / `eye`), deliberately NOT the apps' own `iconId`: both surfaces
  of one artifact carry the same artifact icon, which would make the two buttons indistinguishable.
- **Focus order** editor → viewer (`SURFACE_ROLE_ORDER`).

The mode group got the same accessibility treatment in passing: `role="group"`, `aria-label`
`Mode`/`Modus`, `aria-pressed` per item, and `aria-keyshortcuts` for the two cycling chords on the group
(where they belong — the chords step through the group rather than addressing one button).

### 2.5 Keyboard paths (deliverable 4) — and why they are SHELL verbs, not app keybindings

**Decision: both axes are declared in `SHELL_KEYBINDINGS`, not in `generation3d`'s
`✏️editor/🦀️.rs` beside `mod+z`/`mod+shift+z`.** Three reasons, in order of force:

1. **An app keybinding structurally cannot express either verb.** `handleAppKeydown`
   (`🏛️ShellHost/🟦️.tsx`, the `for (const binding of session.app.keybindings)` loop) resolves
   `binding.action.action` against the **focused window kind's** `actions` and drops the binding when it
   misses (`const definition = actionById.get(...); if (!definition) continue;`). An app keybinding can
   therefore only ever fire a window action that crosses to the guest. Mode switching (`applyModeChange`)
   and role switching (`switchToPluginApp`) are pure shell-state transitions that never reach the guest,
   so a declared `mod+alt+v → switchRole` would be silently swallowed.
2. **Domain-neutral framework, domain-specific extensions.** Declaring them per plugin would duplicate
   the same two verbs in generation3d, assembly, puzzle3d, cad, … for behaviour the shell owns.
3. **The customization mechanism already exists for shell verbs and only for shell verbs.**
   `composeControlKeybindings(keysByActionId, uiKeybindingOverrides)`
   (`🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:176`) merges `SHELL_KEYBINDINGS` → app action
   bindings → user overrides; the **Settings → Keybindings** tree
   (`📌️ChromePanels/🟦️.tsx` `buildSettingsKeybindingsTree`) lists every row of that map with a capture
   button, a reset button and a conflict marker, persisted through `setKeybindingOverride`. Putting the
   ids there is what makes them discoverable and customizable — no second mechanism was invented.

Rows added to `SHELL_KEYBINDINGS`:

| control id | default chord | why that shape |
|---|---|---|
| `playground.navbar.roles.editor` | `mod+alt+e` | `AppRole` is a closed two-value union, so per-button ids are declarable — and because the id doubles as the button's DOM id, `ControlHotkeyBadge` renders the chord badge and `useControlTooltipText` the tooltip for free |
| `playground.navbar.roles.viewer` | `mod+alt+v` | ditto |
| `ui.shell.mode.next` | `mod+alt+arrowright` | positional: mode ids are plugin-authored while `SHELL_KEYBINDINGS` is static, so the group publishes both chords via `aria-keyshortcuts` instead |
| `ui.shell.mode.previous` | `mod+alt+arrowleft` | ditto |

None collides with an existing row (checked against all 23 prior entries, the framework `mod+z`/
`mod+shift+z`/`mod+y` chords, and generation3d's two app bindings).

Bound in `ShellHost` through the shell's own `useActionHotkey(controlId, cb, {preventDefault:true},
deps, {overrides: uiKeybindingOverrides})` (`:7061-7088`), beside the existing `ui.nav.*`/`ui.search.*`
ones, over `applyModeStep` (`:7035`) and `switchToSessionRole` (`:7049`).

**New** `ariaKeyshortcutsText(keys)` in `🖱️ui/🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts` beside
`formatKeybindingShortcut`: rewrites this codebase's chord grammar into the `aria-keyshortcuts` grammar
(space-separated, `+`-joined, `KeyboardEvent.key` spelling, capitalized modifiers, `mod` → `Control`
because ARIA names one concrete chord read verbatim while `formatKeybindingShortcut` stays the
platform-aware VISUAL spelling — the badge shows `⌘️⌥️E` on this Mac while ARIA reads `Control+Alt+E`).

**No Rust change.** Nothing in a guest manifest was touched, so there is no guest keybinding declaration
to test — the decision above is precisely that the guest is the wrong layer.

---

## 3. Tests

### 3.1 Fixture

**New** `🏛️ShellHost/🧫️fixtures/🔀️surface-switch/🔣️.json` — language-neutral, seven sections:
`dialects`, `manifests` (procedural with three surfaces, single-app puzzle, a dialectless app), `boot`
(10 rows), `group` (4), `roleTargets` (4), `switch` (6), `modeSteps` (6), `keybindings` (4) plus
`keybindingOverride`.

### 3.2 Suite

**New** `🧪️tests/🔀️surface-switch/🟦️.ts`, registered in the react package's `engineTestSuites`
(`vitest.config.ts:28`) and behind its own nx target `surface-switch-check` + launch row (§5).

Every row runs at least twice — through the shipped modules and through an **independent in-file oracle**
that computes the same answer differently (`oracleBootApp` rebuilds `<dialect>#<role>` by string
substitution instead of scanning for a sibling; `oracleGroupRoles` counts surfaces per dialect coordinate
instead of looking each role up). Third-party oracles: **ajv** validates the fixture against a schema
declared in the suite; **typescript** extracts the shipped source.

The requested coverage, by row family:

| requirement | rows |
|---|---|
| `?role=viewer` resolves the viewer app | `boot.query-role-viewer-beats-the-playground-app-pin`, `…-editor-keeps-the-editor`, `no-query-falls-back-to-the-env-role`, `unrecognized-…`, `empty-search-…`, `an-explicit-app-pin-is-projected-onto-the-requested-role`, `a-role-with-no-declared-sibling-keeps-the-anchor`, `a-single-app-plugin-keeps-its-only-surface`, `no-anchor-at-all-…`, `an-undeclared-pin-is-a-boot-error` |
| switching retires the previous instance and opens the other app | `switch.a-role-switch-retires-the-predecessor-before-publishing`, `switching-back-retires-the-viewer-instance`, plus three no-op/republish rows and one unresolvable-target row; every row also asserts `created − retired === 0` |
| roles group hidden for single-app plugins | `group.one-surface-for-the-open-dialect-hides-the-group`, `a-single-app-plugin-hides-the-group`, `an-app-without-a-dialect-hides-the-group` |
| keybinding dispatches the switch | each `keybindings` row asserts the declared chord, that `composeControlKeybindings` surfaces it to the Settings registry, its `aria-keyshortcuts` spelling, that `parseOwnedHotkeyChords` + `keyboardEventMatchesOwnedHotkey` match a real jsdom `KeyboardEvent`, and then feeds that match through `roleSwitchTargetV1` → `runSessionAppSwitchV1` and asserts the predecessor was retired. `keybindingOverride` proves a user override replaces the framework chord and is what ARIA republishes. |

### 3.3 The node twin

`nodeTwin()` extracts the SHIPPED source text of `surfaceRoleAppsV1`, `resolveBootPrimaryAppV1`,
`roleSwitchTargetV1`, `stepModeIdV1` and `SURFACE_ROLE_ORDER` by TypeScript AST, transpiles to CommonJS,
and evaluates the whole `boot` + `group` + `roleTargets` + `modeSteps` corpus (24 rows) in a bare
`node -e` process — no vitest, no bundler alias, no jsdom — asserting it answers identically.

### 3.4 Commands run, and their results

```
bun nx run @semio-tech/framework-renderer-react:surface-switch-check
```
→ **1 passed, 0 failed**, with
`[DEBUG] surface-switch boot=10 group=4 roleTargets=4 switch=6 modeSteps=6 keybindings=4 twin=24 PASS`

```
bun nx run @semio-tech/framework-renderer-react:test-long     # the full SEMIO_TEST_LEVEL=long corpus
```
→ **27 test files, 924 tests, 921 passed, 3 failed.** The corpus was 923 tests before this lane
(measured on the same tree) and is 924 after — the one added test is mine and it passes. All three
failures are peer lanes with uncommitted work in flight, reproduced before and after this lane:

| failing test | attribution |
|---|---|
| `🔬️engine-contract > noteShellCommand > buildNoteShellCommandAction …` | **peer.** `buildNoteShellCommandAction` (`🛠️ShellHelpers/🟦️.tsx:335`) now emits `inverseCommandId`/`inverseArgs`; the expectation was not updated. Present in the COMMITTED source (not in any working-tree diff). |
| `🔌️PluginRuntime > surface render ViewModel > binds two instances of one body …` | **peer.** `🔌️PluginRuntime/🟦️.tsx` is uncommitted-modified by the view-context-admission lane. |
| `🔌️PluginRuntime > readAppDocumentPack() …` | **peer.** Same file; the reply now carries an extra `ops` field the expectation does not allow. |

```
bun nx run @semio-tech/ui-react:test-long
```
→ **22 files, 716 tests, 713 passed, 3 failed + 1 unloadable file.** None is this lane's; the one that
touches keybinding code at all was proved pre-existing by removing my four `SHELL_KEYBINDINGS` rows and
re-running it — **identical `1 failed | 6 passed`** with and without them
(`🗑️generated/role-switch/uidialog-with-rows.txt` vs `uidialog-without-rows.txt`). The other three are
`.storybook/🧭️scope-resolution` (`Cannot bundle built-in module "node:sqlite"`),
`📦️react-package-export` (a package-export path expectation), and `UIIntroduction appearance` (a CSS
selector assertion against `🎨️palette.css`).

```
bunx tsc --noEmit -p tsconfig.json        # @semio-tech/framework-renderer-react
```
→ 866 errors, **none in any file this lane touched** (verified by filtering for `surface-switch`,
`ShellHost`, `ShellHelpers`, `keybinding-*`, `boot-query`, `ButtonGroup`). The five remaining ShellHost
errors are the brush-preview and paste-fragment peer lanes' (`brushPreviewJson`, `InteractionState`,
`{action: string}` vs `{args?}`, `applyLeftoverInteractionView`). The repo-wide count moved 868 → 867 →
862 → 866 across four runs purely from concurrent peer edits; the one error this lane introduced
(`AppRole | undefined` at the new resolver's call site) was fixed in run 2.

```
bun nx run @semio-tech/framework-renderer-react:lint      # region/host-contract
bun nx run @semio-tech/ui-react:lint
bun nx run @semio-tech/ui-react:check-chrome-i18n
```
→ all three **pass**.

Raw output: `🗑️generated/role-switch/`.

---

## 4. Runtime proof (browser, 6018)

`🐍️role-switch-probe.mjs` (ticket root) drives Playwright against the live React dev server and writes
`🗑️generated/role-switch/{report.json, *.console.txt, *.png}`. Seven measured states:

| state | role chip | `playground.navbar.roles` pressed | `playground.navbar.modes` pressed |
|---|---|---|---|
| boot `?plugin=generation3d&role=editor` | `editor` | editor=true, viewer=false | edit=true, generate=false |
| boot `?plugin=generation3d&role=viewer` | `viewer` | editor=false, viewer=**true** | *group absent* (viewer has one mode) |
| click `playground.navbar.roles.editor` | `editor` | editor=true | edit=true, generate=false |
| click `playground.navbar.roles.viewer` | `viewer` | viewer=true | *group absent* |
| press `⌘️⌥️E` | `editor` | editor=true | edit=true |
| press `⌘️⌥️→` | `editor` | editor=true | edit=false, **generate=true** |
| press `⌘️⌥️V` | `viewer` | viewer=true | *group absent* |

So **P0 item 2 and P1 item 5 are closed for a real user**: the viewer is reachable by url, by click and
by chord, and mode switching now has a keyboard path.

Group markup measured verbatim:

```
rolesGroup: { role: "group", ariaLabel: "Surface role", items: [
  { id: "playground.navbar.roles.editor", text: "Editor⌘️⌥️E", pressed: "true",  keyshortcuts: "Control+Alt+E" },
  { id: "playground.navbar.roles.viewer", text: "Viewer⌘️⌥️V", pressed: "false", keyshortcuts: "Control+Alt+V" } ] }
modesGroup: { role: "group", ariaLabel: "Mode",
              keyshortcuts: "Control+Alt+ArrowRight Control+Alt+ArrowLeft",
              items: [ {id: "playground.navbar.modes.edit", pressed: "true"},
                       {id: "playground.navbar.modes.generate", pressed: "false"} ] }
```

Guest-side confirmation that the OTHER app instance really mounts (not just the chrome):

```
[DEBUG] contributions document sources {… "opsHead":"doc \"s.procedural.generation3d@1/*#viewer\" …"}
[DEBUG] contributions document sources {… "opsHead":"doc \"s.procedural.generation3d@1/*#editor\" …"}   (after ⌘️⌥️E)
[DEBUG] contributions document sources {… "opsHead":"doc \"s.procedural.generation3d@1/*#viewer\" …"}   (after ⌘️⌥️V)
```

### 4.1 What could NOT be shown, and why — a repo-wide blocker owned by another lane

**`document.querySelectorAll("[data-surface-id]")` is EMPTY in every state, including a plain
`?role=editor` boot.** No window body renders at all on 6018 right now, so
`window:procedural-view-preview` could not be photographed. The cause is **not** this lane:

```
[DEBUG] render failed [unknown] no-code Error: view context rejected at refresh-ui: view context: invalid panel data
[DEBUG] invokeExtension dispatch failed {extensionId: flow-extension-math, capability: evaluate, …}
```

- It reproduces **identically** on `?plugin=generation3d&role=editor`, which is byte-for-byte the
  pre-existing default boot (`🗑️generated/role-switch/boot-editor.console.txt`).
- The message comes from `parseResolvedPluginViewState`
  (`🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:890`), which throws `view context: invalid panel data` when
  `panelJson` **or** `contributionsJson` is not a string or exceeds **65 536 code points**. A non-host
  playground session carries no `panelJson` at all, and `contributionsJson` is always a string
  (`JSON.stringify`), so by elimination the rejected field is `contributionsJson` — built at
  `🏛️ShellHost/🟦️.tsx:4531` from **every loaded plugin's whole manifest** (≈20 plugins for
  generation3d) and injected into the refresh view state at `:4545`. The cap is simply too small for
  that payload.
- The same admission tightening is what fails the three `🔌️PluginRuntime` unit tests above
  (`view context: explicit supported preferences required`, `invalid panel data`), in a file that is
  uncommitted-modified by that peer.

This is a **P0 for the whole playground** — it blocks every window body and every extension round trip on
6018, not just the viewer — and it belongs to the view-context-admission / contributions lane, not here.
It is reported, not worked around.

Screenshots: `🗑️generated/role-switch/{boot-editor,boot-viewer,clicked-editor,clicked-viewer,keyboard-editor,keyboard-mode-next,keyboard-viewer}.png`.

---

## 5. Launch entries

Seed row added to `.vscode/🧩️launch.seed.jsonc`, group `4_gate`, between
`⚖️gate📍️document-opening-scope🌐️shell` (order 411.099845) and `⚖️gate🎥️tutorial-interaction🌐️renderer`
(411.099846), following the neighbours' naming (`⚖️gate<emoji><slug><scope>`) and shape exactly:

| name | order | command |
|---|---|---|
| `⚖️gate🔀️surface-switch🌐️shell` | 411.0998455 | `bun nx run @semio-tech/framework-renderer-react:surface-switch-check` |

`.vscode/launch.json` was regenerated with `bun nx run @semio-tech/plugin-registry:generate`, never
hand-edited; **2483 configurations, 2483 unique names**; a second `generate` produced an identical
sha256 (`9194894c…`) → regeneration-stable. The other three names added by that regeneration
(`🛠️dev🔧️procedural🏙️3d👁️viewer⚛️react`, `…👁️viewer🧊️wgpu🌐️wasm`,
`⚖️gate⏳️async🧱️boxed-fixed-slots-twin`) are peers' seed edits propagating through; they were preserved,
not reverted.

---

## 6. Open items

1. **`view context: invalid panel data` blocks every window body and every extension dispatch on 6018**
   (§4.1). Repo-wide, pre-existing, owned by the view-context-admission/contributions lane. Until it is
   fixed, no lane can produce `data-surface-id`/mesh-count runtime evidence for ANY window — including
   `📓️audit-window-inventory-2026-09-12.md` §4 P0 item 3.
2. The three `framework-renderer-react` and four `ui-react` test failures listed in §3.4 are peers';
   left untouched per the concurrency rule.
3. The viewer launch rows added by `📓️viewer-eval-chain-2026-09-12.md` §5.2 now work for the React
   renderer because of §2.2 — before this lane the react row's `SEMIO_APP_ROLE=viewer` was inert
   (the app pin won). Worth re-reading that report's §5.2 with this correction in mind.
4. Role switching creates a fresh instance each way, so unsaved in-memory surface state does not survive
   a round trip. That is the correct semantics for two different app instances over one artifact, but a
   later lane may want the switch to carry the open document through
   `openDocument`/`retireDocumentAttachment` rather than re-opening it from the example.
5. The 6018 dev server wedged mid-lane (pegged CPU, `goto` timeouts) after the burst of concurrent host
   edits — the known `📓️`/memory pattern. It recovered without a restart; no server was started or
   killed by this lane.

---

## 7. Files created / changed

**Created**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔗️boot-query/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🔀️surface-switch/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧫️fixtures/🔀️surface-switch/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔀️surface-switch/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🐍️role-switch-probe.mjs`

**Changed**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts` — `envAppRole` + query-driven `appRole`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` — `resolveBootPrimaryAppV1` in `establishPrimarySession`; `retireSessionInstance`; `switchToManagedApp` → `switchToPluginApp` (+ four `applyShellUri` call sites); `applyModeStep`; `switchToSessionRole`; four `useActionHotkey` bindings; `sessionRoleApps`; `roleSwitcherElement`; mode-group ARIA; navbar + mobile-panel wiring
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` — `surfaceRoleGroupText`, `appModeGroupText`
- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx` — four `SHELL_KEYBINDINGS` rows + the table's own doc comment explaining the layer decision
- `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts` — `ariaKeyshortcutsText`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` — re-export `ariaKeyshortcutsText`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/{vitest.config.ts,📜️script.ts,📋️project.json}` — the `🔀️surface-switch` suite + `surface-switch-check` target
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json` (regenerated only)
