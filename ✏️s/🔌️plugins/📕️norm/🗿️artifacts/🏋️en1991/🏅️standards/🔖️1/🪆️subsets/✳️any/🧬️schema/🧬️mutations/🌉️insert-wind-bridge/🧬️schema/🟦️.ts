/** 🌉 `insert-wind-bridge` wire twin: the leaf payload `InsertWindBridge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WindBridge, parseWindBridge } from "../../../📸️snapshot/🟦️.ts";

export interface InsertWindBridge {
  index: number;
  item: WindBridge;
}

export const parseInsertWindBridge: NormWireReader<InsertWindBridge> = normWireObject<InsertWindBridge>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseWindBridge) });
