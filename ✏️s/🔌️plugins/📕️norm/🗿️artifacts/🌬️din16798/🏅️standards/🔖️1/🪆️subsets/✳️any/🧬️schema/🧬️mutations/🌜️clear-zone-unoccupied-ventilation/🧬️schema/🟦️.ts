/** 🌜️ `clear-zone-unoccupied-ventilation` wire twin: the leaf payload `ClearZoneUnoccupiedVentilation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";

export interface ClearZoneUnoccupiedVentilation {
  zoneId: string;
}

export const parseClearZoneUnoccupiedVentilation: NormWireReader<ClearZoneUnoccupiedVentilation> = normWireObject<ClearZoneUnoccupiedVentilation>({ zoneId: normWireRequired(normWireString) });
