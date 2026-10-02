/** 🌙 `change-zone-night-ventilation` wire twin: the leaf payload `ChangeZoneNightVentilation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneNightVentilation {
  zoneId: string;
  newNightVentilation: string;
}

export const parseChangeZoneNightVentilation: NormWireReader<ChangeZoneNightVentilation> = normWireObject<ChangeZoneNightVentilation>({ zoneId: normWireRequired(normWireString), newNightVentilation: normWireRequired(normWireString) });
