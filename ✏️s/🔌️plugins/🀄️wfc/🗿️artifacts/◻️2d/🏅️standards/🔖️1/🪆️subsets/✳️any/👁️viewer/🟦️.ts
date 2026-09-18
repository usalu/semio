// 👁️ The `s.wfc.wfc2d@1/*#viewer` surface: one read-only board window and no commands at all.

export const WFC_2D_VIEWER_APP = "s.wfc.wfc2d@1/*#viewer";
export const WFC_2D_VIEW_MODE = "view";
export const WFC_2D_VIEWER_WINDOW = "wfc-2d-board";
export const WFC_2D_VIEWER_BODY = "wfc.wfc2d.board";

/** 👁️ A viewer declares no actions; the constant exists so a caller can assert exactly that. */
export const WFC_2D_VIEWER_COMMANDS: readonly string[] = [];
