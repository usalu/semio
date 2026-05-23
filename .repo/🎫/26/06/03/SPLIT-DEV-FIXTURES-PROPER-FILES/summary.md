# Summary

Split metabolism dev `initialKit` into a canonical store layout under `semio/fixtures/stores/metabolism/wip/initialKit/`:

- `kit.semio.json` (~244 KiB): kit metadata, families, files, qualities, typologies with **shallow** type/design stubs only
- `types/*.type.semio.json` (50 files): full type payloads including representations/connectors
- `designs/*.design.semio.json` (10 files): full design payloads
- `index.semio.json`: id → sidecar path index

Added `assembleSplitInitialKitFromDirectory` / `readInitialKitFixtureFromPath` in `semio/fixtures/script.ts` and matching loaders in C# (`Utility.ReadKitFixtureJson`), Python (`_test_load_json`), Go tests, sketchpad Vite fixture middleware, and `@semio/assets` bootstrap.

Removed the 23 MiB monolithic `kit/dev/metabolism/wip/initialKit/kit.semio.json` and duplicate `types/` / `designs/` trees; representations/icons remain under `kit/dev/metabolism/`.

Dev auto-seed now uses `/fixtures/stores/metabolism/wip/initialKit/kit.semio.json` (assembled at serve/load time).

## Files

- `semio/fixtures/stores/metabolism/wip/initialKit/**` (created)
- `semio/fixtures/script.ts`
- `semio/fixtures/project.json`
- `semio/assets/index.ts`
- `semio/client/lib/sketchpad/js/index.ts`
- `semio/client/lib/sketchpad/js/vite.config.ts`
- `semio/client/lib/net/Semio/Semio.cs`
- `semio/client/lib/net/Semio.Tests/Tests.cs`
- `semio/client/lib/py/main.py`
- `semio/client/lib/go/main_test.go`
- `semio/client/bin/engine/main.py`
- `semio/client/ui/desktop/script.ts`
- Removed: `semio/fixtures/kit/dev/metabolism/wip/initialKit/kit.semio.json`, `types/`, `designs/`
