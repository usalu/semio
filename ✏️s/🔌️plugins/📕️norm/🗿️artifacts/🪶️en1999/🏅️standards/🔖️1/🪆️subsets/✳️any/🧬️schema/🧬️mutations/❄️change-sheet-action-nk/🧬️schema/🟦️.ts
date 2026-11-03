/** ❄️ `change-sheet-action-nk` wire twin: the leaf payload `ChangeSheetActionNK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetActionNK {
  mutation: "changeSheetActionNK";
  coldFormedId: string;
  actionId: string;
  newNK: number;
}

export const parseChangeSheetActionNK: NormWireReader<ChangeSheetActionNK> = normWireObject<ChangeSheetActionNK>({ mutation: normWireRequired(normWireLiteral("changeSheetActionNK")), coldFormedId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newNK: normWireRequired(normWireNumber) });
