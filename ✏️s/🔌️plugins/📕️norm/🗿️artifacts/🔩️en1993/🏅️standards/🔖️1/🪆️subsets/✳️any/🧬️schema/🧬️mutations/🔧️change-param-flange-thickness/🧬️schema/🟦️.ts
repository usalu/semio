/** 🔧️ `change-param-flange-thickness` wire twin: the leaf payload `ChangeParamFlangeThickness`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeParamFlangeThickness {
  id: string;
  newFlangeThickness: number;
}

export const parseChangeParamFlangeThickness: NormWireReader<ChangeParamFlangeThickness> = normWireObject<ChangeParamFlangeThickness>({ id: normWireRequired(normWireString), newFlangeThickness: normWireRequired(normWireNumber) });
