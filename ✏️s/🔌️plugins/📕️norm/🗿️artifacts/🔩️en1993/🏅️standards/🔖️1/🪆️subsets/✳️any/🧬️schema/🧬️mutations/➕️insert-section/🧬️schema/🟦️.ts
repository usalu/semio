/** ➕️ `insert-section` wire twin: the leaf payload `InsertSection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SteelSection, parseSteelSection } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSection {
  index?: number | null;
  section: SteelSection;
}

export const parseInsertSection: NormWireReader<InsertSection> = normWireObject<InsertSection>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), section: normWireRequired(parseSteelSection) });
