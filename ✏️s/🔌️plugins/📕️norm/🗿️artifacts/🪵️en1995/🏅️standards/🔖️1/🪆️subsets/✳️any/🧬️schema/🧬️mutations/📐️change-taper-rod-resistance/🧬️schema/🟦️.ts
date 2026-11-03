/** 📐️ `change-taper-rod-resistance` wire twin: the leaf payload `ChangeTaperRodResistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTaperRodResistance {
  taperId: string;
  newValue: number;
}

export const parseChangeTaperRodResistance: NormWireReader<ChangeTaperRodResistance> = normWireObject<ChangeTaperRodResistance>({ taperId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
