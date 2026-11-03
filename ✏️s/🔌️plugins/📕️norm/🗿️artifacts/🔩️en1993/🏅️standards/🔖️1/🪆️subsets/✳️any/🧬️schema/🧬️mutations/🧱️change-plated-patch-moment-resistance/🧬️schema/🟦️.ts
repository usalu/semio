/** 🧱️ `change-plated-patch-moment-resistance` wire twin: the leaf payload `ChangePlatedPatchMomentResistance`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePlatedPatchMomentResistance {
  id: string;
  newPatchMomentResistance: number;
}

export const parseChangePlatedPatchMomentResistance: NormWireReader<ChangePlatedPatchMomentResistance> = normWireObject<ChangePlatedPatchMomentResistance>({ id: normWireRequired(normWireString), newPatchMomentResistance: normWireRequired(normWireNumber) });
