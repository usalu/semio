/** 🧩️ `change-joint-plate-weld-beta-w` wire twin: the leaf payload `ChangeJointPlateWeldBetaW`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateWeldBetaW {
  id: string;
  newWeldBetaW: number;
}

export const parseChangeJointPlateWeldBetaW: NormWireReader<ChangeJointPlateWeldBetaW> = normWireObject<ChangeJointPlateWeldBetaW>({ id: normWireRequired(normWireString), newWeldBetaW: normWireRequired(normWireNumber) });
