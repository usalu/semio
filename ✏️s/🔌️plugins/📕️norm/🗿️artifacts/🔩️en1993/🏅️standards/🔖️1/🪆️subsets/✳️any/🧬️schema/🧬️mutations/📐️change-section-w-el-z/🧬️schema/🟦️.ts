/** 📐️ `change-section-w-el-z` wire twin: the leaf payload `ChangeSectionWElZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionWElZ {
  id: string;
  newWElZ: number;
}

export const parseChangeSectionWElZ: NormWireReader<ChangeSectionWElZ> = normWireObject<ChangeSectionWElZ>({ id: normWireRequired(normWireString), newWElZ: normWireRequired(normWireNumber) });
