/** 📜️ The wfc3d document's TEXT carrier. `print_dsl` wraps the body in this artifact's own envelope
 * (`semio wfc.wfc3d.dsl v1`) and `parse_dsl` is its exact inverse; the carrier itself is a string. */
export type Wfc3dSnapshotText = string;

export const envelopeId = "wfc.wfc3d";
export const extension = "wfc3d";
export const languageId = "wfc3d.snapshot";
