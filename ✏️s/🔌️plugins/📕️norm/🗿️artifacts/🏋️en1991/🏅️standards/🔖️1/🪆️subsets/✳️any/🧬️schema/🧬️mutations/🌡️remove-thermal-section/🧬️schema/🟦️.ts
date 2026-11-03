/** 🌡 `remove-thermal-section` wire twin: the leaf payload `RemoveThermalSection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface RemoveThermalSection {
  id: string;
}

export const parseRemoveThermalSection: NormWireReader<RemoveThermalSection> = normWireObject<RemoveThermalSection>({ id: normWireRequired(normWireString) });
