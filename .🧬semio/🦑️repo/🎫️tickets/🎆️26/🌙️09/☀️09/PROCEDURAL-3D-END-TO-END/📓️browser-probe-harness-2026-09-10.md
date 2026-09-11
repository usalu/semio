# Browser probe harness (2026-09-10)

Lane: reusable headless verification for `generation3d` on the already-running React serve.
Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`. Script: `🔍️browser-probe.ts` (bun). No cargo / nx / activate. No source edits outside this ticket folder.

## Mechanism

**Playwright `chromium`** (`playwright` / `@playwright/test` ^1.57 already in the repo root `package.json`; browsers present under `~/Library/Caches/ms-playwright/chromium-1234`). Chosen because every prior ticket E2E in this repo uses the same import (`import { chromium } from "playwright"`), so nothing new is installed.

Launch flags (WebGL + WebGPU-capable, same set other tickets use for wgpu):

```
--enable-unsafe-webgpu --enable-webgl --ignore-gpu-blocklist --disable-gpu-sandbox
--use-angle=swiftshader --enable-unsafe-swiftshader --enable-features=Vulkan,UseSkiaRenderer
```

Headless by default; `--headed` for a visible window. Viewport 1440×900.

The probe never starts, restarts, or kills a serve. Default URL is the coordinator's React playground:

`http://127.0.0.1:6018/?plugin=generation3d`

(`?plugin=generation3d` is the contract in `📓️boot-path-audit-2026-09-09.md`. React port 6018, wgpu 6118.)

## Commands

From the ticket folder (quote the emoji path):

```bash
# boot gate (default URL / port 6018)
bun 🔍️browser-probe.ts

# same, explicit
bun 🔍️browser-probe.ts --url=http://127.0.0.1:6018/?plugin=generation3d
bun 🔍️browser-probe.ts --port=6018 --plugin=generation3d

# wgpu later
bun 🔍️browser-probe.ts --url=http://127.0.0.1:6118/?plugin=generation3d
bun 🔍️browser-probe.ts --port=6118 --plugin=generation3d

# scripted interactions
bun 🔍️browser-probe.ts --mode=interact --steps=example,hover,select,orbit --example=box-shell-preview

# labelled observation (writes 🗑️generated/<label>-* plus a run dir)
bun 🔍️browser-probe.ts --label=boot-13-pre --settle=90
```

CLI:

| arg | default | meaning |
|---|---|---|
| `--url=` or positional `http://…` | `http://127.0.0.1:6018/?plugin=generation3d` | full target |
| `--port=` / `--plugin=` | 6018 / generation3d | URL builder when `--url` omitted |
| `--mode=boot\|interact` or `--interact` | boot | interact runs `--steps` after settle |
| `--steps=` | `example,hover,select,orbit` | example picker, hover preview, click-select, orbit drag |
| `--example=` | `box-shell-preview` | picker target (value or label) |
| `--settle=` | 90 | seconds to wait for window bodies |
| `--label=` | `boot-13-pre` | artifact prefix |
| `--timeout=` | 90 | navigation timeout seconds |
| `--headed` | off | visible Chromium |

Exit codes (gate):

- `0` — window **bodies** rendered (`framework.window.*` and/or a canvas). Preview meshes=0 is not fatal.
- `1` — boot fault. Last stdout line is `DIAGNOSIS: …` (also `🗑️generated/<label>-diagnosis.txt`).
- `2` — serve unreachable / navigation failed.
- `3` — probe internal error.

Chrome-only (navbar + dock tabs, empty bodies) is **exit 1**.

## Artifacts (`🗑️generated/`)

Per run dir `probe-<label>-<utc>/`:

| file | contents |
|---|---|
| `console.jsonl` | every console message: `{at, elapsedMs, level, text, location, args}` (near-simultaneous Playwright arg-splits are coalesced) |
| `page-faults.jsonl` | `pageerror` + injected `unhandledrejection` |
| `network.jsonl` | `requestfailed` and HTTP ≥400 |
| `settle.dom.html` | `document.documentElement.outerHTML` |
| `settle.png` | 1440×900 screenshot |
| `settle.snapshot.json` | live DOM census |
| `summary.json` | machine-readable verdict |
| `step-<name>.png` + `.snapshot.json` | after each interact step |

Stable copies for the coordinator (this baseline): `boot-13-pre-summary.json`, `boot-13-pre-console.jsonl`, `boot-13-pre-console-excerpt.txt`, `boot-13-pre-settle.png`, `boot-13-pre-settle.dom.html`, `boot-13-pre-step-example.png`, `boot-13-pre-diagnosis.txt`, `boot-13-pre-probe.log`.

## JSON summary schema (`semio.procedural3d.browser-probe/1`)

```json
{
  "schema": "semio.procedural3d.browser-probe/1",
  "observedAt": "ISO-8601 UTC",
  "wallClockLocal": "Date.toString()",
  "label": "boot-13-pre",
  "url": "http://127.0.0.1:6018/?plugin=generation3d",
  "mode": "boot | interact",
  "ok": false,
  "fatal": true,
  "diagnosis": "one-line gate reason",
  "title": "semio · procedural · 3d",
  "shellRendered": false,
  "chromeRendered": true,
  "windowIds": ["framework.window.proceduralMain"],
  "windowTabs": [{"windowId": "procedural-main", "slot": "mode-dock-tab", "text": "Flow"}],
  "windows": [{"id": "…", "w": 0, "h": 0, "text": "…"}],
  "panels": [{"id": "framework.panel.catalogue", "w": 90, "h": 22, "text": "Catalogue"}],
  "panelTabs": [],
  "canvases": [{"w": 0, "h": 0, "cssW": 0, "cssH": 0}],
  "meshCount": 0,
  "instanceCount": 0,
  "previewHosts": [{"surfaceId": "…", "meshes": 0, "instances": 0, "status": {}, "interaction": {}, "w": 0, "h": 0}],
  "exampleLabel": "Hexagonal Mushroom Column",
  "visibleFaults": [],
  "console": {"total": 0, "errors": 0, "warnings": 0, "faultLike": 0},
  "pageFaults": 0,
  "networkFailures": 0,
  "faultTexts": [],
  "consoleExcerpt": ["level: text"],
  "artifacts": {"dir": "…", "console": "console.jsonl", "settlePng": "settle.png"}
}
```

`meshCount` is `JSON.parse(data-meshes-json).length` on `.semio-world-3d-host` (World3dHost). `windowIds` are the stable `framework.window.*` body ids from `📓️window-kind-actions-2026-09-10.md`. Dock tabs use `data-window-id` and appear **before** bodies mount — the probe records both.

## Baseline observation — boot #13-pre

- **Wall clock:** started `2026-09-10T19:36:49.661Z` (Thu Sep 10 2026 21:36:49 GMT+0200), ended `2026-09-10T19:38:35Z`. Elapsed 105.4 s. `--settle=80 --mode=interact`.
- **Label:** `boot-13-pre` as requested. `📓️status.md` recorded the restage GREEN at **21:29** local (7 minutes earlier). Correlate: this capture may already be on the post-restage wasm under `🔌️plugin/…/dist/dev/🔌️plugin-modules/🌀️procedural/`. The serve was not restarted by this lane.
- **URL:** `http://127.0.0.1:6018/?plugin=generation3d` HTTP 200.
- **Gate:** exit **1**. `DIAGNOSIS: boot fault: chrome rendered but window bodies empty (tabs=procedural-main,procedural-preview; title="semio · procedural · 3d")`

### What the page did

- Title `semio · os` → `semio · procedural · 3d` by ~10 s.
- Navbar + example picker + Edit/Generate + dock tabs **Flow** (`procedural-main`) and **Preview** (`procedural-preview`) painted. Panels present as tabs only: `framework.panel.artifact`, `framework.panel.catalogue`, `framework.panel.inspection`, `framework.panel.history`.
- **No** `framework.window.*` body nodes, **0 canvases**, **0** `.semio-world-3d-host`, **0 meshes**. Both window frames are empty cream rectangles (see `boot-13-pre-settle.png`).
- No pageerrors, no unhandled rejections, no failed network requests, no `SemioFaultError` / trap / `No plugins loaded` / `cabi_realloc` / `Unterminated string`.
- Plugin actor is **alive and busy**: paged `setContributions` + `flowEvalTick` complete for the whole settle window (still going at 105 s). Boot-time `setActiveExample` settled at 6.0 s with `effects: 2`.
- Interact `--example=box-shell-preview`: combobox opened, **9 options**, label changed **Hexagonal Mushroom Column → Box Shell Preview**. Second `setActiveExample` settled at 83.6 s, `effects: 2`. Window bodies still empty — hover / select / orbit failed with `no preview canvas`.

### Verbatim console (coalesced)

From `🗑️generated/boot-13-pre-console-excerpt.txt` / `boot-13-pre-console.jsonl`:

```
[300ms] debug: [vite] connecting...
[322ms] debug: [vite] connected.
[655ms] info: %cDownload the React DevTools for a better development experience: https://react.dev/link/react-devtools font-weight:bold
[1060ms] log: [DEBUG] hot-swap flow-extension-bim {pluginId: flow-extension-bim, version: 0.1.0, addedApps: Array(0), removedApps: Array(0)} [DEBUG] hot-swap flow-extension-brep {pluginId: flow-extension-brep, version: 0.3.0, addedApps: Array(0), removedApps: Array(0)}
[1090ms] log: [DEBUG] hot-swap flow-extension-dictionary … draw … (same shape)
[1111ms] log: [DEBUG] hot-swap flow-extension-math … text … primitive …
[4747ms] warning: [DEBUG] performInvocation {"invocationKind":"action","instanceId":1,"actionId":"setActiveExample"}
[6008ms] warning: [DEBUG] performInvocation settled {"invocationKind":"action","instanceId":1,"actionId":"setActiveExample","frames":2,"frameKinds":["Invocation","Ephemeral"],"historyCursor":null,"historyUpserts":0,"historyCanUndo":null,"effects":2}
[6420ms] warning: [DEBUG] performInvocation settled {"invocationKind":"command","instanceId":1,"actionId":"setContributions",…,"effects":1}
[7191ms] warning: [DEBUG] performInvocation settled {"invocationKind":"command","instanceId":1,"actionId":"flowEvalTick",…,"effects":1}
```

`setContributions` then pages ~every 1.5 s for the rest of the run. At 73.3 s:

```
[73319ms] error: guest linear memory at 322174976 B — 60 % of the 536870912 B budget, past the 60 % install-peak ceiling
```

Interact picker (81.4 s step start):

```
[81968ms] warning: [DEBUG] performInvocation {"invocationKind":"action","instanceId":1,"actionId":"setActiveExample"}
[83605ms] warning: [DEBUG] performInvocation settled {"invocationKind":"action","instanceId":1,"actionId":"setActiveExample","frames":2,"frameKinds":["Invocation","Ephemeral"],…,"effects":2}
```

No `action failed`, no `typed-operation failed`, no `Geometry extension unavailable`. The guest is ticking; the React shell never mounts window bodies / World3dHost.

Flood: `[DEBUG] cooperative-maintenance` and `[DEBUG] plugin_exchange entry` arrive as `console.error` with one Playwright event per argument (thousands of fragments; the probe coalesces within 12 ms). `SEMIO_RUNTIME_DIAGNOSTICS` does not gate these.

## Missing hooks (would make verification stronger)

Do **not** add these from this lane — report only.

1. **`data-boot-phase` / `data-plugin-status` on the shell** — chrome-up vs bodies-up vs trapped vs `No plugins loaded` is currently inferred from title + tabs + canvases. A single attribute (`chrome | bodies | faulted`) would make the gate deterministic under 5 s.
2. **`framework.window.*` body ids are absent until the body mounts.** Dock tabs already have `data-window-id="procedural-main|procedural-preview"`. Giving the empty frame the same stable id (even before content) would match `📓️window-kind-actions-2026-09-10.md` and let the probe wait on `#framework.window.proceduralPreview`.
3. **No stable example-picker id** — only `[role="combobox"]` / `[data-slot="select-trigger"]`. An id such as `framework.navbar.example` would survive locale/label changes.
4. **`data-meshes-json` / `data-status-json` exist only after World3dHost mounts.** A preview-status chip in the empty frame (`phase=idle|computing|faulted`, `meshes=N`) would let the probe report eval progress while the canvas is missing.
5. **Guest-memory / contributions progress is console-only.** The 60 % install-peak line is the only structured signal that paged `setContributions` is the long pole. `data-contributions-page` / `data-guest-memory-bytes` on the shell would turn that into a census field.
6. **DEBUG still uses `console.error`.** Coalescing helps, but a `data-semio-probe=1` query flag that mirrors `SEMIO_RUNTIME_DIAGNOSTICS` and also silences `cooperative-maintenance` / `plugin_exchange` would keep JSONL small enough to diff.

## First-run note

A 55 s boot-only pass at `19:33:47Z` saw the same chrome-empty-bodies state (title flipped, 0 windows). That run's 1.4 MB fragmented console dir was deleted; the 80 s interact pass is the kept baseline.
