/** 🗂️ `remove-sheet-action` wire twin: the leaf payload `RemoveSheetAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveSheetAction {
  mutation: "removeSheetAction";
  coldFormedId: string;
  id: string;
}

export const parseRemoveSheetAction: NormWireReader<RemoveSheetAction> = normWireObject<RemoveSheetAction>({ mutation: normWireRequired(normWireLiteral("removeSheetAction")), coldFormedId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
