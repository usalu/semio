/** ✏️ `rename-layout` — changes the document's identity `name` field. */
export interface RenameLayout {
  newName: string;
}

/** 🔖️ Semantic descriptor mirror: verb=`rename` entity=`layout` kind=`rename-layout` record=`RenamedLayout`. */
export const RenameLayoutKind = "rename-layout" as const;
