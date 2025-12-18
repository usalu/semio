---
slug: FIX-INFINITE-LOOP-FOOTER
summary: Fix infinite loop in DesignAppFooter and TypeAppFooter useEffect dependencies
prompt: Fix infinite loop in DesignAppFooter and TypeAppFooter useEffect dependencies
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-16T17:06:07.812Z'
commit: '0000000000000000000000000000000000000000'
iterations: []
---

# Previously

Opening the Design app and Type app after importing a kit in the sketchpad tests caused:

- Very long hang time / infinite loop warning
- Navbar and footer disappearing
- Only the canvas loading
- "[TypeMesh] File URL not available" errors despite files being in the kit

Multiple root causes identified:

1. **Infinite loop in DesignAppFooter**: `useEffect` had callback functions in dependency array that were recreated on every render
2. **Missing file blobs in metabolism.zip**: The zip only contained the SQLite database, not the actual model files
3. **Navigation before files stored**: Home.tsx navigated to kit before storing file blobs
4. **Unmemoized context values**: PanelSectionProvider and FooterItemProvider created new context objects on every render
5. **Noisy warnings**: TypeMesh logged warnings during normal async blob URL loading

# Plan

1. Identify the infinite loop cause in `DesignAppFooter` - DONE
2. Fix by using refs instead of callbacks in the dependency array - DONE
3. Fix file loading order in Home.tsx (store blobs before navigation) - DONE
4. Memoize context values in PanelSectionProvider and FooterItemProvider - DONE
5. Update regen-metabolism.ts to include actual model files in zip - DONE
6. Fix TypeMesh to only warn when blob URL also fails - DONE
7. Verify the fixes work by running tests - DONE

# Changes

## `js/js/sketchpad/Design.tsx`

Removed `isTagSelected` and `getTypesWithTag` callback functions from the `useEffect` dependency array. Instead:

1. Store `types`, `designTypeGuids`, and `selectedModelTags` in refs
2. Keep refs updated via a separate `useEffect`
3. Define helper functions inside the main `useEffect` that read from refs
4. This prevents infinite loop since the functions aren't in the dependency array and the refs are stable

## `js/js/sketchpad/Home.tsx`

Fixed file loading order in both `handleDrop` and `handleFileInputChange`:

- Before: `navigateToKit()` then `storeFileBlobs()`
- After: `storeFileBlobs()` then `navigateToKit()`

This ensures file blobs are available when the Type/Design app tries to load models.

## `js/js/sketchpad/Sketchpad.tsx`

Memoized context values in providers to prevent unnecessary re-renders:

- `PanelSectionProvider`: Added `useMemo` for context value
- `FooterItemProvider`: Added `useMemo` for context value

## `js/js/sketchpad/Type.tsx`

Fixed TypeMesh warning logic:

- Removed noisy warning when `getFileUrl` returns empty (expected for local files)
- Only warn when both direct URL AND blob URL fail
- This prevents false positive warnings during normal async loading

## `scripts/regen-metabolism.ts`

Updated to include actual model files from `examples/metabolism/`:

- Added `collectFiles` function to recursively gather files
- Filter to only include `representations`, `icons`, and `images` folders
- Zip now includes 321 files (48MB) instead of just the database
