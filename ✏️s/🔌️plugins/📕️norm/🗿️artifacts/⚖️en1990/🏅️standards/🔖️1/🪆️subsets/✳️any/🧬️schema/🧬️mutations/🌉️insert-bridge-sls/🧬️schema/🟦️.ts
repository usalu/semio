/** 🌉 `insert-bridge-sls` wire twin: the leaf payload `InsertBridgeSls`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1990BridgeSls, parseEn1990BridgeSls } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBridgeSls {
  mutation: "insertBridgeSls";
  index?: number;
  item: En1990BridgeSls;
}

export const parseInsertBridgeSls: NormWireReader<InsertBridgeSls> = normWireObject<InsertBridgeSls>({ mutation: normWireRequired(normWireLiteral("insertBridgeSls")), index: normWireOptional(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseEn1990BridgeSls) });
