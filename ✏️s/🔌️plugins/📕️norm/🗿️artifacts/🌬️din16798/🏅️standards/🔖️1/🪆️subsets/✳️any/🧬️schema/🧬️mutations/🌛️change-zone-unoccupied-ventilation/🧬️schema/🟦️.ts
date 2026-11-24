/** 🌛️ `change-zone-unoccupied-ventilation` wire twin: the leaf payload `ChangeZoneUnoccupiedVentilation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `📇️registry/🧬️contract/🏭️generator/🟦️.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "@semio-tech/norm-js/registry/contract";
import { type Din16798UnoccupiedVentilation, parseDin16798UnoccupiedVentilation } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeZoneUnoccupiedVentilation {
  zoneId: string;
  newUnoccupiedVentilation: Din16798UnoccupiedVentilation;
}

export const parseChangeZoneUnoccupiedVentilation: NormWireReader<ChangeZoneUnoccupiedVentilation> = normWireObject<ChangeZoneUnoccupiedVentilation>({ zoneId: normWireRequired(normWireString), newUnoccupiedVentilation: normWireRequired(parseDin16798UnoccupiedVentilation) });
