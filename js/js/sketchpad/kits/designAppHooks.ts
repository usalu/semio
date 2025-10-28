// #region Header

// designAppHooks.ts

// Re-exports design app hooks for use in kits/store.tsx
// Separate file to avoid circular dependency

// #endregion

export {
  useDesignAppDiff,
  useDesignAppHover,
  useDesignAppIsPieceTransitiveHovered,
  useDesignAppSelection,
  useDesignAppStore,
} from "../apps/design/store";
