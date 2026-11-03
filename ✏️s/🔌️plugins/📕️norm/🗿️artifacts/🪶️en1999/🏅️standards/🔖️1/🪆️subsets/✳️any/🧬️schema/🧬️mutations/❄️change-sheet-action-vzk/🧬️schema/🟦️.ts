/** ❄️ `change-sheet-action-vzk` wire twin: the leaf payload `ChangeSheetActionVZK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetActionVZK {
  mutation: "changeSheetActionVZK";
  coldFormedId: string;
  actionId: string;
  newVZK: number;
}

export const parseChangeSheetActionVZK: NormWireReader<ChangeSheetActionVZK> = normWireObject<ChangeSheetActionVZK>({ mutation: normWireRequired(normWireLiteral("changeSheetActionVZK")), coldFormedId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newVZK: normWireRequired(normWireNumber) });
