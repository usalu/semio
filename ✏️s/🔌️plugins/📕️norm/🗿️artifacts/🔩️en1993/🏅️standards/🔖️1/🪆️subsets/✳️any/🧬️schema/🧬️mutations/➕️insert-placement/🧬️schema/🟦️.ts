/** ➕️ `insert-placement` wire twin: the leaf payload `InsertPlacement`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Placement, parsePlacement } from "../../../📸️snapshot/🟦️.ts";

export interface InsertPlacement {
  index?: number | null;
  placement: Placement;
}

export const parseInsertPlacement: NormWireReader<InsertPlacement> = normWireObject<InsertPlacement>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), placement: normWireRequired(parsePlacement) });
