/** 👁️ WFC 3D viewer surface — read-only, one window, no mutating commands. */
export const appId = "s.wfc.wfc3d@1/*#viewer";
export const modeId = "view";
export const windowKinds = ["wfc-3d-view"] as const;
export const bodyKeys = { preview: "wfc.3d.view" } as const;
export const commands = ["noop"] as const;
