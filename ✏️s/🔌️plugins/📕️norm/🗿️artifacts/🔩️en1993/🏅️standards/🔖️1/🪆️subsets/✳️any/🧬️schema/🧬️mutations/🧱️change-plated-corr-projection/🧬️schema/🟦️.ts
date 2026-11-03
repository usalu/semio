/** 🧱️ `change-plated-corr-projection` wire twin: the leaf payload `ChangePlatedCorrProjection`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedCorrProjection {
  id: string;
  newCorrugationProjection: number;
}

export const parseChangePlatedCorrProjection: NormWireReader<ChangePlatedCorrProjection> = normWireObject<ChangePlatedCorrProjection>({ id: normWireRequired(normWireString), newCorrugationProjection: normWireRequired(normWireNumber) });
