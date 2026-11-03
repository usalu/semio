/** 📐️ `change-section-shear-area-z` wire twin: the leaf payload `ChangeSectionShearAreaZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionShearAreaZ {
  id: string;
  newShearAreaZ: number;
}

export const parseChangeSectionShearAreaZ: NormWireReader<ChangeSectionShearAreaZ> = normWireObject<ChangeSectionShearAreaZ>({ id: normWireRequired(normWireString), newShearAreaZ: normWireRequired(normWireNumber) });
