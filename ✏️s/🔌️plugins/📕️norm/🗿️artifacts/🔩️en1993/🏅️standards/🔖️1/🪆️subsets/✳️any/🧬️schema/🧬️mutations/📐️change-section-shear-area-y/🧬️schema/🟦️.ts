/** 📐️ `change-section-shear-area-y` wire twin: the leaf payload `ChangeSectionShearAreaY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionShearAreaY {
  id: string;
  newShearAreaY: number;
}

export const parseChangeSectionShearAreaY: NormWireReader<ChangeSectionShearAreaY> = normWireObject<ChangeSectionShearAreaY>({ id: normWireRequired(normWireString), newShearAreaY: normWireRequired(normWireNumber) });
