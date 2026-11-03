/** 🔩️ `change-joint-kind` wire twin: the leaf payload `ChangeJointKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type JointKind, parseJointKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeJointKind {
  id: string;
  newKind: JointKind;
}

export const parseChangeJointKind: NormWireReader<ChangeJointKind> = normWireObject<ChangeJointKind>({ id: normWireRequired(normWireString), newKind: normWireRequired(parseJointKind) });
