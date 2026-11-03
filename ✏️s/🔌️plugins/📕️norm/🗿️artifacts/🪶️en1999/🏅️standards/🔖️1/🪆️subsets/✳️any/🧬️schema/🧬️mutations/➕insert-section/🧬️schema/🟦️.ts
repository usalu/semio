/** 🗂️ `insert-section` wire twin: the leaf payload `InsertSection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type AluminiumSection, parseAluminiumSection } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSection {
  mutation: "insertSection";
  index: number;
  section: AluminiumSection;
}

export const parseInsertSection: NormWireReader<InsertSection> = normWireObject<InsertSection>({ mutation: normWireRequired(normWireLiteral("insertSection")), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), section: normWireRequired(parseAluminiumSection) });
