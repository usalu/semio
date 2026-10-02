/** 🏚️ `change-cellar-area` wire twin: the leaf payload `ChangeCellarArea`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeCellarArea {
  newCellarAreaM2: number;
}

export const parseChangeCellarArea: NormWireReader<ChangeCellarArea> = normWireObject<ChangeCellarArea>({ newCellarAreaM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
