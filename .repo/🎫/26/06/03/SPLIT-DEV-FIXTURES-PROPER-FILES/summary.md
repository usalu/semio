# Summary

Split metabolism dev `initialKit` into a canonical store layout under `compose/fixtures/stores/metabolism/wip/initialKit/`:

- `kit.compose.json` (~244 KiB): kit metadata, families, files, qualities, typologies with **shallow** type/design stubs only
- `types/*.type.compose.json` (50 files): full type payloads including representations/connectors
- `designs/*.design.compose.json` (10 files): full design payloads
- `index.compose.json`: id → sidecar path index

Added `assembleSplitInitialKitFromDirectory` / `readInitialKitFixtureFromPath` in `compose/fixtures/script.ts` and matching loaders in C# (`Utility.ReadKitFixtureJson`), Python (`_test_load_json`), Go tests, sketchpad Vite fixture middleware, and `@semio-tech/compose-assets` bootstrap.

Removed the 23 MiB monolithic `kit/dev/metabolism/wip/initialKit/kit.compose.json` and duplicate `types/` / `designs/` trees; representations/icons remain under `kit/dev/metabolism/`.

Dev auto-seed now uses `/fixtures/stores/metabolism/wip/initialKit/kit.compose.json` (assembled at serve/load time).

## Files

- `compose/fixtures/stores/metabolism/wip/initialKit/**` (created)
- `compose/fixtures/script.ts`
- `compose/fixtures/project.json`
- `compose/assets/index.ts`
- `compose/client/lib/sketchpad/js/index.ts`
- `compose/client/lib/sketchpad/js/vite.config.ts`
- `compose/client/lib/net/Compose/Compose.cs`
- `compose/client/lib/net/Compose.Tests/Tests.cs`
- `compose/client/lib/py/main.py`
- `compose/client/lib/go/main_test.go`
- `compose/client/bin/engine/main.py`
- `compose/client/ui/desktop/script.ts`
- Removed: `compose/fixtures/kit/dev/metabolism/wip/initialKit/kit.compose.json`, `types/`, `designs/`
