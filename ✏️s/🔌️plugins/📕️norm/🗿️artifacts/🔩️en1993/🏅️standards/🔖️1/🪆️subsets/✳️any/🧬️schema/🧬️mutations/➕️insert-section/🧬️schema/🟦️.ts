/** ➕️ `insert-section` wire twin: the leaf payload `InsertSection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseSteelSection, type SteelSection } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSection {
  index?: number | null;
  section: SteelSection;
}

export const parseInsertSection: NormWireReader<InsertSection> = normWireObject<InsertSection>({ index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), section: normWireRequired(parseSteelSection) });
