/** 🔩️ `change-joint-weld-grade` wire twin: the leaf payload `ChangeJointWeldGrade`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointWeldGrade {
  id: string;
  newWeldGrade: string;
}

export const parseChangeJointWeldGrade: NormWireReader<ChangeJointWeldGrade> = normWireObject<ChangeJointWeldGrade>({ id: normWireRequired(normWireString), newWeldGrade: normWireRequired(normWireString) });
