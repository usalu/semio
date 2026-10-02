/** ➖️ `remove-building` wire twin: the leaf payload `RemoveBuilding`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveBuilding {
  mutation: "removeBuilding";
  index: number;
}

export const parseRemoveBuilding: NormWireReader<RemoveBuilding> = normWireObject<RemoveBuilding>({ mutation: normWireRequired(normWireLiteral("removeBuilding")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
