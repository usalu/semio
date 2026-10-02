/** 🔗 `change-zone-vent-system-id` wire twin: the leaf payload `ChangeZoneVentSystemId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneVentSystemId {
  zoneId: string;
  newVentSystemId: string;
}

export const parseChangeZoneVentSystemId: NormWireReader<ChangeZoneVentSystemId> = normWireObject<ChangeZoneVentSystemId>({ zoneId: normWireRequired(normWireString), newVentSystemId: normWireRequired(normWireString) });
