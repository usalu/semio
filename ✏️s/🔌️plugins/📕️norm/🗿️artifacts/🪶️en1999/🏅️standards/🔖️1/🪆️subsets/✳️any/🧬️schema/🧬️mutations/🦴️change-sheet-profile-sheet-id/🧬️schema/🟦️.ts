/** 🦴️ `change-sheet-profile-sheet-id` wire twin: the leaf payload `ChangeSheetProfileSheetId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetProfileSheetId {
  mutation: "changeSheetProfileSheetId";
  sheetProfileId: string;
  newSheetId: string;
}

export const parseChangeSheetProfileSheetId: NormWireReader<ChangeSheetProfileSheetId> = normWireObject<ChangeSheetProfileSheetId>({ mutation: normWireRequired(normWireLiteral("changeSheetProfileSheetId")), sheetProfileId: normWireRequired(normWireString), newSheetId: normWireRequired(normWireString) });
