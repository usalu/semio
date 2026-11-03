/** ➕️ `insert-restraint` wire twin: the leaf payload `InsertRestraint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type Restraint, parseRestraint } from "../../../📸️snapshot/🟦️.ts";

export interface InsertRestraint {
  index?: number | null;
  restraint: Restraint;
}

export const parseInsertRestraint: NormWireReader<InsertRestraint> = normWireObject<InsertRestraint>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), restraint: normWireRequired(parseRestraint) });
