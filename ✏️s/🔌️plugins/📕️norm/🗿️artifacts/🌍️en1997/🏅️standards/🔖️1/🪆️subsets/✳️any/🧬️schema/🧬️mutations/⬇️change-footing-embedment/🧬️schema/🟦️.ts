/** ⬇️ `change-footing-embedment` wire twin: the leaf payload `ChangeFootingEmbedment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeFootingEmbedment {
  mutation: "changeFootingEmbedment";
  id: string;
  newEmbedment: number;
}

export const parseChangeFootingEmbedment: NormWireReader<ChangeFootingEmbedment> = normWireObject<ChangeFootingEmbedment>({ mutation: normWireRequired(normWireLiteral("changeFootingEmbedment")), id: normWireRequired(normWireString), newEmbedment: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
