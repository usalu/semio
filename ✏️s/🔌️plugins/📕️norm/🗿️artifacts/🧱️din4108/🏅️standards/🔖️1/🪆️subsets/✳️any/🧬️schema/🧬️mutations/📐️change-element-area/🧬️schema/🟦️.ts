/** 📐️ `change-element-area` wire twin: the leaf payload `ChangeElementArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeElementArea {
  elementId: string;
  newAreaM2: number;
}

export const parseChangeElementArea: NormWireReader<ChangeElementArea> = normWireObject<ChangeElementArea>({ elementId: normWireRequired(normWireString), newAreaM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
