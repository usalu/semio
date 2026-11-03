/** ➕️ `insert-wind-face` wire twin: the leaf payload `InsertWindFace`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type WindFace, parseWindFace } from "../../../📸️snapshot/🟦️.ts";

export interface InsertWindFace {
  index: number;
  item: WindFace;
}

export const parseInsertWindFace: NormWireReader<InsertWindFace> = normWireObject<InsertWindFace>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseWindFace) });
