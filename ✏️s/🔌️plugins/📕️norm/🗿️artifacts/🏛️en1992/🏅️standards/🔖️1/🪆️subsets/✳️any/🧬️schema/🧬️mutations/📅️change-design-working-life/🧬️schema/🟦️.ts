/** 📅️ `change-design-working-life` wire twin: the leaf payload `ChangeDesignWorkingLife`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDesignWorkingLife {
  newYears: number;
}

export const parseChangeDesignWorkingLife: NormWireReader<ChangeDesignWorkingLife> = normWireObject<ChangeDesignWorkingLife>({ newYears: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
