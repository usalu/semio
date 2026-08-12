/** 🆕️ `create-position` mutation payload — adds a new position feature to `positions`. */
export interface CreatePosition {
  index: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`create` entity=`position` kind=`create-position` record=`CreatedPosition`. */
export const CreatePositionKind = "create-position" as const;
