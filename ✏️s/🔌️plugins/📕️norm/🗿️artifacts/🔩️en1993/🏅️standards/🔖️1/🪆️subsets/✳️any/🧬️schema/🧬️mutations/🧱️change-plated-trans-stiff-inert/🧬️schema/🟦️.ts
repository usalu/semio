/** 🧱️ `change-plated-trans-stiff-inert` wire twin: the leaf payload `ChangePlatedTransStiffInert`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedTransStiffInert {
  id: string;
  newTransverseStiffenerInertia: number;
}

export const parseChangePlatedTransStiffInert: NormWireReader<ChangePlatedTransStiffInert> = normWireObject<ChangePlatedTransStiffInert>({ id: normWireRequired(normWireString), newTransverseStiffenerInertia: normWireRequired(normWireNumber) });
