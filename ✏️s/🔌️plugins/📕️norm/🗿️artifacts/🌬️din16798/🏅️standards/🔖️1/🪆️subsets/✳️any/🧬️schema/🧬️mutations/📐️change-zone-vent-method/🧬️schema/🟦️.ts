/** 📐️ `change-zone-vent-method` wire twin: the leaf payload `ChangeZoneVentMethod`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneVentMethod {
  zoneId: string;
  newVentMethod: string;
}

export const parseChangeZoneVentMethod: NormWireReader<ChangeZoneVentMethod> = normWireObject<ChangeZoneVentMethod>({ zoneId: normWireRequired(normWireString), newVentMethod: normWireRequired(normWireString) });
