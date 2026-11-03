/** 🔩️ `change-joint-base-moment` wire twin: the leaf payload `ChangeJointBaseMoment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointBaseMoment {
  id: string;
  newBaseMoment: number;
}

export const parseChangeJointBaseMoment: NormWireReader<ChangeJointBaseMoment> = normWireObject<ChangeJointBaseMoment>({ id: normWireRequired(normWireString), newBaseMoment: normWireRequired(normWireNumber) });
