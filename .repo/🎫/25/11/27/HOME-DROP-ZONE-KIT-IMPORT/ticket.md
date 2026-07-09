# Ticket

## Todos

# Previously

User requested two features:

1. Home app should support drag and drop of zip files to create and import kits
2. Both home and kit app should NOT import the .compose folder directly - only import FROM it (via kit.db)

# Plan

1. Fix `KitDropZone` in kit.tsx to skip `.compose/` folder files when extracting individual files (case when no kit.db exists)
2. Create `HomeDropZone` component in home.tsx that uses `importKit` from compose.ts to parse dropped zip files
3. Add i18n translations for the new dropzone UI

# Changes

## kit.tsx - KitDropZone fix

Fixed the `handleDrop` function in `KitDropZone` component to skip files in the `.compose/` folder when extracting individual files from a zip that doesn't contain a valid `kit.db`:

```typescript
// Added condition: !path.startsWith(".compose/")
if (!file.dir && !path.startsWith(".compose/")) {
 // Extract file...
}
```

## home.tsx - HomeDropZone component

Added new `HomeDropZone` component that:

- Wraps both mobile and desktop views in the home app
- Handles dragOver/dragLeave/drop events for zip files
- Uses `importKit` from compose.ts to parse dropped zip files (extracts kit data from `.compose/kit.db`)
- Creates a new kit via `createKit` command
- Adds extracted files to the kit store
- Navigates to the newly imported kit

Added imports:

- `importKit` from `../compose`
- `useTranslation` from `react-i18next`

## locales/en.json and de.json

Added translations for the home dropzone UI:

- `compose.sketchpad.app.home.dropzone.label.normal`
- `compose.sketchpad.app.home.dropzone.label.beginner`
- `compose.sketchpad.app.home.dropzone.description.normal`
- `compose.sketchpad.app.home.dropzone.description.beginner`

## Changes

## Log

## Summary

# Summary

> -
