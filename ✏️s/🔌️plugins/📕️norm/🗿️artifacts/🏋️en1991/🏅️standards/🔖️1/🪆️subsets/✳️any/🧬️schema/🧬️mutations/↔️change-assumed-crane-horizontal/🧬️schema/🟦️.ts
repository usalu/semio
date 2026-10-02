/** ↔️ `change-assumed-crane-horizontal` wire twin: the leaf payload `ChangeAssumedCraneHorizontal`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssumedCraneHorizontal {
  newAssumedCraneHorizontal: number;
}

export const parseChangeAssumedCraneHorizontal: NormWireReader<ChangeAssumedCraneHorizontal> = normWireObject<ChangeAssumedCraneHorizontal>({ newAssumedCraneHorizontal: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
