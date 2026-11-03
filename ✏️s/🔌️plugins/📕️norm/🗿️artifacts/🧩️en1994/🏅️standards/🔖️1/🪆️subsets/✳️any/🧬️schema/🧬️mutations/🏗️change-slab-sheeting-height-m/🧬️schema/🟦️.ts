/** 🏗️ `change-slab-sheeting-height-m` wire twin: the leaf payload `ChangeSlabSheetingHeightM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlabSheetingHeightM {
  id: string;
  newHeightM: number;
}

export const parseChangeSlabSheetingHeightM: NormWireReader<ChangeSlabSheetingHeightM> = normWireObject<ChangeSlabSheetingHeightM>({ id: normWireRequired(normWireString), newHeightM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
