/** ❄️ `change-sheet-action-local-load-k` wire twin: the leaf payload `ChangeSheetActionLocalLoadK`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSheetActionLocalLoadK {
  mutation: "changeSheetActionLocalLoadK";
  coldFormedId: string;
  actionId: string;
  newLocalLoadK: number;
}

export const parseChangeSheetActionLocalLoadK: NormWireReader<ChangeSheetActionLocalLoadK> = normWireObject<ChangeSheetActionLocalLoadK>({ mutation: normWireRequired(normWireLiteral("changeSheetActionLocalLoadK")), coldFormedId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), newLocalLoadK: normWireRequired(normWireNumber) });
