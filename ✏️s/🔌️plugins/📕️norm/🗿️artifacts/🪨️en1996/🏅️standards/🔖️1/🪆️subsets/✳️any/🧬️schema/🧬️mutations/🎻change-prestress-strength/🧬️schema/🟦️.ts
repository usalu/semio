/** 🎻 `change-prestress-strength` wire twin: the leaf payload `ChangePrestressStrength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePrestressStrength {
  wallId: string;
  id: string;
  newFPkPa: number;
}

export const parseChangePrestressStrength: NormWireReader<ChangePrestressStrength> = normWireObject<ChangePrestressStrength>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newFPkPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000})) });
