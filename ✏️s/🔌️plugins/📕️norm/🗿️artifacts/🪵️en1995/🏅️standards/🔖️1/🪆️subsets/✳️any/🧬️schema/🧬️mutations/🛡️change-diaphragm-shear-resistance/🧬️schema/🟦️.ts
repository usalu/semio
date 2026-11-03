/** 🛡️ `change-diaphragm-shear-resistance` wire twin: the leaf payload `ChangeDiaphragmShearResistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDiaphragmShearResistance {
  diaphragmId: string;
  newValue: number;
}

export const parseChangeDiaphragmShearResistance: NormWireReader<ChangeDiaphragmShearResistance> = normWireObject<ChangeDiaphragmShearResistance>({ diaphragmId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
