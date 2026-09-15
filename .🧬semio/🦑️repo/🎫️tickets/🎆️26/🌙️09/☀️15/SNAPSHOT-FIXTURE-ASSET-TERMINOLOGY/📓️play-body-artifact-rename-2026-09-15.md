# Play Body Key Artifact Rename (2026-09-15)

## Scope

Renamed remaining `*.play.document` panel body keys to `*.play.artifact` across the `✏️s/🔌️plugins` tree.

## Changes

- **String keys**: all `bodyKey` / `body_key` values ending in `.play.document` → `.play.artifact` (manifests, norm app-surface fixtures, demonstrator bundle).
- **Rust constants**: `*_PLAY_BODY_DOCUMENT` / `BODY_DOCUMENT` (play artifact panels) → `*_PLAY_BODY_ARTIFACT` / `BODY_ARTIFACT`.
- **Norm app-surface unit test**: uses `FRAMEWORK_PANEL_TAB_ARTIFACT_ID`, `FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL`, and German `Artefakt`.
- **Hub** (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`): native-openable receipt field `document_schema` → `artifact_schema` to match `native_factory` JSON and `AppIo`.

## Volume

167 files under `✏️s/` updated by mechanical replace.

## Verification

- `bun nx run @semio-tech/framework-rs:check --skip-nx-cache` — passed.
- Full `cargo test -p semio-s-plugin-norm` — blocked by unrelated workspace compile errors (`semio_framework_tool_run` in norm dispatch macro; `semio-framework-os-infinite` fixture field renames in parallel work).
