/** 🔹 `change-cursor` mutation payload — moves the process timeline replay cursor. */
export interface ChangeCursor {
  newResolvedUpTo?: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`cursor` kind=`change-cursor` record=`ChangedCursor`. */
export const ChangeCursorKind = "change-cursor" as const;
