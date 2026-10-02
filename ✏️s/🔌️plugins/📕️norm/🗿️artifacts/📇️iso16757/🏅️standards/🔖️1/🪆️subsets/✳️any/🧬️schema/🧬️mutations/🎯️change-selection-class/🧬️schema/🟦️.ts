/** 🎯️ `change-selection-class` wire twin: the leaf payload `ChangeSelectionClass`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSelectionClass {
  newClassId: string;
}

export const parseChangeSelectionClass: NormWireReader<ChangeSelectionClass> = normWireObject<ChangeSelectionClass>({ newClassId: normWireRequired(normWireString) });
