/** ⚙ `insert-support-collision` wire twin: the leaf payload `InsertSupportCollision`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SupportCollision, parseSupportCollision } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSupportCollision {
  index: number;
  item: SupportCollision;
}

export const parseInsertSupportCollision: NormWireReader<InsertSupportCollision> = normWireObject<InsertSupportCollision>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), item: normWireRequired(parseSupportCollision) });
