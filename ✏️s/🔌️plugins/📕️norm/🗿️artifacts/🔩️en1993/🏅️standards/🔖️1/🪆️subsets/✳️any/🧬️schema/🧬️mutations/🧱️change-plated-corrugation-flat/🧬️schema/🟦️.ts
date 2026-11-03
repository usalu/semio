/** 🧱️ `change-plated-corrugation-flat` wire twin: the leaf payload `ChangePlatedCorrugationFlat`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedCorrugationFlat {
  id: string;
  newCorrugationFlat: number;
}

export const parseChangePlatedCorrugationFlat: NormWireReader<ChangePlatedCorrugationFlat> = normWireObject<ChangePlatedCorrugationFlat>({ id: normWireRequired(normWireString), newCorrugationFlat: normWireRequired(normWireNumber) });
