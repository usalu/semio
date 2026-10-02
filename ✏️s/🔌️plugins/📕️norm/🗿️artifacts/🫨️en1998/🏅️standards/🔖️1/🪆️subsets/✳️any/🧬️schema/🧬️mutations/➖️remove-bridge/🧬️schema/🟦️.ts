/** ➖️ `remove-bridge` wire twin: the leaf payload `RemoveBridge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface RemoveBridge {
  mutation: "removeBridge";
  index: number;
}

export const parseRemoveBridge: NormWireReader<RemoveBridge> = normWireObject<RemoveBridge>({ mutation: normWireRequired(normWireLiteral("removeBridge")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
