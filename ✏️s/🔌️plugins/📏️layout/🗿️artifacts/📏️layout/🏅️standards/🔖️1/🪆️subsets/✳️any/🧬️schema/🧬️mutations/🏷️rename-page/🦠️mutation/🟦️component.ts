/** 🏷️ `rename-page` — changes a page's identity `name` field. */
export interface RenamePage {
  id: string;
  newName: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`rename` entity=`page` kind=`rename-page` record=`RenamedPage`. */
export const RenamePageKind = "rename-page" as const;
