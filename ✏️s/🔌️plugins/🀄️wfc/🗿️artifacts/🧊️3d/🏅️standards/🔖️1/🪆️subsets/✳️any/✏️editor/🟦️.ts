/** ✏️ WFC 3D editor surface — app id, window kinds and the command roster the shell dispatches. */
export const appId = "s.wfc.wfc3d@1/*#editor";
export const modeId = "edit";
export const windowKinds = ["wfc-graph", "wfc-3d-preview"] as const;
export const bodyKeys = { graph: "wfc.graph", preview: "wfc.3d.preview" } as const;
export const commands = [
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
