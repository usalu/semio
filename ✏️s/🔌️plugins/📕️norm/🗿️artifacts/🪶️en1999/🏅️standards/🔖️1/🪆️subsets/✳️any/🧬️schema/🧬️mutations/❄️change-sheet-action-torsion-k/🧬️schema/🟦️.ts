/** ❄️ `change-sheet-action-torsion-k` wire twin: the leaf payload `ChangeSheetActionTorsionK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetActionTorsionK {
  mutation: "changeSheetActionTorsionK";
  coldFormedId: string;
  actionId: string;
  newTorsionK: number;
}

export const parseChangeSheetActionTorsionK: NormWireReader<ChangeSheetActionTorsionK> = normWireObject<ChangeSheetActionTorsionK>({ mutation: normWireRequired(normWireLiteral("changeSheetActionTorsionK")), coldFormedId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newTorsionK: normWireRequired(normWireNumber) });
