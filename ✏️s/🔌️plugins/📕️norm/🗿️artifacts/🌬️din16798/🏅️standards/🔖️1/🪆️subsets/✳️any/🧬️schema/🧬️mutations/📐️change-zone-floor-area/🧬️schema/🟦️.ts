/** 📐️ `change-zone-floor-area` wire twin: the leaf payload `ChangeZoneFloorArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneFloorArea {
  zoneId: string;
  newFloorAreaM2: number;
}

export const parseChangeZoneFloorArea: NormWireReader<ChangeZoneFloorArea> = normWireObject<ChangeZoneFloorArea>({ zoneId: normWireRequired(normWireString), newFloorAreaM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
