/** ✏️ `change-joint-ym` wire twin: the leaf payload `ChangeJointYM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointYM {
  mutation: "changeJointYM";
  id: string;
  newYM: number;
}

export const parseChangeJointYM: NormWireReader<ChangeJointYM> = normWireObject<ChangeJointYM>({ mutation: normWireRequired(normWireLiteral("changeJointYM")), id: normWireRequired(normWireString), newYM: normWireRequired(normWireNumber) });
