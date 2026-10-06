/** ⚡️ Text facet declaration for `s.wfc.grid2d` mutations — one keyword per variant, in the
 * `KINDS` order the mutation facet declares. */
export type Grid2dMutationText = string;
export const GRID2D_OP_GRAMMAR_ID = "wfc.grid2d.op";
export const GRID2D_OP_KEYWORDS = [
  "change-seed",
  "resize-grid",
  "change-cell-size",
  "change-periodicity",
  "create-tile",
  "delete-tile",
  "change-tile-weight",
  "change-tile-media",
  "create-rule",
  "delete-rule",
  "pin-cell",
  "unpin-cell",
  "mask-cell",
  "unmask-cell",
] as const;
