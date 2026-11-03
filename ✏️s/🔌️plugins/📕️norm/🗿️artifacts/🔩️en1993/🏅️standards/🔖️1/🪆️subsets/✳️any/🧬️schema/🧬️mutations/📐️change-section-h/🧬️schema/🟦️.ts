/** 📐️ `change-section-h` wire twin: the leaf payload `ChangeSectionH`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionH {
  id: string;
  newH: number;
}

export const parseChangeSectionH: NormWireReader<ChangeSectionH> = normWireObject<ChangeSectionH>({ id: normWireRequired(normWireString), newH: normWireRequired(normWireNumber) });
