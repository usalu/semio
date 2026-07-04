# S Parity Gap Update (2026-07-04)

## Pass 6 — ProductShell chrome + dev infra

### Shell chrome (platform renderer parity)
- **UISearch** (Cmd/Ctrl+P) + navbar toggle — fuzzy command palette over panels, windows, keybindings, catalogue spawn, studio commands
- **UIFind** (Cmd/Ctrl+F) + navbar toggle — per-window find via `UIFindProvider`; media graph nodes registered from `FlowCanvasHost`
- **Display panel** — Windows + Layout tabs with named layout save/apply/delete (`NamedLayoutStore` in framework-core)
- **Settings panel** — theme, compact, expertise
- **Browser History API** — `pushState` + `popstate` sync in studio mode (`useUIHistory`)

### Framework core
- `Store`, `StoragePort`, `NamedLayoutStore`, `createBrowserStoragePort`, `mergeNamedLayouts`

### Dev infra fixes
- `framework/product/os/core/js/index.ts` — minimal OS program registration stub (unblocks writer-core import chain)
- Plugin modules moved from `public/plugin-modules` → `plugin-modules` with Vite alias (fixes Vite 7 public import block)

## Pass 5 — Interactive graph + compiled DAG + drill-in sync
- Graph mutations, spawnApp routing, compiled DAG wire DSL, VFS double-click, bidirectional drill-in, parameters panel, footer undo/redo

## Pass 4 — Instance document sync on drill-in
## Pass 2 — FlowCanvas + WriterCanvas
## Pass 1 — Shell layout + catalogue

## Still open (non-blocking for core studio workflows)
- **Presence peers** overlay on flow canvas (`getPresencePeers` — needs remote backbone)
- **Remote backbone sync** for studio documents (old `RemoteOsBackbone`; new uses in-memory store)
- **Window template** drag-drop in display Windows tab
- **Full VCS envelope** materialization on drill-in reverse sync (current: `patchAppSource` inline JSON)
- **E2E browser verification** pending clean dev server restart on updated vite config

## Tests
- `cargo test -p s-plugin`: 19 passed
- `@semio-tech/framework-renderer-react:test`: 10 passed
