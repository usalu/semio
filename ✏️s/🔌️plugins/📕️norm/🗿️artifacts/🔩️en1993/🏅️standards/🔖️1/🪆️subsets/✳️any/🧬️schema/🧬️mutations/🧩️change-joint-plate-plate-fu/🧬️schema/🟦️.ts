/** 🧩️ `change-joint-plate-plate-fu` wire twin: the leaf payload `ChangeJointPlatePlateFu`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlatePlateFu {
  id: string;
  newPlateFu: number;
}

export const parseChangeJointPlatePlateFu: NormWireReader<ChangeJointPlatePlateFu> = normWireObject<ChangeJointPlatePlateFu>({ id: normWireRequired(normWireString), newPlateFu: normWireRequired(normWireNumber) });
