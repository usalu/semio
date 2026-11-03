/** 🌡 `change-thermal-section-depth` wire twin: the leaf payload `ChangeThermalSectionDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeThermalSectionDepth {
  id: string;
  newDepth: number;
}

export const parseChangeThermalSectionDepth: NormWireReader<ChangeThermalSectionDepth> = normWireObject<ChangeThermalSectionDepth>({ id: normWireRequired(normWireString), newDepth: normWireRequired(normWireNumber) });
