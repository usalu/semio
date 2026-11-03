/** 📏 `change-slab-thickness-m` wire twin: the leaf payload `ChangeSlabThicknessM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeSlabThicknessM {
  id: string;
  newConcreteThicknessM: number;
}

export const parseChangeSlabThicknessM: NormWireReader<ChangeSlabThicknessM> = normWireObject<ChangeSlabThicknessM>({ id: normWireRequired(normWireString), newConcreteThicknessM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
