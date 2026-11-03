/** 🥶️ `change-cf-gross-resistance` wire twin: the leaf payload `ChangeCfGrossResistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfGrossResistance {
  id: string;
  newGrossResistance: number;
}

export const parseChangeCfGrossResistance: NormWireReader<ChangeCfGrossResistance> = normWireObject<ChangeCfGrossResistance>({ id: normWireRequired(normWireString), newGrossResistance: normWireRequired(normWireNumber) });
