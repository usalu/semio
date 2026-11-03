/** 🔧️ `change-param-label` wire twin: the leaf payload `ChangeParamLabel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeParamLabel {
  id: string;
  newLabel: string;
}

export const parseChangeParamLabel: NormWireReader<ChangeParamLabel> = normWireObject<ChangeParamLabel>({ id: normWireRequired(normWireString), newLabel: normWireRequired(normWireString) });
