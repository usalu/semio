/** ✏️ `change-joint-xm` wire twin: the leaf payload `ChangeJointXM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointXM {
  mutation: "changeJointXM";
  id: string;
  newXM: number;
}

export const parseChangeJointXM: NormWireReader<ChangeJointXM> = normWireObject<ChangeJointXM>({ mutation: normWireRequired(normWireLiteral("changeJointXM")), id: normWireRequired(normWireString), newXM: normWireRequired(normWireNumber) });
