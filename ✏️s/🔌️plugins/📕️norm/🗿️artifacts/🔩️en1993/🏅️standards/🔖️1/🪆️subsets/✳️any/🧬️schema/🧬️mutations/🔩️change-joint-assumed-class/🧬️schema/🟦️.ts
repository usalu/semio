/** 🔩️ `change-joint-assumed-class` wire twin: the leaf payload `ChangeJointAssumedClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type StiffnessClass, parseStiffnessClass } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeJointAssumedClass {
  id: string;
  newAssumedClass: StiffnessClass;
}

export const parseChangeJointAssumedClass: NormWireReader<ChangeJointAssumedClass> = normWireObject<ChangeJointAssumedClass>({ id: normWireRequired(normWireString), newAssumedClass: normWireRequired(parseStiffnessClass) });
