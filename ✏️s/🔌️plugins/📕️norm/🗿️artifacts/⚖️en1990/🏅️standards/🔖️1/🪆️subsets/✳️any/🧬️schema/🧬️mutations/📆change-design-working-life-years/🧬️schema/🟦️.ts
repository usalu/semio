/** 📆 `change-design-working-life-years` wire twin: the leaf payload `ChangeDesignWorkingLifeYears`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDesignWorkingLifeYears {
  mutation: "changeDesignWorkingLifeYears";
  newDesignWorkingLifeYears: number;
}

export const parseChangeDesignWorkingLifeYears: NormWireReader<ChangeDesignWorkingLifeYears> = normWireObject<ChangeDesignWorkingLifeYears>({ mutation: normWireRequired(normWireLiteral("changeDesignWorkingLifeYears")), newDesignWorkingLifeYears: normWireRequired(normWireNumber) });
