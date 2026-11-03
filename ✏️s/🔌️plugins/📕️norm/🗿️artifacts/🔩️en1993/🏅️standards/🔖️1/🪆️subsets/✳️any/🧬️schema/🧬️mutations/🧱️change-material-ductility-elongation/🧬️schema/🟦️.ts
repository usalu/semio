/** 🧱️ `change-material-ductility-elongation` wire twin: the leaf payload `ChangeMaterialDuctilityElongation`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMaterialDuctilityElongation {
  id: string;
  newElongation: number;
}

export const parseChangeMaterialDuctilityElongation: NormWireReader<ChangeMaterialDuctilityElongation> = normWireObject<ChangeMaterialDuctilityElongation>({ id: normWireRequired(normWireString), newElongation: normWireRequired(normWireNumber) });
