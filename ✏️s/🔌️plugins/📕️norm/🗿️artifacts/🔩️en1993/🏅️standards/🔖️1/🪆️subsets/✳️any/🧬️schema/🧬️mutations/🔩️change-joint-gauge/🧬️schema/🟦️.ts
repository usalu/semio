/** 🔩️ `change-joint-gauge` wire twin: the leaf payload `ChangeJointGauge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointGauge {
  id: string;
  newGauge: number;
}

export const parseChangeJointGauge: NormWireReader<ChangeJointGauge> = normWireObject<ChangeJointGauge>({ id: normWireRequired(normWireString), newGauge: normWireRequired(normWireNumber) });
