/** 🏷️ `change-silo-height` wire twin: the leaf payload `ChangeSiloHeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeSiloHeight {
  newSiloHeight: number;
}

export const parseChangeSiloHeight: NormWireReader<ChangeSiloHeight> = normWireObject<ChangeSiloHeight>({ newSiloHeight: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
