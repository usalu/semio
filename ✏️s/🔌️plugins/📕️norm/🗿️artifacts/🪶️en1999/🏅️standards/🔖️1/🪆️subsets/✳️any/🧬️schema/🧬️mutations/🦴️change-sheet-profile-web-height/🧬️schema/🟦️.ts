/** 🦴️ `change-sheet-profile-web-height` wire twin: the leaf payload `ChangeSheetProfileWebHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetProfileWebHeight {
  mutation: "changeSheetProfileWebHeight";
  sheetProfileId: string;
  newWebHeight: number;
}

export const parseChangeSheetProfileWebHeight: NormWireReader<ChangeSheetProfileWebHeight> = normWireObject<ChangeSheetProfileWebHeight>({ mutation: normWireRequired(normWireLiteral("changeSheetProfileWebHeight")), sheetProfileId: normWireRequired(normWireString), newWebHeight: normWireRequired(normWireNumber) });
