/** 📈️ `change-element-delta-ur` wire twin: the leaf payload `ChangeElementDeltaUr`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeElementDeltaUr {
  elementId: string;
  newDeltaUR: number;
}

export const parseChangeElementDeltaUr: NormWireReader<ChangeElementDeltaUr> = normWireObject<ChangeElementDeltaUr>({ elementId: normWireRequired(normWireString), newDeltaUR: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
