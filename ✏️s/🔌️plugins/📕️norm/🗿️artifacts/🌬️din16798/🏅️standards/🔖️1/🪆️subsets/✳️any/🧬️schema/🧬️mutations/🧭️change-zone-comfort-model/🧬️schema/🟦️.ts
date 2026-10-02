/** 🧭️ `change-zone-comfort-model` wire twin: the leaf payload `ChangeZoneComfortModel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneComfortModel {
  zoneId: string;
  newComfortModel: string;
}

export const parseChangeZoneComfortModel: NormWireReader<ChangeZoneComfortModel> = normWireObject<ChangeZoneComfortModel>({ zoneId: normWireRequired(normWireString), newComfortModel: normWireRequired(normWireString) });
