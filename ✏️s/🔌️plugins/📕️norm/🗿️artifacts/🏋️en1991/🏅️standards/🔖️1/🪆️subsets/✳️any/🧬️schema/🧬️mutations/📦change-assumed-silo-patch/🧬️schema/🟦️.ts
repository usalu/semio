/** 📦 `change-assumed-silo-patch` wire twin: the leaf payload `ChangeAssumedSiloPatch`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssumedSiloPatch {
  newAssumedSiloPatch: number;
}

export const parseChangeAssumedSiloPatch: NormWireReader<ChangeAssumedSiloPatch> = normWireObject<ChangeAssumedSiloPatch>({ newAssumedSiloPatch: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
