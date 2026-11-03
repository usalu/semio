/** ➕️ `insert-cutout` wire twin: the leaf payload `InsertCutout`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Cutout, parseCutout } from "../../../📸️snapshot/🟦️.ts";

export interface InsertCutout {
  index: number;
  cutout: Cutout;
}

export const parseInsertCutout: NormWireReader<InsertCutout> = normWireObject<InsertCutout>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), cutout: normWireRequired(parseCutout) });
