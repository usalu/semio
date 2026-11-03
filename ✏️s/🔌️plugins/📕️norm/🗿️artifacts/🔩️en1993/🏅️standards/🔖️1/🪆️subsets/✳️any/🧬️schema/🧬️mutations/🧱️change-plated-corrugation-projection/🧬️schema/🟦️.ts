/** 🧱️ `change-plated-corrugation-projection` wire twin: the leaf payload `ChangePlatedCorrugationProjection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedCorrugationProjection {
  id: string;
  newCorrugationProjection: number;
}

export const parseChangePlatedCorrugationProjection: NormWireReader<ChangePlatedCorrugationProjection> = normWireObject<ChangePlatedCorrugationProjection>({ id: normWireRequired(normWireString), newCorrugationProjection: normWireRequired(normWireNumber) });
