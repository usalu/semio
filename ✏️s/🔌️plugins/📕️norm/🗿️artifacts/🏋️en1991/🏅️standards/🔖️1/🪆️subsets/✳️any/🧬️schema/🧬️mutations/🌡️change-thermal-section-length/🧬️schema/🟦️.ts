/** 🌡 `change-thermal-section-length` wire twin: the leaf payload `ChangeThermalSectionLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeThermalSectionLength {
  id: string;
  newLength: number;
}

export const parseChangeThermalSectionLength: NormWireReader<ChangeThermalSectionLength> = normWireObject<ChangeThermalSectionLength>({ id: normWireRequired(normWireString), newLength: normWireRequired(normWireNumber) });
