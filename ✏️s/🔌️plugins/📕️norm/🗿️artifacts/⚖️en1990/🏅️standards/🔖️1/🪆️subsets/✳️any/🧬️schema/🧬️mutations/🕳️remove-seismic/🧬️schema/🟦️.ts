/** 🕳️ `remove-seismic` wire twin: the leaf payload `RemoveSeismic`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveSeismic {
  mutation: "removeSeismic";
  index: number;
}

export const parseRemoveSeismic: NormWireReader<RemoveSeismic> = normWireObject<RemoveSeismic>({ mutation: normWireRequired(normWireLiteral("removeSeismic")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
