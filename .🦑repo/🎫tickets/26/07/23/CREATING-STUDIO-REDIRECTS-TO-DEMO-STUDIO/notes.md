# Creating Studio — re-fix after overwrite

Parallel work had reverted `s/plugin` openStudio/`initial_projection` and left createStudio downloading JSON on the default path.

## What was wrong at runtime
1. `createStudio` (default / Meta+N / "file") emitted `DownloadMediaExport` → browser download.
2. Studio app still booted from `demo_studio_projection()` and `openStudio` only navigated, so the UI showed Demo.

## Fix (re-applied)
- Create = catalog write + Navigate only (no download for any kind).
- `openStudio` → resolve catalog/example → `HostEffect::LoadDocument`.
- Empty `initial_projection`.
- Shell already applied LoadDocument / skipped boot example.

## Rebuild
Reload/rebuild the `s` plugin (e.g. `dev:s`) so the WASM picks this up.
