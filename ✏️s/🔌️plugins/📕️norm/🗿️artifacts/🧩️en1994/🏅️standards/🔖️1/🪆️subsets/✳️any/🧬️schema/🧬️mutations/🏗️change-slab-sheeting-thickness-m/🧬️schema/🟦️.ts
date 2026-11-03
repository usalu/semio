/** 🏗️ `change-slab-sheeting-thickness-m` wire twin: the leaf payload `ChangeSlabSheetingThicknessM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlabSheetingThicknessM {
  id: string;
  newThicknessM: number;
}

export const parseChangeSlabSheetingThicknessM: NormWireReader<ChangeSlabSheetingThicknessM> = normWireObject<ChangeSlabSheetingThicknessM>({ id: normWireRequired(normWireString), newThicknessM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
