/** 🏗 `insert-crane-hoist` wire twin: the leaf payload `InsertCraneHoist`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type CraneHoist, parseCraneHoist } from "../../../📸️snapshot/🟦️.ts";

export interface InsertCraneHoist {
  index: number;
  item: CraneHoist;
}

export const parseInsertCraneHoist: NormWireReader<InsertCraneHoist> = normWireObject<InsertCraneHoist>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseCraneHoist) });
