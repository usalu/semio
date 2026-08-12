/** 🗑️ `delete-region` mutation payload — removes a region feature from `regions` by id. */
export interface DeleteRegion {
  id: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`delete` entity=`region` kind=`delete-region` record=`DeletedRegion`. */
export const DeleteRegionKind = "delete-region" as const;
