/** 🎭️ `change-load-case-situation` wire twin: the leaf payload `ChangeLoadCaseSituation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeLoadCaseSituation {
  wallIndex: number;
  loadCaseIndex: number;
  newDesignSituation: string;
}

export const parseChangeLoadCaseSituation: NormWireReader<ChangeLoadCaseSituation> = normWireObject<ChangeLoadCaseSituation>({ wallIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), loadCaseIndex: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), newDesignSituation: normWireRequired(normWireString) });
