/** 🧱️ `change-plated-patch-moment-res` wire twin: the leaf payload `ChangePlatedPatchMomentRes`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedPatchMomentRes {
  id: string;
  newPatchMomentResistance: number;
}

export const parseChangePlatedPatchMomentRes: NormWireReader<ChangePlatedPatchMomentRes> = normWireObject<ChangePlatedPatchMomentRes>({ id: normWireRequired(normWireString), newPatchMomentResistance: normWireRequired(normWireNumber) });
