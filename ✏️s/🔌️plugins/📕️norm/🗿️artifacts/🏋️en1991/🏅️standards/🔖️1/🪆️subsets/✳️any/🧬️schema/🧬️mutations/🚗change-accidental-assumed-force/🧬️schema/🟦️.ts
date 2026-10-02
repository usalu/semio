/** 🚗 `change-accidental-assumed-force` wire twin: the leaf payload `ChangeAccidentalAssumedForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAccidentalAssumedForce {
  index: number;
  newAssumedForce: number;
}

export const parseChangeAccidentalAssumedForce: NormWireReader<ChangeAccidentalAssumedForce> = normWireObject<ChangeAccidentalAssumedForce>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newAssumedForce: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
