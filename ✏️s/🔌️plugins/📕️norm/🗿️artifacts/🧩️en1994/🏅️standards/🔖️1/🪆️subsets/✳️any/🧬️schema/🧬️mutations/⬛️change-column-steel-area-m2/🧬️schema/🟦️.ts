/** ⬛️ `change-column-steel-area-m2` wire twin: the leaf payload `ChangeColumnSteelAreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnSteelAreaM2 {
  id: string;
  newSteelAM2: number;
}

export const parseChangeColumnSteelAreaM2: NormWireReader<ChangeColumnSteelAreaM2> = normWireObject<ChangeColumnSteelAreaM2>({ id: normWireRequired(normWireString), newSteelAM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
