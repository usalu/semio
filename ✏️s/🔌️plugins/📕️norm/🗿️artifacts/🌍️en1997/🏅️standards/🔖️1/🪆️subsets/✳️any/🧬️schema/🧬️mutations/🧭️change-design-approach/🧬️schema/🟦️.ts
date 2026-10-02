/** 🧭️ `change-design-approach` wire twin: the leaf payload `ChangeDesignApproach`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeDesignApproach {
  mutation: "changeDesignApproach";
  newDesignApproach: string;
}

export const parseChangeDesignApproach: NormWireReader<ChangeDesignApproach> = normWireObject<ChangeDesignApproach>({ mutation: normWireRequired(normWireLiteral("changeDesignApproach")), newDesignApproach: normWireRequired(normWireString) });
