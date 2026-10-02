/** 🔆 `change-assumed-h-net` wire twin: the leaf payload `ChangeAssumedHNet`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeAssumedHNet {
  newAssumedHNet: number;
}

export const parseChangeAssumedHNet: NormWireReader<ChangeAssumedHNet> = normWireObject<ChangeAssumedHNet>({ newAssumedHNet: normWireRequired(normWireNumber) });
