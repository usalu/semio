/** 🦴️ `change-sheet-profile-pitch` wire twin: the leaf payload `ChangeSheetProfilePitch`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetProfilePitch {
  mutation: "changeSheetProfilePitch";
  sheetProfileId: string;
  newPitch: number;
}

export const parseChangeSheetProfilePitch: NormWireReader<ChangeSheetProfilePitch> = normWireObject<ChangeSheetProfilePitch>({ mutation: normWireRequired(normWireLiteral("changeSheetProfilePitch")), sheetProfileId: normWireRequired(normWireString), newPitch: normWireRequired(normWireNumber) });
