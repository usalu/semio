# Dev Cad React End-to-End (2026-09-15)

## Objective

Restore the **🛠️dev📐️cad⚛️react** pipeline (`SEMIO_RENDERER=react`, port `6020`): WASM plugin build → activate → Vite serve → browser-loadable guest.

## Root cause

`@semio-tech/cad-plugin:component-dev` failed, blocking `activate-cad-react-dev` and `serve-cad-react-dev`.

1. **Syntax error** in `CadDiff`: a dangling `#[state(artifact)]` after the last field prevented struct parsing, so `ToValue` / `FromValue` derives never applied (`error: expected identifier, found }`).
2. **Missing dependency**: `semio-s-plugin-cad` dispatch macros reference `semio_framework_tool_run::ToolRunTraceCursor` but `semio-framework-tool-run` was not in `Cargo.toml` (`E0433`).

## Fixes

| File | Change |
|------|--------|
| `✏️s/🔌️plugins/📐️cad/.../🧬️schema/🔺️diff/🦀️.rs` | Remove orphan `#[state(artifact)]` on `CadDiff` |
| `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml` | Add `semio-framework-tool-run = { workspace = true }` |

## Verification

```bash
bun nx run @semio-tech/cad-plugin:component-dev
bun nx run @semio-tech/cad-plugin:test --skip-nx-cache   # 5 passed
CAD_JS_RENDERER_PLAY_PORT=6020 SEMIO_RENDERER=react \
  bun nx run @semio-tech/framework-os-dev:activate-cad-react-dev
CAD_JS_RENDERER_PLAY_PORT=6020 SEMIO_RENDERER=react \
  bun nx run @semio-tech/framework-os-dev:serve-cad-react-dev
```

| Check | Result |
|-------|--------|
| `activate-cad-react-dev` | Success (1 component activated) |
| Vite on `http://127.0.0.1:6020/` | HTTP 200 |
| `/🔌️plugin-modules/📐️cad/semio_s_plugin_cad_component.js` | HTTP 200 |
| `/🔌️plugin-modules/📐️cad/semio_s_plugin_cad_component.core.wasm` | HTTP 200 |

Launch entry **🛠️dev📐️cad⚛️react** (`bun nx run workspace:dev -- cad` with env above) uses the same activate/watch/serve chain.

## Notes

- Dev page logs many `[stale]` warnings for *other* plugins not in the cad session; expected for a cad-only session.
- A prior `cad-plugin:test` failure was a stale Nx cache; `--skip-nx-cache` shows green.
