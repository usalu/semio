/** 🕸️ `change-column-rebar-area-m2` wire twin: the leaf payload `ChangeColumnRebarAreaM2`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnRebarAreaM2 {
  id: string;
  newReinforcementAsM2: number;
}

export const parseChangeColumnRebarAreaM2: NormWireReader<ChangeColumnRebarAreaM2> = normWireObject<ChangeColumnRebarAreaM2>({ id: normWireRequired(normWireString), newReinforcementAsM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
