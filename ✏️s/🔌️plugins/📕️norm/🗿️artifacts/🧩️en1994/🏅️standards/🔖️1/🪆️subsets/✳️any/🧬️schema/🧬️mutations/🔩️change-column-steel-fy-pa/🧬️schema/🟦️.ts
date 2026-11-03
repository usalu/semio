/** 🔩️ `change-column-steel-fy-pa` wire twin: the leaf payload `ChangeColumnSteelFYPa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnSteelFYPa {
  id: string;
  newSteelFYPa: number;
}

export const parseChangeColumnSteelFYPa: NormWireReader<ChangeColumnSteelFYPa> = normWireObject<ChangeColumnSteelFYPa>({ id: normWireRequired(normWireString), newSteelFYPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
