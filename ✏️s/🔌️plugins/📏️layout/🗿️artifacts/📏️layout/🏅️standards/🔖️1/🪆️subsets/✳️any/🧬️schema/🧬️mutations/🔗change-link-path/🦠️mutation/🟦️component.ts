/** 🔗 `change-link-path` — sets an {@link ImageLink}'s file `path`. */
export interface ChangeLinkPath {
  id: string;
  newPath: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`link-path` kind=`change-link-path` record=`ChangedLinkPath`. */
export const ChangeLinkPathKind = "change-link-path" as const;
