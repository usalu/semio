/** 🧵 `change-prestress-steel-f-pk` wire twin: the leaf payload `ChangePrestressSteelFPk`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePrestressSteelFPk {
  gradeId: string;
  newFPk: number;
}

export const parseChangePrestressSteelFPk: NormWireReader<ChangePrestressSteelFPk> = normWireObject<ChangePrestressSteelFPk>({ gradeId: normWireRequired(normWireString), newFPk: normWireRequired(normWireNumber) });
