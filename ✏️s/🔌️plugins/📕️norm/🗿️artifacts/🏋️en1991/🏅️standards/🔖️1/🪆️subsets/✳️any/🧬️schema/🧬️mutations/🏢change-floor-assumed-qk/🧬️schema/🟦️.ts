/** 🏢 `change-floor-assumed-qk` wire twin: the leaf payload `ChangeFloorAssumedQk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeFloorAssumedQk {
  index: number;
  newAssumedQk: number;
}

export const parseChangeFloorAssumedQk: NormWireReader<ChangeFloorAssumedQk> = normWireObject<ChangeFloorAssumedQk>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newAssumedQk: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
