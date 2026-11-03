/** ➕️ `insert-beam` wire twin: the leaf payload `InsertBeam`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type CompositeBeam, parseCompositeBeam } from "../../../📸️snapshot/🟦️.ts";

export interface InsertBeam {
  index: number;
  beam: CompositeBeam;
}

export const parseInsertBeam: NormWireReader<InsertBeam> = normWireObject<InsertBeam>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), beam: normWireRequired(parseCompositeBeam) });
