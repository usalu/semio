/** 🗑️ `delete-route` mutation payload — removes a route feature from `routes` by id. */
export interface DeleteRoute {
  id: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`delete` entity=`route` kind=`delete-route` record=`DeletedRoute`. */
export const DeleteRouteKind = "delete-route" as const;
