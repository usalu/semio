/** 🧱️ `change-plated-patch-length` wire twin: the leaf payload `ChangePlatedPatchLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedPatchLength {
  id: string;
  newPatchLength: number;
}

export const parseChangePlatedPatchLength: NormWireReader<ChangePlatedPatchLength> = normWireObject<ChangePlatedPatchLength>({ id: normWireRequired(normWireString), newPatchLength: normWireRequired(normWireNumber) });
