/** 🎻 `change-prestress-force` wire twin: the leaf payload `ChangePrestressForce`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePrestressForce {
  wallId: string;
  id: string;
  newForceN: number;
}

export const parseChangePrestressForce: NormWireReader<ChangePrestressForce> = normWireObject<ChangePrestressForce>({ wallId: normWireRequired(normWireString), id: normWireRequired(normWireString), newForceN: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000})) });
