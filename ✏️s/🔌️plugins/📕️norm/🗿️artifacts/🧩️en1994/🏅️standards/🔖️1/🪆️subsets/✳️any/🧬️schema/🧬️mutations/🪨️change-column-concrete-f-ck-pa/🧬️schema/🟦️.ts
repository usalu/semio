/** 🪨️ `change-column-concrete-f-ck-pa` wire twin: the leaf payload `ChangeColumnConcreteFCkPa`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeColumnConcreteFCkPa {
  id: string;
  newConcreteFCkPa: number;
}

export const parseChangeColumnConcreteFCkPa: NormWireReader<ChangeColumnConcreteFCkPa> = normWireObject<ChangeColumnConcreteFCkPa>({ id: normWireRequired(normWireString), newConcreteFCkPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0})) });
