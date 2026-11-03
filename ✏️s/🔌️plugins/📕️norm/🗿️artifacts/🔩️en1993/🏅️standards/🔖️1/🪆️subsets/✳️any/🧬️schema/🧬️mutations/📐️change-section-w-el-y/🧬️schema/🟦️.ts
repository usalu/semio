/** 📐️ `change-section-w-el-y` wire twin: the leaf payload `ChangeSectionWElY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSectionWElY {
  id: string;
  newWElY: number;
}

export const parseChangeSectionWElY: NormWireReader<ChangeSectionWElY> = normWireObject<ChangeSectionWElY>({ id: normWireRequired(normWireString), newWElY: normWireRequired(normWireNumber) });
