/** 🌡 `change-thermal-section-surfacing` wire twin: the leaf payload `ChangeThermalSectionSurfacing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeThermalSectionSurfacing {
  id: string;
  newSurfacing: number;
}

export const parseChangeThermalSectionSurfacing: NormWireReader<ChangeThermalSectionSurfacing> = normWireObject<ChangeThermalSectionSurfacing>({ id: normWireRequired(normWireString), newSurfacing: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
