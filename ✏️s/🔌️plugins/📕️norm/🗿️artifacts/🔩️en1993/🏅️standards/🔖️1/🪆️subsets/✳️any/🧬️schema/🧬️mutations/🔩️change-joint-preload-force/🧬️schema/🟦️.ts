/** 🔩️ `change-joint-preload-force` wire twin: the leaf payload `ChangeJointPreloadForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPreloadForce {
  id: string;
  newPreloadForce: number;
}

export const parseChangeJointPreloadForce: NormWireReader<ChangeJointPreloadForce> = normWireObject<ChangeJointPreloadForce>({ id: normWireRequired(normWireString), newPreloadForce: normWireRequired(normWireNumber) });
