/** ❄️ `change-sheet-action-vyk` wire twin: the leaf payload `ChangeSheetActionVYK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetActionVYK {
  mutation: "changeSheetActionVYK";
  coldFormedId: string;
  actionId: string;
  newVYK: number;
}

export const parseChangeSheetActionVYK: NormWireReader<ChangeSheetActionVYK> = normWireObject<ChangeSheetActionVYK>({ mutation: normWireRequired(normWireLiteral("changeSheetActionVYK")), coldFormedId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newVYK: normWireRequired(normWireNumber) });
