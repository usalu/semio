/** 📐️ `change-tributary-area` wire twin: the leaf payload `ChangeTributaryArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeTributaryArea {
  wallIndex: number;
  index: number;
  newTributaryAreaM2: number;
}

export const parseChangeTributaryArea: NormWireReader<ChangeTributaryArea> = normWireObject<ChangeTributaryArea>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newTributaryAreaM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
