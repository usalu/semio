/** 📐️ `change-section-tf` wire twin: the leaf payload `ChangeSectionTf`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionTf {
  id: string;
  newTf: number;
}

export const parseChangeSectionTf: NormWireReader<ChangeSectionTf> = normWireObject<ChangeSectionTf>({ id: normWireRequired(normWireString), newTf: normWireRequired(normWireNumber) });
