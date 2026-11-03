/** ➕️ `insert-fatigue-stress` wire twin: the leaf payload `InsertFatigueStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type FatigueStress, parseFoundationBlock } from "../../../📸️snapshot/🟦️.ts";

export interface InsertFatigueStress {
  index?: number | null;
  fatigueStress: FatigueStress;
}

export const parseInsertFatigueStress: NormWireReader<InsertFatigueStress> = normWireObject<InsertFatigueStress>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), fatigueStress: normWireRequired(parseFoundationBlock) });
