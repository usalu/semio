/** 🔩️ `insert-joint-action` wire twin: the leaf payload `InsertJointAction`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type JointForceAction, parseJointForceAction } from "../../../📸️snapshot/🟦️.ts";

export interface InsertJointAction {
  id: string;
  index?: number | null;
  element: JointForceAction;
}

export const parseInsertJointAction: NormWireReader<InsertJointAction> = normWireObject<InsertJointAction>({ id: normWireRequired(normWireString), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), element: normWireRequired(parseJointForceAction) });
