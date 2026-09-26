/** 🔺️ Sparse En1996Diff — typed field deltas over the EN 1996 snapshot. */
import type { MasonryWall } from "../📸️snapshot/🟦️.ts";

export type En1996Diff = {
  annex?: string;
  masonryClass?: string;
  designSituation?: string;
  storeys?: number;
  walls?: { values: MasonryWall[] };
};
