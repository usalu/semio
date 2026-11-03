/** 🔧️ `change-param-width` wire twin: the leaf payload `ChangeParamWidth`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeParamWidth {
  id: string;
  newWidth: number;
}

export const parseChangeParamWidth: NormWireReader<ChangeParamWidth> = normWireObject<ChangeParamWidth>({ id: normWireRequired(normWireString), newWidth: normWireRequired(normWireNumber) });
