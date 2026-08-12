/** 🧾 `change-data-fields` — whole-field replace for `LayoutSnapshot::data_fields_json`. Semantic replacement for the retired `SetDataFields` generic variant; the `fields:in` workflow port's real, undoable write (see `crate::apps::layout::commands::author::import_media`). */
export interface ChangeDataFields {
  newJson: string | null;
}

/** 🔖️ Semantic descriptor mirror: verb=`change` entity=`data-fields` kind=`change-data-fields` record=`ChangedDataFields`. */
export const ChangeDataFieldsKind = "change-data-fields" as const;
