/** 🏋️ `change-action-t` wire twin: the leaf payload `ChangeActionT`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionT {
  id: string;
  newActionT: number;
}

export const parseChangeActionT: NormWireReader<ChangeActionT> = normWireObject<ChangeActionT>({ id: normWireRequired(normWireString), newActionT: normWireRequired(normWireNumber) });
