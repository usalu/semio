/** 🏗️ `change-slab-sheeting-rib-width-m` wire twin: the leaf payload `ChangeSlabSheetingRibWidthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlabSheetingRibWidthM {
  id: string;
  newRibWidthM: number;
}

export const parseChangeSlabSheetingRibWidthM: NormWireReader<ChangeSlabSheetingRibWidthM> = normWireObject<ChangeSlabSheetingRibWidthM>({ id: normWireRequired(normWireString), newRibWidthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
