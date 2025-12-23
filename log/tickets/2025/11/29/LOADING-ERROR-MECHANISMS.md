---
slug: LOADING-ERROR-MECHANISMS
summary: Add loading spinners and not-found pages to Sketchpad
prompt: Add loading spinners and not-found pages to Sketchpad
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.772Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

Initial implementation of loading and error mechanisms for the Sketchpad app.

# Plan

1. Add Spinner, NotFound, and LoadingRow components to elements.tsx
2. Add loading state for kit import in Home.tsx (loading kits tracked in HomeStore)
3. Update Kit.tsx to use NotFound component for kit not found
4. (Pending) Add loading state for file import in Kit.tsx

# Changes

## assets/icons.ts

- Added `LoaderIcon` (Loader2) for loading spinners
- Added `ArrowLeftIcon` for back navigation

## js/js/sketchpad/elements.tsx

- Added `Spinner` component with size variants (small, medium, large)
- Added `NotFound` component for displaying not-found pages with parent navigation
- Added `LoadingRow` component for displaying loading items in tables/lists

## js/js/sketchpad/Home.tsx

- Added `LoadingKit` interface for tracking kits being imported
- Added `loadingKits` to `HomeState` to track importing kits
- Added `addLoadingKit` and `removeLoadingKit` methods to `HomeStore`
- Updated `HomeDropZone` to add/remove loading kit entries during import
- Updated table rows to include loading kits with spinner and disabled state
- Loading kits display a spinner and are disabled until import completes

## js/js/sketchpad/Kit.tsx

- Imported `NotFound` component
- Updated kit not-found handling to use `NotFound` component with link to home
