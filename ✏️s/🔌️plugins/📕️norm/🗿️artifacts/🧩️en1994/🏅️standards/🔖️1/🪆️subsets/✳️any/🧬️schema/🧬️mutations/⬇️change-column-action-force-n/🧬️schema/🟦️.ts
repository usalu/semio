/** ⬇️ `change-column-action-force-n` wire twin: the leaf payload `ChangeColumnActionForceN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnActionForceN {
  columnId: string;
  id: string;
  newNKN: number;
}

export const parseChangeColumnActionForceN: NormWireReader<ChangeColumnActionForceN> = normWireObject<ChangeColumnActionForceN>({ columnId: normWireRequired(normWireString), id: normWireRequired(normWireString), newNKN: normWireRequired(normWireNumber) });
