// #region 🧲Header
// Mirrors semio/rs `change_command` + `read_command` variant names (camelCase JSON).
// Used for dropdown presets; "raw JSON" allows any nested command list.
// #endregion

/** One JSON object for a single `ChangeKitCommand` (serde **externally tagged**, camelCase variant keys). */
export interface ChangeKitPreset {
  readonly id: string;
  readonly label: string;
  /** Full `ChangeKitCommand` value as JSON object (not wrapped in array). */
  readonly json: string;
}

/** One `ReadKitCommand` JSON value. */
export interface ReadKitPreset {
  readonly id: string;
  readonly label: string;
  readonly json: string;
}

function j(obj: unknown): string {
  return JSON.stringify(obj, null, 2);
}

/** Presets for common **root** `ChangeKitCommand` variants. Replace PLACEHOLDER_* before run. */
export const CHANGE_KIT_PRESETS: readonly ChangeKitPreset[] = [
  { id: "ck-name", label: "Kit: name", json: j({ name: { name: "Kit name (story)" } }) },
  { id: "ck-desc", label: "Kit: description", json: j({ description: { description: "story description" } }) },
  { id: "ck-icon", label: "Kit: icon", json: j({ icon: { icon: "icon-url" } }) },
  { id: "ck-version", label: "Kit: version", json: j({ version: { version: "0.0.0-story" } }) },
  {
    id: "ck-replaceKit",
    label: "Kit: replaceKitFromFullDto (placeholder — replace `dto` with a full `KitFullDto` JSON)",
    json: j({
      replaceKitFromFullDto: {
        dto: { id: "PLACEHOLDER_KIT_ID", name: "replaced" },
      },
    }),
  },
  {
    id: "ck-changeType-name",
    label: "Nested: changeTypeCommands (name)",
    json: j({
      changeTypeCommands: {
        typeId: { id: "PLACEHOLDER_TYPE_ID" },
        commands: [{ name: { name: "Renamed type" } }],
      },
    }),
  },
  {
    id: "ck-changeDesign-name",
    label: "Nested: changeDesignCommands (name)",
    json: j({
      changeDesignCommands: {
        designId: { id: "PLACEHOLDER_DESIGN_ID" },
        commands: [{ name: { name: "Renamed design" } }],
      },
    }),
  },
  {
    id: "ck-changeFile-url",
    label: "Nested: changeFileCommands (url)",
    json: j({
      changeFileCommands: {
        fileId: { id: "PLACEHOLDER_FILE_ID" },
        commands: [{ url: { url: "https://example.com/file" } }],
      },
    }),
  },
  {
    id: "ck-changeFolder-path",
    label: "Nested: changeFolderCommands (path)",
    json: j({
      changeFolderCommands: {
        folderId: { id: "PLACEHOLDER_FOLDER_ID" },
        commands: [{ path: { path: "/story/folder" } }],
      },
    }),
  },
];

/**
 * Flat index of **all** root `ChangeKitCommand` variant keys from
 * [semio/rs/lib.rs](semio/rs/lib.rs) `ChangeKitCommand` (serde camelCase field names).
 * Use with raw JSON editor; invalid / unwired variants surface as `InvalidOperation` in the UI.
 */
export const ALL_CHANGE_KIT_ROOT_KEYS = [
  "replaceKitFromFullDto",
  "name",
  "description",
  "icon",
  "image",
  "preview",
  "version",
  "remote",
  "homepage",
  "license",
  "uri",
  "created",
  "updated",
  "addType",
  "removeType",
  "addDesign",
  "removeDesign",
  "addFile",
  "removeFile",
  "addFolder",
  "removeFolder",
  "addAuthor",
  "removeAuthor",
  "addConcept",
  "removeConcept",
  "addTag",
  "removeTag",
  "addQuality",
  "removeQuality",
  "addKitProp",
  "removeKitProp",
  "addKitAttribute",
  "removeKitAttribute",
  "changeFileCommands",
  "changeFolderCommands",
  "changeAuthorCommands",
  "changeConceptCommands",
  "changeTagCommands",
  "changeKitQualityCommands",
  "changeTypeCommands",
  "changeDesignCommands",
] as const;

export const CHANGE_TYPE_COMMAND_KEYS = [
  "name",
  "description",
  "icon",
  "image",
  "variant",
  "stock",
  "typeVirtual",
  "unit",
  "location",
  "created",
  "updated",
  "addPort",
  "removePort",
  "changePortCommands",
  "addConnector",
  "removeConnector",
  "changeConnectorCommands",
  "addRepresentation",
  "removeRepresentation",
  "changeRepresentationCommands",
  "addTypeAuthor",
  "removeTypeAuthor",
  "addTypeConcept",
  "removeTypeConcept",
  "addTypeTag",
  "removeTypeTag",
  "addTypeQuality",
  "removeTypeQuality",
  "addTypeProp",
  "removeTypeProp",
  "addTypeAttribute",
  "removeTypeAttribute",
] as const;

export const READ_KIT_PRESETS: readonly ReadKitPreset[] = [
  { id: "rk-everything", label: "Read: everything {}", json: j({ everything: {} }) },
  { id: "rk-name", label: "Read: name", json: j({ name: null }) },
  { id: "rk-types", label: "Read: types", json: j({ types: null }) },
  { id: "rk-designs", label: "Read: designs", json: j({ designs: null }) },
  { id: "rk-desc", label: "Read: description", json: j({ description: null }) },
  { id: "rk-files", label: "Read: files (stub → Other)", json: j({ files: null }) },
  {
    id: "rk-type-everything",
    label: "Read: type everything",
    json: j({
      readTypeCommands: {
        id: { id: "PLACEHOLDER_TYPE_ID" },
        commands: [{ everything: null }],
      },
    }),
  },
];

/** Rows for the on-screen coverage checklist (mirrors `ALL_CHANGE_KIT_ROOT_KEYS` + nested groups). */
export const KIT_STORE_COVERAGE_ROWS: readonly { group: string; key: string }[] = [
  { group: "ChangeKit (root)", key: "replaceKitFromFullDto + all ALL_CHANGE_KIT_ROOT_KEYS" },
  { group: "ChangeType", key: "see CHANGE_TYPE_COMMAND_KEYS" },
  { group: "ReadKit", key: "everything, name, description, types, designs, files, readTypeCommands, readDesignCommands" },
];
