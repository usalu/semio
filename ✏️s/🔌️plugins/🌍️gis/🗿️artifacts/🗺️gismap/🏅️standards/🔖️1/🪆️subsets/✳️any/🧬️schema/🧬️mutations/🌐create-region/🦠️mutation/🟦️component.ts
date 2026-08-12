/** 🆕️ `create-region` mutation payload — adds a new region feature to `regions`. */
export interface CreateRegion {
  index: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`create` entity=`region` kind=`create-region` record=`CreatedRegion`. */
export const CreateRegionKind = "create-region" as const;
