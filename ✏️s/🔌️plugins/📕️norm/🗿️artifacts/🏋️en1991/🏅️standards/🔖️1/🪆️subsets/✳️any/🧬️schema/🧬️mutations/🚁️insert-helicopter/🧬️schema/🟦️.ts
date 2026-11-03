/** 🚁 `insert-helicopter` wire twin: the leaf payload `InsertHelicopter`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type HelicopterLanding, parseHelicopterLanding } from "../../../📸️snapshot/🟦️.ts";

export interface InsertHelicopter {
  index: number;
  item: HelicopterLanding;
}

export const parseInsertHelicopter: NormWireReader<InsertHelicopter> = normWireObject<InsertHelicopter>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseHelicopterLanding) });
