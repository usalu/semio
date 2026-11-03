/** 🔩️ `change-joint-weld-force-longitudinal` wire twin: the leaf payload `ChangeJointWeldForceLongitudinal`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointWeldForceLongitudinal {
  id: string;
  newWeldForceLongitudinal: number;
}

export const parseChangeJointWeldForceLongitudinal: NormWireReader<ChangeJointWeldForceLongitudinal> = normWireObject<ChangeJointWeldForceLongitudinal>({ id: normWireRequired(normWireString), newWeldForceLongitudinal: normWireRequired(normWireNumber) });
