/** 🛡 `change-wind-face-assumed-wp` wire twin: the leaf payload `ChangeWindFaceAssumedWp`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeWindFaceAssumedWp {
  index: number;
  newAssumedWp: number;
}

export const parseChangeWindFaceAssumedWp: NormWireReader<ChangeWindFaceAssumedWp> = normWireObject<ChangeWindFaceAssumedWp>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newAssumedWp: normWireRequired(normWireNumber) });
