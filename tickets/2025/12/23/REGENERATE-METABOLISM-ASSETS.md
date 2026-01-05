---
slug: REGENERATE-METABOLISM-ASSETS
prompt: Port was recently renamed to connector. The assets such as metabolism.zip still have sql tables. Regenerate the assets with the scripts. Make sure all metabolism.zip are updated after regenerating such as in public folders on every run.
summary: Regenerate metabolism.zip assets with connector schema and copy to all public folders
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: 2025-12-23T13:31:57.083Z
  finished: 2025-12-23T13:33:04.225Z
commit: b41e500849192cc526ed0ce105fff7e2a478e3f0
model: composer-1
iterations:
  - prompt: Port was recently renamed to connector. The assets such as metabolism.zip still have sql tables. Regenerate the assets with the scripts. Make sure all metabolism.zip are updated after regenerating such as in public folders on every run.
    model: composer-1
    date:
      started: 2025-12-23T13:32:43.828Z
      ended: 2025-12-23T13:32:51.464Z
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: b41e500849192cc526ed0ce105fff7e2a478e3f0
    bundles:
      "@semio":
        files:
          "":
            sections:
              "": {}
---


# Previously

The `metabolism.zip` asset contained SQL tables with the old "port" naming convention instead of "connector". The regeneration script (`scripts/regenerate-metabolism.tsx`) had import issues trying to use `MetabolismKit` from `@semio/assets`, and only wrote the zip file to `assets/semio/metabolism.zip` without copying it to public folders where it's served for development (`js/js/public/`, `js/sketchpad/public/`, `js/play/public/`).

# Plan

1. Fix the import issue in `scripts/regenerate-metabolism.tsx` by reading the JSON file directly
2. Regenerate `metabolism.zip` with the updated SQL schema using "connector" instead of "port"
3. Update the script to automatically copy the regenerated zip to all public folders
4. Verify the schema uses "connector" and not "port"

# Changes

## Fixed Import Problem

Changed from importing `MetabolismKit` from `@semio/assets` (which had module resolution issues) to reading the JSON file directly:

```typescript
const kitPath = join(__dirname, "..", "assets", "semio", "kit_metabolism.json");
const kitJson = readFileSync(kitPath, "utf-8");
const kit = JSON.parse(kitJson) as Kit;
```

## Added Public Folder Copying

Added automatic copying of the regenerated zip to all public folders:

- `js/js/public/metabolism.zip`
- `js/sketchpad/public/metabolism.zip`
- `js/play/public/metabolism.zip`

The script now:

1. Creates public directories if they don't exist using `mkdirSync` with `recursive: true`
2. Copies the buffer to all three public locations
3. Tracks and displays the number of folders copied

## Improved Feedback

Added `copiedCount` state to show how many public folders were updated in the success message.

## Verification

Verified that:

- The regenerated zip contains a `connector` table (not `port`)
- No old `port` table exists
- All 4 copies (main asset + 3 public folders) are synchronized with the same size and timestamp
