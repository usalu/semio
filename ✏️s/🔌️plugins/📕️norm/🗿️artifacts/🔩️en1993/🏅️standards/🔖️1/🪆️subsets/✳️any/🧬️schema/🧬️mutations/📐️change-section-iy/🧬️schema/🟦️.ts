/** 📐️ `change-section-iy` wire twin: the leaf payload `ChangeSectionIy`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionIy {
  id: string;
  newIy: number;
}

export const parseChangeSectionIy: NormWireReader<ChangeSectionIy> = normWireObject<ChangeSectionIy>({ id: normWireRequired(normWireString), newIy: normWireRequired(normWireNumber) });
