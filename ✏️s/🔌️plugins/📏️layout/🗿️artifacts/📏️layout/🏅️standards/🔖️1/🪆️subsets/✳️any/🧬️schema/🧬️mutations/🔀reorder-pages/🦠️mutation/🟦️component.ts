/** 🔀 `reorder-pages` — repositions a page within the display-ordered `pages` list (document page sequence, unlike `stories`/`links` which have no display order). */
export interface ReorderPages {
  id: string;
  toIndex: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`reorder` entity=`pages` kind=`reorder-pages` record=`ReorderedPages`. */
export const ReorderPagesKind = "reorder-pages" as const;
