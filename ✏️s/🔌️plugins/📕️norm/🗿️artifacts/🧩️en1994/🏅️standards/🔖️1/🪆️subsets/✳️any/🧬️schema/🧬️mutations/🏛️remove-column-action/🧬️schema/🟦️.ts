/** 🏛️ `remove-column-action` wire twin: the leaf payload `RemoveColumnAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveColumnAction {
  columnId: string;
  id: string;
}

export const parseRemoveColumnAction: NormWireReader<RemoveColumnAction> = normWireObject<RemoveColumnAction>({ columnId: normWireRequired(normWireString), id: normWireRequired(normWireString) });
