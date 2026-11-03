/** 🌉 `insert-bridge` wire twin: the leaf payload `InsertBridge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1998Bridge, parseEn1998Bridge } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBridge {
  mutation: "insertBridge";
  index?: number | null;
  bridge: En1998Bridge;
}

export const parseInsertBridge: NormWireReader<InsertBridge> = normWireObject<InsertBridge>({ mutation: normWireRequired(normWireLiteral("insertBridge")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), bridge: normWireRequired(parseEn1998Bridge) });
