/** ↔️ `change-beam-transverse-as` wire twin: the leaf payload `ChangeBeamTransverseAs`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeBeamTransverseAs {
  index: number;
  newTransverseAsM2PerM: number;
}

export const parseChangeBeamTransverseAs: NormWireReader<ChangeBeamTransverseAs> = normWireObject<ChangeBeamTransverseAs>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newTransverseAsM2PerM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
