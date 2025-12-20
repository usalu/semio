---
slug: DECOUPLE-YJS-FROM-APP-STORES
prompt: >-
  Decouple yjs from all app stores except Kit. All app stores are still
  entangled with yjs. Only the kit store should use yjs. All other stores should
  use the state machine for state management.
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-20T11:52:27.669Z'
  finished: '2025-12-20T11:54:12.570Z'
summary: >-
  Decoupled yjs from all app stores except Kit. Only KitStore uses yjs for state
  persistence. Design, Type, Quality, Docs, Home, and Feedback apps use
  PlainAppStore/PlainKitDiffAppStore.
commit: 4ff6fd77dee713af972c27bd3761939be4302c80
model: claude-sonnet-4-5
iterations:
  - prompt: 'Removed yjs (yMap, yArray) from all app stores except Kit'
    date:
      started: '2025-12-20T11:52:36.998Z'
      ended: '2025-12-20T11:52:47.415Z'
    model: claude-sonnet-4-5
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 4ff6fd77dee713af972c27bd3761939be4302c80
    files:
      updated:
        - js/js/sketchpad/shared.ts:
            lines:
              added: 3
              removed: 3
        - js/js/sketchpad/Sketchpad.tsx:
            lines:
              added: 34
              removed: 86
        - js/js/sketchpad/Design.tsx:
            lines:
              added: 69
              removed: 88
        - js/js/sketchpad/Quality.tsx:
            lines:
              added: 3
              removed: 3
        - js/js/sketchpad/Docs.tsx:
            lines:
              added: 3
              removed: 3
      created: []
      removed: []
    lines:
      added: 112
      removed: 183
files:
  updated:
    - js/js/sketchpad/Design.tsx:
        lines:
          added: 69
          removed: 88
    - js/js/sketchpad/Docs.tsx:
        lines:
          added: 3
          removed: 3
    - js/js/sketchpad/Quality.tsx:
        lines:
          added: 3
          removed: 3
    - js/js/sketchpad/Sketchpad.tsx:
        lines:
          added: 34
          removed: 86
    - js/js/sketchpad/shared.ts:
        lines:
          added: 3
          removed: 3
  created: []
  removed: []
lines:
  added: 112
  removed: 183
---

# Previously

All app stores (Design, Type, Quality, Docs, Home, Feedback) were entangled with yjs through:

- Factory types requiring `yMap` and `transact` parameters
- Store constructors accepting `yMap` and `transact` parameters
- SketchpadStore creating Y.js maps for each app type

# Plan

1. Update factory types in `shared.ts` to remove yMap/transact for Design/Type/Quality
2. Update factory types in `Sketchpad.tsx` for Home/Docs
3. Update store constructors in Design.tsx, Quality.tsx, Docs.tsx
4. Remove Y.js map creation in SketchpadStore for non-kit apps
5. Clean up unused Y.js types

# Changes

- **shared.ts**: Removed `yMap` and `transact` from `DesignAppStoreFactory`, `TypeAppStoreFactory`, `QualityAppStoreFactory`
- **Sketchpad.tsx**:
  - Removed `yTypeApps`, `yQualityApps`, `yDesignApps`, `yHome` fields
  - Simplified `createTypeApp`, `createQualityApp`, `createDesignApp` to not use Y.js
  - Simplified `deleteTypeApp`, `deleteQualityApp`, `deleteDesignApp` to not use Y.js
  - Removed unused Y.js types (`YDesignApp`, `YTypeApp`, `YQualityApp`, etc.)
- **Design.tsx**: Updated `DesignStore` constructor and factory registration
- **Quality.tsx**: Updated `QualityAppStore` constructor and factory registration
- **Docs.tsx**: Updated `DocsAppStore` constructor and factory registration

Only Kit store (`KitStore` in Kit.tsx) retains Y.js for state persistence via the base `Store` class.
