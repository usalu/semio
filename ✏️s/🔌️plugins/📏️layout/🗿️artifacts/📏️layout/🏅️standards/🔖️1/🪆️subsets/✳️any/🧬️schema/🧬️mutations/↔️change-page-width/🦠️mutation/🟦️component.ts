/** ↔️ `change-page-width` — sets a page's `width` scalar. */
export interface ChangePageWidth {
  id: string;
  newWidth: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`page-width` kind=`change-page-width` record=`ChangedPageWidth`. */
export const ChangePageWidthKind = "change-page-width" as const;
