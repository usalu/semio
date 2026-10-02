/** 📅️ `change-design-situation` wire twin: the leaf payload `ChangeDesignSituation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeDesignSituation {
  mutation: "changeDesignSituation";
  newDesignSituation: string;
}

export const parseChangeDesignSituation: NormWireReader<ChangeDesignSituation> = normWireObject<ChangeDesignSituation>({ mutation: normWireRequired(normWireLiteral("changeDesignSituation")), newDesignSituation: normWireRequired(normWireString) });
