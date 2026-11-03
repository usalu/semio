/** 🔧️ `change-param-web-thickness` wire twin: the leaf payload `ChangeParamWebThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeParamWebThickness {
  id: string;
  newWebThickness: number;
}

export const parseChangeParamWebThickness: NormWireReader<ChangeParamWebThickness> = normWireObject<ChangeParamWebThickness>({ id: normWireRequired(normWireString), newWebThickness: normWireRequired(normWireNumber) });
