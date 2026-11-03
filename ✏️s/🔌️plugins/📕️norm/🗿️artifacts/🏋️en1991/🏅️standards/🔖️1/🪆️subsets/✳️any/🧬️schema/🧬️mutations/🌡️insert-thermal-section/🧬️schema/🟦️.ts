/** 🌡 `insert-thermal-section` wire twin: the leaf payload `InsertThermalSection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ThermalSection, parseThermalSection } from "../../../📸️snapshot/🟦️.ts";

export interface InsertThermalSection {
  index: number;
  item: ThermalSection;
}

export const parseInsertThermalSection: NormWireReader<InsertThermalSection> = normWireObject<InsertThermalSection>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseThermalSection) });
