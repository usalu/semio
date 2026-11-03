/** 🧩️ `change-joint-plate-plate-fy` wire twin: the leaf payload `ChangeJointPlatePlateFy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlatePlateFy {
  id: string;
  newPlateFy: number;
}

export const parseChangeJointPlatePlateFy: NormWireReader<ChangeJointPlatePlateFy> = normWireObject<ChangeJointPlatePlateFy>({ id: normWireRequired(normWireString), newPlateFy: normWireRequired(normWireNumber) });
