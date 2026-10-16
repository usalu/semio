/** ⏫ `change-hoisting-speed` wire twin: the leaf payload `ChangeHoistingSpeed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHoistingSpeed {
  newHoistingSpeed: number;
}

export const parseChangeHoistingSpeed: NormWireReader<ChangeHoistingSpeed> = normWireObject<ChangeHoistingSpeed>({ newHoistingSpeed: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
