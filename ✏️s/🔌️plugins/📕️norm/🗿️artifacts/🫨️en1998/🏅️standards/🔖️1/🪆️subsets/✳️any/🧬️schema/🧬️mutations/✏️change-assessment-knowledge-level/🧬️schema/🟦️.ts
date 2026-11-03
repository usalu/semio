/** ✏️ `change-assessment-knowledge-level` wire twin: the leaf payload `ChangeAssessmentKnowledgeLevel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type KnowledgeLevel, parseKnowledgeLevel } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeAssessmentKnowledgeLevel {
  mutation: "changeAssessmentKnowledgeLevel";
  id: string;
  newKnowledgeLevel: KnowledgeLevel;
}

export const parseChangeAssessmentKnowledgeLevel: NormWireReader<ChangeAssessmentKnowledgeLevel> = normWireObject<ChangeAssessmentKnowledgeLevel>({ mutation: normWireRequired(normWireLiteral("changeAssessmentKnowledgeLevel")), id: normWireRequired(normWireString), newKnowledgeLevel: normWireRequired(parseKnowledgeLevel) });
