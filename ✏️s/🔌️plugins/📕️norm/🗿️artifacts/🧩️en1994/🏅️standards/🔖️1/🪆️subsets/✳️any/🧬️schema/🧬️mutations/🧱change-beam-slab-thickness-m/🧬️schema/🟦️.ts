/** 🧱 `change-beam-slab-thickness-m` wire twin: the leaf payload `ChangeBeamSlabThicknessM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBeamSlabThicknessM {
  id: string;
  newSlabThicknessM: number;
}

export const parseChangeBeamSlabThicknessM: NormWireReader<ChangeBeamSlabThicknessM> = normWireObject<ChangeBeamSlabThicknessM>({ id: normWireRequired(normWireString), newSlabThicknessM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
