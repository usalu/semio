/** 🗑️ `delete-link` — removes an {@link ImageLink} by id; inverse recreates it via `create-link`. */
export interface DeleteLink {
  id: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`delete` entity=`link` kind=`delete-link` record=`DeletedLink`. */
export const DeleteLinkKind = "delete-link" as const;
