/** 🔋 `change-assumed-qf-d` wire twin: the leaf payload `ChangeAssumedQfD`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssumedQfD {
  newAssumedQfD: number;
}

export const parseChangeAssumedQfD: NormWireReader<ChangeAssumedQfD> = normWireObject<ChangeAssumedQfD>({ newAssumedQfD: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
