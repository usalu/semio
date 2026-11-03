/** 🔩️ `change-joint-plate-fy` wire twin: the leaf payload `ChangeJointPlateFy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateFy {
  id: string;
  newPlateFy: number;
}

export const parseChangeJointPlateFy: NormWireReader<ChangeJointPlateFy> = normWireObject<ChangeJointPlateFy>({ id: normWireRequired(normWireString), newPlateFy: normWireRequired(normWireNumber) });
