/** 🔩️ `change-row-row-type` wire twin: the leaf payload `ChangeRowRowType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type JointRowType, parseJointRowType } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeRowRowType {
  id: string;
  newRowType: JointRowType;
}

export const parseChangeRowRowType: NormWireReader<ChangeRowRowType> = normWireObject<ChangeRowRowType>({ id: normWireRequired(normWireString), newRowType: normWireRequired(parseJointRowType) });
