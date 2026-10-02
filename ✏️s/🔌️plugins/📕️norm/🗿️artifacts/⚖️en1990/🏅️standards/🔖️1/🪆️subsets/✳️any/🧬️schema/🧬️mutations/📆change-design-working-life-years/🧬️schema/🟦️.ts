/** 📆 `change-design-working-life-years` wire twin: the leaf payload `ChangeDesignWorkingLifeYears`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeDesignWorkingLifeYears {
  mutation: "changeDesignWorkingLifeYears";
  newDesignWorkingLifeYears: number;
}

export const parseChangeDesignWorkingLifeYears: NormWireReader<ChangeDesignWorkingLifeYears> = normWireObject<ChangeDesignWorkingLifeYears>({ mutation: normWireRequired(normWireLiteral("changeDesignWorkingLifeYears")), newDesignWorkingLifeYears: normWireRequired(normWireNumber) });
