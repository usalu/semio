---
slug: HOME-DROP-ZONE-KIT-IMPORT
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: >-
  Add drag and drop of zip files to home app and fix kit app to not import
  .semio folder
model: claude-sonnet-4.5
input: []
commit: unknown
files: {}
lines:
  added: 0
  removed: 0
---

# Previously

User requested two features:

1. Home app should support drag and drop of zip files to create and import kits
2. Both home and kit app should NOT import the .semio folder directly - only import FROM it (via kit.db)

# Plan

1. Fix `KitDropZone` in kit.tsx to skip `.semio/` folder files when extracting individual files (case when no kit.db exists)
2. Create `HomeDropZone` component in home.tsx that uses `importKit` from semio.ts to parse dropped zip files
3. Add i18n translations for the new dropzone UI

# Changes

## kit.tsx - KitDropZone fix

Fixed the `handleDrop` function in `KitDropZone` component to skip files in the `.semio/` folder when extracting individual files from a zip that doesn't contain a valid `kit.db`:

```typescript
// Added condition: !path.startsWith(".semio/")
if (!file.dir && !path.startsWith(".semio/")) {
  // Extract file...
}
```

## home.tsx - HomeDropZone component

Added new `HomeDropZone` component that:

- Wraps both mobile and desktop views in the home app
- Handles dragOver/dragLeave/drop events for zip files
- Uses `importKit` from semio.ts to parse dropped zip files (extracts kit data from `.semio/kit.db`)
- Creates a new kit via `createKit` command
- Adds extracted files to the kit store
- Navigates to the newly imported kit

Added imports:

- `importKit` from `../semio`
- `useTranslation` from `react-i18next`

## locales/en.json and de.json

Added translations for the home dropzone UI:

- `semio.sketchpad.app.home.dropzone.label.normal`
- `semio.sketchpad.app.home.dropzone.label.beginner`
- `semio.sketchpad.app.home.dropzone.description.normal`
- `semio.sketchpad.app.home.dropzone.description.beginner`
