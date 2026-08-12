/** 🌱️ `create-page` — brings a new {@link Page} into existence in the id-keyed `pages` collection. */
export interface CreatePage {
  page: unknown;
  index: number | null;
}

/** 🔖️ Semantic descriptor mirror: verb=`create` entity=`page` kind=`create-page` record=`CreatedPage`. */
export const CreatePageKind = "create-page" as const;
