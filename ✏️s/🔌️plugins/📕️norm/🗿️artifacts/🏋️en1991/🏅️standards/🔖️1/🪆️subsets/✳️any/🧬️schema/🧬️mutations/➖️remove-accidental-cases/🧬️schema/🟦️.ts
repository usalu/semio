/** ➖️ `remove-accidental-cases` wire twin: the leaf payload `RemoveAccidentalCases`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveAccidentalCases {
  index: number;
}

export const parseRemoveAccidentalCases: NormWireReader<RemoveAccidentalCases> = normWireObject<RemoveAccidentalCases>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
