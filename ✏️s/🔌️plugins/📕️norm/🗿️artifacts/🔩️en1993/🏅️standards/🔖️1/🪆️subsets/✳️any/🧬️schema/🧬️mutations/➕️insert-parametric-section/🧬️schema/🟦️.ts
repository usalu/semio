/** ➕️ `insert-parametric-section` wire twin: the leaf payload `InsertParametricSection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ParametricSection, parseParametricSection } from "../../../📸️snapshot/🟦️.ts";

export interface InsertParametricSection {
  index?: number | null;
  parametricSection: ParametricSection;
}

export const parseInsertParametricSection: NormWireReader<InsertParametricSection> = normWireObject<InsertParametricSection>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), parametricSection: normWireRequired(parseParametricSection) });
