/** 📐️ `change-section-it` wire twin: the leaf payload `ChangeSectionIt`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionIt {
  id: string;
  newIt: number;
}

export const parseChangeSectionIt: NormWireReader<ChangeSectionIt> = normWireObject<ChangeSectionIt>({ id: normWireRequired(normWireString), newIt: normWireRequired(normWireNumber) });
