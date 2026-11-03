/** 🏋️ `change-action-mz` wire twin: the leaf payload `ChangeActionMz`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeActionMz {
  id: string;
  newActionMz: number;
}

export const parseChangeActionMz: NormWireReader<ChangeActionMz> = normWireObject<ChangeActionMz>({ id: normWireRequired(normWireString), newActionMz: normWireRequired(normWireNumber) });
