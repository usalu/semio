/** 📅 `change-design-working-life-category` wire twin: the leaf payload `ChangeDesignWorkingLifeCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeDesignWorkingLifeCategory {
  mutation: "changeDesignWorkingLifeCategory";
  newDesignWorkingLifeCategory: number;
}

export const parseChangeDesignWorkingLifeCategory: NormWireReader<ChangeDesignWorkingLifeCategory> = normWireObject<ChangeDesignWorkingLifeCategory>({ mutation: normWireRequired(normWireLiteral("changeDesignWorkingLifeCategory")), newDesignWorkingLifeCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
