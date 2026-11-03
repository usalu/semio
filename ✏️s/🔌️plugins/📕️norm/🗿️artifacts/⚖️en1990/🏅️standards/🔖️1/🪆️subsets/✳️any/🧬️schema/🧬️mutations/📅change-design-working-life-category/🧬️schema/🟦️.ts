/** 📅 `change-design-working-life-category` wire twin: the leaf payload `ChangeDesignWorkingLifeCategory`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireLiteral, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDesignWorkingLifeCategory {
  mutation: "changeDesignWorkingLifeCategory";
  newDesignWorkingLifeCategory: number;
}

export const parseChangeDesignWorkingLifeCategory: NormWireReader<ChangeDesignWorkingLifeCategory> = normWireObject<ChangeDesignWorkingLifeCategory>({ mutation: normWireRequired(normWireLiteral("changeDesignWorkingLifeCategory")), newDesignWorkingLifeCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
