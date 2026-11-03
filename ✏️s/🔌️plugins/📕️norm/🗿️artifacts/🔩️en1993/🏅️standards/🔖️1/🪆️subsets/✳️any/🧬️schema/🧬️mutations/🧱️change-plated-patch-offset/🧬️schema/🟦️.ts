/** 🧱️ `change-plated-patch-offset` wire twin: the leaf payload `ChangePlatedPatchOffset`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedPatchOffset {
  id: string;
  newPatchOffset: number;
}

export const parseChangePlatedPatchOffset: NormWireReader<ChangePlatedPatchOffset> = normWireObject<ChangePlatedPatchOffset>({ id: normWireRequired(normWireString), newPatchOffset: normWireRequired(normWireNumber) });
