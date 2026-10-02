/** 🌉 `insert-bridge` wire twin: the leaf payload `InsertBridge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1998Bridge, parseEn1998Bridge } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBridge {
  mutation: "insertBridge";
  index: number;
  bridge: En1998Bridge;
}

export const parseInsertBridge: NormWireReader<InsertBridge> = normWireObject<InsertBridge>({ mutation: normWireRequired(normWireLiteral("insertBridge")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), bridge: normWireRequired(parseEn1998Bridge) });
