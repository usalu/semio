/** ➕️ `create-frame` — inserts a new {@link Frame} into a page's `frames` list (paint-order significant), optionally registering it on one of the page's layers. */
export interface CreateFrame {
  pageId: string;
  frame: unknown;
  index: number | null;
  layerId: string | null;
}

/** 🔖️ Semantic descriptor mirror: verb=`create` entity=`frame` kind=`create-frame` record=`CreatedFrame`. */
export const CreateFrameKind = "create-frame" as const;
