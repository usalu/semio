/** 🔩️ `change-joint-column-moment-res` wire twin: the leaf payload `ChangeJointColumnMomentRes`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointColumnMomentRes {
  id: string;
  newColumnMomentResistance: number;
}

export const parseChangeJointColumnMomentRes: NormWireReader<ChangeJointColumnMomentRes> = normWireObject<ChangeJointColumnMomentRes>({ id: normWireRequired(normWireString), newColumnMomentResistance: normWireRequired(normWireNumber) });
