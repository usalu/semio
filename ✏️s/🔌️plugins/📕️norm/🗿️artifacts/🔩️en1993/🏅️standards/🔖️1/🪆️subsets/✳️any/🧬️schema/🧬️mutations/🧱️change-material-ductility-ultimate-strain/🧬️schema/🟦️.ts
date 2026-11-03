/** 🧱️ `change-material-ductility-ultimate-strain` wire twin: the leaf payload `ChangeMaterialDuctilityUltimateStrain`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMaterialDuctilityUltimateStrain {
  id: string;
  newUltimateStrain: number;
}

export const parseChangeMaterialDuctilityUltimateStrain: NormWireReader<ChangeMaterialDuctilityUltimateStrain> = normWireObject<ChangeMaterialDuctilityUltimateStrain>({ id: normWireRequired(normWireString), newUltimateStrain: normWireRequired(normWireNumber) });
