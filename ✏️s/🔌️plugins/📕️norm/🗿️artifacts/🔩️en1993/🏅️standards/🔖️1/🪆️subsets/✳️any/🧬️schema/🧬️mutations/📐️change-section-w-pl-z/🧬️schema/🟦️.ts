/** 📐️ `change-section-w-pl-z` wire twin: the leaf payload `ChangeSectionWPlZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionWPlZ {
  id: string;
  newWPlZ: number;
}

export const parseChangeSectionWPlZ: NormWireReader<ChangeSectionWPlZ> = normWireObject<ChangeSectionWPlZ>({ id: normWireRequired(normWireString), newWPlZ: normWireRequired(normWireNumber) });
