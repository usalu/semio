/** ❄️ `change-sheet-action-myk` wire twin: the leaf payload `ChangeSheetActionMYK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetActionMYK {
  mutation: "changeSheetActionMYK";
  coldFormedId: string;
  actionId: string;
  newMYK: number;
}

export const parseChangeSheetActionMYK: NormWireReader<ChangeSheetActionMYK> = normWireObject<ChangeSheetActionMYK>({ mutation: normWireRequired(normWireLiteral("changeSheetActionMYK")), coldFormedId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newMYK: normWireRequired(normWireNumber) });
