/** 🧱️ `change-plated-patch-moment` wire twin: the leaf payload `ChangePlatedPatchMoment`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedPatchMoment {
  id: string;
  newPatchMoment: number;
}

export const parseChangePlatedPatchMoment: NormWireReader<ChangePlatedPatchMoment> = normWireObject<ChangePlatedPatchMoment>({ id: normWireRequired(normWireString), newPatchMoment: normWireRequired(normWireNumber) });
