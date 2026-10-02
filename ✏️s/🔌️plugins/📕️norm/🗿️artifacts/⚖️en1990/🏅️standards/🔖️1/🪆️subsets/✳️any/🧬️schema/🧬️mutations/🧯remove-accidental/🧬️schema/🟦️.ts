/** 🧯 `remove-accidental` wire twin: the leaf payload `RemoveAccidental`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveAccidental {
  mutation: "removeAccidental";
  index: number;
}

export const parseRemoveAccidental: NormWireReader<RemoveAccidental> = normWireObject<RemoveAccidental>({ mutation: normWireRequired(normWireLiteral("removeAccidental")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
