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
