/** ❄️ `change-sheet-action-mzk` wire twin: the leaf payload `ChangeSheetActionMZK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetActionMZK {
  mutation: "changeSheetActionMZK";
  coldFormedId: string;
  actionId: string;
  newMZK: number;
}

export const parseChangeSheetActionMZK: NormWireReader<ChangeSheetActionMZK> = normWireObject<ChangeSheetActionMZK>({ mutation: normWireRequired(normWireLiteral("changeSheetActionMZK")), coldFormedId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newMZK: normWireRequired(normWireNumber) });
