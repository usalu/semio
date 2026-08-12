/** 🗑️ `delete-position` mutation payload — removes a position feature from `positions` by id. */
export interface DeletePosition {
  id: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`delete` entity=`position` kind=`delete-position` record=`DeletedPosition`. */
export const DeletePositionKind = "delete-position" as const;
