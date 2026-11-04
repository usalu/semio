# 🚪️ T18 Entry Audit (haiku-entry-audit, read-only)

## Architecture

- **Editors.** Stdio editors are per-subset `#editor`/`#viewer` surfaces built from window kits `framework.window.{document,image,media,mesh,table,text,tree}`. Ten hub family plugins publish them: `🌎️hub/🧩️compositions/🗄️stdio` plus `🧩️extensions/{🖼️image,🎵️media,🛠️cad,🏠️bim,🔺️mesh,📘️pdf,💼️office,🧿️semio,🔢️binary}`. The composition generator emits `OnArtifactKind` activations (`🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts:50-51`).
- **Open path.** Every open goes through the single relay `os.open-artifact`, using the explicit dialect coordinate `artifactKind/standard/subset#role`. There is no MIME or extension routing on open; extensions are used only for export naming and natural-file validation (`ShellHelpers/🧾️transfer-checkin-tutorial`).
  - Relay: `🧰️framework/🔨️modules/🛰️backbone/🧭️opening-relay/🟦️.ts`.
  - Resolver: `🧰️framework/🔨️modules/🎠️kernel/📖️opening-resolver/🟦️.ts:111-123`.
  - Rust twin: `💻️os/🔨️modules/🔌️plugin/🖥️host/🧭️routers/🧩️app-router-and-opening-resolver/🦀️.rs:229-242`.
- **Editor precedence.** Explicit plugin+app, then user pin, then owner surface, then first entry, then `surface.unknown-dialect`. There is no fallback viewer.

## Entry Points

| ID | Entry point | Path |
| --- | --- | --- |
| E1 | Space index "open" | `relay_open_artifact` (`🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/📥️ingress/🦀️.rs:64-81`) |
| E2 | "Open with" picker | `os.open-artifact-with` |
| E3 | Document-panel tabs | — |
| E4 | Create artifact | `os.create-space-artifact` → `openReadySpaceArtifactCreation` |
| E5 | Hub-created artifact (wgpu) | — |
| E6 | Open natural file | `openNaturalFileOwnerV1` + `importBytes` |
| E7 | Sync attach | — |

No deep routes for artifacts were found. The boot-query and MCP artifact frames were not traced.

## Defects

- **D1.** wgpu: when no owner app is found or the install fails, the artifact is seeded into the currently mounted session, i.e. the wrong app (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/📨️command-dispatch/📇️directory-and-hub/🦀️.rs:340-368`).
- **D2.** wgpu falls back from a viewer request to the Editor role (`…/🧲️plugins/📨️command-dispatch/🦀️.rs:202`).
- **D3.** React fails silently when the plugin install fails (`🏛️ShellHost/📬️artifact-opening/🔸️open-artifact-app/🟦️.ts:32-33`). Partly fixed by T02 F1.
- **D4.** Pins: the React `setDefaultApp`/`openArtifact` wire calls are inert (`🔌️PluginRuntime/🔗️channel-adapter/🟦️.ts:135-349`). wgpu ignores `default_app_pins`. Host `OpeningPreferences` is never written.
- **D5.** No test opens any of the 40 stdio artifacts; the opening tests use synthetic `s.draw.draw`.

## Startup

- **Documented command.** README:138-163 uses `repo dashboard run playground:s --param renderer=react` (react port 6070, wgpu 6066) and `os-hub:dev` (port 8787). Agents MUST NOT use `repo dashboard`, so the non-dashboard equivalent is owned by T05.
- **Vite.** The vite config (`💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`) uses `S_OS_PORT ?? 6066` with `strictPort`, `SEMIO_RENDERER` defaulting to react, and `SEMIO_PLUGIN` defaulting to framework-empty.
- **Data dir.** `.🧬semio/🔗space/s-dev`.
- **Test harness.** There is no Playwright runner for `s`. `chromium.launch` helpers live in `💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts`.

## Follow-up (T18b, static)

- Non-dashboard start command:
  1. `repo run @semio-tech/framework-os-dev:activate-s-react-prototype`, once.
  2. `repo run @semio-tech/framework-os-dev:dev-s-react-prototype`, which serves `http://127.0.0.1:6070/`.
  3. Optional env: `S_DATA_DIR=<repo>/.🧬semio/🔗space/s-dev`, `S_LOCAL_ONLY=1`, `S_HUB_URL=http://127.0.0.1:8787`.
  4. Hub: `repo run os-hub:dev` (port 8787).
- Dashboard bug: it launches `dev-s-react-dev`, but the generator emits only `-prototype|-release` (`🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/📚️registry-record/🦀️.rs:47` vs `📥️inference/🎮️playground-targets/🟨️.mjs:210-225`). The hub secure suite also passes `serve s react dev`, which `ServeScript` rejects (`🌎️hub/🎛️commands/🏗️build-and-dev/🟦️.ts:285-287`).
- The `s` playground row uses the GIS `devContribution` for its vite config and entry (`✏️s/🧑‍💻dev/🎭️variants/🌍️gis`). All 10 stdio family plugins are in the `s` build inventory.
- No URL deep link can open an artifact (boot-query reads only role and example). MCP `ui.control` cannot open artifacts: the AgentBridge accepts only focusWindow, setPanelPath and setPanelVisible. Browser automation must therefore drive the UI (space index open, create, open natural file).

## Browser Selector Map (T18c, static; full detail in the T18c report)

- **Space artifact rows.** Row id is `artifact:<id>`. The row action button is `[data-table-row-action][aria-label=<action_open>]`. Create: `#s-space-create-artifact` opens the `createArtifact` dialog (fields `name`, `kindChoice`). Progress: `[data-semio-artifact-creation=<phase>]`.
- **Open with.** Panel tab `framework.panel.artifact`. Rows `artifact.openWith.<role>.<plugin>.<app>`.
- **Open file.** The `Open File…` action. The input is transient, so use Playwright `waitForEvent("filechooser")`.
- **Save and sync.** `#s-sync-status`, `[data-semio-sync-status][role=status]`, `[data-semio-checkin-band]`, `#s-checkin-status`.
- **Windows and readiness.** `[data-slot=window][id]`, `[data-slot=window-body]`. Faults: `[data-semio-window-fault]`. Readiness: `[data-semio-os-ready]` / `[data-semio-os-error]`. Notices: `[data-semio-transient-notice][data-notice-code]`.
- **Undo/redo.** `mod+z` / `mod+shift+z`; history items `framework.history.undo` / `framework.history.redo`. Details controls carry `data-ui-node-id` / `data-ui-node-key`.
- **Accessibility defects (routed to T02).** Unnamed form controls are possible. The set-as-default toggle's name changes with state. Error notices are polite (`role=status`). The Close button name is ambiguous. "Loading surface…" has no live region.

## Browser Prerequisites (T17c, static)

- **Not yet provisioned on this host.** No root `node_modules/playwright` and no `ms-playwright` cache. Sequence: `repo tools setup all`, then `repo run workspace:deps-browsers` (runs `playwright install chromium`, a network download). Playwright 1.64.1 per `bun.lock`.
- **System libraries.** Chromium's system libraries install through `sudo -n pacman`, which may be blocked (no passwordless sudo, and SteamOS has a read-only rootfs). This is a potential host blocker; verify.
- **WASM/fonts for the React prototype.** Fonts are embedded via typst-assets. The binaryen, wasm-pack, trunk and wasm-bindgen installs are network fetches.
- **File-URL bug.** `🧑‍💻dev/🔌️vite-plugins/🗄️backbone-storage/🟦️.ts:46,51,79` slices `file://` and `folder://` without percent-decoding, so emoji or space paths break. Routed to T05.
