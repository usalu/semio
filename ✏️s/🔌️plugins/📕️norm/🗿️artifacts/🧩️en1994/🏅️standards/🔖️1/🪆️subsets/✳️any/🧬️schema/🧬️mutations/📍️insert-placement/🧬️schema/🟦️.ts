/** 📍️ `insert-placement` wire twin: the leaf payload `InsertPlacement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Placement, parsePlacement } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPlacement {
  index: number;
  item: Placement;
}

export const parseInsertPlacement: NormWireReader<InsertPlacement> = normWireObject<InsertPlacement>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parsePlacement) });
