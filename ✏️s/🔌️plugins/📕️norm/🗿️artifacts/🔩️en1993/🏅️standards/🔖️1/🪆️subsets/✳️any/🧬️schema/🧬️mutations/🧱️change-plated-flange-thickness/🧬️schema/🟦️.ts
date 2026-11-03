/** 🧱️ `change-plated-flange-thickness` wire twin: the leaf payload `ChangePlatedFlangeThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedFlangeThickness {
  id: string;
  newFlangeThickness: number;
}

export const parseChangePlatedFlangeThickness: NormWireReader<ChangePlatedFlangeThickness> = normWireObject<ChangePlatedFlangeThickness>({ id: normWireRequired(normWireString), newFlangeThickness: normWireRequired(normWireNumber) });
