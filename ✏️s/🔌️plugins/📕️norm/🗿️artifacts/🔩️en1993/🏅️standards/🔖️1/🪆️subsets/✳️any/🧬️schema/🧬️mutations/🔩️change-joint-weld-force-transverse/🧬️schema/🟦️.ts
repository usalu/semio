/** 🔩️ `change-joint-weld-force-transverse` wire twin: the leaf payload `ChangeJointWeldForceTransverse`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointWeldForceTransverse {
  id: string;
  newWeldForceTransverse: number;
}

export const parseChangeJointWeldForceTransverse: NormWireReader<ChangeJointWeldForceTransverse> = normWireObject<ChangeJointWeldForceTransverse>({ id: normWireRequired(normWireString), newWeldForceTransverse: normWireRequired(normWireNumber) });
