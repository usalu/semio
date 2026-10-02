/** 🏠️ `change-envelope-n50` wire twin: the leaf payload `ChangeEnvelopeN50`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeEnvelopeN50 {
  newEnvelopeN50HInv: number;
}

export const parseChangeEnvelopeN50: NormWireReader<ChangeEnvelopeN50> = normWireObject<ChangeEnvelopeN50>({ newEnvelopeN50HInv: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
