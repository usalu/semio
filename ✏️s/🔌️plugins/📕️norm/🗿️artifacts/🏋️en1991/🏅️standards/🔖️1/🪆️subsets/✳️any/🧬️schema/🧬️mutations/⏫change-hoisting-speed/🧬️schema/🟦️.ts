/** ⏫ `change-hoisting-speed` wire twin: the leaf payload `ChangeHoistingSpeed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeHoistingSpeed {
  newHoistingSpeed: number;
}

export const parseChangeHoistingSpeed: NormWireReader<ChangeHoistingSpeed> = normWireObject<ChangeHoistingSpeed>({ newHoistingSpeed: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
