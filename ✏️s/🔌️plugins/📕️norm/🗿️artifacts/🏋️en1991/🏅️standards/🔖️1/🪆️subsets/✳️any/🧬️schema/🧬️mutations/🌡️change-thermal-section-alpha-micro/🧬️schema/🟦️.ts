/** 🌡 `change-thermal-section-alpha-micro` wire twin: the leaf payload `ChangeThermalSectionAlphaMicro`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeThermalSectionAlphaMicro {
  id: string;
  newAlphaMicro: number;
}

export const parseChangeThermalSectionAlphaMicro: NormWireReader<ChangeThermalSectionAlphaMicro> = normWireObject<ChangeThermalSectionAlphaMicro>({ id: normWireRequired(normWireString), newAlphaMicro: normWireRequired(normWireNumber) });
