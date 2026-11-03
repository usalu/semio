/** 🧵 `change-prestress-steel-fp01k` wire twin: the leaf payload `ChangePrestressSteelFP01k`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePrestressSteelFP01k {
  gradeId: string;
  newFP01k: number;
}

export const parseChangePrestressSteelFP01k: NormWireReader<ChangePrestressSteelFP01k> = normWireObject<ChangePrestressSteelFP01k>({ gradeId: normWireRequired(normWireString), newFP01k: normWireRequired(normWireNumber) });
