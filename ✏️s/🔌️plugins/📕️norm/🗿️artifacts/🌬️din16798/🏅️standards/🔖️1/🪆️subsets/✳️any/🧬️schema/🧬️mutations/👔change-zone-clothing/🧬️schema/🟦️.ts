/** 👔 `change-zone-clothing` wire twin: the leaf payload `ChangeZoneClothing`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneClothing {
  zoneId: string;
  newClothingClo: number;
}

export const parseChangeZoneClothing: NormWireReader<ChangeZoneClothing> = normWireObject<ChangeZoneClothing>({ zoneId: normWireRequired(normWireString), newClothingClo: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
