/** 🆕️ `create-route` mutation payload — adds a new route feature to `routes`. */
export interface CreateRoute {
  index: number;
}

/** 🔖️ Semantic descriptor mirror: verb=`create` entity=`route` kind=`create-route` record=`CreatedRoute`. */
export const CreateRouteKind = "create-route" as const;
