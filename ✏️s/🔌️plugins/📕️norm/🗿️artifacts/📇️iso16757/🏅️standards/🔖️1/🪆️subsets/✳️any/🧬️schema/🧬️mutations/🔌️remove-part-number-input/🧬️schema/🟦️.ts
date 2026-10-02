/** 🔌️ `remove-part-number-input` wire twin: the leaf payload `RemovePartNumberInput`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemovePartNumberInput {
  key: string;
}

export const parseRemovePartNumberInput: NormWireReader<RemovePartNumberInput> = normWireObject<RemovePartNumberInput>({ key: normWireRequired(normWireString) });
