/** 🕸️ `change-column-rebar-f-yk-pa` wire twin: the leaf payload `ChangeColumnRebarFYkPa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnRebarFYkPa {
  id: string;
  newReinforcementFYkPa: number;
}

export const parseChangeColumnRebarFYkPa: NormWireReader<ChangeColumnRebarFYkPa> = normWireObject<ChangeColumnRebarFYkPa>({ id: normWireRequired(normWireString), newReinforcementFYkPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
