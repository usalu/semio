/** 📈️ `change-element-delta-ug` wire twin: the leaf payload `ChangeElementDeltaUg`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeElementDeltaUg {
  elementId: string;
  newDeltaUG: number;
}

export const parseChangeElementDeltaUg: NormWireReader<ChangeElementDeltaUg> = normWireObject<ChangeElementDeltaUg>({ elementId: normWireRequired(normWireString), newDeltaUG: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
