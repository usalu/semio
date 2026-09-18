// ✏️ The `s.wfc.wfc2d@1/*#editor` surface, as a TypeScript caller sees it: the app id, the two window
// kinds and the command vocabulary. The behaviour is Rust (`🦀️.rs`); this leaf is the identity.

export const WFC_2D_EDITOR_APP = "s.wfc.wfc2d@1/*#editor";
export const WFC_2D_EDIT_MODE = "edit";

/** 🪟️ The two window kinds the edit mode lays out 50/50. `wfc-graph` is shared with `wfc3d`. */
export const WFC_2D_EDITOR_WINDOWS = { graph: "wfc-graph", preview: "wfc-2d-preview" } as const;
export const WFC_2D_EDITOR_BODIES = { graph: "wfc.graph", preview: "wfc.wfc2d.preview" } as const;

/** 🎮️ Every command the editor dispatches: fifteen document verbs plus two per-pane view verbs. */
export const WFC_2D_EDITOR_COMMANDS = [
  "change-seed",
  "create-slot",
  "delete-slot",
  "move-slot",
  "resize-slot",
  "connect-slots",
  "disconnect-slots",
  "pin-slot",
  "unpin-slot",
  "create-tile",
  "delete-tile",
  "change-tile-weight",
  "change-tile-media",
  "create-rule",
  "delete-rule",
  "change-camera",
  "change-active-tile",
] as const;
