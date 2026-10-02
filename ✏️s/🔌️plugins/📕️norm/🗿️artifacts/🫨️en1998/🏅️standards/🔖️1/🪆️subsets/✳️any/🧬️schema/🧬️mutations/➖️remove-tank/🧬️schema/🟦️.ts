/** ➖️ `remove-tank` wire twin: the leaf payload `RemoveTank`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveTank {
  mutation: "removeTank";
  index: number;
}

export const parseRemoveTank: NormWireReader<RemoveTank> = normWireObject<RemoveTank>({ mutation: normWireRequired(normWireLiteral("removeTank")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
