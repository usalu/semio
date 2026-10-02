/** ➖️ `remove-tower` wire twin: the leaf payload `RemoveTower`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveTower {
  mutation: "removeTower";
  index: number;
}

export const parseRemoveTower: NormWireReader<RemoveTower> = normWireObject<RemoveTower>({ mutation: normWireRequired(normWireLiteral("removeTower")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
