/** 🧱️ `change-plated-stiffener-flange-thk` wire twin: the leaf payload `ChangePlatedStiffenerFlangeThk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedStiffenerFlangeThk {
  id: string;
  newStiffenerFlangeThickness: number;
}

export const parseChangePlatedStiffenerFlangeThk: NormWireReader<ChangePlatedStiffenerFlangeThk> = normWireObject<ChangePlatedStiffenerFlangeThk>({ id: normWireRequired(normWireString), newStiffenerFlangeThickness: normWireRequired(normWireNumber) });
