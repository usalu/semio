/** 📐 `update-page-margins` — atomically sets a page's four margin fields together (a facet that's never meaningfully edited one field at a time — a margins dialog writes all four at once). */
export interface UpdatePageMargins {
  id: string;
  top: number;
  right: number;
  bottom: number;
  left: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`update` entity=`page-margins` kind=`update-page-margins` record=`UpdatedPageMargins`. */
export const UpdatePageMarginsKind = "update-page-margins" as const;
