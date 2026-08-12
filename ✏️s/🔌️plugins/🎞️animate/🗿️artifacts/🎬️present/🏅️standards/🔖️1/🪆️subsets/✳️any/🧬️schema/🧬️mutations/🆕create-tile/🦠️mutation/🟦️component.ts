/** 🔹 `create-tile` mutation payload — adds a new figure tile crop to tiles. */
export interface CreateTile {
  index: number;
  tile: unknown;
}

/** 🔖️ Semantic descriptor mirror: verb=`create` entity=`tile` kind=`create-tile` record=`CreatedTile`. */
export const CreateTileKind = "create-tile" as const;
