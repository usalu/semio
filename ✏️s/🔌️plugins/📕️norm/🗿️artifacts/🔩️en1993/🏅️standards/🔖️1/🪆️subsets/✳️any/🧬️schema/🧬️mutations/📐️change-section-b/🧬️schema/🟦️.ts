/** 📐️ `change-section-b` wire twin: the leaf payload `ChangeSectionB`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionB {
  id: string;
  newB: number;
}

export const parseChangeSectionB: NormWireReader<ChangeSectionB> = normWireObject<ChangeSectionB>({ id: normWireRequired(normWireString), newB: normWireRequired(normWireNumber) });
