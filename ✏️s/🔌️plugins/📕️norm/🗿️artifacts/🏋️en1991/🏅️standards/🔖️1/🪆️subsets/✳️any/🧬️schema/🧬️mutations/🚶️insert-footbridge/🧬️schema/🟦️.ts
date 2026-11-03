/** 🚶 `insert-footbridge` wire twin: the leaf payload `InsertFootbridge`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Footbridge, parseFootbridge } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFootbridge {
  index: number;
  item: Footbridge;
}

export const parseInsertFootbridge: NormWireReader<InsertFootbridge> = normWireObject<InsertFootbridge>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseFootbridge) });
