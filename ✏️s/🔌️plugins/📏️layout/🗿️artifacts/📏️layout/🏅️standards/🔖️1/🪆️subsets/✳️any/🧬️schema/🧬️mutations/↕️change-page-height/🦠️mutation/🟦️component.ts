/** ↕️ `change-page-height` — sets a page's `height` scalar. */
export interface ChangePageHeight {
  id: string;
  newHeight: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`page-height` kind=`change-page-height` record=`ChangedPageHeight`. */
export const ChangePageHeightKind = "change-page-height" as const;
