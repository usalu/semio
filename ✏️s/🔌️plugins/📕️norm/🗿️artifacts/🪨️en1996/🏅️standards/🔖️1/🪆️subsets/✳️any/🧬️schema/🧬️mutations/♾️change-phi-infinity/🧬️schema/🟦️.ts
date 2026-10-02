/** ♾️ `change-phi-infinity` wire twin: the leaf payload `ChangePhiInfinity`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangePhiInfinity {
  index: number;
  newPhiInfinity: number;
}

export const parseChangePhiInfinity: NormWireReader<ChangePhiInfinity> = normWireObject<ChangePhiInfinity>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newPhiInfinity: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
