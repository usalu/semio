/** ➖️ `remove-retaining-wall` wire twin: the leaf payload `RemoveRetainingWall`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveRetainingWall {
  mutation: "removeRetainingWall";
  index: number;
}

export const parseRemoveRetainingWall: NormWireReader<RemoveRetainingWall> = normWireObject<RemoveRetainingWall>({ mutation: normWireRequired(normWireLiteral("removeRetainingWall")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
