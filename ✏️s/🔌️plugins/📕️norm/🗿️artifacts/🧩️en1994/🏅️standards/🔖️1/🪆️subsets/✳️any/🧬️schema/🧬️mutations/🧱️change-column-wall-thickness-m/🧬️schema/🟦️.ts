/** 🧱️ `change-column-wall-thickness-m` wire twin: the leaf payload `ChangeColumnWallThicknessM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnWallThicknessM {
  id: string;
  newWallThicknessM: number;
}

export const parseChangeColumnWallThicknessM: NormWireReader<ChangeColumnWallThicknessM> = normWireObject<ChangeColumnWallThicknessM>({ id: normWireRequired(normWireString), newWallThicknessM: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
