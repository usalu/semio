/** 📐️ `insert-section-point` wire twin: the leaf payload `InsertSectionPoint`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type SectionPoint, parseSectionPoint } from "../../../📸️snapshot/🟦️.ts";

export interface InsertSectionPoint {
  memberId: string;
  index: number;
  sectionPoint: SectionPoint;
}

export const parseInsertSectionPoint: NormWireReader<InsertSectionPoint> = normWireObject<InsertSectionPoint>({ memberId: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), sectionPoint: normWireRequired(parseSectionPoint) });
