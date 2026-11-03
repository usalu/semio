/** 🧱️ `change-fastened-joint-gauge` wire twin: the leaf payload `ChangeFastenedJointGauge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFastenedJointGauge {
  id: string;
  newGauge: number;
}

export const parseChangeFastenedJointGauge: NormWireReader<ChangeFastenedJointGauge> = normWireObject<ChangeFastenedJointGauge>({ id: normWireRequired(normWireString), newGauge: normWireRequired(normWireNumber) });
