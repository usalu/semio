/** 🎭️ `change-design-situation` wire twin: the leaf payload `ChangeDesignSituation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1996DesignSituation, parseEn1996DesignSituation } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeDesignSituation {
  newDesignSituation: En1996DesignSituation;
}

export const parseChangeDesignSituation: NormWireReader<ChangeDesignSituation> = normWireObject<ChangeDesignSituation>({ newDesignSituation: normWireRequired(parseEn1996DesignSituation) });
