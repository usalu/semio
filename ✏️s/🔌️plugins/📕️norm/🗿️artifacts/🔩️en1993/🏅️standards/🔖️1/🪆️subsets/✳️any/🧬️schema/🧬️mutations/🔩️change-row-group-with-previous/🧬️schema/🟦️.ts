/** 🔩️ `change-row-group-with-previous` wire twin: the leaf payload `ChangeRowGroupWithPrevious`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeRowGroupWithPrevious {
  id: string;
  newGroupWithPrevious: number;
}

export const parseChangeRowGroupWithPrevious: NormWireReader<ChangeRowGroupWithPrevious> = normWireObject<ChangeRowGroupWithPrevious>({ id: normWireRequired(normWireString), newGroupWithPrevious: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
