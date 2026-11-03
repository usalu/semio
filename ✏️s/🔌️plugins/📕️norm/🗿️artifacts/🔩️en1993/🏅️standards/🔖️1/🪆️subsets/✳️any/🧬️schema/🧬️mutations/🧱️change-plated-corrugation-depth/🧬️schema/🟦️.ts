/** 🧱️ `change-plated-corrugation-depth` wire twin: the leaf payload `ChangePlatedCorrugationDepth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedCorrugationDepth {
  id: string;
  newCorrugationDepth: number;
}

export const parseChangePlatedCorrugationDepth: NormWireReader<ChangePlatedCorrugationDepth> = normWireObject<ChangePlatedCorrugationDepth>({ id: normWireRequired(normWireString), newCorrugationDepth: normWireRequired(normWireNumber) });
