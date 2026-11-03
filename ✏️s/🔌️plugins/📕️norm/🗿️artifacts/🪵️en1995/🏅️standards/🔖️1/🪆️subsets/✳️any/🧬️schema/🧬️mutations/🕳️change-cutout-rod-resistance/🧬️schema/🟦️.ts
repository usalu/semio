/** 🕳️ `change-cutout-rod-resistance` wire twin: the leaf payload `ChangeCutoutRodResistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCutoutRodResistance {
  cutoutId: string;
  newValue: number;
}

export const parseChangeCutoutRodResistance: NormWireReader<ChangeCutoutRodResistance> = normWireObject<ChangeCutoutRodResistance>({ cutoutId: normWireRequired(normWireString), newValue: normWireRequired(normWireNumber) });
