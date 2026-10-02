/** 🔎️ `change-investigation-depth` wire twin: the leaf payload `ChangeInvestigationDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeInvestigationDepth {
  mutation: "changeInvestigationDepth";
  newInvestigationDepth: number;
}

export const parseChangeInvestigationDepth: NormWireReader<ChangeInvestigationDepth> = normWireObject<ChangeInvestigationDepth>({ mutation: normWireRequired(normWireLiteral("changeInvestigationDepth")), newInvestigationDepth: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
