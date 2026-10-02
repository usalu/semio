/** 🫧 `change-zone-co2` wire twin: the leaf payload `ChangeZoneCo2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeZoneCo2 {
  zoneId: string;
  newCo2Ppm: number;
}

export const parseChangeZoneCo2: NormWireReader<ChangeZoneCo2> = normWireObject<ChangeZoneCo2>({ zoneId: normWireRequired(normWireString), newCo2Ppm: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
