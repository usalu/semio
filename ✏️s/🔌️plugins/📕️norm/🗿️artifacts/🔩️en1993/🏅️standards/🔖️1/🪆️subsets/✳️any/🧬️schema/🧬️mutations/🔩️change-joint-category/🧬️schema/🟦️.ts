/** 🔩️ `change-joint-category` wire twin: the leaf payload `ChangeJointCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type JointCategory, parseJointCategory } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeJointCategory {
  id: string;
  newCategory: JointCategory;
}

export const parseChangeJointCategory: NormWireReader<ChangeJointCategory> = normWireObject<ChangeJointCategory>({ id: normWireRequired(normWireString), newCategory: normWireRequired(parseJointCategory) });
