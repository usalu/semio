/** 🗑️ `delete-page` — removes a {@link Page} by id; inverse recreates it via `create-page`. */
export interface DeletePage {
  id: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`delete` entity=`page` kind=`delete-page` record=`DeletedPage`. */
export const DeletePageKind = "delete-page" as const;
