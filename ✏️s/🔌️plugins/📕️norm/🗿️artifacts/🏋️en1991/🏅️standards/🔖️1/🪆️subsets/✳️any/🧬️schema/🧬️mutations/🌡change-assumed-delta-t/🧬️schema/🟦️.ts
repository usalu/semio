/** 🌡 `change-assumed-delta-t` wire twin: the leaf payload `ChangeAssumedDeltaT`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssumedDeltaT {
  newAssumedDeltaT: number;
}

export const parseChangeAssumedDeltaT: NormWireReader<ChangeAssumedDeltaT> = normWireObject<ChangeAssumedDeltaT>({ newAssumedDeltaT: normWireRequired(normWireNumber) });
