/** 🔢 `change-pile-count` wire twin: the leaf payload `ChangePileCount`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangePileCount {
  mutation: "changePileCount";
  id: string;
  newCount: number;
}

export const parseChangePileCount: NormWireReader<ChangePileCount> = normWireObject<ChangePileCount>({ mutation: normWireRequired(normWireLiteral("changePileCount")), id: normWireRequired(normWireString), newCount: normWireRequired(normWireRange(normWireInteger, {"minimum":1})) });
