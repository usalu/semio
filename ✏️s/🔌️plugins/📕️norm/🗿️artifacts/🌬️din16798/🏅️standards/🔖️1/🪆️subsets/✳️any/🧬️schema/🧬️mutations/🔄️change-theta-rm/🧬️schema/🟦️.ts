/** 🔄️ `change-theta-rm` wire twin: the leaf payload `ChangeThetaRm`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeThetaRm {
  newThetaRmC: number;
}

export const parseChangeThetaRm: NormWireReader<ChangeThetaRm> = normWireObject<ChangeThetaRm>({ newThetaRmC: normWireRequired(normWireNumber) });
