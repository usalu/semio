/** 🔢 `change-frame-columns` — sets a `Frame::Text`'s `columns` count. A no-op on non-text frames. */
export interface ChangeFrameColumns {
  pageId: string;
  frameId: string;
  newColumns: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`frame-columns` kind=`change-frame-columns` record=`ChangedFrameColumns`. */
export const ChangeFrameColumnsKind = "change-frame-columns" as const;
