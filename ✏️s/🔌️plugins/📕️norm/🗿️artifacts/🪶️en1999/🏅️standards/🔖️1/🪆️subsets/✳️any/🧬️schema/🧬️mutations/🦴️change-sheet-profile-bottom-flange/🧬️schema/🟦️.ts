/** 🦴️ `change-sheet-profile-bottom-flange` wire twin: the leaf payload `ChangeSheetProfileBottomFlange`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetProfileBottomFlange {
  mutation: "changeSheetProfileBottomFlange";
  sheetProfileId: string;
  newBottomFlange: number;
}

export const parseChangeSheetProfileBottomFlange: NormWireReader<ChangeSheetProfileBottomFlange> = normWireObject<ChangeSheetProfileBottomFlange>({ mutation: normWireRequired(normWireLiteral("changeSheetProfileBottomFlange")), sheetProfileId: normWireRequired(normWireString), newBottomFlange: normWireRequired(normWireNumber) });
