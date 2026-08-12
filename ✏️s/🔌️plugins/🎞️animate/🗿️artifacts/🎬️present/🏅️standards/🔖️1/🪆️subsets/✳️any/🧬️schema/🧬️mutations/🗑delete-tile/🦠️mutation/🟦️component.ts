/** 🔹 `delete-tile` mutation payload — removes a figure tile crop from tiles by id. */
export interface DeleteTile {
  id: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`delete` entity=`tile` kind=`delete-tile` record=`DeletedTile`. */
export const DeleteTileKind = "delete-tile" as const;
