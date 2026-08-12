/** 🏛️ `update-page-columns` — atomically sets a page's column count and gutter together. */
export interface UpdatePageColumns {
  id: string;
  count: number;
  gutter: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`update` entity=`page-columns` kind=`update-page-columns` record=`UpdatedPageColumns`. */
export const UpdatePageColumnsKind = "update-page-columns" as const;
