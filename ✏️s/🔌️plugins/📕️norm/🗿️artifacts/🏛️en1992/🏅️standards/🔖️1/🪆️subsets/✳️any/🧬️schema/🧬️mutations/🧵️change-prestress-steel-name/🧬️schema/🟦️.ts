/** 🧵 `change-prestress-steel-name` wire twin: the leaf payload `ChangePrestressSteelName`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePrestressSteelName {
  gradeId: string;
  newName: string;
}

export const parseChangePrestressSteelName: NormWireReader<ChangePrestressSteelName> = normWireObject<ChangePrestressSteelName>({ gradeId: normWireRequired(normWireString), newName: normWireRequired(normWireString) });
