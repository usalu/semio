/** 📅️ `change-design-working-life` wire twin: the leaf payload `ChangeDesignWorkingLife`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";

export interface ChangeDesignWorkingLife {
  newYears: number;
}

export const parseChangeDesignWorkingLife: NormWireReader<ChangeDesignWorkingLife> = normWireObject<ChangeDesignWorkingLife>({ newYears: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
