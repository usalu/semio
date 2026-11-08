/** 🦴️ `change-sheet-profile-top-flange` wire twin: the leaf payload `ChangeSheetProfileTopFlange`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetProfileTopFlange {
  mutation: "changeSheetProfileTopFlange";
  sheetProfileId: string;
  newTopFlange: number;
}

export const parseChangeSheetProfileTopFlange: NormWireReader<ChangeSheetProfileTopFlange> = normWireObject<ChangeSheetProfileTopFlange>({ mutation: normWireRequired(normWireLiteral("changeSheetProfileTopFlange")), sheetProfileId: normWireRequired(normWireString), newTopFlange: normWireRequired(normWireNumber) });
