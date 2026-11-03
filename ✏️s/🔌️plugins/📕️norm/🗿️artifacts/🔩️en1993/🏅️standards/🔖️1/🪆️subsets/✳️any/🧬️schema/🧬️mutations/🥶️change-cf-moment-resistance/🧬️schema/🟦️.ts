/** 🥶️ `change-cf-moment-resistance` wire twin: the leaf payload `ChangeCfMomentResistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfMomentResistance {
  id: string;
  newMomentResistance: number;
}

export const parseChangeCfMomentResistance: NormWireReader<ChangeCfMomentResistance> = normWireObject<ChangeCfMomentResistance>({ id: normWireRequired(normWireString), newMomentResistance: normWireRequired(normWireNumber) });
