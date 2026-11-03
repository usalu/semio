/** 🧱️ `change-plated-stiffener-ecc` wire twin: the leaf payload `ChangePlatedStiffenerEcc`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedStiffenerEcc {
  id: string;
  newStiffenerEccentricity: number;
}

export const parseChangePlatedStiffenerEcc: NormWireReader<ChangePlatedStiffenerEcc> = normWireObject<ChangePlatedStiffenerEcc>({ id: normWireRequired(normWireString), newStiffenerEccentricity: normWireRequired(normWireNumber) });
