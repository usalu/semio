# 🖥️ OS shell chrome live (native Storybook fallback)

Launch entry: `os-shell-frames-native` (`.claude/launch.json`, port 6090). Inline `sh -c`, bun + Storybook dev, `STORYBOOK_SCOPE=ui`, no repo graph, no cargo, no wasm.

## Why this and not the OS playground
The playground needs the repo graph service (crashes) plus wasm plugin components (`ensureBootComponents`, cargo). The `ui` Storybook scope mounts the real React chrome elements (`WindowChrome`, `Window`, `Layout`, `PanelTabBar`, `ContextMenu`, `Canvas`, `Footer`) with the real stylesheet.

## Source change needed
`.storybook/main.ts`: when the `framework` scope is inactive, a pre-plugin stubs unbuilt `@semio-tech/framework-*-rs` / `flow-core` specifiers (Vite dev does not drop the dead `__STORYBOOK_HOST_WASM__` branch in `preview.tsx`, so unbuilt `pkg/` failed import analysis).

## Routes (base `http://127.0.0.1:6090/iframe.html?id=<id>&viewMode=story`; or the `/?path=/story/<id>` manager)
- Window frames and chips (`data-window-frame`, `data-window-frame-chip`; verified 2 frames / 2 chips): `🖱️ui⚛️react-mode--tab-stack`, `…-mode--maximize-stack`, `…-mode--quad-layout`, `…-panel--panel-chrome-hosted-top-middle|bottom-middle|bottom-right`, `…-paneltabbar--chrome-hosted`
- Window: `…-window--default`, `--with-controls`, `--with-utility-bar`, `--with-engagement`
- Dock stacks / panels: `…-providers--panel-dock-story`, `…-layout--default|panel-default`
- Panel tab bars: `…-paneltabbar--panel-variant|mobile-variant`, `…-tabs--default`
- Context menu: `…-contextmenu--default|grouped|controlled` (right-click), `…-canvaspickmenu--default`
- Canvas footer chips: `…-canvas--default`, `…-footer--default`, `…-statuschip--states`

## Not reachable
The real OS shell (`FrameworkOsShell`, `framework/os` scope, `Shell` story, playground) needs built wasm engines and plugin modules (cargo via `repo cargo`); none are built, so the full shell with plugin apps is not served. Navigate via `127.0.0.1` (the browser pane denied `localhost`).
