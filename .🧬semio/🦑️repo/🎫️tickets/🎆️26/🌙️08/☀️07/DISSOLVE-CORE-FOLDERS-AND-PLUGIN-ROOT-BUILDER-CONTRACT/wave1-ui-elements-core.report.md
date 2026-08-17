# Wave 1 — UI elements `🫀️core` dissolve

**Ticket:** `26/08/07/DISSOLVE-CORE-FOLDERS-AND-PLUGIN-ROOT-BUILDER-CONTRACT`  
**Scope:** `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🫀️core/` only (no other `🫀️core` trees touched).

## Lifted folders

All 12 concept folders are now direct siblings under `🧱️elements/`:

| Folder | Notes |
|--------|--------|
| `🆔ElementId` | lifted |
| `🌈️Surface` | lifted |
| `🎛️Chrome` | lifted |
| `🏷️ClassNames` | lifted |
| `🏷️Label` | lifted (+ `⌨️component.rs`) |
| `🏷️UiLabel` | lifted |
| `🐚️ShellScope` | lifted |
| `🐹️ElementProps` | lifted |
| `📚️I18n` | lifted |
| `🔌️Ports` | lifted; renamed from `🔌Ports` (VS16 fix) |
| `🚗️UiDriver` | lifted |
| `🧭️Flow` | lifted |

`🧱️elements/🫀️core/` removed (empty after lift).

## Import / path updates (UI tree)

- **67 files** under `🖱️ui/` patched (`🔧️wave1-ui-elements-core.mjs` → `wave1-ui-elements-core.lift-log.json`).
- Element imports: `../🫀️core/<Concept>/` → `../<Concept>/`.
- React barrel `📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`: `../../../../🧱️elements/🫀️core/…` → `../../../../🧱️elements/…`.
- **TUI** `⌨️tui/🦀️component.rs` Label `#[path]`: `…/🫀️core/🏷️Label/⌨️component.rs` → `…/🏷️Label/⌨️component.rs`.
- **Lifted modules** with cross-depth fixes:
  - `🏷️Label/🟦️component.tsx`: `../../🪵Tree/` → `../🪵Tree/`.
  - `🐚️ShellScope/🟦️component.tsx`: `../../../📦️packages/` → `../../📦️packages/`.
- Internal Ports references: `../🔌Ports/` → `../🔌️Ports/`.

## Verification

- `🧱️elements/🫀️core` absent on disk.
- `bunx tsc --noEmit` in `@semio-tech/ui-react`: **no** `Cannot find module` / `🫀️core` path resolution errors (pre-existing unrelated errors remain).
- No `from "…🫀️core…"` import paths left under `🖱️ui/` except unrelated math DSL import in `🖼️assets/🟦️icon_resolver.ts` (`🧮️math/…/🫀️core/…` — different core).

## Deferred

See `deferred-ui-elements.json` (comment-only reference outside `🖱️ui/`).

## Artifacts

- `🔧️wave1-ui-elements-core.mjs` — lift + patch runner (ticket folder).
- `wave1-ui-elements-core.lift-log.json` — machine summary.
