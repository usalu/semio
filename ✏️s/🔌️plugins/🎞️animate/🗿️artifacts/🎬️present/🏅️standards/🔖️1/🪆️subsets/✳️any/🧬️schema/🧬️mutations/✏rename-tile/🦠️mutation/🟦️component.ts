/** 🔹 `rename-tile` mutation payload — sets a figure tile crop's display name. */
export interface RenameTile {
  id: string;
  newName: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`rename` entity=`tile` kind=`rename-tile` record=`RenamedTile`. */
export const RenameTileKind = "rename-tile" as const;
