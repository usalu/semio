/** 🔩️ `change-joint-plate-fu` wire twin: the leaf payload `ChangeJointPlateFu`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateFu {
  id: string;
  newPlateFu: number;
}

export const parseChangeJointPlateFu: NormWireReader<ChangeJointPlateFu> = normWireObject<ChangeJointPlateFu>({ id: normWireRequired(normWireString), newPlateFu: normWireRequired(normWireNumber) });
