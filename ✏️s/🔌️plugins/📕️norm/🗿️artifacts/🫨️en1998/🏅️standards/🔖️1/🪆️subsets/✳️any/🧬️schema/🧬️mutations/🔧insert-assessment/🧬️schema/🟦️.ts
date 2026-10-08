/** 🔧 `insert-assessment` wire twin: the leaf payload `InsertAssessment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireNullable, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1998Assessment, parseEn1998Assessment } from "../../../📸️snapshot/🟦️.ts";

export interface InsertAssessment {
  mutation: "insertAssessment";
  index?: number | null;
  assessment: En1998Assessment;
}

export const parseInsertAssessment: NormWireReader<InsertAssessment> = normWireObject<InsertAssessment>({ mutation: normWireRequired(normWireLiteral("insertAssessment")), index: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), assessment: normWireRequired(parseEn1998Assessment) });
