/** 🧪 `change-cement-type` wire twin: the leaf payload `ChangeCementType`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeCementType {
  newCementType: string;
}

export const parseChangeCementType: NormWireReader<ChangeCementType> = normWireObject<ChangeCementType>({ newCementType: normWireRequired(normWireString) });
