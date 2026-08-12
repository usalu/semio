/** 🖇️ `create-link` — brings a new {@link ImageLink} into existence in the id-keyed `links` collection. */
export interface CreateLink {
  link: unknown;
  index: number | null;
}

/** 🔖️ Semantic descriptor mirror: verb=`create` entity=`link` kind=`create-link` record=`CreatedLink`. */
export const CreateLinkKind = "create-link" as const;
