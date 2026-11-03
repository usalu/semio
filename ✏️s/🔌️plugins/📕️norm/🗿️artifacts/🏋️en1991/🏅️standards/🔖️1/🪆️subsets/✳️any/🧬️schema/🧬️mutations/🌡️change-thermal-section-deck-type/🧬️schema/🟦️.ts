/** 🌡 `change-thermal-section-deck-type` wire twin: the leaf payload `ChangeThermalSectionDeckType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeThermalSectionDeckType {
  id: string;
  newDeckType: number;
}

export const parseChangeThermalSectionDeckType: NormWireReader<ChangeThermalSectionDeckType> = normWireObject<ChangeThermalSectionDeckType>({ id: normWireRequired(normWireString), newDeckType: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
